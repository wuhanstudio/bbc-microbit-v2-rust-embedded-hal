#![no_main]
#![no_std]

use panic_halt as _;
use cortex_m_rt::entry;

use nrf52833_hal as hal;
use rtt_target::{rprintln, rtt_init_print};

use fugit::ExtU64;

pub mod ticker;
pub mod timer;

use ticker::Ticker;
use timer::Timer;

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let p = hal::pac::Peripherals::take().unwrap();

    let ticker = Ticker::new(p.RTC0);
    let mut timer = Timer::new(1000.millis(), &ticker);

    loop {
        if timer.is_ready() {
            rprintln!("Hello, world!");
            timer = Timer::new(1000.millis(), &ticker);
        }
    }
}
