use core::cell::RefCell;
use core::sync::atomic::Ordering;
use core::sync::atomic::AtomicU32;

use critical_section::Mutex;

use nrf52833_pac::interrupt;
use nrf52833_pac::{NVIC, RTC0};
use nrf52833_hal::Rtc;
use nrf52833_hal::rtc::RtcInterrupt;

use fugit::{Instant, Duration};

pub type TickInstant = Instant<u64, 1, 32_768>; // 32.768 kHz clock
pub type TickDuration = Duration<u64, 1, 32_768>; // 32.768 kHz clock

static TICKER: Ticker = Ticker{
    ovf_count: AtomicU32::new(0),
    rtc: Mutex::new(RefCell::new(None)),
};

pub struct Ticker {
    ovf_count: AtomicU32,
    rtc: Mutex<RefCell<Option<Rtc<RTC0>>>>,
}

impl Ticker {
    pub fn init(rtc0: RTC0, nvic: &mut NVIC) {
        let mut rtc = Rtc::new(rtc0, 0).unwrap();
        rtc.enable_counter();

        rtc.trigger_overflow();
        while rtc.get_counter() == 0 {};

        rtc.enable_event(RtcInterrupt::Overflow);
        rtc.enable_interrupt(RtcInterrupt::Overflow, Some(nvic));
        critical_section::with(|cs| {
            TICKER.rtc.replace(cs, Some(rtc));
        });
    }

    pub fn now() -> TickInstant {
        let ticks = {
            loop {
                let ovf_before = TICKER.ovf_count.load(Ordering::SeqCst);
                let counter = critical_section::with(|cs| {
                    TICKER.rtc.borrow_ref(cs).as_ref().unwrap().get_counter()
                });
                let ovf = TICKER.ovf_count.load(Ordering::SeqCst);
                if ovf_before == ovf {
                    break (ovf as u64) << 24 | counter as u64;
                }
            }
        };
        TickInstant::from_ticks(ticks)
    }
}

#[interrupt]
// Handle RTC0 interrupt
fn RTC0() {
    critical_section::with(|cs| {
        let mut rm_rtc = TICKER.rtc.borrow_ref_mut(cs);
        let rtc = rm_rtc.as_mut().unwrap();
        if rtc.is_event_triggered(RtcInterrupt::Overflow) {
            rtc.reset_event(RtcInterrupt::Overflow);
            TICKER.ovf_count.fetch_add(1, Ordering::Relaxed);
        }

        let _ = rtc.is_event_triggered(RtcInterrupt::Overflow);
    });
}
