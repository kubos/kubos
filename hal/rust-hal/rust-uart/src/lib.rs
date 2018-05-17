//
// Copyright (C) 2018 Kubos Corporation
//
// Licensed under the Apache License, Version 2.0 (the "License")
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

#![deny(missing_docs)]

//! A generalized HAL for communicating over serial ports
#[macro_use]
extern crate failure;
extern crate serial;

mod error;
pub mod mock;
#[cfg(test)]
mod tests;

pub use error::*;
use serial::prelude::*;
use std::cell::RefCell;
use std::io::prelude::*;
use std::time::Duration;

/// Wrapper for UART stream
pub struct Connection {
    /// Any boxed stream that allows for communication over serial ports
    pub stream: Box<Stream>,
}

impl Connection {
    /// Constructor to creation connection with provided stream
    pub fn new(stream: Box<Stream>) -> Connection {
        Connection { stream }
    }

    /// Convenience constructor to create connection from bus path
    pub fn from_path(
        bus: &str,
        settings: serial::PortSettings,
        timeout: Duration,
    ) -> UartResult<Connection> {
        Ok(Connection {
            stream: Box::new(SerialStream::new(bus, settings, timeout)?),
        })
    }

    /// Writes out raw bytes to the stream
    pub fn write(&self, data: &[u8]) -> UartResult<()> {
        self.stream.write(data)
    }

    /// Reads messages upto specified length recieved on the bus
    pub fn read(&self, len: usize, timeout: Duration) -> UartResult<Vec<u8>> {
        self.stream.read(len, timeout)
    }
}

/// This trait is used to represent streams and allows for mocking for api unit tests
pub trait Stream: Send {
    /// Write raw bytes to stream
    fn write(&self, data: &[u8]) -> UartResult<()>;

    /// Read upto a specified amount of raw bytes from the stream
    fn read(&self, len: usize, timeout: Duration) -> UartResult<Vec<u8>>;
}

// This is the actual stream that data is tranferred over
struct SerialStream {
    port: RefCell<serial::SystemPort>,
    timeout: Duration,
}

impl SerialStream {
    fn new(bus: &str, settings: serial::PortSettings, timeout: Duration) -> UartResult<Self> {
        let mut port = serial::open(bus)?;

        port.configure(&settings)?;

        Ok(SerialStream {
            port: RefCell::new(port),
            timeout,
        })
    }
}

// Read and write implementations for the serial stream
impl Stream for SerialStream {
    fn write(&self, data: &[u8]) -> UartResult<()> {
        let mut port = self.port.try_borrow_mut().map_err(|_| UartError::PortBusy)?;
        port.set_timeout(self.timeout)?;

        port.write_all(data)?;

        Ok(())
    }

    fn read(&self, len: usize, timeout: Duration) -> UartResult<Vec<u8>> {
        let mut port = self.port.try_borrow_mut().map_err(|_| UartError::PortBusy)?;

        port.set_timeout(timeout)?;

        let mut response: Vec<u8> = vec![0; len];

        port.read_exact(response.as_mut_slice())?;

        Ok(response)
    }
}
