#![deny(missing_docs)]

//! A generalized HAL for communicating over serial ports
#[macro_use]
extern crate failure;
extern crate serial;

use std::io::prelude::*;
use serial::prelude::*;
use std::time::Duration;
use std::cell::RefCell;
use std::error::Error;

/// Wrapper for UART stream
pub struct Connection {
    /// Any boxed stream that allows for communication over serial ports
    pub stream: Box<Stream>,
}

/// Custom errors for UART actions
#[derive(Fail, Display, Debug, Clone, PartialEq)]
pub enum UartError {
    /// Catch-all error case
    #[display(fmt = "Generic Error")]
    GenericError,
    #[display(fmt = "Serial Error: {}", description)]
    /// An error was thrown by the serial driver
    SerialError {
        /// The underlying error type
        cause: serial::ErrorKind,
        /// Error description
        description: String,
    },
    #[display(fmt = "IO Error: {}", description)]
    /// An I/O error was thrown by the kernel
    IoError {
        /// The underlying error type
        cause: std::io::ErrorKind,
        /// Error description
        description: String,
    },
}

impl From<std::io::Error> for UartError {
    fn from(error: std::io::Error) -> Self {
        UartError::IoError {
            cause: error.kind(),
            description: error.description().to_owned(),
        }
    }
}

impl From<serial::Error> for UartError {
    fn from(error: serial::Error) -> Self {
        UartError::SerialError {
            cause: error.kind(),
            description: error.description().to_owned(),
        }
    }
}

/// Errors that occur while reading from and writing to stream
pub type UartResult<T> = Result<T, UartError>;

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
        println!("Connection write");
        self.stream.write(data)
    }

    /// Reads messages upto specified length recieved on the bus
    pub fn read(&self, len: usize, timeout: Duration) -> UartResult<Vec<u8>> {
        println!("Connection read");
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
