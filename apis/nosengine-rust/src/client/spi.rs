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
// Contributed by: Timothy Scott (tmscott@mix.wvu.edu) and Sebastian Hamel (sebastian.hamel@rockets.utoledo.edu)
//

//! This module contains the SPI functionality from NOSEngine.
//!
//! # Examples
//!
//! ```norun
//! # extern crate nosengine_rust;
//! # use nosengine_rust::client::spi::*;
//! # use nosengine_rust::ffi::spi::SPIDirection;
//! # use std::slice;
//! let master = SPIMaster::new("tcp://localhost:12001", "spi19").unwrap();
//!
//! extern "C" fn callback(dir: SPIDirection, buffer: *mut u8, len: usize) -> usize {
//!     let data = unsafe{ slice::from_raw_parts_mut(buffer, len) };
//!     match dir {
//!         SPIDirection::Read => {
//!             for i in 0..len {
//!                 data[i] = (i + 5) as u8;
//!             }
//!             len
//!         },
//!         SPIDirection::Write => {
//!             assert_eq!(data, &[1u8, 2, 3, 4]);
//!             len
//!         }
//!     }
//! }
//!
//! let slave = SPISlave::new(1, "tcp://localhost:12001", "spi19", callback).unwrap();
//!
//! assert_eq!(master.write(&[1u8, 2, 3, 4]), Err(SPIError::ChipSelectionError));
//! master.chip_select(1);
//! assert_eq!(master.write(&[1u8, 2, 3, 4]), Ok(()));
//! assert_eq!(master.read(4), Ok(vec![5u8, 6, 7, 8]));
//! ```

use super::ffi::spi;
use std::error::Error;
use std::ffi;
use std::ffi::CString;
use std::fmt;

/// This enum represents any type of error that can occur when interacting with SPI.
#[derive(Debug, Clone, PartialEq)]
pub enum SPIError {
    /// An error occurred when converting a Rust string to a C string.
    /// Specifically, the Rust string contained a null character, which cannot be represented
    /// in C strings.
    StringError {
        /// Description from the underlying std::ffi::NulError
        description: String,
        /// Index in the original string of the problematic null character
        position: usize,
    },
    /// There was an error when creating the SPI.
    SPICreationError,
    /// This error is raised when a read or write is attempted when either no chip is selected,
    /// or the selected chip is not found on this bus
    ChipSelectionError,
}

impl From<ffi::NulError> for SPIError {
    fn from(err: ffi::NulError) -> Self {
        SPIError::StringError {
            description: String::from(err.description()),
            position: err.nul_position(),
        }
    }
}

impl fmt::Display for SPIError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SPIError::StringError {
                description,
                position,
            } => write!(f, "Null character at index {}: {}", position, description),
            SPIError::SPICreationError => write!(f, "Error while creating SPI node"),
            SPIError::ChipSelectionError => write!(f, "No SPI chip selected"),
        }
    }
}

/// Represents a master on an SPI bus.
pub struct SPIMaster<'a> {
    spi_ptr: *mut spi::SPIHandle,
    /// NOSEngine connection string
    pub connection: &'a str,
    /// Name of this bus to which this master is connected
    pub bus: &'a str,
}

impl<'a> SPIMaster<'a> {
    /// This function creates a new SPI master on the given bus. There can be only one: If you attempt to create
    /// another master on the same bus, this function will return `SPIError::SPICreationError`.
    ///
    /// # Arguments
    ///
    /// * `connection`: NOSEngine connection string
    /// * `bus`: Name of the bus on which to create the master
    ///
    /// # Examples
    ///
    /// ```norun
    /// # extern crate nosengine_rust;
    /// # use nosengine_rust::client::spi::*;
    /// let master = SPIMaster::new("tcp://localhost:12001", "spi20");
    /// assert!(master.is_ok());
    /// ```
    pub fn new(connection: &'a str, bus: &'a str) -> Result<SPIMaster<'a>, SPIError> {
        let c_connection = CString::new(connection)?;
        let c_bus = CString::new(bus)?;

        let spi_ptr = spi::spi_init_master(c_connection.as_ptr(), c_bus.as_ptr());

        if spi_ptr.is_null() {
            Err(SPIError::SPICreationError)
        } else {
            Ok(SPIMaster {
                spi_ptr,
                connection,
                bus,
            })
        }
    }

    /// This function reads bytes from the currently-selected chip. If no chip is selected, then
    /// this will return `Err`.
    ///
    /// # Arguments
    ///
    /// * `num_bytes`: How many bytes to read from the device
    ///
    /// # Examples
    ///
    /// See [`nosengine-rust::client::spi`](../spi/index.html#examples)
    pub fn read(&self, num_bytes: usize) -> Result<Vec<u8>, SPIError> {
        let mut rbuf: Vec<u8> = vec![0; num_bytes];
        rbuf.resize(num_bytes, 0u8);
        match spi::spi_read(self.spi_ptr, rbuf.as_mut_ptr(), num_bytes) {
            spi::SPIStatus::Success => Ok(rbuf),
            spi::SPIStatus::Failure => Err(SPIError::ChipSelectionError),
        }
    }

    /// This function writes bytes to the currently-selected chip. If no chip is selected, then this will
    /// return `Err`.
    ///
    /// # Arguments
    ///
    /// * `data`: Bytes to write to the device
    ///
    /// # Examples
    ///
    /// See [`nosengine-rust::client::spi`](../spi/index.html#examples)
    pub fn write(&self, data: &[u8]) -> Result<(), SPIError> {
        match spi::spi_write(self.spi_ptr, data.as_ptr(), data.len()) {
            spi::SPIStatus::Success => Ok(()),
            spi::SPIStatus::Failure => Err(SPIError::ChipSelectionError),
        }
    }

    /// Select the device to communicate with.
    ///
    /// # Examples
    ///
    /// See [`nosengine-rust::client::spi`](../spi/index.html#examples)
    pub fn chip_select(&self, cs: u8) {
        spi::spi_select_chip(self.spi_ptr, cs);
    }

    /// Unselect the current device.
    pub fn chip_unselect(&self) {
        spi::spi_unselect_chip(self.spi_ptr);
    }
}

impl<'a> Drop for SPIMaster<'a> {
    fn drop(&mut self) {
        spi::spi_close(&mut self.spi_ptr as *mut *mut spi::SPIHandle);
    }
}

/// This struct represents an SPI Slave.
pub struct SPISlave<'a> {
    spi_ptr: *mut spi::SPIHandle,
    /// The NOSEngine connection string
    pub connection: &'a str,
    /// Name of the bus to which this slave is connected
    pub bus: &'a str,
    /// Chip select number of this slave
    pub cs: u8,
}

impl<'a> SPISlave<'a> {
    /// Construct a new SPI slave. The given callback will be run every time the master reads
    /// from or writes to this slave. If a slave with the given chip select already exists on
    /// this bus, this function returns `SPIError::SPICreationError`.
    ///
    /// # Arguments
    ///
    /// * `cs`: Chip select number for this slave. Must be unique on a bus
    /// * `connection`: NOSEngine connection string
    /// * `bus`: Name of the bus to connect to
    /// * `callback`: Callback that runs every time the master reads from or writes to this device.
    ///     The callback is responsible for checking whether it is reading or writing, performing
    ///     the appropriate action, then returning the number of bytes read or written. The
    ///     arguments to the callback are:
    ///     * `SPIDirection`: Specifies whether this is a read or write
    ///     * `*mut u8`: The buffer which either contains the data being written to this device, or
    ///         to which this device should write data. It is guaranteed to have enough bytes of
    ///         valid memory based on the length argument
    ///     * `usize`: The number of bytes being read or written
    pub fn new(
        cs: u8,
        connection: &'a str,
        bus: &'a str,
        callback: extern "C" fn(spi::SPIDirection, *mut u8, usize) -> usize,
    ) -> Result<SPISlave<'a>, SPIError> {
        let c_connection = CString::new(connection)?;
        let c_bus = CString::new(bus)?;

        let spi_ptr = spi::spi_init_slave(cs, c_connection.as_ptr(), c_bus.as_ptr(), callback);

        if spi_ptr.is_null() {
            Err(SPIError::SPICreationError)
        } else {
            Ok(SPISlave {
                spi_ptr,
                connection,
                bus,
                cs,
            })
        }
    }
}

impl<'a> Drop for SPISlave<'a> {
    fn drop(&mut self) {
        spi::spi_close(&mut self.spi_ptr as *mut *mut spi::SPIHandle);
    }
}
