#![no_std]

mod constants;

use constants::*;
use embedded_hal::delay::DelayNs;
use embedded_hal::i2c::I2c;

#[derive(Debug, Default)]
struct CalibrationData {
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
    oss: u8,
    sea_level_pressure: i32,
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
            oss: 0,
            sea_level_pressure: 101_325,
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

    fn read_ut(&mut self) -> Result<i32, I2C::Error> {
        let mut ut: i32;
        let mut rx: [u8; 1] = [0];

        self.i2c
            .write(self.address, &[BMP180_CTRL_MEAS_REG, 0x2E])?;
        self.delayer.delay_ms(5);

        self.i2c
            .write_read(self.address, &[BMP180_OUT_MSB_REG], &mut rx)?;
        ut = (rx[0] as i32) << 8;
        self.i2c
            .write_read(self.address, &[BMP180_OUT_LSB_REG], &mut rx)?;
        ut |= rx[0] as i32;

        Ok(ut)
    }

    fn read_up(&mut self) -> Result<i32, I2C::Error> {
        let mut rx_buffer: [u8; 4] = [0; 4];

        self.i2c.write(
            self.address,
            &[BMP180_CTRL_MEAS_REG, 0x34 + (self.oss << 6)],
        )?;
        self.delayer.delay_us(match self.oss {
            0 => 4500,
            1 => 7500,
            2 => 13_000,
            3 => 25_500,
            _ => 25_500, // Value shouldn't be outside of range
        });

        self.i2c
            .write_read(self.address, &[BMP180_OUT_MSB_REG], &mut rx_buffer[1..2])?;
        self.i2c
            .write_read(self.address, &[BMP180_OUT_LSB_REG], &mut rx_buffer[2..3])?;
        self.i2c
            .write_read(self.address, &[BMP180_OUT_XLSB_REG], &mut rx_buffer[3..4])?;
        let up = i32::from_be_bytes(rx_buffer) >> (8 - self.oss);

        Ok(up)
    }

    /// Calculates temperature from uncompensated temperature value
    ///
    /// # Returns
    ///
    /// The value of `temperature` and the calculated `b5` coefficient.
    ///
    fn calculate_temperature(&self, ut: i32) -> (f32, i32) {
        let x1 = (ut as i32 - self.calib_data.ac6 as i32) * self.calib_data.ac5 as i32 >> 15;
        let x2 = ((self.calib_data.mc as i32) << 11) / (x1 + self.calib_data.md as i32);
        let b5 = x1 + x2;
        let temperature = ((b5 + 8) >> 4) as f32 / 10.0;

        (temperature, b5)
    }

    /// Calculates pressure from uncompensated pressure value
    ///
    /// # Arguments
    ///
    /// * `b5` - B5 coefficient from temperature calculation.
    /// * `up` - Uncompensated pressure.
    ///
    /// # Returns
    ///
    /// The value of `pressure`.
    ///
    fn calculate_pressure(&self, b5: i32, up: i32) -> i32 {
        let b6: i32 = b5 as i32 - 4000;
        let x1 = self.calib_data.b2 as i32 * ((b6 * b6) >> 12) >> 11;
        let x2 = (self.calib_data.ac2 as i32 * b6) >> 11;
        let x3 = x1 + x2;
        let b3 = (((self.calib_data.ac1 as i32 * 4 + x3) >> self.oss) + 2) / 4;
        let mut x1 = (self.calib_data.ac3 as i32 * b6) >> 13;
        let mut x2 = self.calib_data.b1 as i32 * ((b6 * b6) >> 12) >> 16;
        let x3 = (x1 + x2 + 2) >> 2;
        let b4 = (self.calib_data.ac4 as u32 * (x3 as u32 + 0x8000)) >> 15;
        let b7 = (up as u32 - b3 as u32) * (50_000 >> self.oss);
        let p = if b7 < 0x80000000 {
            ((b7 as i64 * 2) / b4 as i64) as i32
        } else {
            (b7 as i32 / b4 as i32) * 2
        };
        x1 = (p >> 8) * (p >> 8);
        x1 = (x1 * 3038) >> 16;
        x2 = (-7357 * p) >> 16;
        let pressure = p + ((x1 + x2 + 3791) >> 4);

        pressure
    }

    pub fn get_temperature(&mut self) -> Result<f32, I2C::Error> {
        let ut = self.read_ut()?;
        let (temperature, _) = self.calculate_temperature(ut);

        Ok(temperature)
    }

    pub fn get_pressure(&mut self) -> Result<i32, I2C::Error> {
        let ut = self.read_ut()?;
        let (_, b5) = self.calculate_temperature(ut);
        let up = self.read_up()?;

        Ok(self.calculate_pressure(b5, up))
    }

    pub fn get_altitude(&mut self) -> Result<f32, I2C::Error> {
        let pressure = self.get_pressure()?;
        let p_sea_level_ratio: f32 = pressure as f32 / self.sea_level_pressure as f32;
        let altitude = 44_330.0 * (1.0 - libm::powf(p_sea_level_ratio, 1.0 / 5.255));

        Ok(altitude)
    }

    pub fn set_oversampling_setting(&mut self, oss: u8) {
        assert!(oss <= 3);
        self.oss = oss;
    }
}
