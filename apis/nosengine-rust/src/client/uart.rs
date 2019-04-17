//! Provides functionality for communicating using UART on NOSEngine.
//!

use super::ffi::uart;
use libc;
use std::error::Error;
use std::ffi;
use std::ffi::CString;
use std::fmt;
use std::slice;

/// This enum represents any type of error that can occur when interacting with UART.
#[derive(Debug, Clone, PartialEq)]
pub enum UARTError {
    /// An error occurred when converting a Rust string to a C string.
    /// Specifically, the Rust string contained a null character, which cannot be represented
    /// in C strings.
    StringError {
        /// Description from the underlying std::ffi::NulError
        description: String,
        /// Index in the original string of the problematic null character
        position: usize,
    },
    /// There was an error when creating the UART.
    UARTCreationError,
}

impl From<ffi::NulError> for UARTError {
    fn from(err: ffi::NulError) -> Self {
        UARTError::StringError {
            description: String::from(err.description()),
            position: err.nul_position(),
        }
    }
}

impl fmt::Display for UARTError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UARTError::StringError {
                description,
                position,
            } => write!(f, "Null character at index {}: {}", position, description),
            UARTError::UARTCreationError => write!(f, "Error while creating UART node"),
        }
    }
}

/// This struct represents one UART connection.
#[derive(Clone)]
pub struct UART {
    uart_ptr: *mut uart::UARTHandle,
    /// The name of this UART port. Must be unique on a bus
    pub name: String,
    /// The connection string to the server
    pub connection: String,
    /// The port number for this UART connection
    pub port: u8,
}

unsafe impl Sync for UART {}

impl UART {
    /// Create a new UART connection. If there are already two connections on the given
    /// port on the given bus, then this function returns `None`.
    ///
    /// # Arguments
    ///
    /// * `name`: Name of this UART connection. Must be unique on a bus.
    /// * `connection`: Connection string to server. Usually of the form `tcp://<domain>:<port>`,
    ///     but can take other forms. See the connection string section of the NOSEngine
    ///     user manual for more.
    /// * `bus`: Name of the bus to which this UART should connect. If the bus does not already
    ///     exist, it will be created automatically.
    /// * `port`: A number to identify a unique port. Multiple ports can exist on the same bus,
    ///     but only two UART connections can be opened on a given port on a given bus.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate nosengine_rust;
    /// # use nosengine_rust::client::uart::*;
    /// let uart = UART::new("uart1", "tcp://localhost:12001", "uart2", 10);
    /// assert!(uart.is_ok());
    /// ```
    ///
    /// If there is already a node on the bus with the same name, this function returns `None`.
    ///
    /// ```
    /// # extern crate nosengine_rust;
    /// # use nosengine_rust::client::uart::*;
    /// let uart = UART::new("uart14", "tcp://localhost:12001", "uart2", 10);
    /// assert!(uart.is_ok());
    /// let uart2 = UART::new("uart14", "tcp://localhost:12001", "uart2", 10);
    /// assert!(uart2.is_err());
    /// ```
    ///
    /// However, once a UART is dropped, you can create another node with the same name.
    ///
    /// ```
    /// # extern crate nosengine_rust;
    /// # use nosengine_rust::client::uart::*;
    /// {
    ///     let uart = UART::new("uart15", "tcp://localhost:12001", "uart2", 10);
    ///     assert!(uart.is_ok());
    /// } // uart is dropped here
    /// let uart2 = UART::new("uart15", "tcp://localhost:12001", "uart2", 10);
    /// assert!(uart2.is_ok());
    /// ```
    pub fn new(name: &str, connection: &str, bus: &str, port: u8) -> Result<UART, UARTError> {
        let c_name = CString::new(name)?;
        let c_connection = CString::new(connection)?;
        let c_bus = CString::new(bus)?;

        let uart_ptr =
            uart::uart_open(c_name.as_ptr(), c_connection.as_ptr(), c_bus.as_ptr(), port);

        if uart_ptr.is_null() {
            Err(UARTError::UARTCreationError)
        } else {
            Ok(UART {
                uart_ptr,
                name: String::from(name),
                connection: String::from(connection),
                port,
            })
        }
    }

    /// Read bytes from this UART's buffer. Will read at most `num_bytes` bytes, but if
    /// fewer bytes are available (including 0), the resulting vector will be smaller
    /// than `num_bytes`.
    ///
    /// # Arguments
    ///
    /// * `num_bytes`: The maximum number of bytes to read
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate nosengine_rust;
    /// # use nosengine_rust::client::uart::*;
    /// let uart1 = UART::new("uart2", "tcp://localhost:12001", "uart2", 11).unwrap();
    /// let uart2 = UART::new("uart3", "tcp://localhost:12001", "uart2", 11).unwrap();
    ///
    /// uart1.write(&[1u8, 2, 3, 4]);
    /// let result = uart2.read(100);
    /// assert_eq!(result, &[1u8, 2, 3, 4]);
    /// ```
    pub fn read(&self, num_bytes: usize) -> Vec<u8> {
        let mut buffer: Vec<u8> = Vec::with_capacity(num_bytes);
        let len = uart::uart_read(self.uart_ptr, buffer.as_mut_ptr(), num_bytes);
        assert!(len <= num_bytes);
        unsafe {
            buffer.set_len(len);
        }
        buffer
    }

    /// Write the given bytes to the UART port.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate nosengine_rust;
    /// # use nosengine_rust::client::uart::*;
    /// let uart1 = UART::new("uart4", "tcp://localhost:12001", "uart2", 12).unwrap();
    /// let uart2 = UART::new("uart5", "tcp://localhost:12001", "uart2", 12).unwrap();
    ///
    /// uart1.write(&[1u8, 2, 3, 4]);
    /// let result = uart2.read(100);
    /// assert_eq!(result, &[1u8, 2, 3, 4]);
    /// ```
    pub fn write(&self, data: &[u8]) -> usize {
        uart::uart_write(self.uart_ptr, data.as_ptr(), data.len())
    }

    /// Write one individual byte to the UART port.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate nosengine_rust;
    /// # use nosengine_rust::client::uart::*;
    /// let uart1 = UART::new("uart6", "tcp://localhost:12001", "uart2", 13).unwrap();
    /// let uart2 = UART::new("uart7", "tcp://localhost:12001", "uart2", 13).unwrap();
    /// uart1.putc(12u8);
    /// let result = uart2.getc();
    /// assert_eq!(result, Some(12u8));
    /// ```
    pub fn putc(&self, c: u8) {
        uart::uart_putc(self.uart_ptr, c)
    }

    /// Retrieve one individual byte from the UART port. If no bytes are available, then
    /// `None` is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate nosengine_rust;
    /// # use nosengine_rust::client::uart::*;
    /// let uart1 = UART::new("uart8", "tcp://localhost:12001", "uart2", 14).unwrap();
    /// let uart2 = UART::new("uart9", "tcp://localhost:12001", "uart2", 14).unwrap();
    ///
    /// uart1.putc(12u8);
    /// let result = uart2.getc();
    /// assert_eq!(result, Some(12u8));
    /// let result = uart2.getc();
    /// assert_eq!(result, None);
    /// ```
    pub fn getc(&self) -> Option<u8> {
        let mut c = 0u8;
        match uart::uart_getc(self.uart_ptr, &mut c as *mut u8) {
            uart::UARTStatus::Success => Some(c),
            uart::UARTStatus::Failure => None,
        }
    }

    /// Set a callback which will run whenever this UART port receives data.
    ///
    /// # Arguments
    ///
    /// * `func`: A callback with the following parameters:
    ///     * `&[u8]`: The data that was just written to this port
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate nosengine_rust;
    /// # use nosengine_rust::client::uart::*;
    /// let uart1 = UART::new("uart10", "tcp://localhost:12001", "uart2", 15).unwrap();
    /// let mut uart2 = UART::new("uart11", "tcp://localhost:12001", "uart2", 15).unwrap();
    ///
    /// uart2.set_callback(move |data: &[u8]|{
    ///     assert_eq!(data, &[1u8, 2, 3, 4]);
    /// });
    ///
    /// uart1.write(&[1u8, 2, 3, 4]);
    /// ```
    pub fn set_callback<F>(&mut self, func: F)
    where
        F: FnMut(&[u8]) -> (),
        F: 'static,
    {
        extern "C" fn c_callback(data: *const u8, len: usize, user: *mut libc::c_void) {
            unsafe {
                let func = user as *mut Box<FnMut(&[u8])>;
                let data = slice::from_raw_parts(data, len);
                (*func)(data);
            }
        }

        let func = Box::new(func) as Box<FnMut(&[u8])>;
        let func = Box::into_raw(Box::new(func));

        uart::uart_set_read_callback(self.uart_ptr, c_callback, func as *mut libc::c_void);
    }

    /// Return the number of bytes waiting to be read by this UART.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate nosengine_rust;
    /// # use nosengine_rust::client::uart::*;
    /// let uart1 = UART::new("uart12", "tcp://localhost:12001", "uart2", 16).unwrap();
    /// let uart2 = UART::new("uart13", "tcp://localhost:12001", "uart2", 16).unwrap();
    ///
    /// uart1.write(&[1u8, 2, 3, 4]);
    /// assert_eq!(uart2.available(), 4);
    /// uart2.read(2);
    /// assert_eq!(uart2.available(), 2);
    /// ```
    pub fn available(&self) -> usize {
        uart::uart_available(self.uart_ptr) as usize
    }
}

impl Drop for UART {
    fn drop(&mut self) {
        uart::uart_close(&mut self.uart_ptr as *mut *mut uart::UARTHandle);
    }
}

unsafe impl Send for UART {}
