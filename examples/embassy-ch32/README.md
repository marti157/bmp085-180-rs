## CH32 with Embassy example

Using [ch32-hal](https://github.com/ch32-rs/ch32-hal/tree/main), which requires Nightly Rust at the time of this publication. Define your chip model the ch32-hal features (`Cargo.toml`).

To flash and monitor SDI output, install [wlink](https://github.com/ch32-rs/wlink). Consult the docs for your chip model's SDI pins.

`$ cargo run --release`
