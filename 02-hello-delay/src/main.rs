#![no_main]
#![no_std]

use panic_halt as _;

use cortex_m_rt::entry;

use nrf52833_hal::delay::Delay;
use embedded_hal::delay::DelayNs;

// Debugging via RTT, no serrial port needed
use rtt_target::rtt_init_print;
use rtt_target::rprintln;

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let cp = cortex_m::Peripherals::take().unwrap();
    let mut delay = Delay::new(cp.SYST);

    loop {
        rprintln!("Hello, world!");
        delay.delay_ms(1000_u32);
    }
}
