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
// Contributed by: Timothy Scott (tmscott@mix.wvu.edu)
//

//! This module wraps the C interface of NOSEngine. It provides the basic functionality
//! for base buses and communication. All functions that are prefixed by `NE_` are the unsafe
//! C functions. All others are safe wrappers around the C functions.
//!
//! # Examples
//!
//! This example demonstrates the use of a callback to asynchronously reply to a message.
//!
//! ```norun
//! extern crate nosengine_rust;
//! use std::ffi::CString;
//! use nosengine_rust::ffi::*;
//! use std::{slice, ptr};
//!
//! let busname = CString::new("testbus").unwrap();
//! let connection = CString::new("tcp://localhost:12001").unwrap();
//! let node1_name = CString::new("node1").unwrap();
//! let node2_name = CString::new("node2").unwrap();
//!
//! let mut bus: *mut BusHandle = create_bus2(busname.as_ptr(), connection.as_ptr());
//!
//! let mut data_node1: *mut DataNodeHandle = create_data_node(bus,node1_name.as_ptr());
//!
//! let mut data_node2: *mut DataNodeHandle = create_data_node(bus,node2_name.as_ptr());
//!
//! extern "C" fn callback(node: *mut DataNodeHandle, msg: *mut MessageHandle){
//! let data = unsafe {
//!     slice::from_raw_parts(
//!     message_get_user_data(msg),
//!     message_get_user_data_length(msg)
//! )};
//! assert_eq!(data, &[1u8, 2, 3, 4]);
//!     data_node_send_reply_message_sync(node, msg, 4, [5u8, 6, 7, 8].as_ptr());
//! }
//! data_node_set_message_received_callback(data_node2, callback);
//!
//! let data = &[1u8, 2, 3, 4];
//! let mut msg: *mut MessageHandle = ptr::null_mut();
//! data_node_send_request_message_sync(
//!     data_node1,
//!     node2_name.as_ptr(),
//!     data.len(),
//!     data.as_ptr(),
//!     &mut msg as *mut *mut MessageHandle
//! );
//!
//! let response = unsafe {
//!     slice::from_raw_parts(
//!         message_get_user_data(msg),
//!         message_get_user_data_length(msg)
//!     )
//! };
//! assert_eq!(response, &[5u8, 6, 7, 8]);
//!
//! destroy_message(&mut msg as *mut *mut MessageHandle);
//! destroy_data_node(bus, &mut data_node1 as *mut *mut DataNodeHandle);
//! destroy_data_node(bus, &mut data_node2 as *mut *mut DataNodeHandle);
//! destroy_bus(&mut bus as *mut *mut BusHandle);
//! ```
//!
//! This example uses a separate thread with a second data node synchronously waiting for a message.
//!
//! ```norun
//! # extern crate nosengine_rust;
//! # use std::ffi::CString;
//! # use nosengine_rust::ffi::*;
//! # use std::{slice, ptr, thread, time};
//!
//! let bus_name = CString::new("testbus").unwrap();
//! let connection = CString::new("tcp://localhost:12001").unwrap();
//! let mut bus: *mut BusHandle = create_bus2(bus_name.as_ptr(), connection.as_ptr());
//!
//! let node1_name = CString::new("node3").unwrap();
//! let mut node1: *mut DataNodeHandle = create_data_node(bus, node1_name.as_ptr());
//!
//! let node2_name = CString::new("node4").unwrap();
//!
//!
//! let join_handle = thread::spawn( move ||{
//!     let mut bus: *mut BusHandle = create_bus2(bus_name.as_ptr(), connection.as_ptr());
//!
//!     let node2_name = CString::new("node4").unwrap();
//!     let mut node2: *mut DataNodeHandle = create_data_node(bus, node2_name.as_ptr());
//!
//!     println!("I'm in the thread now!");
//!
//!     let mut msg = data_node_receive_message_sync(node2);
//!     let data = unsafe {
//!         slice::from_raw_parts(
//!             message_get_user_data(msg),
//!             message_get_user_data_length(msg)
//!         )
//!     };
//!     assert_eq!(data, [1u8, 2, 3, 4]);
//!     destroy_message(&mut msg as *mut *mut MessageHandle);
//!     destroy_data_node(bus, &mut node2 as *mut *mut DataNodeHandle);
//!     destroy_bus(&mut bus as *mut *mut BusHandle);
//! });
//!
//! // Wait to make sure the other node has been created
//! thread::sleep(time::Duration::from_millis(100));
//!
//! let data = [1u8, 2, 3, 4];
//! data_node_send_message_sync(node1, node2_name.as_ptr(), data.len(), data.as_ptr());
//!
//! join_handle.join().unwrap();
//!
//! destroy_data_node(bus, &mut node1 as *mut *mut DataNodeHandle);
//! destroy_bus(&mut bus as *mut *mut BusHandle);
//! ```

pub mod i2c;
pub mod spi;
pub mod uart;

use libc::c_char;

/// This represents a pointer to an opaque C struct.
pub enum BusHandle {}

/// This represents a pointer to an opaque C struct.
pub enum DataNodeHandle {}

/// This represents a pointer to an opaque C struct.
pub enum MessageHandle {}

/// This returns the most recent error encountered by NOSEngine.
pub fn error() -> ErrorCode {
    unsafe { NE_error() }
}

/// This returns a string describing the most recent error encountered by NOSEngine.
/// Note: No guarantees can be made about the lifetime of this pointer, so if needed, make a copy
/// of the contents as soon as possible.
pub fn error_string() -> *const c_char {
    unsafe { NE_error_string() }
}

/// Creates a new bus, or connects to an existing bus.
///
/// # Arguments
///
/// * `name`: Name of the bus to create, as a null-terminated C string
/// * `server_uri`: Connection string to server, as null-terminated C string
///
/// # Examples
///
/// ```norun
/// # extern crate nosengine_rust;
/// # use std::ffi::CString;
/// # use nosengine_rust::ffi::*;
/// #
/// let bus_name = CString::new("testbus").unwrap();
/// let connection = CString::new("tcp://localhost:12001").unwrap();
///
/// let mut bus: *mut BusHandle = create_bus2(bus_name.as_ptr(), connection.as_ptr());
/// assert!(!bus.is_null());
/// # destroy_bus(&mut bus as *mut *mut BusHandle);
/// ```
pub fn create_bus2(name: *const c_char, server_uri: *const c_char) -> *mut BusHandle {
    unsafe { NE_create_bus2(name, server_uri) }
}

/// This function cleans up the memory of a `*mut BusHandle`.
///
/// # Arguments
///
/// * `bus`: Mutable pointer to the pointer to the bus. After this function is run, the
///     pointer to the bus will be made `null`.
///
/// # Examples
///
/// ```norun
/// # extern crate nosengine_rust;
/// # use std::ffi::CString;
/// # use nosengine_rust::ffi::*;
/// #
/// let bus_name = CString::new("testbus").unwrap();
/// let connection = CString::new("tcp://localhost:12001").unwrap();
///
/// let mut bus: *mut BusHandle = create_bus2(bus_name.as_ptr(), connection.as_ptr());
///
/// destroy_bus(&mut bus as *mut *mut BusHandle);
/// assert!(bus.is_null());
/// ```
pub fn destroy_bus(bus: *mut *mut BusHandle) {
    unsafe { NE_destroy_bus(bus) }
}

/// This function creates a pointer to an opaque data node struct.
///
/// Note: If a node with the given name already exists on the given bus, then this function
/// returns a pointer to the existing node. Therefore, if two nodes are created with the same
/// name, and one is freed, then the other becomes invalid.
///
/// # Arguments
///
/// * `bus`: Mutable pointer to the bus on which this node is to be created
/// * `name`: Name of the data node, as null-terminated C string
///
/// # Examples
///
/// ```norun
/// # extern crate nosengine_rust;
/// # use nosengine_rust::ffi::*;
/// # use std::ffi::CString;
/// #
/// # let bus_name = CString::new("testbus").unwrap();
/// # let connection = CString::new("tcp://localhost:12001").unwrap();
/// #
/// # let mut bus: *mut BusHandle = create_bus2(bus_name.as_ptr(), connection.as_ptr());
/// #
/// let node1_name = CString::new("node5").unwrap();
///
/// let mut node1: *mut DataNodeHandle = create_data_node(bus, node1_name.as_ptr());
///
/// assert!(!node1.is_null());
/// #
/// # destroy_data_node(bus, &mut node1 as *mut *mut DataNodeHandle);
/// # destroy_bus(&mut bus as *mut *mut BusHandle);
/// ```
pub fn create_data_node(bus: *mut BusHandle, name: *const c_char) -> *mut DataNodeHandle {
    unsafe { NE_create_data_node(bus, name) }
}

/// This function frees up the memory associated with a data node pointer.
///
/// Note: If another node with the same name was created on the same bus, then it will share
/// the same memory as this data node. Therefore, after this data node is destroyed, the other
/// data node pointer will become invalid.
///
/// # Arguments
///
/// * `bus`: Mutable pointer to the bus on which this node was created
/// * `node`: Mutable pointer to the mutable pointer to the data node. After this function is
///     called, the pointer is made to be null.
///
/// # Examples
///
/// ```norun
/// # extern crate nosengine_rust;
/// # use nosengine_rust::ffi::*;
/// # use std::ffi::CString;
/// #
/// # let bus_name = CString::new("testbus").unwrap();
/// # let connection = CString::new("tcp://localhost:12001").unwrap();
/// #
/// # let mut bus: *mut BusHandle = create_bus2(bus_name.as_ptr(), connection.as_ptr());
/// #
/// let node1_name = CString::new("node6").unwrap();
///
/// let mut node1: *mut DataNodeHandle = create_data_node(bus, node1_name.as_ptr());
///
/// destroy_data_node(bus, &mut node1 as *mut *mut DataNodeHandle);
/// assert!(node1.is_null());
/// # destroy_bus(&mut bus as *mut *mut BusHandle);
/// ```
pub fn destroy_data_node(bus: *mut BusHandle, node: *mut *mut DataNodeHandle) {
    unsafe { NE_destroy_data_node(bus, node) }
}

/// This function synchronously sends a message from one data node to another on the same bus.
///
/// # Arguments
///
/// * `node`: Pointer to a node created using the `create_data_node` function
/// * `dest`: Name of the node to which this message should be sent, as a null-terminated C string
/// * `length`: Size of the message to be sent, in bytes
/// * `data`: Pointer to the array of bytes to be sent. There must be at least `length` bytes of
///     valid memory after this pointer
///
/// # Examples
///
/// ```norun
/// # extern crate nosengine_rust;
/// # use nosengine_rust::ffi::*;
/// # use std::ffi::CString;
/// #
/// # let bus_name = CString::new("testbus").unwrap();
/// # let connection = CString::new("tcp://localhost:12001").unwrap();
/// #
/// # let mut bus: *mut BusHandle = create_bus2(bus_name.as_ptr(), connection.as_ptr());
/// #
/// let node1_name = CString::new("node7").unwrap();
/// let mut node1: *mut DataNodeHandle = create_data_node(bus, node1_name.as_ptr());
///
/// let node2_name = CString::new("node8").unwrap();
/// let mut node2: *mut DataNodeHandle = create_data_node(bus, node2_name.as_ptr());
///
/// let data = &[1u8, 2, 3, 4];
/// data_node_send_message_sync(
///     node1,
///     node2_name.as_ptr(),
///     data.len(),
///     data.as_ptr()
///  );
///
/// assert_eq!(error(), ErrorCode::Ok);
/// #
/// # destroy_data_node(bus, &mut node1 as *mut *mut DataNodeHandle);
/// # destroy_data_node(bus, &mut node2 as *mut *mut DataNodeHandle);
/// # destroy_bus(&mut bus as *mut *mut BusHandle);
/// ```
pub fn data_node_send_message_sync(
    node: *mut DataNodeHandle,
    dest: *const c_char,
    length: usize,
    data: *const u8,
) {
    unsafe { NE_data_node_send_message_sync(node, dest, length, data) }
}

/// This function synchronously sends a message, and blocks until the recipient replies using
/// `data_node_send_reply_message_sync`
///
/// # Arguments
///
/// * `node`: Pointer to a node created using the `create_data_node` function
/// * `destination`: Name of the node to which this message should be sent, as a null-terminated C string
/// * `length`: Size of the message to be sent, in bytes
/// * `data`: Pointer to the array of bytes to be sent. There must be at least `length` bytes of
///     valid memory after this pointer
/// * `response`: Pointer to an empty pointer to a MessageHandle. This pointer is modified to point
///     to a valid `MessageHandle` by this function
///
/// # Safety
///
/// You MUST eventually call `destroy_message` on the response message placed into the
/// `response` pointer, or it will be a memory leak.
///
/// # Examples
///
/// See [`nosengine-rust::ffi`](../ffi/index.html#examples)
pub fn data_node_send_request_message_sync(
    node: *mut DataNodeHandle,
    destination: *const c_char,
    length: usize,
    data: *const u8,
    response: *mut *mut MessageHandle,
) {
    unsafe {
        NE_data_node_send_request_message_sync(node, destination, length, data, response);
    }
}

/// This function sends a reply message. When a node sends a message using `data_node_send_request_message_sync`,
/// it will block until the recipient replies using this function.
///
/// # Arguments
///
/// * `node`: Pointer to a node created using the `create_data_node` function
/// * `original_message`: Pointer to the message which this node received, and which is awaiting
///     a response.
/// * `length`: Size of the message to be sent, in bytes
/// * `data`: Pointer to the array of bytes to be sent. There must be at least `length` bytes of
///     valid memory after this pointer
///
/// # Examples
///
/// See [`nosengine-rust::ffi`](../ffi/index.html#examples)
pub fn data_node_send_reply_message_sync(
    node: *mut DataNodeHandle,
    original_message: *const MessageHandle,
    length: usize,
    data: *const u8,
) {
    unsafe { NE_data_node_send_reply_message_sync(node, original_message, length, data) }
}

/// This function sets a callback function which will be called when this node receives a message.
///
/// # Safety
///
/// You must NOT call any `destroy_` functions on the two pointers passed to this callback.
/// NOSEngine takes care of freeing them after the callback returns.
///
/// # Arguments
///
/// * `node`: Pointer to the node with which this callback will be associated
/// * `callback`: The function which will be called when a message is received.
///     * Callback parameters:
///         * `*mut DataNodeHandle`: Pointer to the node to which this callback is attached, which just
///             received a message
///         * `*mut MessageHandle`: Pointer to the message this node just received. Do NOT call
///             `destroy_message()` on this pointer within this callback.
///
/// # Examples
///
/// See [`nosengine-rust::ffi`](../ffi/index.html#examples)
pub fn data_node_set_message_received_callback(
    node: *mut DataNodeHandle,
    callback: extern "C" fn(*mut DataNodeHandle, *mut MessageHandle),
) {
    unsafe { NE_data_node_set_message_received_callback(node, callback) }
}

/// This function blocks until a message is received. If a message is waiting in the queue, then this function
/// returns it immediately.
///
/// # Arguments
///
/// * `node`: Node to wait for messages on
///
/// # Examples
///
/// See [`nosengine-rust::ffi`](../ffi/index.html#examples)
pub fn data_node_receive_message_sync(node: *mut DataNodeHandle) -> *mut MessageHandle {
    unsafe { NE_data_node_receive_message_sync(node) }
}

/// This function returns the number of bytes of data in a message.
///
/// # Arguments
///
/// * `message`: Pointer to a message returned by one of the communication functions
///
/// # Example
///
/// See [`nosengine-rust::ffi`](../ffi/index.html#examples)
pub fn message_get_user_data_length(message: *const MessageHandle) -> usize {
    unsafe { NE_message_get_user_data_length(message) }
}

/// This function returns a pointer to the first byte of data in the message.
///
/// # Arguments
///
/// * `message`: Pointer to a message returned by one of the communication functions
///
/// # Example
///
/// See [`nosengine-rust::ffi`](../ffi/index.html#examples)
pub fn message_get_user_data(message: *const MessageHandle) -> *const u8 {
    unsafe { NE_message_get_user_data(message) }
}

/// This function destroys a message and frees the associated data.
///
/// # Arguments
///
/// * `message`: Pointer to a pointer to a message. This function makes the pointer null.
///
/// # Example
///
/// See [`nosengine-rust::ffi`](../ffi/index.html#examples)
pub fn destroy_message(message: *mut *mut MessageHandle) {
    unsafe { NE_destroy_message(message) }
}

/// This enum represents all of the different possible errors returned by NOSEngine.
#[derive(Debug, Clone, Eq, PartialEq)]
#[repr(C)]
#[allow(missing_docs)]
pub enum ErrorCode {
    Ok,
    InvalidArg,
    InvalidDest,
    Rejected,
    RoutingFailed,
    InvalidResponseCode,
    Timeout,
    Exception,
    NonException,
    Unimplemented,
    Unknown,
    Count,
}

extern "C" {
    fn NE_error() -> ErrorCode;
    fn NE_error_string() -> *const c_char;

    fn NE_create_bus2(name: *const c_char, server_uri: *const c_char) -> *mut BusHandle;
    fn NE_destroy_bus(bus: *mut *mut BusHandle);
    fn NE_create_data_node(bus: *mut BusHandle, name: *const c_char) -> *mut DataNodeHandle;
    fn NE_destroy_data_node(bus: *mut BusHandle, node: *mut *mut DataNodeHandle);
    fn NE_data_node_send_message_sync(
        node: *mut DataNodeHandle,
        dest: *const c_char,
        length: usize,
        data: *const u8,
    );
    // You MUST destroy the message returned by this function.
    fn NE_data_node_send_request_message_sync(
        node: *mut DataNodeHandle,
        destination: *const c_char,
        length: usize,
        data: *const u8,
        response: *mut *mut MessageHandle,
    );
    fn NE_data_node_send_reply_message_sync(
        node: *mut DataNodeHandle,
        original_message: *const MessageHandle,
        length: usize,
        data: *const u8,
    );
    // Must NOT destroy message passed to this callback
    fn NE_data_node_set_message_received_callback(
        node: *mut DataNodeHandle,
        callback: extern "C" fn(*mut DataNodeHandle, *mut MessageHandle),
    );
    fn NE_data_node_receive_message_sync(node: *mut DataNodeHandle) -> *mut MessageHandle;
    fn NE_message_get_user_data_length(message: *const MessageHandle) -> usize;
    fn NE_message_get_user_data(message: *const MessageHandle) -> *const u8;
    fn NE_destroy_message(message: *mut *mut MessageHandle);
}
