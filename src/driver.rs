use crate::constants::*;
use crate::logic;
use crate::types::*;
use embedded_hal::delay::DelayNs;
use embedded_hal::i2c::I2c;

impl<I2C, D> BMP<I2C, D>
where
    I2C: I2c,
    D: DelayNs,
{
    /// Creates a new [`BMP`](BMP) driver instance, valid for both the BMP085 and BMP180 modules.
    ///
    /// ### Arguments
    ///
    /// * `i2c` - A properly initialized/configured `embedded-hal` I2C peripheral.
    /// * `delayer` - `embedded-hal` delay for your chip.
    /// * `config` - Driver's initial [`configuration`](Config).
    ///
    /// ### Example
    ///
    /// ```ignore
    /// let i2c = I2C::new(peripherals.I2C1, 100.kHz());
    /// let delay = Delay::new();
    ///
    /// let mut my_bmp = BMP::new(i2c, delay, Default::default());
    /// ```
    pub fn new(i2c: I2C, delayer: D, config: Config) -> Self {
        Self {
            i2c,
            delayer,
            address: config.address,
            calib_data: CalibrationData::default(),
            oss: config.oss,
            sea_level_pressure: config.sea_level_pressure,
        }
    }

    fn read_id(&mut self) -> Result<u8, I2C::Error> {
        let mut id = [0];
        self.i2c.write_read(self.address, &[BMP_ID_REG], &mut id)?;
        Ok(id[0])
    }

    /// Check if the BMP device is properly connected, can be used before initializing the driver.
    ///
    /// ### Arguments
    ///
    /// None
    ///
    /// ### Returns
    ///
    /// `Ok` if the device was detected and validated, `Err(msg)` otherwise, with `msg` containing more information.
    pub fn test_connection(&mut self) -> Result<(), BMPError<I2C::Error>> {
        match self.read_id() {
            Ok(0x55) => Ok(()),
            Err(i2c_err) => Err(BMPError::I2C(i2c_err)),
            _ => Err(BMPError::InvalidDeviceId),
        }
    }

    #[inline(always)]
    fn read16_i2c(&mut self, reg_h: u8, reg_l: u8, rx: &mut [u8; 2]) -> Result<u16, I2C::Error> {
        self.i2c.write_read(self.address, &[reg_h], &mut rx[0..1])?;
        self.i2c.write_read(self.address, &[reg_l], &mut rx[1..2])?;
        Ok(((rx[0] as u16) << 8) | (rx[1] as u16))
    }

    /// Initialize and calibrate the driver.
    ///
    /// ### Arguments
    ///
    /// None
    ///
    /// ### Returns
    ///
    /// `Ok` if the device was properly initialized
    pub fn init(&mut self) -> Result<(), BMPError<I2C::Error>> {
        let mut rx: [u8; 2] = [0, 0];

        self.calib_data.ac1 = self.read16_i2c(BMP_AC1_MSB_REG, BMP_AC1_LSB_REG, &mut rx)? as i16;
        self.calib_data.ac2 = self.read16_i2c(BMP_AC2_MSB_REG, BMP_AC2_LSB_REG, &mut rx)? as i16;
        self.calib_data.ac3 = self.read16_i2c(BMP_AC3_MSB_REG, BMP_AC3_LSB_REG, &mut rx)? as i16;
        self.calib_data.ac4 = self.read16_i2c(BMP_AC4_MSB_REG, BMP_AC4_LSB_REG, &mut rx)?;
        self.calib_data.ac5 = self.read16_i2c(BMP_AC5_MSB_REG, BMP_AC5_LSB_REG, &mut rx)?;
        self.calib_data.ac6 = self.read16_i2c(BMP_AC6_MSB_REG, BMP_AC6_LSB_REG, &mut rx)?;
        self.calib_data.b1 = self.read16_i2c(BMP_B1_MSB_REG, BMP_B1_LSB_REG, &mut rx)? as i16;
        self.calib_data.b2 = self.read16_i2c(BMP_B2_MSB_REG, BMP_B2_LSB_REG, &mut rx)? as i16;
        self.calib_data.mb = self.read16_i2c(BMP_MB_MSB_REG, BMP_MB_LSB_REG, &mut rx)? as i16;
        self.calib_data.mc = self.read16_i2c(BMP_MC_MSB_REG, BMP_MC_LSB_REG, &mut rx)? as i16;
        self.calib_data.md = self.read16_i2c(BMP_MD_MSB_REG, BMP_MD_LSB_REG, &mut rx)? as i16;

        Ok(())
    }

    fn read_uncompensated_temperature(&mut self) -> Result<i32, I2C::Error> {
        let mut rx: [u8; 2] = [0, 0];

        self.i2c.write(self.address, &[BMP_CTRL_MEAS_REG, 0x2E])?;
        self.delayer.delay_ms(5);

        Ok(self.read16_i2c(BMP_OUT_MSB_REG, BMP_OUT_LSB_REG, &mut rx)? as i32)
    }

    fn read_uncompensated_pressure(&mut self) -> Result<i32, I2C::Error> {
        let mut rx_buffer: [u8; 4] = [0; 4];

        self.i2c.write(
            self.address,
            &[BMP_CTRL_MEAS_REG, 0x34 + (self.oss.val() << 6)],
        )?;
        self.delayer.delay_us(match self.oss.val() {
            0 => 4500,
            1 => 7500,
            2 => 13_000,
            3 => 25_500,
            _ => 25_500, // Value shouldn't be outside of range
        });

        self.i2c
            .write_read(self.address, &[BMP_OUT_MSB_REG], &mut rx_buffer[1..2])?;
        self.i2c
            .write_read(self.address, &[BMP_OUT_LSB_REG], &mut rx_buffer[2..3])?;
        self.i2c
            .write_read(self.address, &[BMP_OUT_XLSB_REG], &mut rx_buffer[3..4])?;
        let up = i32::from_be_bytes(rx_buffer) >> (8 - self.oss.val());

        Ok(up)
    }

    /// Measure and calculate temperature from the BMP device.
    ///
    /// ### Arguments
    ///
    /// None
    ///
    /// ### Returns
    ///
    /// `temperature` in degrees Celsius (ºC)
    pub fn read_temperature(&mut self) -> Result<f32, BMPError<I2C::Error>> {
        let ut = self.read_uncompensated_temperature()?;
        let (temperature, _) = logic::calculate_temperature(&self.calib_data, ut);

        Ok(temperature)
    }

    /// Measure and calculate pressure from the BMP device.
    ///
    /// ### Arguments
    ///
    /// None
    ///
    /// ### Returns
    ///
    /// `pressure` in pascals (Pa)
    pub fn read_pressure(&mut self) -> Result<i32, BMPError<I2C::Error>> {
        let ut = self.read_uncompensated_temperature()?;
        let (_, b5) = logic::calculate_temperature(&self.calib_data, ut);
        let up = self.read_uncompensated_pressure()?;

        match logic::calculate_pressure(&self.calib_data, self.oss.val(), b5, up) {
            Some(pressure) => Ok(pressure),
            None => Err(BMPError::InvalidCalibrationData),
        }
    }

    /// Calculate altitude from pressure pressure measurement on the BMP device.
    /// Uses the pressure at sea level to perform the calculation.
    ///
    /// ### Arguments
    ///
    /// None
    ///
    /// ### Returns
    ///
    /// `altitude` in meters (m)
    pub fn read_altitude(&mut self) -> Result<f32, BMPError<I2C::Error>> {
        let pressure = self.read_pressure()?;
        Ok(logic::calculate_altitude(pressure, self.sea_level_pressure))
    }

    /// Trigger a soft reset of the BMP device. It will perform the same sequence as power on reset.
    ///
    /// ### Arguments
    ///
    /// None
    ///
    /// ### Returns
    ///
    /// None
    pub fn soft_reset(&mut self) -> Result<(), BMPError<I2C::Error>> {
        self.i2c.write(self.address, &[BMP_SOFT_RST_REG, 0xB6])?;
        Ok(())
    }

    /// Set the oversampling setting for the driver's measurements.
    /// See [Oss](Oss).
    ///
    /// ### Arguments
    ///
    /// * `oss` - Driver's new [Oss](Oss) setting
    ///
    /// ### Returns
    ///
    /// Nothing
    pub fn set_oversampling_setting(&mut self, oss: Oss) {
        self.oss = oss;
    }

    /// Set the value for pressure at sea level. This will alter the altitude calculation.
    /// Certain atmospheric conditions can cause a variation in atmospheric pressure, so fine-tuning
    /// this value can yield more accurate results.
    ///
    /// ### Arguments
    ///
    /// * `sea_level_pressure` - Value for the pressure at sea level (in Pa).
    ///
    /// ### Returns
    ///
    /// Nothing
    pub fn set_sea_level_pressure(&mut self, sea_level_pressure: i32) {
        assert!(sea_level_pressure > 0);
        self.sea_level_pressure = sea_level_pressure;
    }
}
