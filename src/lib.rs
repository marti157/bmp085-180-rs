#![no_std]

mod constants;
use constants::{BMP180_DEVICE_ADDR, BMP180_ID_REG};
use embedded_hal::i2c::I2c;

pub struct BMP180<I2C> {
    i2c: I2C,
    address: u8,
}

impl<I2C> BMP180<I2C>
where
    I2C: I2c,
{
    pub fn new(i2c: I2C) -> Self {
        Self {
            i2c,
            address: BMP180_DEVICE_ADDR,
        }
    }

    pub fn read_id(&mut self) -> Result<u8, I2C::Error> {
        let mut id = [0];
        self.i2c
            .write_read(self.address, &[BMP180_ID_REG], &mut id)?;
        Ok(id[0])
    }
}
