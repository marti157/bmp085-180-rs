#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use esp32c3_hal::{clock::ClockControl, i2c::I2C, peripherals::Peripherals, prelude::*, IO};
use esp_backtrace as _;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    let mut i2c0 = I2C::new(
        peripherals.I2C0,
        io.pins.gpio3,
        io.pins.gpio2,
        100u32.kHz(),
        &clocks,
    );

    loop {}
}
