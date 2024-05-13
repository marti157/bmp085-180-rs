use crate::constants::*;
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

/// Used to configure the driver's oversampling setting. The higher the value, the more measurements are taken and more accurate the results are,
/// although the measurement will take longer. Only applies to pressure measurements.
pub enum Oss {
    /// Up to 4.5ms measurement time, 1 sample
    LowPower,
    /// Up to 7.5ms measurement time, 2 samples
    Standard,
    /// Up to 13.5ms measurement time, 4 samples
    HighRes,
    /// Up to 25.5ms measurement time, 8 samples
    UltraHighRes,
}

impl Oss {
    fn val(&self) -> u8 {
        match *self {
            Oss::LowPower => 0,
            Oss::Standard => 1,
            Oss::HighRes => 2,
            Oss::UltraHighRes => 3,
        }
    }
}

/// BMP085/BMP180 driver.
pub struct BMP<I2C, D> {
    i2c: I2C,
    delayer: D,
    address: u8,
    calib_data: CalibrationData,
    oss: Oss,
    sea_level_pressure: i32,
}

/// Driver configuration, used only during driver initialization.
pub struct Config {
    pub oss: Oss,
    /// Device I2C address, default is 0x77
    pub address: u8,
}

impl Default for Config {
    /// Default configuration for the BMP085/180 driver.
    /// Oversampling setting defaults to [`LowPower`](Oss::LowPower)
    fn default() -> Self {
        Config {
            oss: Oss::LowPower,
            address: BMP_DEVICE_ADDR,
        }
    }
}

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
            sea_level_pressure: 101_325,
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
    /// * `i2c` - A properly initialized/configured `embedded-hal` I2C peripheral.
    /// * `delayer` - `embedded-hal` delay for your chip.
    /// * `config` - Driver's initial [`configuration`](Config).
    ///
    /// ### Returns
    ///
    /// `Ok` if the device was detected and validated, `Err(msg)` otherwise, with `msg` containing more information.
    pub fn test_connection(&mut self) -> Result<(), &'static str> {
        match self.read_id() {
            Ok(0x55) => Ok(()),
            Err(_) => Err("I2C error"),
            _ => Err("Unrecognized device identifier"),
        }
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
    pub fn init(&mut self) -> Result<(), I2C::Error> {
        let mut rx = [0];

        self.i2c
            .write_read(self.address, &[BMP_AC1_MSB_REG], &mut rx)?;
        self.calib_data.ac1 = (rx[0] as i16) << 8;
        self.i2c
            .write_read(self.address, &[BMP_AC1_LSB_REG], &mut rx)?;
        self.calib_data.ac1 |= rx[0] as i16;
        self.i2c
            .write_read(self.address, &[BMP_AC2_MSB_REG], &mut rx)?;
        self.calib_data.ac2 = (rx[0] as i16) << 8;
        self.i2c
            .write_read(self.address, &[BMP_AC2_LSB_REG], &mut rx)?;
        self.calib_data.ac2 |= rx[0] as i16;
        self.i2c
            .write_read(self.address, &[BMP_AC3_MSB_REG], &mut rx)?;
        self.calib_data.ac3 = (rx[0] as i16) << 8;
        self.i2c
            .write_read(self.address, &[BMP_AC3_LSB_REG], &mut rx)?;
        self.calib_data.ac3 |= rx[0] as i16;
        self.i2c
            .write_read(self.address, &[BMP_AC4_MSB_REG], &mut rx)?;
        self.calib_data.ac4 = (rx[0] as u16) << 8;
        self.i2c
            .write_read(self.address, &[BMP_AC4_LSB_REG], &mut rx)?;
        self.calib_data.ac4 |= rx[0] as u16;
        self.i2c
            .write_read(self.address, &[BMP_AC5_MSB_REG], &mut rx)?;
        self.calib_data.ac5 = (rx[0] as u16) << 8;
        self.i2c
            .write_read(self.address, &[BMP_AC5_LSB_REG], &mut rx)?;
        self.calib_data.ac5 |= rx[0] as u16;
        self.i2c
            .write_read(self.address, &[BMP_AC6_MSB_REG], &mut rx)?;
        self.calib_data.ac6 = (rx[0] as u16) << 8;
        self.i2c
            .write_read(self.address, &[BMP_AC6_LSB_REG], &mut rx)?;
        self.calib_data.ac6 |= rx[0] as u16;
        self.i2c
            .write_read(self.address, &[BMP_B1_MSB_REG], &mut rx)?;
        self.calib_data.b1 = (rx[0] as i16) << 8;
        self.i2c
            .write_read(self.address, &[BMP_B1_LSB_REG], &mut rx)?;
        self.calib_data.b1 |= rx[0] as i16;
        self.i2c
            .write_read(self.address, &[BMP_B2_MSB_REG], &mut rx)?;
        self.calib_data.b2 = (rx[0] as i16) << 8;
        self.i2c
            .write_read(self.address, &[BMP_B2_LSB_REG], &mut rx)?;
        self.calib_data.b2 |= rx[0] as i16;
        self.i2c
            .write_read(self.address, &[BMP_MB_MSB_REG], &mut rx)?;
        self.calib_data.mb = (rx[0] as i16) << 8;
        self.i2c
            .write_read(self.address, &[BMP_MB_LSB_REG], &mut rx)?;
        self.calib_data.mb |= rx[0] as i16;
        self.i2c
            .write_read(self.address, &[BMP_MC_MSB_REG], &mut rx)?;
        self.calib_data.mc = (rx[0] as i16) << 8;
        self.i2c
            .write_read(self.address, &[BMP_MC_LSB_REG], &mut rx)?;
        self.calib_data.mc |= rx[0] as i16;
        self.i2c
            .write_read(self.address, &[BMP_MD_MSB_REG], &mut rx)?;
        self.calib_data.md = (rx[0] as i16) << 8;
        self.i2c
            .write_read(self.address, &[BMP_MD_LSB_REG], &mut rx)?;
        self.calib_data.md |= rx[0] as i16;

        Ok(())
    }

    fn read_ut(&mut self) -> Result<i32, I2C::Error> {
        let mut ut: i32;
        let mut rx: [u8; 1] = [0];

        self.i2c.write(self.address, &[BMP_CTRL_MEAS_REG, 0x2E])?;
        self.delayer.delay_ms(5);

        self.i2c
            .write_read(self.address, &[BMP_OUT_MSB_REG], &mut rx)?;
        ut = (rx[0] as i32) << 8;
        self.i2c
            .write_read(self.address, &[BMP_OUT_LSB_REG], &mut rx)?;
        ut |= rx[0] as i32;

        Ok(ut)
    }

    fn read_up(&mut self) -> Result<i32, I2C::Error> {
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

    /// Calculates temperature from uncompensated temperature value
    ///
    /// ### Returns
    ///
    /// The value of `temperature` and the calculated `b5` coefficient.
    fn calculate_temperature(&self, ut: i32) -> (f32, i32) {
        let x1 = (ut as i32 - self.calib_data.ac6 as i32) * self.calib_data.ac5 as i32 >> 15;
        let x2 = ((self.calib_data.mc as i32) << 11) / (x1 + self.calib_data.md as i32);
        let b5 = x1 + x2;
        let temperature = ((b5 + 8) >> 4) as f32 / 10.0;

        (temperature, b5)
    }

    /// Calculates pressure from uncompensated pressure value
    ///
    /// ### Arguments
    ///
    /// * `b5` - B5 coefficient from temperature calculation.
    /// * `up` - Uncompensated pressure.
    ///
    /// ### Returns
    ///
    /// The value of `pressure`.
    fn calculate_pressure(&self, b5: i32, up: i32) -> i32 {
        let b6: i32 = b5 as i32 - 4000;
        let x1 = self.calib_data.b2 as i32 * ((b6 * b6) >> 12) >> 11;
        let x2 = (self.calib_data.ac2 as i32 * b6) >> 11;
        let x3 = x1 + x2;
        let b3 = (((self.calib_data.ac1 as i32 * 4 + x3) << self.oss.val()) + 2) / 4;
        let mut x1 = (self.calib_data.ac3 as i32 * b6) >> 13;
        let mut x2 = self.calib_data.b1 as i32 * ((b6 * b6) >> 12) >> 16;
        let x3 = (x1 + x2 + 2) >> 2;
        let b4 = (self.calib_data.ac4 as u32 * (x3 as u32 + 0x8000)) >> 15;
        let b7 = (up as u32 - b3 as u32) * (50_000 >> self.oss.val());
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

    /// Measure and calculate temperature from the BMP device.
    ///
    /// ### Arguments
    ///
    /// None
    ///
    /// ### Returns
    ///
    /// `temperature` in degrees Celsius (ºC)
    pub fn get_temperature(&mut self) -> Result<f32, I2C::Error> {
        let ut = self.read_ut()?;
        let (temperature, _) = self.calculate_temperature(ut);

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
    pub fn get_pressure(&mut self) -> Result<i32, I2C::Error> {
        let ut = self.read_ut()?;
        let (_, b5) = self.calculate_temperature(ut);
        let up = self.read_up()?;

        Ok(self.calculate_pressure(b5, up))
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
    pub fn get_altitude(&mut self) -> Result<f32, I2C::Error> {
        let pressure = self.get_pressure()?;
        let p_sea_level_ratio: f32 = pressure as f32 / self.sea_level_pressure as f32;
        let altitude = 44_330.0 * (1.0 - libm::powf(p_sea_level_ratio, 1.0 / 5.255));

        Ok(altitude)
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
}
