#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use panic_halt as _;

use nrf52833_hal::Timer;

// Debugging via RTT, no serrial port needed
use defmt_rtt as _;

#[entry]
fn main() -> ! {
    let p = nrf52833_hal::pac::Peripherals::take().unwrap();
    let mut timer = Timer::new(p.TIMER0);

    loop {
        defmt::info!("Hello, world!");
        timer.delay_ms(1000_u32);
    }
}
