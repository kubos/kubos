/*
 * Copyright (C) 2018 Kubos Corporation
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#![deny(missing_docs)]
#![deny(warnings)]

//! I2C device connection abstractions

#[cfg(not(feature = "nos3"))]
use i2c_linux::I2c;
#[cfg(feature = "nos3")]
use nosengine_rust::client::i2c::I2CMaster;
#[cfg(feature = "nos3")]
use std::io::ErrorKind;
use std::io::Result;
use std::thread;
use std::time::Duration;

/// High level read/write trait for I2C connections to implement
pub trait Stream {
    /// Writes an I2C command
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
    /// `rx_len`  - Amount of data to read
    fn read(&self, command: Command, rx_len: usize) -> Result<Vec<u8>>;

    /// Writes I2C command and reads result
    ///
    /// # Arguments
    ///
    /// `command` - Command to write and read from
    /// `rx_len`  - Amount of data to read
    /// `delay`   - Delay between writing and reading
    fn transfer(&self, command: Command, rx_len: usize, delay: Duration) -> Result<Vec<u8>>;
}

/// An implementation of `i2c_hal::Stream` which uses the `i2c_linux` crate
/// for communication with actual I2C hardware.
pub struct I2CStream {
    #[cfg(not(feature = "nos3"))]
    path: String,
    slave: u16,
    #[cfg(feature = "nos3")]
    nos: Option<(String, String, u16)>,
}

#[cfg(not(feature = "nos3"))]
impl I2CStream {
    /// Creates new I2CStream instance
    ///
    /// # Arguments
    ///
    /// `path` - File system path to I2C device handle
    /// `slave` - Address of slave I2C device
    pub fn new(path: &str, slave: u16) -> Self {
        Self {
            path: path.to_string(),
            slave,
        }
    }
}

#[cfg(not(feature = "nos3"))]
impl Stream for I2CStream {
    /// Writing
    fn write(&self, command: Command) -> Result<()> {
        let mut i2c = I2c::from_path(self.path.clone())?;
        i2c.smbus_set_slave_address(self.slave, false)?;
        i2c.i2c_write_block_data(command.cmd, &command.data)
    }

    /// Reading
    fn read(&self, command: Command, rx_len: usize) -> Result<Vec<u8>> {
        let mut i2c = I2c::from_path(self.path.clone())?;
        i2c.smbus_set_slave_address(self.slave, false)?;
        let mut data = vec![0; rx_len];
        i2c.i2c_read_block_data(command.cmd, &mut data)?;
        Ok(data)
    }

    /// Read/Write transaction
    fn transfer(&self, command: Command, rx_len: usize, delay: Duration) -> Result<Vec<u8>> {
        let mut i2c = I2c::from_path(self.path.clone())?;
        let mut data = vec![0; rx_len];
        i2c.smbus_set_slave_address(self.slave, false)?;

        i2c.i2c_set_retries(5)?;
        i2c.i2c_write_block_data(command.cmd, &command.data)?;
        thread::sleep(delay);
        i2c.i2c_read_block_data(command.cmd, &mut data)?;
        Ok(data)
    }
}

#[cfg(feature = "nos3")]
impl I2CStream {
    fn read_config(
        path: &str,
    ) -> std::result::Result<(String, String, u16), Box<std::error::Error>> {
        let mut config = (include_str!("../../SimConfig.toml"))
            .parse::<toml::Value>()?
            .try_into::<toml::value::Table>()?;

        let connection = config
            .remove("connection")
            .ok_or(std::io::Error::new(
                ErrorKind::Other,
                "Error parsing SimConfig.toml",
            ))?
            .try_into::<String>()?;

        let mut table = config
            .remove("i2c")
            .ok_or(std::io::Error::new(
                ErrorKind::Other,
                "Error parsing SimConfig.toml",
            ))?
            .try_into::<toml::value::Table>()?;

        let master_address = table
            .remove("master_address")
            .ok_or(std::io::Error::new(
                ErrorKind::Other,
                "Error parsing SimConfig.toml",
            ))?
            .try_into::<i32>()?;

        let master_address = master_address as u16;

        let busname = table
            .remove("busnames")
            .ok_or(std::io::Error::new(
                ErrorKind::Other,
                "Error parsing SimConfig.toml",
            ))?
            .try_into::<toml::value::Table>()?
            .remove(path)
            .ok_or(std::io::Error::new(
                ErrorKind::Other,
                "Error parsing SimConfig.toml",
            ))?
            .try_into::<String>()?;

        Ok((connection, busname, master_address))
    }

    fn get_nos_connection(&self) -> Result<I2CMaster> {
        match &self.nos {
            Some((connection, busname, master_address)) => {
                println!(
                    "Conn: {}, busname: {}, addr: {}",
                    connection, busname, master_address
                );
                match I2CMaster::new(*master_address, connection.as_str(), busname.as_str()) {
                    Ok(i2c) => Ok(i2c),
                    Err(_) => Err(std::io::Error::new(
                        ErrorKind::Other,
                        "Error connecting to NOSEngine.",
                    )),
                }
            }
            None => Err(std::io::Error::new(
                ErrorKind::Other,
                "Error connecting to NOSEngine.",
            )),
        }
    }

    /// Creates new I2CStream instance.
    ///
    /// # Arguments
    ///
    /// `path` - File system path to I2C device handle
    /// `slave` - Address of slave I2C device
    pub fn new(path: &str, slave: u16) -> Self {
        match Self::read_config(path) {
            Ok((connection, busname, master_address)) => Self {
                slave,
                nos: Some((connection, busname, master_address)),
            },
            Err(_) => Self { slave, nos: None },
        }
    }
}

#[cfg(feature = "nos3")]
impl Stream for I2CStream {
    /// Writing
    fn write(&self, command: Command) -> Result<()> {
        let i2c = self.get_nos_connection()?;
        let cmd = &[command.cmd];
        let data = command.data.as_slice();
        let comm: Vec<_> = cmd.iter().chain(data).cloned().collect();
        match i2c.write(self.slave, &comm) {
            Ok(_) => Ok(()),
            Err(_) => Err(std::io::Error::new(
                ErrorKind::Other,
                "Error connecting to NOSEngine.".to_string(),
            )),
        }
    }

    /// Reading
    fn read(&self, command: Command, rx_len: usize) -> Result<Vec<u8>> {
        let i2c = self.get_nos_connection()?;
        match i2c
            .write(self.slave, &[command.cmd])
            .and_then(|()| i2c.read(self.slave, rx_len))
        {
            Ok(data) => Ok(data),
            Err(e) => Err(std::io::Error::new(ErrorKind::Other, e)),
        }
    }

    /// Read/Write transaction
    fn transfer(&self, command: Command, rx_len: usize, delay: Duration) -> Result<Vec<u8>> {
        thread::sleep(delay);
        let i2c = self.get_nos_connection()?;
        let cmd = &[command.cmd];
        let data = command.data.as_slice();
        let comm: Vec<_> = cmd.iter().chain(data).cloned().collect();
        match i2c.transaction(self.slave, &comm, rx_len) {
            Ok(data) => Ok(data),
            Err(e) => Err(std::io::Error::new(ErrorKind::Other, e)),
        }
    }
}

/// Struct for abstracting I2C command/data structure
#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    /// I2C command or registry
    pub cmd: u8,
    /// Data to write to registry
    pub data: Vec<u8>,
}

/// Struct for communicating with an I2C device
pub struct Connection {
    stream: Box<dyn Stream + Send>,
}

impl Connection {
    /// I2C connection constructor
    ///
    /// # Arguments
    ///
    /// `path` - Path to I2C device
    /// `slave` - I2C slave address to read/write to
    pub fn new(stream: Box<dyn Stream + Send>) -> Self {
        Self { stream }
    }

    /// Convenience constructor for creating a Connection with an I2CStream.
    ///
    /// # Arguments
    ///
    /// `path` - Path to I2C device
    /// `slave` - I2C slave address
    pub fn from_path(path: &str, slave: u16) -> Self {
        Self {
            stream: Box::new(I2CStream::new(path, slave)),
        }
    }

    /// Writes an I2C command
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
    /// `rx_len`  - Amount of data to read
    pub fn read(&self, command: Command, rx_len: usize) -> Result<Vec<u8>> {
        self.stream.read(command, rx_len)
    }

    /// Writes I2C command and reads result
    ///
    /// # Arguments
    ///
    /// `command` - Command to write and read from
    /// `rx_len`  - Amount of data to read
    /// `delay` - Delay between writing and reading
    pub fn transfer(&self, command: Command, rx_len: usize, delay: Duration) -> Result<Vec<u8>> {
        self.stream.transfer(command, rx_len, delay)
    }
}
