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
use crate::blinky::Event;

use statig::prelude::*;

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let p = hal::pac::Peripherals::take().unwrap();

    // Enable the low-power/low-frequency clock which is required by the RTC.
    let clocks = hal::clocks::Clocks::new(p.CLOCK);
    clocks.start_lfclk();

    let mut cp = hal::pac::CorePeripherals::take().unwrap();

    Ticker::init(p.RTC0, &mut cp.NVIC);
    let mut blinky_task = Blinky::default().uninitialized_state_machine().init();

    rprintln!("Waiting for events at {} ms", Ticker::now().duration_since_epoch().to_millis());
    loop {
        if blinky_task.timer.is_ready() {
            let time = Ticker::now();
            rprintln!("Blinky Event triggered at {} ticks, {} ms", time.ticks(), time.duration_since_epoch().to_millis());
            blinky_task.handle(&Event::TimerElapsed);
        }
    }
}
