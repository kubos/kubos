#![deny(missing_docs)]
#![deny(warnings)]

//! I2c device connection abstractions

extern crate i2c_linux;

use i2c_linux::I2c;
use std::io::Result;
use std::thread;
use std::time::Duration;

/// High level read/write trait for I2C connections to implement
pub trait Stream {
    /// Writes an I2c command
    ///
    /// # Arguments
    ///
    /// `command` - Command to write
    fn write(&self, command: Command) -> Result<()>;

    /// Reads command result
    ///
    /// # Arguments
    ///
    /// `command` - Command to read result from
    fn read(&self, command: Command) -> Result<Vec<u8>>;

    /// Writes I2c command and reads result
    ///
    /// # Arguments
    ///
    /// `command` - Command to write and read from
    /// `delay` - Delay between writing and reading
    fn transfer(&self, command: Command, delay: Duration) -> Result<Vec<u8>>;
}

/// An implementation of `i2c_hal::Stream` which uses the `i2c_linux` crate
/// for communication with actual I2c hardware.
pub struct I2cStream {
    path: String,
    slave: u16,
}

impl I2cStream {
    /// Creates new I2cStream instance
    ///
    /// # Arguments
    ///
    /// `path` - File system path to I2c device handle
    /// `slave` - Address of slave I2c device
    pub fn new(path: &str, slave: u16) -> Self {
        Self {
            path: path.to_string(),
            slave,
        }
    }
}

impl Stream for I2cStream {
    /// Writing
    fn write(&self, command: Command) -> Result<()> {
        let mut i2c = I2c::from_path(self.path.clone())?;
        i2c.smbus_set_slave_address(self.slave, false)?;
        i2c.i2c_write_block_data(command.cmd, &command.data)
    }

    /// Reading
    fn read(&self, command: Command) -> Result<Vec<u8>> {
        let mut i2c = I2c::from_path(self.path.clone())?;
        i2c.smbus_set_slave_address(self.slave, false)?;
        let mut data = vec![0; 4];
        i2c.i2c_read_block_data(command.cmd, &mut data)?;
        Ok(data)
    }

    /// Read/Write transaction
    fn transfer(&self, command: Command, delay: Duration) -> Result<Vec<u8>> {
        let mut i2c = I2c::from_path(self.path.clone())?;
        let mut data = vec![0; 4];
        i2c.smbus_set_slave_address(self.slave, false)?;

        i2c.i2c_set_retries(5)?;
        i2c.i2c_write_block_data(command.cmd, &command.data)?;
        thread::sleep(delay);
        i2c.i2c_read_block_data(command.cmd, &mut data)?;
        Ok(data)
    }
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
    stream: Box<Stream>,
}

impl Connection {
    /// I2c connection constructor
    ///
    /// # Arguments
    ///
    /// `path` - Path to I2c device
    /// `slave` - I2c slave address to read/write to
    pub fn new(stream: Box<Stream>) -> Self {
        Self { stream }
    }

    /// Convenience constructor for creating a Connection with an I2cStream.
    ///
    /// # Arguments
    ///
    /// `path` - Path to I2c device
    /// `slave` - I2c slave address
    pub fn from_path(path: &str, slave: u16) -> Self {
        Self {
            stream: Box::new(I2cStream::new(path, slave)),
        }
    }

    /// Writes an I2c command
    ///
    /// # Arguments
    ///
    /// `command` - Command to write
    pub fn write(&self, command: Command) -> Result<()> {
        self.stream.write(command)
    }

    /// Reads command result
    ///
    /// # Arguments
    ///
    /// `command` - Command to read result from
    pub fn read(&self, command: Command) -> Result<Vec<u8>> {
        self.stream.read(command)
    }

    /// Writes I2c command and reads result
    ///
    /// # Arguments
    ///
    /// `command` - Command to write and read from
    /// `delay` - Delay between writing and reading
    pub fn transfer(&self, command: Command, delay: Duration) -> Result<Vec<u8>> {
        self.stream.transfer(command, delay)
    }
}
