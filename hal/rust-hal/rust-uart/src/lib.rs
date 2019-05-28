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

mod error;
pub mod mock;
#[cfg(test)]
mod tests;

pub use crate::error::*;
#[cfg(feature = "nos3")]
use nosengine_rust::client::uart;
#[cfg(not(feature = "nos3"))]
use serial::prelude::*;
use std::cell::RefCell;
#[allow(unused_imports)]
use std::io::prelude::*;
use std::time::Duration;
#[cfg(feature = "nos3")]
use std::{sync, thread, time::Instant};

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
#[cfg(not(feature = "nos3"))]
struct SerialStream {
    port: RefCell<serial::SystemPort>,
    timeout: Duration,
}

#[cfg(not(feature = "nos3"))]
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
#[cfg(not(feature = "nos3"))]
impl Stream for SerialStream {
    fn write(&self, data: &[u8]) -> UartResult<()> {
        let mut port = self
            .port
            .try_borrow_mut()
            .map_err(|_| UartError::PortBusy)?;
        port.set_timeout(self.timeout)?;

        port.write_all(data)?;

        Ok(())
    }

    fn read(&self, len: usize, timeout: Duration) -> UartResult<Vec<u8>> {
        let mut port = self
            .port
            .try_borrow_mut()
            .map_err(|_| UartError::PortBusy)?;

        port.set_timeout(timeout)?;

        let mut response: Vec<u8> = vec![0; len];

        port.read_exact(response.as_mut_slice())?;

        Ok(response)
    }
}

#[cfg(feature = "nos3")]
struct SerialStream {
    port: sync::Arc<sync::Mutex<uart::UART>>,
    min_timeout: Duration,
}

#[cfg(feature = "nos3")]
impl SerialStream {
    fn new(bus: &str, _settings: serial::PortSettings, _timeout: Duration) -> UartResult<Self> {
        let mut config = (include_str!("../../SimConfig.toml"))
            .parse::<toml::Value>()?
            .try_into::<toml::value::Table>()?;

        let connection = config
            .remove("connection")
            .ok_or(UartError::GenericError)?
            .try_into::<String>()?;

        let nodename = config
            .remove("nodename")
            .ok_or(UartError::GenericError)?
            .try_into::<String>()?;

        let busname = config
            .remove("busnames")
            .ok_or(UartError::GenericError)?
            .try_into::<toml::value::Table>()?
            .remove("uart")
            .ok_or(UartError::GenericError)?
            .try_into::<toml::value::Table>()?
            .remove(bus)
            .ok_or(UartError::GenericError)?
            .try_into::<String>()?;

        let min_timeout = config
            .remove("min_timeout")
            .ok_or(UartError::GenericError)?
            .try_into::<i64>()?;

        let port = uart::UART::new(nodename.as_str(), connection.as_str(), busname.as_str(), 1)?;
        Ok(SerialStream {
            port: sync::Arc::new(sync::Mutex::new(port)),
            min_timeout: Duration::from_millis(min_timeout as u64),
        })
    }
}

#[cfg(feature = "nos3")]
impl Stream for SerialStream {
    fn write(&self, data: &[u8]) -> UartResult<()> {
        self.port.lock().unwrap_or_default()
    }

    fn read(&self, len: usize, timeout: Duration) -> UartResult<Vec<u8>> {
        let timeout = if timeout < self.min_timeout {
            self.min_timeout
        } else {
            timeout
        };

        let (tx, rx) = sync::mpsc::channel::<Vec<u8>>();
        let port = self.port.clone();

        // Because NOSEngine doesn't support timeouts, I have to roll my own timeout
        thread::spawn(move || {
            let port = port.lock().unwrap();
            let mut data: Vec<u8> = port.read(len);
            let start = Instant::now();
            // When reading from a NOSEngine port, it never blocks. This means it often returns
            // zero bytes. However, the real UART port blocks until it either reads the requested
            // number of bytes or times out. So to emulate that behavior, I poll the NOSEngine
            // port until I either get the requested number of bytes or time out.
            while data.len() < len && Instant::now() - start < timeout {
                let mut newdata = port.read(len);
                data.append(&mut newdata);
                thread::sleep(Duration::from_millis(100))
            }
            tx.send(data).unwrap();
        });

        let result = rx.recv()?;
        if result.len() < len {
            let description = String::from("UART Read timed out");
            Err(UartError::IoError {
                cause: std::io::ErrorKind::TimedOut,
                description,
            })
        } else {
            Ok(result)
        }
    }
}
