#![no_main]
#![no_std]

use panic_halt as _;

use cortex_m_rt::entry;

use embedded_hal::digital::{InputPin, OutputPin};
use nrf52833_hal::{gpio, pac};

// Debugging via RTT, no serrial port needed
use rtt_target::rtt_init_print;

// use defmt_rtt as _;

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let peripherals = pac::Peripherals::take().unwrap();
    let p0 = gpio::p0::Parts::new(peripherals.P0);

    let _row1 = p0.p0_21.into_push_pull_output(gpio::Level::High);
    let mut col1 = p0.p0_28.into_push_pull_output(gpio::Level::Low);

    let mut button_a = p0.p0_14.into_pullup_input();

    loop {
        if button_a.is_low().unwrap() {
            rtt_target::rprintln!("Button A pressed");
            col1.set_low().unwrap();

            // DEBUG via defmt
            // defmt::info!("Button A pressed");
        } else {
            col1.set_high().unwrap();
            rtt_target::rprintln!("Button A released");

            // DEBUG via defmt
            // defmt::info!("Button A released");
        }
    }
}
