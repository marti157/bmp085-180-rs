#![no_std]
#![no_main]

use bmp180_rs::BMP180;
use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl, delay::Delay, gpio::IO, i2c::I2C, peripherals::Peripherals, prelude::*,
};

#[entry]
fn main() -> ! {
    esp_println::logger::init_logger_from_env();
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
    let mut bmp180 = BMP180::new(i2c, delay);

    match bmp180.test_connection() {
        Ok(_) => log::info!("Device connected"),
        Err(msg) => log::error!("{}", msg),
    }

    bmp180.init().unwrap();
    log::info!("Device init");

    delay.delay(500.millis());

    let temp = bmp180.get_temperature().unwrap();
    log::info!("Temperature: {}", temp);

    let pres = bmp180.get_pressure().unwrap();
    log::info!("Pressure: {}", pres);

    loop {}
}
