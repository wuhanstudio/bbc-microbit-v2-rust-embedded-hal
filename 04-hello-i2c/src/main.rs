#![no_main]
#![no_std]

use cortex_m_rt::entry;
use cortex_m::asm::wfi;

use nrf52833_hal::{pac, gpio, twim::*};

// Debugging via RTT, no serrial port needed
use rtt_target::rtt_init_print;

use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    rtt_target::rprintln!("Panic occurred: {}", _info);
    loop {}
}

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let peripherals = pac::Peripherals::take().unwrap();
    

    let p0 = gpio::p0::Parts::new(peripherals.P0);

    let scl = p0.p0_08.into_floating_input().degrade();
    let sda = p0.p0_16.into_floating_input().degrade();

    let mut i2c = Twim::new(peripherals.TWIM0, Pins { scl, sda }, Frequency::K100);

    // Scan I2C bus from address 0x00 to 0x80
    rtt_target::rdbg!("Scanning I2C bus from 0x00 to 0x80...");

    // Print header
    for header in 0x0..0x10 {
        rtt_target::rprint!("{:02x} ", header);
    }
    rtt_target::rprintln!();

    for addr in 0x00..0x80 {
        let byte: [u8; 1] = [0; 1];
        let res = i2c.write(addr, &byte);
        if res.is_ok() {
            rtt_target::rprint!("{:02x}", addr);
        }
        else {
            rtt_target::rprint!("..");
        }
        if addr % 0x10 == 0x0F {
            rtt_target::rprintln!();
        } else {
            rtt_target::rprint!(" ");
        }
    }

    let mut acc = [0u8];
    let mut mag = [0u8];

    const ACCELEROMETER_ADDR: u8 = 0b0011001; // 0x19
    const MAGNETOMETER_ADDR: u8 = 0b0011110; // 0x1e

    // Do NOT use const here (EasyDMA limitation)
    let ACCELEROMETER_ID_REG: u8 = 0x0f;
    let MAGNETOMETER_ID_REG: u8 = 0x4f;

    i2c.write_then_read(ACCELEROMETER_ADDR, &[ACCELEROMETER_ID_REG], &mut acc).unwrap();
    rtt_target::rprintln!("The accelerometer chip's id is: {:#b}", acc[0]);

    i2c.write_then_read(MAGNETOMETER_ADDR, &[MAGNETOMETER_ID_REG], &mut mag).unwrap();
    rtt_target::rprintln!("The magnetometer chip's id is: {:#b}", mag[0]);

    loop {
        wfi();
    }
}
