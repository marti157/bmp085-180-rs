use bmp085_180_rs::{BMPError, Config, Oss, BMP};
use embedded_hal::i2c::ErrorKind;
use embedded_hal_mock::eh1::delay::NoopDelay;
use embedded_hal_mock::eh1::i2c::{Mock as I2cMock, Transaction as I2cTransaction};

fn get_init_coeficient_expectations() -> Vec<I2cTransaction> {
    (0xAA..=0xBF)
        .map(|value| I2cTransaction::write_read(0x77, vec![value], vec![0x12]))
        .collect::<Vec<_>>()
}

#[test]
fn test_connection_ok_with_valid_id() {
    let expectations = [I2cTransaction::write_read(0x77, vec![0xD0], vec![0x55])];
    let mut i2c = I2cMock::new(&expectations);
    let mut bmp = BMP::new(i2c.clone(), NoopDelay, Default::default());

    assert_eq!(bmp.test_connection(), Ok(()));
    i2c.done();
}

#[test]
fn test_connection_fails_with_invalid_id() {
    let expectations = [I2cTransaction::write_read(0x77, vec![0xD0], vec![0xFF])];
    let mut i2c = I2cMock::new(&expectations);
    let mut bmp = BMP::new(i2c.clone(), NoopDelay, Default::default());

    assert_eq!(bmp.test_connection(), Err(BMPError::InvalidDeviceId));
    i2c.done();
}

#[test]
fn test_connection_fails_if_i2c_error() {
    let expectations =
        [I2cTransaction::write_read(0x77, vec![0xD0], vec![0xFF]).with_error(ErrorKind::Other)];
    let mut i2c = I2cMock::new(&expectations);
    let mut bmp = BMP::new(i2c.clone(), NoopDelay, Default::default());

    assert_eq!(bmp.test_connection(), Err(BMPError::I2C(ErrorKind::Other)));
    i2c.done();
}

#[test]
fn init_ok_given_calibration_values() {
    let expectations: Vec<I2cTransaction> = (0xAA..=0xBF)
        .map(|value| I2cTransaction::write_read(0x77, vec![value], vec![0xFF]))
        .collect::<Vec<_>>();
    let mut i2c = I2cMock::new(&expectations);
    let mut bmp = BMP::new(i2c.clone(), NoopDelay, Default::default());

    assert_eq!(bmp.init(), Ok(()));
    i2c.done();
}

#[test]
fn init_fails_if_i2c_error() {
    let expectations =
        [I2cTransaction::write_read(0x77, vec![0xAA], vec![0xFF]).with_error(ErrorKind::Bus)];
    let mut i2c = I2cMock::new(&expectations);
    let mut bmp = BMP::new(i2c.clone(), NoopDelay, Default::default());

    assert_eq!(bmp.init(), Err(BMPError::I2C(ErrorKind::Bus)));
    i2c.done();
}

#[test]
fn read_temperature_ok_given_readings() {
    let expectations = [
        get_init_coeficient_expectations(),
        vec![
            I2cTransaction::write(0x77, vec![0xF4, 0x2E]),
            I2cTransaction::write_read(0x77, vec![0xF6], vec![0xFF]),
            I2cTransaction::write_read(0x77, vec![0xF7], vec![0xFF]),
        ],
    ]
    .concat();
    let mut i2c = I2cMock::new(&expectations);
    let mut bmp = BMP::new(i2c.clone(), NoopDelay, Default::default());

    bmp.init().unwrap();
    let temperature = bmp.read_temperature();

    assert!(temperature.is_ok());
    assert!(temperature.unwrap().is_finite());
    i2c.done();
}

#[test]
fn read_temperature_fails_if_i2c_error() {
    let expectations = [
        get_init_coeficient_expectations(),
        vec![I2cTransaction::write(0x77, vec![0xF4, 0x2E]).with_error(ErrorKind::Overrun)],
    ]
    .concat();
    let mut i2c = I2cMock::new(&expectations);
    let mut bmp = BMP::new(i2c.clone(), NoopDelay, Default::default());

    bmp.init().unwrap();

    assert_eq!(
        bmp.read_temperature(),
        Err(BMPError::I2C(ErrorKind::Overrun))
    );
    i2c.done();
}

#[test]
fn read_pressure_ok_given_readings() {
    let expectations = [
        get_init_coeficient_expectations(),
        vec![
            I2cTransaction::write(0x77, vec![0xF4, 0x2E]),
            I2cTransaction::write_read(0x77, vec![0xF6], vec![0xFF]),
            I2cTransaction::write_read(0x77, vec![0xF7], vec![0xFF]),
            I2cTransaction::write(0x77, vec![0xF4, 0x34]),
            I2cTransaction::write_read(0x77, vec![0xF6], vec![0x33]),
            I2cTransaction::write_read(0x77, vec![0xF7], vec![0x38]),
            I2cTransaction::write_read(0x77, vec![0xF8], vec![0x00]),
        ],
    ]
    .concat();
    let mut i2c = I2cMock::new(&expectations);
    let mut bmp = BMP::new(i2c.clone(), NoopDelay, Default::default());

    bmp.init().unwrap();
    let pressure = bmp.read_pressure();

    assert!(pressure.is_ok());
    assert!(pressure.unwrap().is_positive());
    i2c.done();
}

#[test]
fn read_pressure_highest_resolution_ok_given_readings() {
    let expectations = [
        get_init_coeficient_expectations(),
        vec![
            I2cTransaction::write(0x77, vec![0xF4, 0x2E]),
            I2cTransaction::write_read(0x77, vec![0xF6], vec![0xFF]),
            I2cTransaction::write_read(0x77, vec![0xF7], vec![0xFF]),
            I2cTransaction::write(0x77, vec![0xF4, 0xF4]),
            I2cTransaction::write_read(0x77, vec![0xF6], vec![0x33]),
            I2cTransaction::write_read(0x77, vec![0xF7], vec![0x38]),
            I2cTransaction::write_read(0x77, vec![0xF8], vec![0x80]),
        ],
    ]
    .concat();
    let mut i2c = I2cMock::new(&expectations);
    let bmp_config = Config {
        oss: Oss::UltraHighRes,
        ..Config::default()
    };
    let mut bmp = BMP::new(i2c.clone(), NoopDelay, bmp_config);

    bmp.init().unwrap();
    let pressure = bmp.read_pressure();

    assert!(pressure.is_ok());
    assert!(pressure.unwrap().is_positive());
    i2c.done();
}

#[test]
fn read_pressure_fails_if_i2c_error() {
    let expectations = [
        get_init_coeficient_expectations(),
        vec![
            I2cTransaction::write(0x77, vec![0xF4, 0x2E]),
            I2cTransaction::write_read(0x77, vec![0xF6], vec![0xFF]),
            I2cTransaction::write_read(0x77, vec![0xF7], vec![0xFF]),
            I2cTransaction::write(0x77, vec![0xF4, 0x34]),
            I2cTransaction::write_read(0x77, vec![0xF6], vec![0x33])
                .with_error(ErrorKind::ArbitrationLoss),
        ],
    ]
    .concat();
    let mut i2c = I2cMock::new(&expectations);
    let mut bmp = BMP::new(i2c.clone(), NoopDelay, Default::default());

    bmp.init().unwrap();

    assert_eq!(
        bmp.read_pressure(),
        Err(BMPError::I2C(ErrorKind::ArbitrationLoss))
    );
    i2c.done();
}

#[test]
fn read_altitude_ok_given_readings() {
    let expectations = [
        get_init_coeficient_expectations(),
        vec![
            I2cTransaction::write(0x77, vec![0xF4, 0x2E]),
            I2cTransaction::write_read(0x77, vec![0xF6], vec![0xFF]),
            I2cTransaction::write_read(0x77, vec![0xF7], vec![0xFF]),
            I2cTransaction::write(0x77, vec![0xF4, 0x34]),
            I2cTransaction::write_read(0x77, vec![0xF6], vec![0x33]),
            I2cTransaction::write_read(0x77, vec![0xF7], vec![0x38]),
            I2cTransaction::write_read(0x77, vec![0xF8], vec![0x00]),
        ],
    ]
    .concat();
    let mut i2c = I2cMock::new(&expectations);
    let mut bmp = BMP::new(i2c.clone(), NoopDelay, Default::default());

    bmp.init().unwrap();
    let altitude = bmp.read_altitude();

    assert!(altitude.is_ok());
    assert!(altitude.unwrap().is_finite());
    i2c.done();
}

#[test]
fn read_altitude_fails_if_i2c_error() {
    let expectations = [
        get_init_coeficient_expectations(),
        vec![
            I2cTransaction::write(0x77, vec![0xF4, 0x2E]),
            I2cTransaction::write_read(0x77, vec![0xF6], vec![0xFF]),
            I2cTransaction::write_read(0x77, vec![0xF7], vec![0xFF]),
            I2cTransaction::write(0x77, vec![0xF4, 0x34]),
            I2cTransaction::write_read(0x77, vec![0xF6], vec![0x33]).with_error(ErrorKind::Other),
        ],
    ]
    .concat();
    let mut i2c = I2cMock::new(&expectations);
    let mut bmp = BMP::new(i2c.clone(), NoopDelay, Default::default());

    bmp.init().unwrap();

    assert_eq!(bmp.read_pressure(), Err(BMPError::I2C(ErrorKind::Other)));
    i2c.done();
}
