#![no_main]
#![no_std]

use panic_halt as _;

use cortex_m_rt::entry;

use nrf52833_hal::delay::Delay;
use embedded_hal::delay::DelayNs;
// use nrf52833_hal::Timer;

use nrf52833_hal as hal;

use core::fmt::Write;
use hal::{gpio, uarte, uarte::Uarte};

// Debugging via RTT, no serrial port needed
use rtt_target::rtt_init_print;
use rtt_target::rprintln;

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let p = nrf52833_hal::pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    // let mut timer = Timer::new(p.TIMER0);
    let mut delay = Delay::new(cp.SYST);

    let (uart0, cdc_pins) = {
        let p0 = gpio::p0::Parts::new(p.P0);
        let p1 = gpio::p1::Parts::new(p.P1);
        (
            p.UARTE0,
            uarte::Pins {
                txd: p0.p0_06.into_push_pull_output(gpio::Level::High).degrade(),
                rxd: p1.p1_08.into_floating_input().degrade(),
                cts: None,
                rts: None,
            },
        )
    };

    let mut uarte = Uarte::new(
        uart0,
        cdc_pins,
        uarte::Parity::EXCLUDED,
        uarte::Baudrate::BAUD115200,
    );

    loop {
        rprintln!("Hello, world!");
        write!(uarte, "Hello, World!\r\n").unwrap();
    
        delay.delay_ms(1000_u32);
        // timer.delay_ms(1000_u32);
    }
}
