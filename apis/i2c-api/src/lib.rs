#![deny(missing_docs)]
#![deny(warnings)]

//! I2c device connection abstractions

extern crate i2c_linux;

use std::io::Result;
use i2c_linux::I2c;

/// High level read/write trait for I2C connections to implement
pub trait Stream {
    /// Write bytes
    fn write(&self, data: &[u8]) -> Result<()>;
    /// Read bytes and return vector
    fn read(&self, length: usize) -> Result<Vec<u8>>;
}

/// Struct for communicating with an I2c device
pub struct Connection {
    path: String,
    slave: u16,
}

impl Connection {
    /// I2c connection constructor
    ///
    /// # Arguments
    ///
    /// `path` - Path to I2c device
    /// `slave` - I2c slave address to read/write to
    pub fn new(path: String, slave: u16) -> Self {
        Connection { path, slave }
    }
}

impl Stream for Connection {
    /// Writing
    fn write(&self, data: &[u8]) -> Result<()> {
        let mut i2c = I2c::from_path(self.path.clone())?;
        i2c.smbus_set_slave_address(self.slave, false)?;
        i2c.smbus_write_byte(data[0])
    }

    /// Reading
    fn read(&self, _length: usize) -> Result<Vec<u8>> {
        let mut i2c = I2c::from_path(self.path.clone())?;
        i2c.smbus_set_slave_address(self.slave, false)?;
        let data = i2c.smbus_read_byte()?;
        Ok(vec![data])
    }
}
