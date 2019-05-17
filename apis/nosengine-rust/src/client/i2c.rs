//! Wraps the I2C functionality of NOSEngine.
//!
//! # Examples
//!
//! ```norun
//! # extern crate nosengine_rust;
//! # use nosengine_rust::client::i2c::*;
//! # use nosengine_rust::ffi::i2c::I2CDirection;
//! # use std::slice;
//! let master = I2CMaster::new(9u16, "tcp://localhost:12001", "i2c19").unwrap();
//!
//! extern "C" fn callback(dir: I2CDirection, buffer: *mut u8, len: usize) -> usize {
//!     let data = unsafe{ slice::from_raw_parts_mut(buffer, len) };
//!     match dir {
//!         I2CDirection::Read => {
//!             for i in 0..len {
//!                 data[i] = (i + 5) as u8;
//!             }
//!             len
//!         },
//!         I2CDirection::Write => {
//!             assert_eq!(data, &[1u8, 2, 3, 4]);
//!             len
//!         }
//!     }
//! }
//!
//! let slave = I2CSlave::new(8u16, "tcp://localhost:12001", "i2c19", callback).unwrap();
//!
//! assert_eq!(master.write(8u16, &[1u8, 2, 3, 4]), Ok(()));
//! assert_eq!(master.read(8u16, 4), Ok(vec![5u8, 6, 7, 8]));
//! ```

use super::ffi::i2c;
use std::error::Error;
use std::ffi;
use std::ffi::CString;
use std::fmt;

/// This enum represents any type of error that can occur when interacting with I2C
#[derive(Debug, Clone, PartialEq)]
pub enum I2CError {
    /// An error occurred when converting a Rust string to a C string.
    /// Specifically, the Rust string contained a null character, which cannot be represented
    /// in C strings.
    StringError {
        /// Description from the underlying std::ffi::NulError
        description: String,
        /// Index in the original string of the problematic null character
        position: usize,
    },
    /// There was an error when creating the I2C.
    I2CCreationError,
    /// This error is raised when an I2C device is created with an invalid address.
    InvalidAddress {
        /// The address which was attempted
        address: u16,
    },
    /// Attempted to read or write to an address that doesn't exist.
    UnknownAddress {
        /// The address which was not found
        address: u16,
    },
}

impl Error for I2CError {
    fn description(&self) -> &str {
        unimplemented!()
    }
}

impl From<ffi::NulError> for I2CError {
    fn from(err: ffi::NulError) -> Self {
        I2CError::StringError {
            description: String::from(err.description()),
            position: err.nul_position(),
        }
    }
}

impl fmt::Display for I2CError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            I2CError::StringError {
                description,
                position,
            } => write!(f, "Null character at index {}: {}", position, description),
            I2CError::I2CCreationError => write!(f, "Error while creating I2C node"),
            I2CError::InvalidAddress { address } => write!(
                f,
                "Invalid address {}: Must be between 8 and 127, inclusive.",
                address
            ),
            I2CError::UnknownAddress { address } => {
                write!(f, "Address {} not found on this bus.", address)
            }
        }
    }
}

/// This struct represents a master on an I2C bus.
pub struct I2CMaster<'a> {
    i2c_ptr: *mut i2c::I2CHandle,
    /// NOSEngine connection string
    pub connection: &'a str,
    /// Name of this bus to which this master is connected
    pub bus: &'a str,
    /// Address of this master
    pub address: u16,
}

impl<'a> I2CMaster<'a> {
    /// Creates a new I2C master on the given bus. There can be only one: If you attempt to create
    /// another master with the same address on the same bus, this function will return `None`.
    ///
    /// # Arguments
    ///
    /// * `address`: Address of this I2C master
    /// * `connection`: NOSEngine connection string
    /// * `bus`: Name of the bus on which to create the master
    ///
    /// # Examples
    ///
    /// ```norun
    /// # extern crate nosengine_rust;
    /// # use nosengine_rust::client::i2c::*;
    /// let master = I2CMaster::new(9u16, "tcp://localhost:12001", "i2c20");
    /// assert!(master.is_ok());
    /// // 2 masters on a bus is OK as long as they have different addresses
    /// let master = I2CMaster::new(10u16, "tcp://localhost:12001", "i2c20");
    /// assert!(master.is_ok());
    /// // This fails because the address 10u16 is already taken.
    /// let master = I2CMaster::new(10u16, "tcp://localhost:12001", "i2c20");
    /// assert!(master.is_err());
    /// ```
    pub fn new(address: u16, connection: &'a str, bus: &'a str) -> Result<I2CMaster<'a>, I2CError> {
        if address < 8 || address > 127 {
            return Err(I2CError::InvalidAddress { address });
        }
        let c_connection = CString::new(connection)?;
        let c_bus = CString::new(bus)?;

        let i2c_ptr = i2c::i2c_init_master(address, c_connection.as_ptr(), c_bus.as_ptr());

        if i2c_ptr.is_null() {
            Err(I2CError::I2CCreationError)
        } else {
            Ok(I2CMaster {
                i2c_ptr,
                connection,
                bus,
                address,
            })
        }
    }

    /// This function reads bytes from the given address.
    ///
    /// # Arguments
    ///
    /// * `num_bytes`: How many bytes to read from the device
    ///
    /// # Examples
    ///
    /// See [`nosengine-rust::client::i2c`](../i2c/index.html#examples)
    pub fn read(&self, address: u16, num_bytes: usize) -> Result<Vec<u8>, I2CError> {
        if address < 8 || address > 127 {
            return Err(I2CError::InvalidAddress { address });
        }
        let mut rbuf: Vec<u8> = vec![0; num_bytes];
        rbuf.resize(num_bytes, 0u8);
        match i2c::i2c_read(self.i2c_ptr, address, rbuf.as_mut_ptr(), num_bytes) {
            i2c::I2CStatus::Success => Ok(rbuf),
            i2c::I2CStatus::Failure => Err(I2CError::UnknownAddress { address }),
        }
    }

    /// This function writes bytes to the given address.
    ///
    /// # Arguments
    ///
    /// * `data`: Bytes to write to the device
    ///
    /// # Examples
    ///
    /// See [`nosengine-rust::client::i2c`](../i2c/index.html#examples)
    pub fn write(&self, address: u16, data: &[u8]) -> Result<(), I2CError> {
        if address < 8 || address > 127 {
            return Err(I2CError::InvalidAddress { address });
        }
        match i2c::i2c_write(self.i2c_ptr, address, data.as_ptr(), data.len()) {
            i2c::I2CStatus::Success => {
                println!("i2c.write: {:?}", data);
                Ok(())
            }
            i2c::I2CStatus::Failure => Err(I2CError::UnknownAddress { address }),
        }
    }

    /// This function writes bytes to the given address followed by a read
    /// # Arguments
    ///
    /// * `tx_data`: Bytes to write to the device
    /// * `rx_len`: Number of bytes expected to be read
    pub fn transaction(
        &self,
        address: u16,
        tx_data: &[u8],
        rx_len: usize,
    ) -> Result<Vec<u8>, I2CError> {
        let mut rbuf: Vec<u8> = vec![0; rx_len];
        rbuf.resize(rx_len, 0u8);
        match i2c::i2c_transaction(
            self.i2c_ptr,
            address,
            tx_data.as_ptr(),
            tx_data.len(),
            rbuf.as_mut_ptr(),
            rx_len,
        ) {
            i2c::I2CStatus::Success => Ok(rbuf),
            i2c::I2CStatus::Failure => Err(I2CError::UnknownAddress { address }),
        }
    }
}

impl<'a> Drop for I2CMaster<'a> {
    fn drop(&mut self) {
        i2c::i2c_close(&mut self.i2c_ptr as *mut *mut i2c::I2CHandle);
    }
}

/// This struct represents an I2C Slave.
pub struct I2CSlave<'a> {
    i2c_ptr: *mut i2c::I2CHandle,
    /// The NOSEngine connection string
    pub connection: &'a str,
    /// Name of the bus to which this slave is connected
    pub bus: &'a str,
    /// Address of this slave
    pub address: u16,
}

impl<'a> I2CSlave<'a> {
    /// Constructs a new I2C slave. The given callback will run every time the master reads
    /// from or writes to this slave. If a slave with the given address already exists on
    /// this bus, this function returns `None`.
    ///
    /// # Arguments
    ///
    /// * `address`: Address for this slave. Must be unique on a bus
    /// * `connection`: NOSEngine connection string
    /// * `bus`: Name of the bus to connect to
    /// * `callback`: Callback that runs every time the master reads from or writes to this device.
    ///     The callback is responsible for checking whether it is reading or writing, performing
    ///     the appropriate action, then returning the number of bytes read or written. The
    ///     arguments to the callback are:
    ///     * `I2CDirection`: Specifies whether this is a read or write
    ///     * `*mut u8`: The buffer which either contains the data being written to this device, or
    ///         to which this device should write data. It is guaranteed to have enough bytes of
    ///         valid memory based on the length argument
    ///     * `usize`: The number of bytes being read or written
    pub fn new(
        address: u16,
        connection: &'a str,
        bus: &'a str,
        callback: extern "C" fn(i2c::I2CDirection, *mut u8, usize) -> usize,
    ) -> Result<I2CSlave<'a>, I2CError> {
        let c_connection = CString::new(connection)?;
        let c_bus = CString::new(bus)?;

        let i2c_ptr = i2c::i2c_init_slave(address, c_connection.as_ptr(), c_bus.as_ptr(), callback);

        if i2c_ptr.is_null() {
            Err(I2CError::I2CCreationError)
        } else {
            Ok(I2CSlave {
                i2c_ptr,
                connection,
                bus,
                address,
            })
        }
    }
}

impl<'a> Drop for I2CSlave<'a> {
    fn drop(&mut self) {
        i2c::i2c_close(&mut self.i2c_ptr as *mut *mut i2c::I2CHandle);
    }
}
