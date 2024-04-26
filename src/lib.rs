#![no_std]

mod constants;
use constants::*;
use embedded_hal::delay::DelayNs;
use embedded_hal::i2c::I2c;

pub struct BMP180<I2C, D> {
    i2c: I2C,
    address: u8,
    delayer: D,
}

impl<I2C, D> BMP180<I2C, D>
where
    I2C: I2c,
    D: DelayNs,
{
    pub fn new(i2c: I2C, delayer: D) -> Self {
        Self {
            i2c,
            address: BMP180_DEVICE_ADDR,
            delayer,
        }
    }

    pub fn read_id(&mut self) -> Result<u8, I2C::Error> {
        let mut id = [0];
        self.i2c
            .write_read(self.address, &[BMP180_ID_REG], &mut id)?;
        Ok(id[0])
    }

    pub fn read_ut(&mut self) -> Result<u16, I2C::Error> {
        let mut ut: u16;
        let mut rx = [0];

        self.i2c
            .write(self.address, &[BMP180_CTRL_MEAS_REG, 0x2E])?;
        self.delayer.delay_ms(5);

        self.i2c
            .write_read(self.address, &[BMP180_OUT_MSB_REG], &mut rx)?;
        ut = (rx[0] as u16) << 8;
        self.i2c
            .write_read(self.address, &[BMP180_OUT_LSB_REG], &mut rx)?;
        ut |= rx[0] as u16;

        Ok(ut)
    }
}
