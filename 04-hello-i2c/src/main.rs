#![no_main]
#![no_std]

use cortex_m_rt::entry;
use cortex_m::asm::wfi;
use panic_halt as _;

use embedded_hal::i2c::I2c;
use nrf52833_hal::{pac, gpio, twim::*};

// Debugging via RTT, no serrial port needed
use rtt_target::rtt_init_print;

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let peripherals = pac::Peripherals::take().unwrap();

    let p0 = gpio::p0::Parts::new(peripherals.P0);
    let scl = p0.p0_08.into_floating_input().degrade();
    let sda = p0.p0_16.into_floating_input().degrade();

    let mut twim = Twim::new(peripherals.TWIM0, Pins { scl, sda }, Frequency::K100);

    rtt_target::rprintln!("Start i2c scanning...");
    rtt_target::rprintln!();

    // Print header
    for header in 0x0..0x10 {
        rtt_target::rprint!("{:02x} ", header);
    }
    rtt_target::rprintln!();

    // 0x00 -> 0x7F
    for addr in 0x00..0x80 {
        // Write the empty array and check the slave response.
        let byte: [u8; 1] = [0; 1];
        if twim.write(addr, &byte).is_ok() {
            rtt_target::rprint!("{:02x}", addr);
        } else {
            rtt_target::rprint!("..");
        }
        if addr % 0x10 == 0x0F {
            rtt_target::rprintln!();
        } else {
            rtt_target::rprint!(" ");
        }
    }

    rtt_target::rprintln!();
    rtt_target::rprintln!("Done!");

    const ACCELEROMETER_ADDR: u8 = 0b0011001; // 0x19
    const MAGNETOMETER_ADDR: u8 = 0b0011110; // 0x1E

    const ACCELEROMETER_ID_REG: u8 = 0x0f;
    const MAGNETOMETER_ID_REG: u8 = 0x4f;

    let mut acc = [0_u8];
    let mut mag = [0_u8];

    // First write the address + register onto the bus, then read the chip's responses
    twim.write_read(ACCELEROMETER_ADDR, &[ACCELEROMETER_ID_REG], &mut acc).unwrap();
    twim.write_read(MAGNETOMETER_ADDR, &[MAGNETOMETER_ID_REG], &mut mag).unwrap();

    rtt_target::rprintln!("The accelerometer chip's id is: {:#b}", acc[0]);
    rtt_target::rprintln!("The magnetometer chip's id is: {:#b}", mag[0]);

    loop {
        wfi();
    }
}
