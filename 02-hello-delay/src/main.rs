#![no_main]
#![no_std]

// use panic_halt as _;

use cortex_m_rt::entry;

// Debugging via RTT, no serrial port needed
use rtt_target::rtt_init_print;
use rtt_target::rprintln;

// Delay
use nrf52833_hal::delay::Delay;
use embedded_hal::delay::DelayNs;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    rtt_target::rprintln!("Panic occurred: {}", _info);
    loop {}
}

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let cp = cortex_m::Peripherals::take().unwrap();

    if let Some(_cp) = cortex_m::Peripherals::take() {
        rprintln!("Core peripherals taken successfully.");
    } else {
        rprintln!("Failed to take core peripherals.");
    };

    let mut delay = Delay::new(cp.SYST);

    loop {
        rprintln!("Hello, world!");
        delay.delay_ms(1000_u32);
    }
}
