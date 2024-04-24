#![no_std]
#![no_main]

use bmp180_rs::BMP180;
use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl, delay::Delay, gpio::IO, i2c::I2C, peripherals::Peripherals, prelude::*,
};

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();

    let clocks = ClockControl::max(system.clock_control).freeze();
    let delay = Delay::new(&clocks);

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    let i2c = I2C::new(
        peripherals.I2C0,
        io.pins.gpio21,
        io.pins.gpio22,
        100.kHz(),
        &clocks,
        None,
    );

    esp_println::logger::init_logger_from_env();

    let mut bmp180 = BMP180::new(i2c);
    let id = bmp180.read_id().unwrap();
    log::info!("Device ID is 0x{:x}", id);

    delay.delay(500.millis());

    loop {}
}
