use crate::types::CalibrationData;

/// Calculates temperature from uncompensated temperature value
///
/// ### Returns
///
/// The value of `temperature` and the calculated `b5` coefficient.
pub fn calculate_temperature(calib_data: &CalibrationData, ut: i32) -> (f32, i32) {
    let x1 = (ut as i32 - calib_data.ac6 as i32) * calib_data.ac5 as i32 >> 15;
    let x2 = ((calib_data.mc as i32) << 11) / (x1 + calib_data.md as i32);
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
pub fn calculate_pressure(calib_data: &CalibrationData, oss: u8, b5: i32, up: i32) -> Option<i32> {
    let b6: i32 = b5 as i32 - 4000;
    let x1 = calib_data.b2 as i32 * ((b6 * b6) >> 12) >> 11;
    let x2 = (calib_data.ac2 as i32 * b6) >> 11;
    let x3 = x1 + x2;
    let b3 = (((calib_data.ac1 as i32 * 4 + x3) << oss) + 2) / 4;
    let mut x1 = (calib_data.ac3 as i32 * b6) >> 13;
    let mut x2 = calib_data.b1 as i32 * ((b6 * b6) >> 12) >> 16;
    let x3 = (x1 + x2 + 2) >> 2;
    let b4 = (calib_data.ac4 as u32 * (x3 as u32 + 0x8000)) >> 15;
    let b7 = (up as u32).checked_sub(b3 as u32)? * (50_000 >> oss);
    let p = if b7 < 0x80000000 {
        ((b7 as i64 * 2) / b4 as i64) as i32
    } else {
        (b7 / b4 * 2) as i32
    };
    x1 = (p >> 8).checked_mul(p >> 8)?;
    x1 = (x1 * 3038) >> 16;
    x2 = (-7357 * p) >> 16;
    let pressure = p + ((x1 + x2 + 3791) >> 4);

    Some(pressure)
}

pub fn calculate_altitude(pressure: i32, sea_level_pressure: i32) -> f32 {
    let p_sea_level_ratio: f32 = pressure as f32 / sea_level_pressure as f32;
    let altitude = 44_330.0 * (1.0 - libm::powf(p_sea_level_ratio, 1.0 / 5.255));
    altitude
}

#[cfg(test)]
mod tests {
    use super::*;

    const CALIB_DATA: CalibrationData = CalibrationData {
        ac1: 408,
        ac2: -72,
        ac3: -14383,
        ac4: 32741,
        ac5: 32757,
        ac6: 23153,
        b1: 6190,
        b2: 4,
        mb: -32768,
        mc: -8711,
        md: 2868,
    };

    #[test]
    fn calculates_temperature_correctly() {
        let ut = 27898;
        let (temperature, b5) = calculate_temperature(&CALIB_DATA, ut);

        assert!((temperature - 15.0).abs() < 0.1);
        assert!((b5 - 2399).abs() <= 1)
    }

    #[test]
    fn calculates_pressure_correctly() {
        let oss = 0;
        let b5 = 2399;
        let up = 23_843;
        let pressure = calculate_pressure(&CALIB_DATA, oss, b5, up);

        assert_eq!(pressure.unwrap(), 69964);
    }

    #[test]
    fn calculates_pressure_correctly_oss_1() {
        let oss = 1;
        let b5 = 2399;
        let up = 47_686;
        let pressure = calculate_pressure(&CALIB_DATA, oss, b5, up);

        assert!((pressure.unwrap() - 69964).abs() < 3);
    }

    #[test]
    fn calculates_pressure_correctly_oss_3() {
        let oss = 3;
        let b5 = 2399;
        let up = 190_744;
        let pressure = calculate_pressure(&CALIB_DATA, oss, b5, up);

        assert!((pressure.unwrap() - 69964).abs() < 3);
    }

    #[test]
    fn fails_calculating_pressure_given_invalid_calib_data() {
        let oss = 0;
        let b5 = 2399;
        let up = 23_843;
        let calib_data = CalibrationData {
            ac1: i16::MAX,
            ..CALIB_DATA
        };
        let pressure = calculate_pressure(&calib_data, oss, b5, up);

        assert!(pressure.is_none());
    }

    #[test]
    fn calculates_altitude_correctly() {
        let pressure: i32 = 93_810;
        let sea_level_pressure: i32 = 101_325;
        let altitude = calculate_altitude(pressure, sea_level_pressure);

        assert!((altitude - 645.0).abs() < 0.5);
    }
}
