#![no_std]
#![no_main]

use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};

use embassy_time::Timer;
use embassy_executor::Spawner;
use embassy_nrf::gpio;

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
        if (x % 5) == 0 {
            Poll::Ready(())
        } else {
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

#[embassy_executor::task]
async fn task_1() {
    let count = CountFuture;
    loop {
        count.await;
        rprintln!("[task_1] Hello Count {}", TICKS.load(Ordering::Relaxed));
        Timer::after_millis(2000).await;
    }
}

#[embassy_executor::task]
async fn task_2() {
    loop {
        rprintln!("(task_2) Hello World");
        Timer::after_millis(6000).await;
    }
}

#[embassy_executor::task]
async fn task_led(mut rows: [gpio::Output<'static>; 5], mut cols: [gpio::Output<'static>; 5]) {
    loop {
        // Led On
        for row in rows.iter_mut() {
            row.set_high();
        }
        for col in cols.iter_mut() {
            col.set_low();
        }
        Timer::after_millis(1000).await;

        // Led Off
        for row in rows.iter_mut() {
            row.set_high();
        }
        for col in cols.iter_mut() {
            col.set_high();
        }
        Timer::after_millis(1000).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    rtt_init_print!();

    let p = embassy_nrf::init(Default::default());

    let r1 = gpio::Output::new(p.P0_21, gpio::Level::Low, gpio::OutputDrive::Standard);
    let r2 = gpio::Output::new(p.P0_22, gpio::Level::High, gpio::OutputDrive::Standard);
    let r3 = gpio::Output::new(p.P0_15, gpio::Level::High, gpio::OutputDrive::Standard);
    let r4 = gpio::Output::new(p.P0_24, gpio::Level::High, gpio::OutputDrive::Standard);
    let r5 = gpio::Output::new(p.P0_19, gpio::Level::High, gpio::OutputDrive::Standard);
    let rows = [r1, r2, r3, r4, r5];

    let c1 = gpio::Output::new(p.P0_28, gpio::Level::Low, gpio::OutputDrive::Standard);
    let c2 = gpio::Output::new(p.P0_11, gpio::Level::Low, gpio::OutputDrive::Standard);
    let c3 = gpio::Output::new(p.P0_31, gpio::Level::Low, gpio::OutputDrive::Standard);
    let c4 = gpio::Output::new(p.P1_05, gpio::Level::Low, gpio::OutputDrive::Standard);
    let c5 = gpio::Output::new(p.P0_30, gpio::Level::Low, gpio::OutputDrive::Standard);
    let cols = [c1, c2, c3, c4, c5];

    spawner.spawn(task_1()).unwrap();
    spawner.spawn(task_2()).unwrap();
    spawner.spawn(task_led(rows, cols)).unwrap();
}
