#![no_std]
#![no_main]

use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};

use embassy_time::Timer;
use embassy_executor::Spawner;

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

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    rtt_init_print!();

    let _p = embassy_nrf::init(Default::default());

    spawner.spawn(task_1()).unwrap();
    spawner.spawn(task_2()).unwrap();
}
