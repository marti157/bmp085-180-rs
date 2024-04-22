#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{clock::ClockControl, delay::Delay, gpio::IO, i2c::I2C, peripherals::Peripherals, prelude::*};

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();

    let clocks = ClockControl::max(system.clock_control).freeze();
    let delay = Delay::new(&clocks);

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    let mut i2c = I2C::new(
        peripherals.I2C0,
        io.pins.gpio3,
        io.pins.gpio2,
        100.kHz(),
        &clocks,
        None,
    );

    esp_println::logger::init_logger_from_env();

    loop {
        log::info!("Hello world!");
        delay.delay(500.millis());
    }
}
