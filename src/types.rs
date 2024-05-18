use crate::constants::{BMP_DEVICE_ADDR, DEFAULT_SEA_LEVEL_PESSURE};

/// BMP085/BMP180 driver.
pub struct BMP<I2C, D> {
    pub(crate) i2c: I2C,
    pub(crate) delayer: D,
    pub(crate) address: u8,
    pub(crate) calib_data: CalibrationData,
    pub(crate) oss: Oss,
    pub(crate) sea_level_pressure: i32,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CalibrationData {
    pub ac1: i16,
    pub ac2: i16,
    pub ac3: i16,
    pub ac4: u16,
    pub ac5: u16,
    pub ac6: u16,
    pub b1: i16,
    pub b2: i16,
    pub mb: i16,
    pub mc: i16,
    pub md: i16,
}

/// Used to configure the driver's oversampling setting. The higher the value, the more measurements are taken and more accurate the results are,
/// although the measurement will take longer. Only applies to pressure measurements.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
    pub(crate) fn val(&self) -> u8 {
        match *self {
            Oss::LowPower => 0,
            Oss::Standard => 1,
            Oss::HighRes => 2,
            Oss::UltraHighRes => 3,
        }
    }
}

/// Driver configuration, used only during driver initialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Config {
    pub oss: Oss,
    /// Device I2C address, default is 0x77.
    pub address: u8,
    /// Pressure at sea level in Pa, used in altitude calculation.
    /// This value can change slightly under atmospheric conditions, so you can fine-tune it here.
    /// Default is `101_325`.
    pub sea_level_pressure: i32,
}

impl Default for Config {
    /// Default configuration for the BMP085/180 driver.
    /// Oversampling setting defaults to [`LowPower`](Oss::LowPower)
    fn default() -> Self {
        Config {
            oss: Oss::LowPower,
            address: BMP_DEVICE_ADDR,
            sea_level_pressure: DEFAULT_SEA_LEVEL_PESSURE,
        }
    }
}
