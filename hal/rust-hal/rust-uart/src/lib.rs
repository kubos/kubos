#![deny(missing_docs)]

//! A generalized HAL for communicating over serial ports
extern crate serial;

use std::io::prelude::*;
use serial::prelude::*;
use std::time::Duration;
use std::cell::RefCell;

/// Wrapper for UART stream
pub struct Connection {
    /// Any boxed stream that allows for communication over serial ports
    pub stream: Box<Stream>,
}

/// Errors that occur while reading from and writing to stream
pub type UartResult<T> = Result<T, std::io::Error>;

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
pub trait Stream {
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
        let mut port = self.port.try_borrow_mut().unwrap();
        port.set_timeout(self.timeout)?;

        port.write_all(data)?;

        Ok(())
    }

    fn read(&self, len: usize, timeout: Duration) -> UartResult<Vec<u8>> {
        let mut port = self.port.try_borrow_mut().unwrap();

        port.set_timeout(timeout)?;

        let mut response: Vec<u8> = vec![0; len];

        port.read_exact(response.as_mut_slice())?;

        Ok(response)
    }
}
