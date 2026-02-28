use core::cell::RefCell;
use core::sync::atomic::Ordering;
use core::sync::atomic::AtomicU32;
use core::cell::RefMut;

use critical_section::Mutex;

use nrf52833_pac::interrupt;
use nrf52833_pac::{NVIC, RTC0};
use nrf52833_hal::Rtc;
use fugit::{Instant, Duration};

use nrf52833_hal::rtc::RtcInterrupt;
use nrf52833_hal::rtc::RtcCompareReg;

use heapless::{binary_heap::Min, BinaryHeap};

use crate::executor::wake_task;

pub const MAX_DEADLINES: usize = 8;
pub static WAKE_DEADLINES: Mutex<RefCell<BinaryHeap<(u64, usize), Min, MAX_DEADLINES>>> =
    Mutex::new(RefCell::new(BinaryHeap::new()));

pub type TickInstant = Instant<u64, 1, 32_768>; // 32.768 kHz clock
pub type TickDuration = Duration<u64, 1, 32_768>; // 32.768 kHz clock

pub static TICKER: Ticker = Ticker {
    ovf_count: AtomicU32::new(0),
    rtc: Mutex::new(RefCell::new(None)),
};

pub struct Ticker {
    pub ovf_count: AtomicU32,
    pub rtc: Mutex<RefCell<Option<Rtc<RTC0>>>>,
}

impl Ticker {
    pub fn init(rtc0: RTC0, nvic: &mut NVIC) {
        let mut rtc = Rtc::new(rtc0, 0).unwrap();
        rtc.enable_counter();
        #[cfg(feature = "trigger-overflow")]
        {
            rtc.trigger_overflow();
            // wait for the counter to initialize with its close-to-overflow
            // value before going any further, otherwise one of the tasks could
            // schedule a wakeup that will get skipped over when init happens.
            while rtc.get_counter() == 0 {}
        }
        rtc.enable_event(RtcInterrupt::Overflow);
        rtc.enable_interrupt(RtcInterrupt::Overflow, Some(nvic));
        rtc.enable_interrupt(RtcInterrupt::Compare0, Some(nvic));
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

/// Deadlines can only be scheduled in a COMPARE register if they fall within
/// the current overflow-cycle/epoch, and also are not too close to the current
/// counter value. (see nRF52833 Product Specification section 6.20.7)
pub fn schedule_wakeup(
    mut rm_deadlines: RefMut<BinaryHeap<(u64, usize), Min, MAX_DEADLINES>>,
    mut rm_rtc: RefMut<Option<Rtc<RTC0>>>,
) {
    let rtc = rm_rtc.as_mut().unwrap();
    while let Some((deadline, task_id)) = rm_deadlines.peek() {
        let ovf_count = (*deadline >> 24) as u32;
        if ovf_count == TICKER.ovf_count.load(Ordering::Relaxed) {
            let counter = (*deadline & 0xFF_FF_FF) as u32;
            if counter > (rtc.get_counter() + 1) {
                rtc.set_compare(RtcCompareReg::Compare0, counter).ok();
                rtc.enable_event(RtcInterrupt::Compare0);
            } else {
                // Wake now if it's too close or already past,
                // then try again with the next available deadline
                wake_task(*task_id);
                rm_deadlines.pop();
                continue;
            }
        }
        break;
    }
    if rm_deadlines.is_empty() {
        rtc.disable_event(RtcInterrupt::Compare0);
    }
}

#[interrupt]
fn RTC0() {
    critical_section::with(|cs| {
        let mut rm_rtc = TICKER.rtc.borrow_ref_mut(cs);
        let rtc = rm_rtc.as_mut().unwrap();
        if rtc.is_event_triggered(RtcInterrupt::Overflow) {
            rtc.reset_event(RtcInterrupt::Overflow);
            TICKER.ovf_count.fetch_add(1, Ordering::Relaxed);
        }
        if rtc.is_event_triggered(RtcInterrupt::Compare0) {
            rtc.reset_event(RtcInterrupt::Compare0);
        }

        // For OVF & COMPARE0 events, schedule the next wakeup. This should also
        // kill enough clock cycles to allow the event flags to clear.
        // (see nRF52833 Product Specification section 6.1.8)
        schedule_wakeup(WAKE_DEADLINES.borrow_ref_mut(cs), rm_rtc);
    });
}