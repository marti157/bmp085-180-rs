#![no_std]

mod constants;
use constants::*;
use embedded_hal::delay::DelayNs;
use embedded_hal::i2c::I2c;

#[derive(Debug, Default)]
pub struct CalibrationData {
    ac1: i16,
    ac2: i16,
    ac3: i16,
    ac4: u16,
    ac5: u16,
    ac6: u16,
    b1: i16,
    b2: i16,
    mb: i16,
    mc: i16,
    md: i16,
}

pub struct BMP180<I2C, D> {
    i2c: I2C,
    delayer: D,
    address: u8,
    calib_data: CalibrationData,
}

impl<I2C, D> BMP180<I2C, D>
where
    I2C: I2c,
    D: DelayNs,
{
    pub fn new(i2c: I2C, delayer: D) -> Self {
        Self {
            i2c,
            delayer,
            address: BMP180_DEVICE_ADDR,
            calib_data: CalibrationData::default(),
        }
    }

    fn read_id(&mut self) -> Result<u8, I2C::Error> {
        let mut id = [0];
        self.i2c
            .write_read(self.address, &[BMP180_ID_REG], &mut id)?;
        Ok(id[0])
    }

    pub fn test_connection(&mut self) -> Result<(), &'static str> {
        match self.read_id() {
            Ok(0x55) => Ok(()),
            Err(_) => Err("I2C error"),
            _ => Err("Unrecognized device identifier"),
        }
    }

    pub fn init(&mut self) -> Result<(), I2C::Error> {
        let mut rx = [0];

        self.i2c
            .write_read(self.address, &[BMP180_AC1_MSB_REG], &mut rx)?;
        self.calib_data.ac1 = (rx[0] as i16) << 8;
        self.i2c
            .write_read(self.address, &[BMP180_AC1_LSB_REG], &mut rx)?;
        self.calib_data.ac1 |= rx[0] as i16;
        self.i2c
            .write_read(self.address, &[BMP180_AC2_MSB_REG], &mut rx)?;
        self.calib_data.ac2 = (rx[0] as i16) << 8;
        self.i2c
            .write_read(self.address, &[BMP180_AC2_LSB_REG], &mut rx)?;
        self.calib_data.ac2 |= rx[0] as i16;
        self.i2c
            .write_read(self.address, &[BMP180_AC3_MSB_REG], &mut rx)?;
        self.calib_data.ac3 = (rx[0] as i16) << 8;
        self.i2c
            .write_read(self.address, &[BMP180_AC3_LSB_REG], &mut rx)?;
        self.calib_data.ac3 |= rx[0] as i16;
        self.i2c
            .write_read(self.address, &[BMP180_AC4_MSB_REG], &mut rx)?;
        self.calib_data.ac4 = (rx[0] as u16) << 8;
        self.i2c
            .write_read(self.address, &[BMP180_AC4_LSB_REG], &mut rx)?;
        self.calib_data.ac4 |= rx[0] as u16;
        self.i2c
            .write_read(self.address, &[BMP180_AC5_MSB_REG], &mut rx)?;
        self.calib_data.ac5 = (rx[0] as u16) << 8;
        self.i2c
            .write_read(self.address, &[BMP180_AC5_LSB_REG], &mut rx)?;
        self.calib_data.ac5 |= rx[0] as u16;
        self.i2c
            .write_read(self.address, &[BMP180_AC6_MSB_REG], &mut rx)?;
        self.calib_data.ac6 = (rx[0] as u16) << 8;
        self.i2c
            .write_read(self.address, &[BMP180_AC6_LSB_REG], &mut rx)?;
        self.calib_data.ac6 |= rx[0] as u16;
        self.i2c
            .write_read(self.address, &[BMP180_B1_MSB_REG], &mut rx)?;
        self.calib_data.b1 = (rx[0] as i16) << 8;
        self.i2c
            .write_read(self.address, &[BMP180_B1_LSB_REG], &mut rx)?;
        self.calib_data.b1 |= rx[0] as i16;
        self.i2c
            .write_read(self.address, &[BMP180_B2_MSB_REG], &mut rx)?;
        self.calib_data.b2 = (rx[0] as i16) << 8;
        self.i2c
            .write_read(self.address, &[BMP180_B2_LSB_REG], &mut rx)?;
        self.calib_data.b2 |= rx[0] as i16;
        self.i2c
            .write_read(self.address, &[BMP180_MB_MSB_REG], &mut rx)?;
        self.calib_data.mb = (rx[0] as i16) << 8;
        self.i2c
            .write_read(self.address, &[BMP180_MB_LSB_REG], &mut rx)?;
        self.calib_data.mb |= rx[0] as i16;
        self.i2c
            .write_read(self.address, &[BMP180_MC_MSB_REG], &mut rx)?;
        self.calib_data.mc = (rx[0] as i16) << 8;
        self.i2c
            .write_read(self.address, &[BMP180_MC_LSB_REG], &mut rx)?;
        self.calib_data.mc |= rx[0] as i16;
        self.i2c
            .write_read(self.address, &[BMP180_MD_MSB_REG], &mut rx)?;
        self.calib_data.md = (rx[0] as i16) << 8;
        self.i2c
            .write_read(self.address, &[BMP180_MD_LSB_REG], &mut rx)?;
        self.calib_data.md |= rx[0] as i16;

        Ok(())
    }

    fn read_ut(&mut self) -> Result<i16, I2C::Error> {
        let mut ut: i16;
        let mut rx = [0];

        self.i2c
            .write(self.address, &[BMP180_CTRL_MEAS_REG, 0x2E])?;
        self.delayer.delay_ms(5);

        self.i2c
            .write_read(self.address, &[BMP180_OUT_MSB_REG], &mut rx)?;
        ut = (rx[0] as i16) << 8;
        self.i2c
            .write_read(self.address, &[BMP180_OUT_LSB_REG], &mut rx)?;
        ut |= rx[0] as i16;

        Ok(ut)
    }

    pub fn get_temperature(&mut self) -> Result<f32, I2C::Error> {
        let ut = self.read_ut()?;
        let x1 = (ut as i32 - self.calib_data.ac6 as i32) * self.calib_data.ac5 as i32 >> 15;
        let x2 = ((self.calib_data.mc as i32) << 11) / (x1 + self.calib_data.md as i32);
        let b5 = x1 + x2;
        let temperature = ((b5 + 8) >> 4) as f32 / 10.0;

        Ok(temperature)
    }
}
