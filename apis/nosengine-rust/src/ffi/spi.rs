//! Contains thin wrappers over the NOSEngine SPI API.
//!
//! # Examples
//!
//! ```
//! # extern crate nosengine_rust;
//! # use nosengine_rust::ffi::spi::*;
//! # use std::ffi::CString;
//! # use std::slice;
//! let connection = CString::new("tcp://localhost:12001").unwrap();
//! let bus = CString::new("spi10").unwrap();
//! let mut master = spi_init_master(connection.as_ptr(), bus.as_ptr());
//!
//! extern "C" fn callback(dir: SPIDirection, buffer: *mut u8, len: usize) -> usize {
//!     let mut buffer = unsafe { slice::from_raw_parts_mut(buffer, len) };
//!     match dir {
//!         SPIDirection::Read => {
//!             for i in 0u8..4u8 {
//!                 buffer[i as usize] = i + 5;
//!             }
//!             len
//!         },
//!         SPIDirection::Write => {
//!             assert_eq!(buffer, &[1u8, 2, 3, 4]);
//!             len
//!         }
//!     }
//! }
//! let mut slave = spi_init_slave(1, connection.as_ptr(), bus.as_ptr(), callback);
//!
//! let mut wbuf = vec![1u8, 2u8, 3u8, 4u8];
//! let mut rbuf = vec![0u8, 0u8, 0u8, 0u8];
//!
//! spi_select_chip(master, 1u8);
//! let result = spi_transaction(master, wbuf.as_ptr(), 4, rbuf.as_mut_ptr(), 4);
//! assert_eq!(result, SPIStatus::Success);
//! assert_eq!(rbuf, &[5u8, 6, 7, 8]);
//!
//! spi_close(&mut slave as *mut *mut SPIHandle);
//! spi_close(&mut master as *mut *mut SPIHandle);
//! ```

use libc::c_char;

/// Initializes an SPI master on the given bus. Only one SPI master can exist on a bus.
///
/// # Arguments
///
/// * `connection`: NOSEngine connection string
/// * `bus`: Name of bus to use for SPI
///
/// # Examples
/// ```
/// # extern crate nosengine_rust;
/// # use nosengine_rust::ffi::spi::*;
/// # use std::ffi::CString;
/// let connection = CString::new("tcp://localhost:12001").unwrap();
/// let bus = CString::new("spi1").unwrap();
/// let mut master = spi_init_master(connection.as_ptr(), bus.as_ptr());
/// assert!(!master.is_null());
/// let mut master2 = spi_init_master(connection.as_ptr(), bus.as_ptr());
/// assert!(master2.is_null());
/// spi_close(&mut master as *mut *mut SPIHandle);
/// spi_close(&mut master2 as *mut *mut SPIHandle);
/// ```
pub fn spi_init_master(connection: *const c_char, bus: *const c_char) -> *mut SPIHandle {
    unsafe { NE_spi_init_master(connection, bus) }
}

/// Initializes an SPI slave on the given bus, with the given chip select.
///
/// # Arguments
///
/// * `cs`: Chip select number. Each SPI slave on a bus must have a unique CS number
/// * `connection`: NOSEngine connection string
/// * `bus`: Name of the bus to use for SPI
/// * `callback`: Function which will be called every time a master reads or writes to this slave.
///     This callback should return the number of bytes it has read or written.
///     The callback's parameters are:
///     * `SPIDirection`: Indicates whether the master is reading or writing
///     * `*mut u8`: This is a buffer. If the master is writing, then the slave should read the
///         data from this buffer. If the master is reading, then the slave should write into
///         this buffer.
///     * `usize`: The number of bytes that should be read or written
///
/// # Examples
///
/// ```
/// # extern crate nosengine_rust;
/// # use nosengine_rust::ffi::spi::*;
/// # use std::ffi::CString;
/// let connection = CString::new("tcp://localhost:12001").unwrap();
/// let bus = CString::new("spi2").unwrap();
///
/// extern "C" fn callback(dir: SPIDirection, buffer: *mut u8, len: usize) -> usize {0}
///
/// let mut slave = spi_init_slave(1, connection.as_ptr(), bus.as_ptr(), callback);
/// assert!(!slave.is_null());
/// spi_close(&mut slave as *mut *mut SPIHandle);
/// ```
pub fn spi_init_slave(
    cs: u8,
    connection: *const c_char,
    bus: *const c_char,
    callback: extern "C" fn(SPIDirection, *mut u8, usize) -> usize,
) -> *mut SPIHandle {
    unsafe { NE_spi_init_slave(cs, connection, bus, callback) }
}

/// Closes an SPI connection and frees up all associated memory.
///
/// # Arguments
///
/// * `spi`: A pointer to a pointer to an SPI handle. This handle will be made null by this
///     function.
///
/// # Examples
/// ```
/// # extern crate nosengine_rust;
/// # use nosengine_rust::ffi::spi::*;
/// # use std::ffi::CString;
/// let connection = CString::new("tcp://localhost:12001").unwrap();
/// let bus = CString::new("spi3").unwrap();
/// let mut master = spi_init_master(connection.as_ptr(), bus.as_ptr());
/// assert!(!master.is_null());
/// spi_close(&mut master as *mut *mut SPIHandle);
/// assert!(master.is_null());
/// ```
pub fn spi_close(spi: *mut *mut SPIHandle) {
    unsafe { NE_spi_close(spi) }
}

/// When an SPI master writes, it only writes to the currently-selected chip. If no chip is
/// selected, the write does nothing.
///
/// # Arguments
///
/// * `spi`: Handle to and SPI master
/// * `cs`: The number of the chip you want to select
pub fn spi_select_chip(spi: *mut SPIHandle, cs: u8) {
    unsafe { NE_spi_select_chip(spi, cs) }
}

/// De-selects the currently-selected chip
///
/// # Arguments
///
/// * `spi`: Handle to and SPI master
pub fn spi_unselect_chip(spi: *mut SPIHandle) {
    unsafe { NE_spi_unselect_chip(spi) }
}

/// Reads from the currently-selected chip. Does nothing and returns an error if there is no
/// currently-selected chip.
///
/// # Arguments
///
/// * `spi`: Handle to an SPI master
/// * `rbuf`: A buffer in which the read data will be stored. Must contain at least `rlen`
///     consecutive bytes of valid memory.
/// * `rlen`: Maximum number of bytes to read
///
/// # Examples
///
/// ```
/// # extern crate nosengine_rust;
/// # use nosengine_rust::ffi::spi::*;
/// # use std::ffi::CString;
/// # use std::slice;
/// let connection = CString::new("tcp://localhost:12001").unwrap();
/// let bus = CString::new("spi4").unwrap();
/// let mut master = spi_init_master(connection.as_ptr(), bus.as_ptr());
///
/// extern "C" fn callback(dir: SPIDirection, buffer: *mut u8, len: usize) -> usize {
///     assert_eq!(dir, SPIDirection::Read);
///     let mut buffer = unsafe { slice::from_raw_parts_mut(buffer, len) };
///     for i in 1u8..5u8 {
///         buffer[(i - 1) as usize] = i;
///     }
///     len
/// }
/// let mut slave = spi_init_slave(1, connection.as_ptr(), bus.as_ptr(), callback);
///
/// let mut buffer = vec![0u8, 0u8, 0u8, 0u8];
///
/// let result = spi_read(master, buffer.as_mut_ptr(), 4);
/// // Expect a failure because no chip was selected.
/// assert_eq!(result, SPIStatus::Failure);
///
/// spi_select_chip(master, 1u8);
/// let result = spi_read(master, buffer.as_mut_ptr(), 4);
/// assert_eq!(result, SPIStatus::Success);
/// assert_eq!(buffer, &[1u8, 2, 3, 4]);
///
/// spi_close(&mut slave as *mut *mut SPIHandle);
/// spi_close(&mut master as *mut *mut SPIHandle);
/// ```
pub fn spi_read(spi: *mut SPIHandle, rbuf: *mut u8, rlen: usize) -> SPIStatus {
    unsafe { NE_spi_read(spi, rbuf, rlen) }
}

/// Writes to the currently-selected chip. Does nothing and returns an error if there is no
/// currently-selected chip.
///
/// # Arguments
///
/// * `spi`: Handle to an SPI master
/// * `wbuf`: A buffer from which the slave will read data. Must contain at least `wlen`
///     consecutive bytes of valid memory.
/// * `wlen`: Number of bytes to write
///
/// # Examples
///
/// ```
/// # extern crate nosengine_rust;
/// # use nosengine_rust::ffi::spi::*;
/// # use std::ffi::CString;
/// # use std::slice;
/// let connection = CString::new("tcp://localhost:12001").unwrap();
/// let bus = CString::new("spi5").unwrap();
/// let mut master = spi_init_master(connection.as_ptr(), bus.as_ptr());
///
/// extern "C" fn callback(dir: SPIDirection, buffer: *mut u8, len: usize) -> usize {
///     assert_eq!(dir, SPIDirection::Write);
///     let mut buffer = unsafe { slice::from_raw_parts_mut(buffer, len) };
///     assert_eq!(buffer, [1u8, 2, 3, 4]);
///     len
/// }
/// let mut slave = spi_init_slave(1, connection.as_ptr(), bus.as_ptr(), callback);
///
/// let data = &[1u8, 2, 3, 4];
///
/// let result = spi_write(master, data.as_ptr(), 4);
/// // Expect a failure because no chip was selected.
/// assert_eq!(result, SPIStatus::Failure);
///
/// spi_select_chip(master, 1u8);
/// let result = spi_write(master, data.as_ptr(), 4);
/// assert_eq!(result, SPIStatus::Success);
///
/// spi_close(&mut slave as *mut *mut SPIHandle);
/// spi_close(&mut master as *mut *mut SPIHandle);
/// ```
pub fn spi_write(spi: *mut SPIHandle, wbuf: *const u8, wlen: usize) -> SPIStatus {
    unsafe { NE_spi_write(spi, wbuf, wlen) }
}

/// Performs an SPI transaction, which consists of a write followed by a read.
///
/// # Arguments
///
/// * `spi`: Handle to an SPI master
/// * `wbuf`: A buffer from which the slave will read data. Must contain at least `wlen`
///     consecutive bytes of valid memory.
/// * `wlen`: Number of bytes to write
/// * `rbuf`: A buffer in which the read data will be stored. Must contain at least `rlen`
///     consecutive bytes of valid memory.
/// * `rlen`: Maximum number of bytes to read
///
/// # Examples
///
/// See [`nosengine-rust::ffi::spi`](../spi/index.html#examples)
pub fn spi_transaction(
    spi: *mut SPIHandle,
    wbuf: *const u8,
    wlen: usize,
    rbuf: *mut u8,
    rlen: usize,
) -> SPIStatus {
    unsafe { NE_spi_transaction(spi, wbuf, wlen, rbuf, rlen) }
}

#[derive(Debug, Eq, PartialEq)]
#[repr(C)]
/// This enum represents the success or failure of an SPI action.
pub enum SPIStatus {
    /// Success
    Success,
    /// Failure
    Failure,
}

#[derive(Debug, Eq, PartialEq)]
#[repr(C)]
/// This enum represents the type of SPI operation.
pub enum SPIDirection {
    /// Write to slave
    Write,
    /// Read from slave
    Read,
}

/// This enum represents an opaque handle to either an SPI slave or and SPI master. You must keep track
/// of where you got this handle from to know whether it is a master or slave.
pub enum SPIHandle {}

extern "C" {
    fn NE_spi_init_master(connection: *const c_char, bus: *const c_char) -> *mut SPIHandle;
    fn NE_spi_init_slave(
        cs: u8,
        connection: *const c_char,
        bus: *const c_char,
        callback: extern "C" fn(SPIDirection, *mut u8, usize) -> usize,
    ) -> *mut SPIHandle;
    fn NE_spi_close(spi: *mut *mut SPIHandle);
    fn NE_spi_select_chip(spi: *mut SPIHandle, cs: u8);
    fn NE_spi_unselect_chip(spi: *mut SPIHandle);
    fn NE_spi_read(spi: *mut SPIHandle, rbuf: *mut u8, rlen: usize) -> SPIStatus;
    fn NE_spi_write(spi: *mut SPIHandle, wbuf: *const u8, wlen: usize) -> SPIStatus;
    fn NE_spi_transaction(
        spi: *mut SPIHandle,
        wbuf: *const u8,
        wlen: usize,
        rbuf: *mut u8,
        rlen: usize,
    ) -> SPIStatus;
}
