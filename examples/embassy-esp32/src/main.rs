#![no_std]
#![no_main]

use bmp085_180_rs::BMP;
use embassy_executor::Spawner;
use embassy_time::{Delay, Timer};
use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl,
    embassy,
    gpio::IO,
    i2c::I2C,
    peripherals::{Peripherals, I2C0},
    prelude::*,
    timer::TimerGroup,
    Async,
};

#[embassy_executor::task]
async fn print_temperature_pressure(mut bmp180: BMP<I2C<'static, I2C0, Async>, Delay>) {
    loop {
        let temp = bmp180.read_temperature().await.unwrap();
        esp_println::println!("Temperature: {} ºC", temp);

        let pres = bmp180.read_pressure().await.unwrap();
        esp_println::println!("Pressure: {} Pa", pres);

        let alt = bmp180.read_altitude().await.unwrap();
        esp_println::println!("Altitude: {} m", alt);

        Timer::after_secs(1).await;
    }
}

#[embassy_executor::task]
async fn long_running() {
    loop {
        esp_println::println!("Hello from long running task");

        Timer::after_secs(2).await;
    }
}

#[main]
async fn main(spawner: Spawner) {
    esp_println::logger::init_logger_from_env();
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::max(system.clock_control).freeze();

    let timg0 = TimerGroup::new_async(peripherals.TIMG0, &clocks);
    embassy::init(&clocks, timg0);

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    let i2c = I2C::new_async(
        peripherals.I2C0,
        io.pins.gpio21,
        io.pins.gpio22,
        100.kHz(),
        &clocks,
    );
    let mut bmp180 = BMP::new(i2c, Delay, Default::default());

    match bmp180.test_connection().await {
        Ok(_) => esp_println::println!("Device connected"),
        Err(msg) => {
            log::error!("Device not found: {:?}", msg);
            panic!();
        }
    }

    bmp180.init().await.unwrap();
    esp_println::println!("Device init");

    spawner.spawn(print_temperature_pressure(bmp180)).unwrap();
    spawner.spawn(long_running()).unwrap();
}
