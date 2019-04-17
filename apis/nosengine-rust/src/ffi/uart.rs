//! This module contains the wrappers around the C functions associated with UART.

use libc::{c_char, c_void};

/// This enum represents a pointer to an opaque C struct.
pub enum UARTHandle {}

/// This function creates a new UART connection. Only two UART connections can be created on the same port
/// on the same bus. If a third is attempted, this function will return a null pointer.
///
/// # Arguments
///
/// * `name`: Name of this UART connection. Must be unique on a bus
/// * `connection`: Connection string to the NOSEngine server
/// * `bus`: Name of the bus that will hold this UART connection
/// * `port`: A number representing a serial port
///
/// # Safety
///
/// After a UART connection is created, it must be cleaned up using `uart_close`.
///
/// # Examples
///
/// ```
/// # extern crate nosengine_rust;
/// # use nosengine_rust::ffi::uart::*;
/// # use std::ffi::CString;
/// let conn = CString::new("tcp://localhost:12001").unwrap();
/// let bus = CString::new("uart").unwrap();
/// let name1 = CString::new("uart1").unwrap();
/// let name2 = CString::new("uart2").unwrap();
/// let name3 = CString::new("uart3").unwrap();
///
/// let mut uart1 = uart_open(name1.as_ptr(), conn.as_ptr(), bus.as_ptr(), 1);
/// assert!(!uart1.is_null());
/// let mut uart2 = uart_open(name2.as_ptr(), conn.as_ptr(), bus.as_ptr(), 1);
/// assert!(!uart2.is_null());
/// let mut uart3 = uart_open(name3.as_ptr(), conn.as_ptr(), bus.as_ptr(), 1);
/// assert!(uart3.is_null());
///
/// # uart_close(&mut uart1 as *mut *mut UARTHandle);
/// # uart_close(&mut uart2 as *mut *mut UARTHandle);
/// # uart_close(&mut uart3 as *mut *mut UARTHandle);
/// ```
pub fn uart_open(
    name: *const c_char,
    connection: *const c_char,
    bus: *const c_char,
    port: u8,
) -> *mut UARTHandle {
    unsafe { NE_uart_open(name, connection, bus, port) }
}

/// This function closes a UART connection and cleans up all associated memory. After running this function,
/// the pointer becomes invalid.
///
/// # Arguments
///
/// * `uart`: Pointer to a UART handle, which is made null by this function
///
/// # Safety
///
/// If any functions are called on the UART handle after closing it, they will fail.
///
/// # Examples
/// ```
/// # extern crate nosengine_rust;
/// # use nosengine_rust::ffi::uart::*;
/// # use std::ffi::CString;
/// let conn = CString::new("tcp://localhost:12001").unwrap();
/// let bus = CString::new("uart").unwrap();
/// let name1 = CString::new("uart4").unwrap();
/// let name2 = CString::new("uart5").unwrap();
///
/// let mut uart1 = uart_open(name1.as_ptr(), conn.as_ptr(), bus.as_ptr(), 2);
/// let mut uart2 = uart_open(name2.as_ptr(), conn.as_ptr(), bus.as_ptr(), 2);
///
/// uart_close(&mut uart1 as *mut *mut UARTHandle);
/// assert!(uart1.is_null());
/// uart_close(&mut uart2 as *mut *mut UARTHandle);
/// assert!(uart2.is_null());
/// ```
pub fn uart_close(uart: *mut *mut UARTHandle) -> UARTStatus {
    unsafe { NE_uart_close(uart) }
}

/// This function sets a callback which will execute whenever this UART receives data.
///
/// # Arguments
///
/// * `uart`: Pointer to the UART
/// * `callback`: Callback which has the following parameters:
///     * `data`: Bytes that were written
///     * `len`: Number of bytes that were written
///     * `user`: User-specified data that is passed every time this callback runs
/// * `user`: User-specified data that will be passed to the callback every time it runs
///
/// # Examples
///
/// ```
/// # extern crate nosengine_rust;
/// # extern crate libc;
/// # use nosengine_rust::ffi::uart::*;
/// # use std::ffi::CString;
/// # use libc::c_void;
/// # use std::{slice, ptr};
/// # let conn = CString::new("tcp://localhost:12001").unwrap();
/// # let bus = CString::new("uart").unwrap();
/// # let name1 = CString::new("uart6").unwrap();
/// # let name2 = CString::new("uart7").unwrap();
/// let mut uart1 = uart_open(name1.as_ptr(), conn.as_ptr(), bus.as_ptr(), 3);
/// let mut uart2 = uart_open(name2.as_ptr(), conn.as_ptr(), bus.as_ptr(), 3);
///
/// extern "C" fn callback(data: *const u8, len: usize, user: *mut c_void){
///     let data = unsafe{ slice::from_raw_parts(data, len) };
///     assert_eq!(data, &[1u8, 2, 3, 4]);
/// }
/// uart_set_read_callback(uart2, callback, ptr::null_mut());
/// uart_write(uart1, [1u8, 2, 3, 4].as_ptr(), 4);
///
/// # uart_close(&mut uart1 as *mut *mut UARTHandle);
/// # uart_close(&mut uart2 as *mut *mut UARTHandle);
/// ```
pub fn uart_set_read_callback(
    uart: *mut UARTHandle,
    callback: extern "C" fn(data: *const u8, len: usize, _user: *mut c_void),
    user: *mut c_void,
) {
    unsafe {
        NE_uart_set_read_callback(uart, callback, user);
    }
}

/// This function reads data that has been sent to this UART.
///
/// # Arguments
///
/// * `uart`: Handle to the UART
/// * `buffer`: A buffer which must contain at lease `len` consecutive bytes of valid memory
/// * `len`: Maximum number of bytes to read. This function may read fewer than this, but
///     never more.
///
/// # Examples
///
/// ```
/// # extern crate nosengine_rust;
/// # use nosengine_rust::ffi::uart::*;
/// # use std::ffi::CString;
/// # let conn = CString::new("tcp://localhost:12001").unwrap();
/// # let bus = CString::new("uart").unwrap();
/// # let name1 = CString::new("uart8").unwrap();
/// # let name2 = CString::new("uart9").unwrap();
/// let mut uart1 = uart_open(name1.as_ptr(), conn.as_ptr(), bus.as_ptr(), 4);
/// let mut uart2 = uart_open(name2.as_ptr(), conn.as_ptr(), bus.as_ptr(), 4);
///
/// uart_write(uart1, [1u8, 2, 3, 4].as_ptr(), 4);
///
/// let buffer = &mut [0u8, 0, 0, 0];
/// uart_read(uart2, buffer.as_mut_ptr(), 4);
/// assert_eq!(buffer, &[1u8, 2, 3, 4]);
///
/// # uart_close(&mut uart1 as *mut *mut UARTHandle);
/// # uart_close(&mut uart2 as *mut *mut UARTHandle);
/// ```
pub fn uart_read(uart: *mut UARTHandle, buffer: *mut u8, len: usize) -> usize {
    unsafe { NE_uart_read(uart, buffer, len) }
}

/// This function reads one individual character from this UART connection.
/// If there are no characters available, then `UARTStatus::Failure` is returned.
///
/// # Arguments
///
/// * `uart`: UART handle
/// * `c`: Pointer to a u8, where the result, if any, will be stored.
///
/// # Examples
///
/// ```
/// # extern crate nosengine_rust;
/// # use nosengine_rust::ffi::uart::*;
/// # use std::ffi::CString;
/// # let conn = CString::new("tcp://localhost:12001").unwrap();
/// # let bus = CString::new("uart").unwrap();
/// # let name1 = CString::new("uart10").unwrap();
/// # let name2 = CString::new("uart11").unwrap();
/// let mut uart1 = uart_open(name1.as_ptr(), conn.as_ptr(), bus.as_ptr(), 5);
/// let mut uart2 = uart_open(name2.as_ptr(), conn.as_ptr(), bus.as_ptr(), 5);
///
/// uart_putc(uart1, 7u8);
///
/// let mut c = 0u8;
/// let result = uart_getc(uart2, &mut c as *mut u8);
/// assert_eq!(result, UARTStatus::Success);
/// assert_eq!(c, 7u8);
/// # uart_close(&mut uart1 as *mut *mut UARTHandle);
/// # uart_close(&mut uart2 as *mut *mut UARTHandle);
/// ```
pub fn uart_getc(uart: *mut UARTHandle, c: *mut u8) -> UARTStatus {
    unsafe { NE_uart_getc(uart, c) }
}

/// This function writes data to the UART port.
///
/// # Arguments
///
/// * `uart`: UART handle
/// * `buffer`: Buffer to write data from. Must contain at least `length` consecutive bytes
///     of valid memory.
/// * `length`: Max number of bytes to write from `buffer`.
///
/// # Examples
///
/// ```
/// # extern crate nosengine_rust;
/// # use nosengine_rust::ffi::uart::*;
/// # use std::ffi::CString;
/// # let conn = CString::new("tcp://localhost:12001").unwrap();
/// # let bus = CString::new("uart").unwrap();
/// # let name1 = CString::new("uart12").unwrap();
/// # let name2 = CString::new("uart13").unwrap();
/// let mut uart1 = uart_open(name1.as_ptr(), conn.as_ptr(), bus.as_ptr(), 6);
/// let mut uart2 = uart_open(name2.as_ptr(), conn.as_ptr(), bus.as_ptr(), 6);
///
/// uart_write(uart1, [1u8, 2, 3, 4].as_ptr(), 4);
///
/// let buffer = &mut [0u8, 0, 0, 0];
/// uart_read(uart2, buffer.as_mut_ptr(), 4);
/// assert_eq!(buffer, &[1u8, 2, 3, 4]);
///
/// # uart_close(&mut uart1 as *mut *mut UARTHandle);
/// # uart_close(&mut uart2 as *mut *mut UARTHandle);
/// ```
pub fn uart_write(uart: *mut UARTHandle, buffer: *const u8, length: usize) -> usize {
    unsafe { NE_uart_write(uart, buffer, length) }
}

/// This function writes a single byte to the UART port.
///
/// # Arguments
///
/// * `uart`: UART handle
/// * `c`: Byte to write
///
/// # Examples
///
/// ```
/// # extern crate nosengine_rust;
/// # use nosengine_rust::ffi::uart::*;
/// # use std::ffi::CString;
/// # let conn = CString::new("tcp://localhost:12001").unwrap();
/// # let bus = CString::new("uart").unwrap();
/// # let name1 = CString::new("uart14").unwrap();
/// # let name2 = CString::new("uart15").unwrap();
/// let mut uart1 = uart_open(name1.as_ptr(), conn.as_ptr(), bus.as_ptr(), 7);
/// let mut uart2 = uart_open(name2.as_ptr(), conn.as_ptr(), bus.as_ptr(), 7);
///
/// uart_putc(uart1, 7u8);
///
/// let mut c = 0u8;
/// let result = uart_getc(uart2, &mut c as *mut u8);
/// assert_eq!(result, UARTStatus::Success);
/// assert_eq!(c, 7u8);
/// # uart_close(&mut uart1 as *mut *mut UARTHandle);
/// # uart_close(&mut uart2 as *mut *mut UARTHandle);
/// ```
pub fn uart_putc(uart: *mut UARTHandle, c: u8) {
    unsafe { NE_uart_putc(uart, c) }
}

/// This function returns the number of bytes available to be read.
///
/// # Arguments
///
/// * `uart`: UART handle
///
/// # Examples
///
/// ```
/// # extern crate nosengine_rust;
/// # use nosengine_rust::ffi::uart::*;
/// # use std::ffi::CString;
/// # let conn = CString::new("tcp://localhost:12001").unwrap();
/// # let bus = CString::new("uart").unwrap();
/// # let name1 = CString::new("uart16").unwrap();
/// # let name2 = CString::new("uart17").unwrap();
/// let mut uart1 = uart_open(name1.as_ptr(), conn.as_ptr(), bus.as_ptr(), 8);
/// let mut uart2 = uart_open(name2.as_ptr(), conn.as_ptr(), bus.as_ptr(), 8);
///
/// uart_write(uart1, [1u8, 2, 3, 4].as_ptr(), 4);
///
/// assert_eq!(uart_available(uart2), 4);
///
/// # uart_close(&mut uart1 as *mut *mut UARTHandle);
/// # uart_close(&mut uart2 as *mut *mut UARTHandle);
/// ```
pub fn uart_available(uart: *mut UARTHandle) -> usize {
    unsafe { NE_uart_available(uart) }
}

#[derive(Debug, Eq, PartialEq)]
#[repr(C)]
#[allow(missing_docs)]
pub enum UARTStatus {
    Success,
    Failure,
}

extern "C" {
    fn NE_uart_open(
        name: *const c_char,
        connection: *const c_char,
        bus: *const c_char,
        port: u8,
    ) -> *mut UARTHandle;
    fn NE_uart_close(uart: *mut *mut UARTHandle) -> UARTStatus;
    fn NE_uart_set_read_callback(
        uart: *mut UARTHandle,
        callback: extern "C" fn(*const u8, usize, *mut c_void),
        user: *mut c_void,
    );
    fn NE_uart_read(uart: *mut UARTHandle, buffer: *mut u8, len: usize) -> usize;
    fn NE_uart_getc(uart: *mut UARTHandle, c: *mut u8) -> UARTStatus;
    fn NE_uart_write(uart: *mut UARTHandle, buffer: *const u8, length: usize) -> usize;
    fn NE_uart_putc(uart: *mut UARTHandle, c: u8);
    fn NE_uart_available(uart: *mut UARTHandle) -> usize;
}
