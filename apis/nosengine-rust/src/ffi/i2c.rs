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

//! This module contains thin wrappers over the NOSEngine I2C API.
//!
//! # Examples
//!
//! ```norun
//! # extern crate nosengine_rust;
//! # use nosengine_rust::ffi::i2c::*;
//! # use std::ffi::CString;
//! # use std::slice;
//! let connection = CString::new("tcp://localhost:12001").unwrap();
//! let bus = CString::new("i2c10").unwrap();
//! let mut master = i2c_init_master(9u16, connection.as_ptr(), bus.as_ptr());
//!
//! extern "C" fn callback(dir: I2CDirection, buffer: *mut u8, len: usize) -> usize {
//!     let mut buffer = unsafe { slice::from_raw_parts_mut(buffer, len) };
//!     match dir {
//!         I2CDirection::Read => {
//!             for i in 0u8..4u8 {
//!                 buffer[i as usize] = i + 5;
//!             }
//!             len
//!         },
//!         I2CDirection::Write => {
//!             assert_eq!(buffer, &[1u8, 2, 3, 4]);
//!             len
//!         }
//!     }
//! }
//! let mut slave = i2c_init_slave(10u16, connection.as_ptr(), bus.as_ptr(), callback);
//!
//! let mut wbuf = vec![1u8, 2u8, 3u8, 4u8];
//! let mut rbuf = vec![0u8, 0u8, 0u8, 0u8];
//!
//! let result = i2c_transaction(master, 10u16, wbuf.as_ptr(), 4, rbuf.as_mut_ptr(), 4);
//! assert_eq!(result, I2CStatus::Success);
//! assert_eq!(rbuf, &[5u8, 6, 7, 8]);
//!
//! i2c_close(&mut slave as *mut *mut I2CHandle);
//! i2c_close(&mut master as *mut *mut I2CHandle);
//! ```

use libc::c_char;

/// This function initializes an I2C master on the given bus, with the given address.
/// Only one master can exist per bus.
///
/// # Arguments
///
/// * `address`: Address of the master. Must be between 8 and 127, inclusive.
/// * `connection`: NOSEngine connection string
/// * `bus`: Name of bus to use for I2C
///
/// # Examples
/// ```norun
/// # extern crate nosengine_rust;
/// # use nosengine_rust::ffi::i2c::*;
/// # use std::ffi::CString;
/// let connection = CString::new("tcp://localhost:12001").unwrap();
/// let bus = CString::new("i2c1").unwrap();
/// let mut master = i2c_init_master(9u16, connection.as_ptr(), bus.as_ptr());
/// assert!(!master.is_null());
/// let mut master2 = i2c_init_master(9u16, connection.as_ptr(), bus.as_ptr());
/// assert!(master2.is_null());
/// i2c_close(&mut master as *mut *mut I2CHandle);
/// i2c_close(&mut master2 as *mut *mut I2CHandle);
/// ```
pub fn i2c_init_master(
    address: u16,
    connection: *const c_char,
    bus: *const c_char,
) -> *mut I2CHandle {
    unsafe { NE_i2c_init_master(address, connection, bus) }
}

/// This function initializes an I2C slave on the given bus, with the given address.
///
/// # Arguments
///
/// * `address`: Unique address of the slave. Must be unique, and must be between
///     8 and 127, inclusive.
/// * `connection`: NOSEngine connection string
/// * `bus`: Name of the bus to use for I2C
/// * `callback`: Function which will be called every time a master reads or writes to this slave.
///     This callback should return the number of bytes it has read or written.
///     The callback's parameters are:
///     * `I2CDirection`: Indicates whether the master is reading or writing
///     * `*mut u8`: This is a buffer. If the master is writing, then the slave should read the
///         data from this buffer. If the master is reading, then the slave should write into
///         this buffer.
///     * `usize`: The number of bytes that should be read or written
///
/// # Examples
///
/// ```norun
/// # extern crate nosengine_rust;
/// # use nosengine_rust::ffi::i2c::*;
/// # use std::ffi::CString;
/// let connection = CString::new("tcp://localhost:12001").unwrap();
/// let bus = CString::new("i2c2").unwrap();
///
/// extern "C" fn callback(dir: I2CDirection, buffer: *mut u8, len: usize) -> usize {0}
///
/// let mut slave = i2c_init_slave(10u16, connection.as_ptr(), bus.as_ptr(), callback);
/// assert!(!slave.is_null());
/// i2c_close(&mut slave as *mut *mut I2CHandle);
/// ```
pub fn i2c_init_slave(
    address: u16,
    connection: *const c_char,
    bus: *const c_char,
    callback: extern "C" fn(I2CDirection, *mut u8, usize) -> usize,
) -> *mut I2CHandle {
    unsafe { NE_i2c_init_slave(address, connection, bus, callback) }
}

/// This function closes an I2C connection and frees up all associated memory.
///
/// # Arguments
///
/// * `i2c`: A pointer to a pointer to an I2C handle. This handle will be made null by this
///     function.
///
/// # Examples
/// ```norun
/// # extern crate nosengine_rust;
/// # use nosengine_rust::ffi::i2c::*;
/// # use std::ffi::CString;
/// let connection = CString::new("tcp://localhost:12001").unwrap();
/// let bus = CString::new("i2c3").unwrap();
/// let mut master = i2c_init_master(9u16, connection.as_ptr(), bus.as_ptr());
/// assert!(!master.is_null());
/// i2c_close(&mut master as *mut *mut I2CHandle);
/// assert!(master.is_null());
/// ```
pub fn i2c_close(i2c: *mut *mut I2CHandle) {
    unsafe { NE_i2c_close(i2c) }
}

/// This function reads from the specified address.
///
/// # Arguments
///
/// * `i2c`: Handle to an I2C master
/// * `address`: Address from which you are reading
/// * `rbuf`: A buffer in which the read data will be stored. Must contain at lease `rlen`
///     consecutive bytes of valid memory.
/// * `rlen`: Maximum number of bytes to read
///
/// # Examples
///
/// ```norun
/// # extern crate nosengine_rust;
/// # use nosengine_rust::ffi::i2c::*;
/// # use std::ffi::CString;
/// # use std::slice;
/// let connection = CString::new("tcp://localhost:12001").unwrap();
/// let bus = CString::new("i2c4").unwrap();
/// let mut master = i2c_init_master(9u16, connection.as_ptr(), bus.as_ptr());
///
/// extern "C" fn callback(dir: I2CDirection, buffer: *mut u8, len: usize) -> usize {
///     assert_eq!(dir, I2CDirection::Read);
///     let mut buffer = unsafe { slice::from_raw_parts_mut(buffer, len) };
///     for i in 1u8..5u8 {
///         buffer[(i - 1) as usize] = i;
///     }
///     len
/// }
/// let mut slave = i2c_init_slave(10u16, connection.as_ptr(), bus.as_ptr(), callback);
///
/// let mut buffer = vec![0u8, 0u8, 0u8, 0u8];
///
/// let result = i2c_read(master, 10u16, buffer.as_mut_ptr(), 4);
/// // Expect a failure because no chip was selected.
/// // assert_eq!(result, I2CStatus::Failure);
///
/// i2c_close(&mut master as *mut *mut I2CHandle);
/// i2c_close(&mut slave as *mut *mut I2CHandle);
/// ```
pub fn i2c_read(i2c: *mut I2CHandle, address: u16, rbuf: *mut u8, rlen: usize) -> I2CStatus {
    unsafe { NE_i2c_read(i2c, address, rbuf, rlen) }
}

/// This function writes to the specified address.
///
/// # Arguments
///
/// * `i2c`: Handle to an I2C master
/// * `address`: Address to which you are writing
/// * `wbuf`: A buffer from which the slave will read data. Must contain at lease `wlen`
///     consecutive bytes of valid memory.
/// * `wlen`: Number of bytes to write
///
/// # Examples
///
/// ```norun
/// # extern crate nosengine_rust;
/// # use nosengine_rust::ffi::i2c::*;
/// # use std::ffi::CString;
/// # use std::slice;
/// let connection = CString::new("tcp://localhost:12001").unwrap();
/// let bus = CString::new("i2c5").unwrap();
/// let mut master = i2c_init_master(9u16, connection.as_ptr(), bus.as_ptr());
///
/// extern "C" fn callback(dir: I2CDirection, buffer: *mut u8, len: usize) -> usize {
///     assert_eq!(dir, I2CDirection::Write);
///     let mut buffer = unsafe { slice::from_raw_parts_mut(buffer, len) };
///     assert_eq!(buffer, [1u8, 2, 3, 4]);
///     len
/// }
/// let mut slave = i2c_init_slave(10u16, connection.as_ptr(), bus.as_ptr(), callback);
///
/// let data = &[1u8, 2, 3, 4];
///
/// let result = i2c_write(master, 10u16, data.as_ptr(), 4);
/// assert_eq!(result, I2CStatus::Success);
///
/// i2c_close(&mut slave as *mut *mut I2CHandle);
/// i2c_close(&mut master as *mut *mut I2CHandle);
/// ```
pub fn i2c_write(i2c: *mut I2CHandle, address: u16, wbuf: *const u8, wlen: usize) -> I2CStatus {
    unsafe { NE_i2c_write(i2c, address, wbuf, wlen) }
}

/// This function performs an I2C transaction, which consists of a write followed by a read.
///
/// # Arguments
///
/// * `i2c`: Handle to an I2C master
/// * `address`: Address of the slave in this transaction
/// * `wbuf`: A buffer from which the slave will read data. Must contain at lease `wlen`
///     consecutive bytes of valid memory.
/// * `wlen`: Number of bytes to write
/// * `rbuf`: A buffer in which the read data will be stored. Must contain at lease `rlen`
///     consecutive bytes of valid memory.
/// * `rlen`: Maximum number of bytes to read
///
/// # Examples
///
/// See [`nosengine-rust::ffi::i2c`](../i2c/index.html#examples)
pub fn i2c_transaction(
    i2c: *mut I2CHandle,
    address: u16,
    wbuf: *const u8,
    wlen: usize,
    rbuf: *mut u8,
    rlen: usize,
) -> I2CStatus {
    unsafe { NE_i2c_transaction(i2c, address, wbuf, wlen, rbuf, rlen) }
}

#[derive(Debug, Eq, PartialEq)]
#[repr(C)]
#[allow(missing_docs)]
pub enum I2CStatus {
    Success,
    Failure,
}

#[derive(Debug, Eq, PartialEq)]
#[repr(C)]
#[allow(missing_docs)]
pub enum I2CDirection {
    Write,
    Read,
}

/// This enum represents a handle to an opaque C struct.
pub enum I2CHandle {}

extern "C" {
    fn NE_i2c_init_master(
        address: u16,
        connection: *const c_char,
        bus: *const c_char,
    ) -> *mut I2CHandle;
    fn NE_i2c_init_slave(
        address: u16,
        connection: *const c_char,
        bus: *const c_char,
        callback: extern "C" fn(I2CDirection, *mut u8, usize) -> usize,
    ) -> *mut I2CHandle;
    fn NE_i2c_close(i2c: *mut *mut I2CHandle);
    fn NE_i2c_read(i2c: *mut I2CHandle, address: u16, rbuf: *mut u8, rlen: usize) -> I2CStatus;
    fn NE_i2c_write(i2c: *mut I2CHandle, address: u16, wbuf: *const u8, wlen: usize) -> I2CStatus;
    fn NE_i2c_transaction(
        i2c: *mut I2CHandle,
        address: u16,
        wbuf: *const u8,
        wlen: usize,
        rbuf: *mut u8,
        rlen: usize,
    ) -> I2CStatus;
}
