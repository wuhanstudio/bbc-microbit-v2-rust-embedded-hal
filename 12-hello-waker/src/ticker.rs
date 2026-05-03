use core::cell::RefCell;
use core::sync::atomic::Ordering;
use core::sync::atomic::AtomicU32;

use critical_section::Mutex;

use nrf52833_pac::interrupt;
use nrf52833_pac::{NVIC, RTC0};
use nrf52833_hal::Rtc;
use nrf52833_hal::rtc::RtcInterrupt;
use nrf52833_hal::rtc::RtcCompareReg;

use fugit::{Instant, Duration};

use rtt_target::rprintln;

pub type TickInstant = Instant<u64, 1, 32_768>; // 32.768 kHz clock
pub type TickDuration = Duration<u64, 1, 32_768>; // 32.768 kHz clock

use heapless::Vec;
use core::task::Waker;

const MAX_TIMERS: usize = 8;

pub struct TimerEntry {
    pub deadline: TickInstant,
    pub waker: Waker,
}

pub static TIMERS: Mutex<RefCell<Vec<TimerEntry, MAX_TIMERS>>> =
    Mutex::new(RefCell::new(Vec::new()));

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

    pub fn set_compare(deadline: TickInstant) {
        let now = Ticker::now();
        critical_section::with(|cs| {
            let mut rtc_ref = TICKER.rtc.borrow_ref_mut(cs);
            let rtc = rtc_ref.as_mut().unwrap();

            // rprintln!("Now {}, Register {}", now, deadline);

            let ticks = if deadline <= now {
                now.ticks() + 1
            } else {
                deadline.ticks()
            };

            let compare = (ticks & 0x00FF_FFFF) as u32;

            rtc.set_compare(RtcCompareReg::Compare0, compare).unwrap();
            rtc.enable_event(RtcInterrupt::Compare0);
            rtc.enable_interrupt(RtcInterrupt::Compare0, None);
        });
    }
}

#[interrupt]
// Handle RTC0 interrupt
fn RTC0() {
    let now = Ticker::now();
    rprintln!("RTC Triggered at {}", now.duration_since_epoch().to_millis());

    let mut next_deadline = None;
    critical_section::with(|cs| {
        let mut rm_rtc = TICKER.rtc.borrow_ref_mut(cs);
        let rtc = rm_rtc.as_mut().unwrap();

        if rtc.is_event_triggered(RtcInterrupt::Compare0) {
            rtc.reset_event(RtcInterrupt::Compare0);
            let timers = &mut *TIMERS.borrow_ref_mut(cs);

            let mut i = 0;
            while i < timers.len() {
                if now >= timers[i].deadline {
                    timers[i].waker.wake_by_ref();
                    timers.swap_remove(i);
                } else {
                    i += 1;
                }
            }

            // Schedule next compare
            next_deadline = timers.iter().map(|t| t.deadline).min();
        }

        if rtc.is_event_triggered(RtcInterrupt::Overflow) {
            rtc.reset_event(RtcInterrupt::Overflow);
            TICKER.ovf_count.fetch_add(1, Ordering::Relaxed);
        }
        let _ = rtc.is_event_triggered(RtcInterrupt::Overflow);
    });

    if let Some(next) = next_deadline {
        Ticker::set_compare(next);
    }
}
