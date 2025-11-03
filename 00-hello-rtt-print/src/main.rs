#![no_main]
#![no_std]

use panic_halt as _;

use cortex_m_rt::entry;

use embedded_hal::delay::DelayNs;
use nrf52833_hal::Timer;

// Debugging via RTT, no serrial port needed
use rtt_target::rtt_init_print;
use rtt_target::rprintln;

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let p = nrf52833_hal::pac::Peripherals::take().unwrap();
    let mut timer = Timer::new(p.TIMER0);

    loop {
        rprintln!("Hello, world!");
        timer.delay_ms(1000_u32);
    }
}
