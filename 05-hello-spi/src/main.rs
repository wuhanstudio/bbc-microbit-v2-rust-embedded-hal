#![no_main]
#![no_std]

use panic_halt as _;
use cortex_m::asm::wfi;
use cortex_m_rt::entry;

use nrf52833_hal::{gpio, pac};

// I2C
// use nrf52833_hal::twim::Twim;
// use ssd1306::I2CDisplayInterface;

// SPI
use nrf52833_hal::spim::Spim;

// Delay
use nrf52833_hal::delay::Delay;
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;

// Debugging via RTT, no serrial port needed
use rtt_target::rtt_init_print;

use embedded_graphics::{
    image::{Image, ImageRaw},
    mono_font::{MonoTextStyleBuilder, ascii::FONT_6X10},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};
use ssd1306::{Ssd1306, prelude::*};

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let peripherals = pac::Peripherals::take().unwrap();

    let p0 = gpio::p0::Parts::new(peripherals.P0);
    let p1 = gpio::p1::Parts::new(peripherals.P1);
    
    // Setup the SPI display
    let spiclk = p0.p0_17.into_push_pull_output(gpio::Level::Low).degrade();
    let spimosi = p0.p0_13.into_push_pull_output(gpio::Level::Low).degrade();
    let spimiso = p0.p0_01.into_floating_input().degrade();

    let pins = nrf52833_hal::spim::Pins {
        sck: Some(spiclk),
        miso: Some(spimiso),
        mosi: Some(spimosi),
    };

    let spi = Spim::new(
        peripherals.SPIM0,
        pins,
        nrf52833_hal::spim::Frequency::K500,
        nrf52833_hal::spim::MODE_0,
        0,
    );

    let cs = p1.p1_02.into_push_pull_output(gpio::Level::High).degrade();
    let mut rst = p0.p0_10.into_push_pull_output(gpio::Level::Low).degrade();
    let dc = p0.p0_09.into_push_pull_output(gpio::Level::High).degrade();

    let interface = SPIInterface::new(spi, dc, cs);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();

    let cp = cortex_m::Peripherals::take().unwrap();
    let mut delay = Delay::new(cp.SYST);

    rst.set_low().unwrap();
    delay.delay_ms(10u32);
    rst.set_high().unwrap();

    // Setup the I2C display
    // let scl = p0.p0_26.into_floating_input().degrade();
    // let sda = p1.p1_00.into_floating_input().degrade();
    // let twim = Twim::new(peripherals.TWIM1, nrf52833_hal::twim::Pins { scl, sda }, nrf52833_hal::twim::Frequency::K100);

    // let interface = I2CDisplayInterface::new(twim);
    // let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
    //     .into_buffered_graphics_mode();

    // Initialize the display
    display.init().unwrap();

    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(BinaryColor::On)
        .build();

    Text::with_baseline("Hello", Point::zero(), text_style, Baseline::Top)
        .draw(&mut display)
        .unwrap();

    Text::with_baseline("Rust", Point::new(0, 16), text_style, Baseline::Top)
        .draw(&mut display)
        .unwrap();

    let raw: ImageRaw<BinaryColor> = ImageRaw::new(include_bytes!("./rust.raw"), 64);
    let im = Image::new(&raw, Point::new(32, 0));
    im.draw(&mut display).unwrap();

    display.flush().unwrap();

    loop {
        wfi();
    }
}
