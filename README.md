# Rust driver for BMP085 & BMP180

[![Build](https://github.com/marti157/bmp085-180-rs/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/marti157/bmp085-180-rs/actions/workflows/rust.yml)
[![Crate](https://img.shields.io/crates/v/bmp085-180-rs.svg)](https://crates.io/crates/bmp085-180-rs)
[![Documentation](https://docs.rs/bmp085-180-rs/badge.svg)](https://docs.rs/bmp085-180-rs)

`bmp085-180-rs` is a `no-std` Rust driver implementation for the BMP085 & BMP180 sensors using `embedded-hal` traits, for integration with most target platforms.

The Bosch [BMP085](https://www.sparkfun.com/datasheets/Components/General/BST-BMP085-DS000-05.pdf) & [BMP180](https://cdn-shop.adafruit.com/datasheets/BST-BMP180-DS000-09.pdf) are barometric pressure & temperature sensor modules. Both are no longer in production.

### Installation

```sh
$ cargo add bmp085-180-rs
```

### Features

| Feature | Description               |
| ------- | ------------------------- |
| `sync`  | Blocking transactions     |
| `async` | Non-blocking transactions |

### Usage

If you require `async` support, make sure to enable the following feature in your `Cargo.toml`:

```toml
bmp085-180-rs = { version = "1.0.0", features = [ "async" ] }
```

The default is `sync`.

See [examples](https://github.com/marti157/bmp085-180-rs/tree/main/examples) for both blocking & async usage with esp32.

### License

The MIT License (MIT). Please see [LICENSE](https://github.com/marti157/bmp085-180-rs/tree/main/LICENSE) for more information.
