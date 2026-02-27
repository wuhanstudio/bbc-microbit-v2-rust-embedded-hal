#![no_std]
#![no_main]

use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};

use embassy_time::Timer;
use embassy_executor::Spawner;

#[embassy_executor::task]
async fn task_1() {
    let mut i = 0;
    loop {
        rprintln!("[task_1] Hello Task {}", i);
        i = i + 1;
        Timer::after_millis(2000).await;
    }
}

#[embassy_executor::task]
async fn task_2() {
    let mut i = 0;
    loop {
        rprintln!("[task_2] Hello World {}", i);
        i = i + 1;
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
