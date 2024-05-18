use bmp085_180_rs::{BMPError, BMP};
use embedded_hal::i2c::ErrorKind;
use embedded_hal_mock::eh1::delay::NoopDelay;
use embedded_hal_mock::eh1::i2c::{Mock as I2cMock, Transaction as I2cTransaction};

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
