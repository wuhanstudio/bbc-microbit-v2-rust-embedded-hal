#![no_main]
#![no_std]

use panic_halt as _;
use cortex_m_rt::entry;
use nrf52833_hal as hal;
use nrf52833_hal::gpio;

use embedded_hal::digital::OutputPin;
use fugit::ExtU64;

use rtt_target::{rprintln, rtt_init_print};

use core::future::Future;
use core::slice::RSplitMut;
use core::task::{Context, Poll};
use core::sync::atomic::{AtomicU32, AtomicBool, Ordering};
use core::pin::Pin;
use core::pin::pin;

pub mod ticker;
pub mod timer;
pub mod executor;

use crate::ticker::Ticker;
use crate::timer::delay;
use crate::executor::Task;

static TICKS: AtomicU32 = AtomicU32::new(0);

struct CountFuture;

impl Future for CountFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
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
        CountFuture {}.await;
        rprintln!("[task_1] Hello Count {}", TICKS.load(Ordering::Relaxed));
    }
}

async fn task_2() {
    loop {
        delay(2000.millis()).await;
        rprintln!("[task_2] Hello World");
    }
}

async fn task_led<P: OutputPin, const N: usize>(mut rows: [P; N], mut cols: [P; N]) {
    loop {
        for row in rows.iter_mut() {
            row.set_high().unwrap();
        }

        for col in cols.iter_mut() {
            col.set_low().unwrap();
        }

        delay(1000.millis()).await;

        for row in rows.iter_mut() {
            row.set_high().unwrap();
        }

        for col in cols.iter_mut() {
            col.set_high().unwrap();
        }

        delay(1000.millis()).await;
    }
}

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let p = hal::pac::Peripherals::take().unwrap();
    let mut cp = hal::pac::CorePeripherals::take().unwrap();

    let clocks = hal::clocks::Clocks::new(p.CLOCK);
    clocks.start_lfclk();

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

    let t1 = Task {
        future: pin!(task_1()),
        ready: AtomicBool::new(true),
    };

    let t2 = Task {
        future: pin!(task_2()),
        ready: AtomicBool::new(true),
    };

    let t3 = Task {
        future: pin!(task_led(rows, cols)),
        ready: AtomicBool::new(true),
    };

    executor::run_tasks(&mut [t1, t2, t3]);
}
