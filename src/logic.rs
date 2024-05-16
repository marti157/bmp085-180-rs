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
pub fn calculate_pressure(calib_data: &CalibrationData, oss: u8, b5: i32, up: i32) -> i32 {
    let b6: i32 = b5 as i32 - 4000;
    let x1 = calib_data.b2 as i32 * ((b6 * b6) >> 12) >> 11;
    let x2 = (calib_data.ac2 as i32 * b6) >> 11;
    let x3 = x1 + x2;
    let b3 = (((calib_data.ac1 as i32 * 4 + x3) << oss) + 2) / 4;
    let mut x1 = (calib_data.ac3 as i32 * b6) >> 13;
    let mut x2 = calib_data.b1 as i32 * ((b6 * b6) >> 12) >> 16;
    let x3 = (x1 + x2 + 2) >> 2;
    let b4 = (calib_data.ac4 as u32 * (x3 as u32 + 0x8000)) >> 15;
    let b7 = (up as u32 - b3 as u32) * (50_000 >> oss);
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
