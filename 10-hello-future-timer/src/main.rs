#![no_main]
#![no_std]

use panic_halt as _;
use cortex_m_rt::entry;
use nrf52833_hal as hal;

use rtt_target::{rprintln, rtt_init_print};

pub mod ticker;
pub mod timer;

use crate::ticker::Ticker;

pub mod blinky;
use crate::blinky::Blinky;
use crate::blinky::blinky_poll;

use statig::prelude::*;
use cassette::Cassette;

use core::pin::Pin;
use core::task::Poll;
use core::task::Context;
use core::sync::atomic::{AtomicU32, Ordering};

static TICKS: AtomicU32 = AtomicU32::new(0);

#[derive(Clone, Copy)]
struct CountFuture;

impl Future for CountFuture {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let x = TICKS.fetch_add(1, Ordering::SeqCst);
        if (x % 30000) == 0 {
            Poll::Ready(())
        } else {
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

async fn task_1() {
    let count = CountFuture;
    loop {
        count.await;
        rprintln!("[task_1] Hello Count {}", TICKS.load(Ordering::Relaxed));
    }
}

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let p = hal::pac::Peripherals::take().unwrap();

    // Enable the low-power/low-frequency clock which is required by the RTC.
    let clocks = hal::clocks::Clocks::new(p.CLOCK);
    clocks.start_lfclk();

    let mut cp = hal::pac::CorePeripherals::take().unwrap();

    Ticker::init(p.RTC0, &mut cp.NVIC);
    let mut blinky_task: InitializedStateMachine<Blinky> = Blinky::default().uninitialized_state_machine().init();

    rprintln!("Waiting for events at {} ms", Ticker::now().duration_since_epoch().to_millis());

    let t1 = core::pin::pin!(task_1());
    let mut cm = Cassette::new(t1);

    loop {
        if let Some(x) = cm.poll_on() {
            rprintln!("Done!: `{:?}`", x);
        }
        blinky_poll(&mut blinky_task);
    }
}
