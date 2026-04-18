#![no_main]
#![no_std]

use panic_halt as _;
use cortex_m_rt::entry;
use nrf52833_hal as hal;
use nrf52833_hal::gpio;

use rtt_target::{rprintln, rtt_init_print};

pub mod ticker;
pub mod timer;

use crate::ticker::Ticker;
use crate::timer::delay;

use fugit::ExtU64;

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

// #[derive(Clone, Copy)]
struct CountFuture;

impl Future for CountFuture {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Not allowed here due to Pin
        // let _moved = *self;

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
    loop {
        let count = CountFuture{};
        count.await;
        rprintln!("[task_1] Hello Count {}", TICKS.load(Ordering::Relaxed));
    }
}

async fn task_2() {
    loop {
        delay(2000.millis()).await;
        rprintln!("[task_2] Hello World");
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
    let p0 = gpio::p0::Parts::new(p.P0);
    let p1 = gpio::p1::Parts::new(p.P1);

    let r1 = p0.p0_21.into_push_pull_output(gpio::Level::High).degrade();
    let r2 = p0.p0_22.into_push_pull_output(gpio::Level::High).degrade();
    let r3 = p0.p0_15.into_push_pull_output(gpio::Level::High).degrade();
    let r4 = p0.p0_24.into_push_pull_output(gpio::Level::High).degrade();
    let r5 = p0.p0_19.into_push_pull_output(gpio::Level::High).degrade();
    let rows = [r1, r2, r3, r4, r5];

    let c1 = p0.p0_28.into_push_pull_output(gpio::Level::Low).degrade();
    let c2 = p0.p0_11.into_push_pull_output(gpio::Level::Low).degrade();
    let c3 = p0.p0_31.into_push_pull_output(gpio::Level::Low).degrade();
    let c4 = p1.p1_05.into_push_pull_output(gpio::Level::Low).degrade();
    let c5 = p0.p0_30.into_push_pull_output(gpio::Level::Low).degrade();
    let cols = [c1, c2, c3, c4, c5];

    Ticker::init(p.RTC0, &mut cp.NVIC);
    let mut blinky_task: InitializedStateMachine<Blinky<_, 5>> = Blinky::new(rows, cols).uninitialized_state_machine().init();

    rprintln!("Waiting for events at {} ms", Ticker::now().duration_since_epoch().to_millis());

    let t1 = core::pin::pin!(task_1());
    let mut cm = Cassette::new(t1);
    
    let t2 = core::pin::pin!(task_2());
    let mut cm2 = Cassette::new(t2);

    loop {
        cm.poll_on();
        cm2.poll_on();
        blinky_poll(&mut blinky_task);
    }
}
