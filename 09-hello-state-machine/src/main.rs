#![no_main]
#![no_std]

use panic_halt as _;
use cortex_m_rt::entry;

use nrf52833_hal as hal;
use nrf52833_hal::gpio;
use nrf52833_hal::uarte;

use rtt_target::{rprintln, rtt_init_print};

pub mod ticker;
use crate::ticker::Ticker;

pub mod timer;

pub mod blinky;
use crate::blinky::Blinky;
use crate::blinky::blinky_poll;

pub mod hello;
use crate::hello::Hello;
use crate::hello::hello_poll;

use statig::prelude::*;

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let p = hal::pac::Peripherals::take().unwrap();

    // Enable the low-power/low-frequency clock which is required by the RTC.
    let clocks = hal::clocks::Clocks::new(p.CLOCK);
    clocks.start_lfclk();

    let mut cp = hal::pac::CorePeripherals::take().unwrap();
    let p0 = gpio::p0::Parts::new(p.P0);
    let p1 = gpio::p1::Parts::new(p.P1);

    let r1 = p0.p0_21.into_push_pull_output(gpio::Level::High).degrade();
    let r2 = p0.p0_22.into_push_pull_output(gpio::Level::High).degrade();
    let r3 = p0.p0_15.into_push_pull_output(gpio::Level::High).degrade();
    let r4 = p0.p0_24.into_push_pull_output(gpio::Level::High).degrade();
    let r5 = p0.p0_19.into_push_pull_output(gpio::Level::High).degrade();
    let rows = [r1, r2, r3, r4, r5];

    let c1 = p0.p0_28.into_push_pull_output(gpio::Level::Low).degrade();
    let c2: gpio::Pin<gpio::Output<gpio::PushPull>> = p0.p0_11.into_push_pull_output(gpio::Level::Low).degrade();
    let c3: gpio::Pin<gpio::Output<gpio::PushPull>> = p0.p0_31.into_push_pull_output(gpio::Level::Low).degrade();
    let c4: gpio::Pin<gpio::Output<gpio::PushPull>> = p1.p1_05.into_push_pull_output(gpio::Level::Low).degrade();
    let c5: gpio::Pin<gpio::Output<gpio::PushPull>> = p0.p0_30.into_push_pull_output(gpio::Level::Low).degrade();
    let cols = [c1, c2, c3, c4, c5];

    Ticker::init(p.RTC0, &mut cp.NVIC);
    let mut blinky_task: InitializedStateMachine<Blinky<_, 5>> = Blinky::new(rows, cols).uninitialized_state_machine().init();

    let (uart0, cdc_pins) = {
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

    let uarte = uarte::Uarte::new(
        uart0,
        cdc_pins,
        uarte::Parity::EXCLUDED,
        uarte::Baudrate::BAUD115200,
    );

    let mut hello_task: InitializedStateMachine<Hello> = Hello::new(uarte).uninitialized_state_machine().init();
    rprintln!("Waiting for events at {} ms", Ticker::now().duration_since_epoch().to_millis());
    loop {
        blinky_poll(&mut blinky_task);
        hello_poll(&mut hello_task);
    }
}
