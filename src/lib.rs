//! This crate provides a driver for both the [BMP085](https://www.sparkfun.com/datasheets/Components/General/BST-BMP085-DS000-05.pdf) and
//! [BMP180](https://cdn-shop.adafruit.com/datasheets/BST-BMP180-DS000-09.pdf) digital pressure sensors, which additionally provide the ability to measure temperature.
//!
//! The driver implements both `embedded-hal` and `embedded-hal-async` traits for ease of use with any compatible chipset HAL.
//!
//! ### Features
//!
//! The default working mode is `sync` (blocking). To use non-blocking calls, enable the `async` feature:
//! ```toml
//! bmp085-180-rs = { version = "1.0.0", features = [ "async" ] }
//! ```
//!
//! ### Usage
//!
//! See the following driver methods:
//!
//! #### [`BMP::read_temperature`](BMP::read_temperature)
//!
//! #### [`BMP::read_pressure`](BMP::read_pressure)
//!
//! #### [`BMP::read_altitude`](BMP::read_altitude)

#![no_std]

#[cfg(all(feature = "async", feature = "sync"))]
compile_error!("Both `sync` and `async` features cannot be enabled.");

mod constants;
mod driver;
mod logic;
mod types;

pub use types::{BMPError, Config, Oss, BMP};
