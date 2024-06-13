// See examples at https://github.com/ch32-rs/ch32-hal/
fn main() {
    println!("cargo:rustc-link-arg-bins=-Tlink.x");
}
