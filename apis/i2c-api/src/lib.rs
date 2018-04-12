#![deny(missing_docs)]
#![deny(warnings)]

//! I2c device connection abstractions

extern crate i2c_linux;

use std::io::Result;
use i2c_linux::I2c;

/// High level read/write trait for I2C connections to implement
pub trait Stream {
    /// Write bytes
    fn write(&self, command: Command) -> Result<()>;
    /// Read bytes and return vector
    fn read(&self, command: Command) -> Result<Vec<u8>>;
    /// Write & Read in one transaction
    fn transfer(&self, command: Command) -> Result<Vec<u8>>;
}

/// Struct for abstracting I2c command/data structure
#[derive(Debug)]
pub struct Command {
    /// I2c command or registry
    pub cmd: u8,
    /// Data to write to registry
    pub data: Vec<u8>,
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
    pub fn new(path: &str, slave: u16) -> Self {
        Connection {
            path: path.to_string(),
            slave,
        }
    }
}

impl Stream for Connection {
    /// Writing
    fn write(&self, command: Command) -> Result<()> {
        let mut i2c = I2c::from_path(self.path.clone())?;
        println!("i2c_connection writing {:?} to {}", command, self.slave);
        i2c.smbus_set_slave_address(self.slave, false)?;
        i2c.i2c_write_block_data(command.cmd, &command.data)
    }

    /// Reading
    fn read(&self, command: Command) -> Result<Vec<u8>> {
        let mut i2c = I2c::from_path(self.path.clone())?;
        println!("i2c connection reading from {}", self.slave);
        i2c.smbus_set_slave_address(self.slave, false)?;
        let mut data = vec![0; 4];
        i2c.i2c_read_block_data(command.cmd, &mut data)?;
        Ok(data)
    }

    /// Read/Write transaction
    fn transfer(&self, _command: Command) -> Result<Vec<u8>> {
        let data = vec![0; 4];
        Ok(data)
    }
}
