//! This crate provides a driver for both the [BMP085](https://www.sparkfun.com/datasheets/Components/General/BST-BMP085-DS000-05.pdf) and
//! [BMP180](https://cdn-shop.adafruit.com/datasheets/BST-BMP180-DS000-09.pdf) digital pressure sensors, which additionally provide the ability to measure temperature.
//!
//! The driver implements `embedded-hal` traits for ease of use with any compatible chipset HAL. Only blocking I2C methods are implemented, for now.
//!
//! See the following driver methods:
//!
//! #### [`BMP::read_temperature`](BMP::read_temperature)
//!
//! #### [`BMP::read_pressure`](BMP::read_pressure)
//!
//! #### [`BMP::read_altitude`](BMP::read_altitude)

#![no_std]

mod constants;
mod driver;
mod logic;
mod types;

pub use types::{Config, Oss, BMP};
