#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![feature(impl_trait_in_assoc_type)]

use bmp085_180_rs::BMP;
use ch32_hal as hal;
use embassy_executor::Spawner;
use embassy_time::{Delay, Timer};
use hal::{bind_interrupts, i2c::I2c, mode::Blocking, peripherals, println, time::Hertz};
use qingke::riscv;

bind_interrupts!(struct Irqs {
    I2C1_EV => hal::i2c::EventInterruptHandler<peripherals::I2C1>;
    I2C1_ER => hal::i2c::ErrorInterruptHandler<peripherals::I2C1>;
});

#[embassy_executor::task]
async fn print_temperature_pressure(
    mut bmp180: BMP<I2c<'static, peripherals::I2C1, Blocking>, Delay>,
) {
    loop {
        let temp = bmp180.read_temperature().unwrap();
        println!("Temperature: {} ºC", temp);

        let pres = bmp180.read_pressure().unwrap();
        println!("Pressure: {} Pa", pres);

        let alt = bmp180.read_altitude().unwrap();
        println!("Altitude: {} m", alt);

        Timer::after_secs(1).await;
    }
}

#[embassy_executor::task]
async fn long_running() {
    loop {
        println!("Hello from long running task");

        Timer::after_secs(2).await;
    }
}

#[embassy_executor::main(entry = "qingke_rt::entry")]
async fn main(spawner: Spawner) {
    hal::debug::SDIPrint::enable();
    let mut config = hal::Config::default();
    config.rcc = hal::rcc::Config::SYSCLK_FREQ_96MHZ_HSI;
    let p = hal::init(config);
    hal::embassy::init();

    // If we don't wait, embassy time doesn't set the alarm properly and eventually panics
    // due to multiplication overflow
    riscv::asm::delay(1000000);

    println!("Embassy initialized");

    let scl = p.PB6;
    let sda = p.PB7;
    // TODO: Use I2c async driver
    let i2c = I2c::new_blocking(p.I2C1, scl, sda, Hertz::hz(400_000), Default::default());
    let mut bmp180 = BMP::new(i2c, Delay, Default::default());

    match bmp180.test_connection() {
        Ok(_) => println!("Device connected"),
        Err(_) => {
            println!("Device not found");
            panic!();
        }
    }

    bmp180.init().unwrap();
    println!("Device init");

    spawner.spawn(print_temperature_pressure(bmp180)).unwrap();
    spawner.spawn(long_running()).unwrap();
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("\nPanic: {info}");

    loop {}
}
