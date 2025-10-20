#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::digital::{InputPin, OutputPin};
use panic_halt as _;

use nrf52833_hal::{gpio, pac};

// Debugging via RTT, no serrial port needed
use rtt_target::rtt_init_print;
use rtt_target::rprintln;

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let peripherals = pac::Peripherals::take().unwrap();
    let p0 = gpio::p0::Parts::new(peripherals.P0);

    let _row1 = p0.p0_21.into_push_pull_output(gpio::Level::High);
    let mut _col1 = p0.p0_28.into_push_pull_output(gpio::Level::Low);

    let mut button_a = p0.p0_14.into_pullup_input();

    loop {
        if button_a.is_low().unwrap() {
            rprintln!("Button A pressed");
            _col1.set_low().unwrap();
        } else {
            rprintln!("Button A released");
            _col1.set_high().unwrap();
        }
    }
}
