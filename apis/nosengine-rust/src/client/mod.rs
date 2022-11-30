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

//! This module contains wrappers around all of the FFI code to make a cleaner API.
//!
//! This module contains three main data types:
//!
//! * `Bus`: Represents one communication channel, over which `Message`s are sent.
//! * `DataNode`: Any number of `DataNode`s can connect to a `Bus`, and any `DataNode` can send
//!     and receive `Message`s with any other `DataNode` on the same `Bus`.
//! * `Message`: One discrete piece of data, which is passed from one `DataNode` to another.
//!
//! # Examples
//!
//! Using multithreading to send and receive messages. Note that multiple instances of the
//! same `Bus` can be created.
//! ```norun
//! # extern crate nosengine_rust;
//! # use nosengine_rust::client::*;
//! # use std::thread;
//! # use std::time::Duration;
//! let join_handle = thread::spawn(||{
//!     let bus = Bus::new("testbus2", "tcp://localhost:12001").unwrap();
//!     let node = DataNode::new(&bus, "node2").unwrap();
//!     let message = node.receive_message().unwrap();
//!     assert_eq!(message.get_contents(), &[1u8, 2, 3, 4]);
//!     node.send_reply_message(&message, &[5u8, 6, 7, 8]);
//! });
//! thread::sleep(Duration::from_millis(100));
//! let bus = Bus::new("testbus2", "tcp://localhost:12001").unwrap();
//! let node = DataNode::new(&bus, "node1").unwrap();
//! let response = node.send_request_message("node2", &[1u8, 2, 3, 4]).unwrap();
//! assert_eq!(response.get_contents(), &[5u8, 6, 7, 8]);
//! join_handle.join().unwrap();
//! ```
//!
//! Sending and receiving messages on one thread.
//!
//! ```norun
//! # extern crate nosengine_rust;
//! # use nosengine_rust::client::*;
//! let bus = Bus::new("testbus2", "tcp://localhost:12001").unwrap();
//! let node1 = DataNode::new(&bus, "node3").unwrap();
//! let node2 = DataNode::new(&bus, "node4").unwrap();
//!
//! node1.send_message("node4", &[1u8, 2, 3, 4]).unwrap();
//! let msg = node2.receive_message().unwrap();
//! assert_eq!(msg.get_contents(), &[1u8, 2, 3, 4]);
//! ```
//!
//! Sending and receiving messages using a callback.
//!
//! ```norun
//! # extern crate nosengine_rust;
//! # use nosengine_rust::client::*;
//! # use nosengine_rust::ffi;
//! let bus = Bus::new("testbus2", "tcp://localhost:12001").unwrap();
//! let node1 = DataNode::new(&bus, "node5").unwrap();
//! let node2 = DataNode::new(&bus, "node6").unwrap();
//!
//! extern "C" fn callback(node_ptr: *mut ffi::DataNodeHandle, msg_ptr: *mut ffi::MessageHandle){
//!     assert_eq!(unsafe{Message::get_contents_from_ptr(msg_ptr)}, &[1u8, 2, 3, 4]);
//!     DataNode::send_reply_message_ptr(node_ptr, msg_ptr, &[5u8, 6, 7, 8]);
//! }
//! node2.set_message_callback(callback);
//! let response = node1.send_request_message("node6", &[1u8, 2, 3, 4]).unwrap();
//! assert_eq!(response.get_contents(), &[5u8, 6, 7, 8]);
//! ```

pub mod i2c;
pub mod spi;
pub mod uart;

use super::ffi;
use std;
use std::error::Error;
use std::fmt;
use std::sync::Arc;

/// This function returns the most recent NOSEngine error in the current thread.
fn get_nos_error() -> NosError {
    let err_str = unsafe { std::ffi::CStr::from_ptr(ffi::error_string()) };
    match err_str.to_str() {
        Ok(err_cstr) => NosError::NosEngineError {
            error_code: ffi::error(),
            description: String::from(err_cstr),
        },
        Err(_) => NosError::NosEngineError {
            error_code: ffi::ErrorCode::Unknown,
            description: String::from("Unknown error"),
        },
    }
}

/// This enum represents any type of error that can occur when interacting with NOSEngine.
#[derive(Debug, Clone, PartialEq)]
pub enum NosError {
    /// An error occurred when converting a Rust string to a C string.
    /// Specifically, the Rust string contained a null character, which cannot be represented
    /// in C strings.
    StringError {
        /// Description from the underlying std::ffi::NulError
        description: String,
        /// Index in the original string of the problematic null character
        position: usize,
    },
    /// An error was raised by NOSEngine.
    NosEngineError {
        /// What kind of error did NOSEngine raise
        error_code: ffi::ErrorCode,
        /// Description of error, from NOSEngine
        description: String,
    },
}

impl From<std::ffi::NulError> for NosError {
    fn from(err: std::ffi::NulError) -> Self {
        NosError::StringError {
            description: err.to_string(),
            position: err.nul_position(),
        }
    }
}

impl fmt::Display for NosError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NosError::StringError {
                description,
                position,
            } => write!(f, "Null character at index {}: {}", position, description),
            NosError::NosEngineError {
                error_code,
                description,
            } => write!(
                f,
                "NOSEngine internal error {:?}: {}",
                error_code, description
            ),
        }
    }
}

impl Error for NosError {
    fn description(&self) -> &str {
        match self {
            NosError::StringError { description, .. } => &description,
            NosError::NosEngineError { description, .. } => &description,
        }
    }
}

/// This struct represents one bus on the NOSEngine server.
pub struct Bus {
    /// Pointer to the C struct for the bus.
    bus_ptr: *mut ffi::BusHandle,
    /// Name of this bus
    pub name: String,
    /// Connection string to the server
    pub connection: String,
}

impl Bus {
    /// This function constructs a new bus. If the server at the given connection string already has a bus
    /// with the given name, then this new bus simply connects to that same bus. However,
    /// it still references its own memory, independent from the original bus.
    ///
    /// # Arguments
    ///
    /// * `name`: The name of this bus
    /// * `connection`: The NOSEngine connection string. This will generally be of the format
    ///     `tcp://<domain>:<port>`, but can take other forms. See the NOSEngine user manual for
    ///     more information about connection strings.
    ///
    /// # Examples
    ///
    /// ```norun
    /// # extern crate nosengine_rust;
    /// # use nosengine_rust::client::*;
    /// #
    /// let bus = Bus::new("testbus2", "tcp://localhost:12001").expect("Error creating bus");
    /// ```
    pub fn new(name: &str, connection: &str) -> Result<Arc<Bus>, NosError> {
        let c_name = std::ffi::CString::new(name)?;
        let c_connection = std::ffi::CString::new(connection)?;

        let c_bus = ffi::create_bus2(c_name.as_ptr(), c_connection.as_ptr());

        match ffi::error() {
            ffi::ErrorCode::Ok => Ok(Arc::new(Bus {
                bus_ptr: c_bus,
                name: name.to_owned(),
                connection: connection.to_owned(),
            })),
            _ => Err(get_nos_error()),
        }
    }
}

/// When a `Bus` is dropped, it frees the associated pointer to the C struct
impl Drop for Bus {
    fn drop(&mut self) {
        if !self.bus_ptr.is_null() {
            ffi::destroy_bus(&mut self.bus_ptr as *mut *mut ffi::BusHandle);
        }
    }
}

unsafe impl Send for Bus {}

/// This struct is one discrete message passed between data nodes. This struct can only be obtained by
/// using one of the functions in this file for receiving messages.
pub struct Message {
    msg_ptr: *mut ffi::MessageHandle,
}

impl Message {
    /// This function returns the data stored in this message.
    pub fn get_contents(&self) -> &[u8] {
        // This is fine, because Message objects are only constructed within this module,
        // in situations where self.msg_pointer is valid indefinitely
        unsafe { Message::get_contents_from_ptr(self.msg_ptr) }
    }

    /// This function returns the data stored in a `*mut ffi::Message`. This function should only be used
    /// from within a callback set with `DataNode::set_message_callback`.
    ///
    /// # Safety
    ///
    /// The returned slice is only guaranteed to be valid within the scope of the callback from
    /// which this function was called.
    pub unsafe fn get_contents_from_ptr<'a>(msg_ptr: *mut ffi::MessageHandle) -> &'a [u8] {
        std::slice::from_raw_parts(
            ffi::message_get_user_data(msg_ptr),
            ffi::message_get_user_data_length(msg_ptr),
        )
    }
}

impl Drop for Message {
    fn drop(&mut self) {
        if !self.msg_ptr.is_null() {
            ffi::destroy_message(&mut self.msg_ptr as *mut *mut ffi::MessageHandle);
        }
    }
}

/// `DataNode`s send data to each other over buses.
pub struct DataNode {
    node_ptr: *mut ffi::DataNodeHandle,
    bus_ptr: Arc<Bus>,
    /// Name of this data node
    pub name: String,
}

impl DataNode {
    /// Creates a data node on the supplied bus. Data node names need to be unique, but no error will be
    /// thrown if they are not unique! For example, the following code will result in an invalid
    /// memory access:
    ///
    /// ```norun
    /// # extern crate nosengine_rust;
    /// # use nosengine_rust::client::*;
    /// let bus = Bus::new("testbus2", "tcp://localhost:12001").unwrap();
    /// let node1 = DataNode::new(&bus, "node").unwrap();
    /// let node2 = DataNode::new(&bus, "node2").unwrap();
    /// {
    ///     let node3 = DataNode::new(&bus, "node").unwrap();
    /// }
    /// node1.send_message("node2", &[1u8, 2, 3]).unwrap();
    /// ```
    ///
    /// This is because, when a `DataNode` is dropped, it destroys the associated C struct. Then,
    /// when the duplicate `DataNode` tries to use its pointer to this same C struct, it is now
    /// an invalid pointer.
    ///
    /// # Arguments
    /// * `name`: Name of the node to be created. Must be unique on a bus.
    ///
    /// # Examples
    /// ```norun
    /// # extern crate nosengine_rust;
    /// # use nosengine_rust::client::*;
    /// let bus = Bus::new("testbus2", "tcp://localhost:12001").unwrap();
    /// let node = DataNode::new(&bus, "testnode").unwrap();
    /// ```
    pub fn new(bus: &Arc<Bus>, name: &str) -> Result<DataNode, NosError> {
        let c_name = std::ffi::CString::new(name)?;

        let c_node = ffi::create_data_node(bus.bus_ptr, c_name.as_ptr());

        match ffi::error() {
            ffi::ErrorCode::Ok => Ok(DataNode {
                node_ptr: c_node,
                bus_ptr: bus.clone(),
                name: name.to_owned(),
            }),
            _ => Err(get_nos_error()),
        }
    }

    /// Sends a message to the specified node. The recipient node does not need to have been created
    /// in the same thread, in the same program, or even at all. If the recipient node does
    /// not exist, the message will simply be lost.
    ///
    /// # Arguments
    ///
    /// * `destination`: The (case-sensitive) name of the recipient node
    /// * `data`: The bytes to be sent to the recipient
    ///
    /// # Examples
    ///
    /// ```norun
    /// # extern crate nosengine_rust;
    /// # use nosengine_rust::client::*;
    /// let bus = Bus::new("testbus2", "tcp://localhost:12001").unwrap();
    /// let node1 = DataNode::new(&bus, "node9").unwrap();
    /// node1.send_message("nowhere", &[1u8, 2, 3, 4]).unwrap();
    /// ```
    ///
    /// See [`nosengine-rust::client`](../client/index.html#examples) for more.
    pub fn send_message(&self, destination: &str, data: &[u8]) -> Result<(), NosError> {
        let destination = std::ffi::CString::new(destination)?;

        ffi::data_node_send_message_sync(
            self.node_ptr,
            destination.as_ptr(),
            data.len(),
            data.as_ptr(),
        );
        match ffi::error() {
            ffi::ErrorCode::Ok => Ok(()),
            _ => Err(get_nos_error()),
        }
    }

    /// This function sends a message to the specified node, and blocks until it receives a response. The
    /// recipient must respond with `DataNode::send_reply_message_ptr` or `DataNode::send_reply_message`, or
    /// else this function will block indefinitely.
    ///
    /// # Arguments
    /// * `destination`: Name of the recipient node
    /// * `data`: Data to be sent
    ///
    /// # Examples
    ///
    /// See [`nosengine-rust::client`](../client/index.html#examples)
    pub fn send_request_message(
        &self,
        destination: &str,
        data: &[u8],
    ) -> Result<Message, NosError> {
        let destination = std::ffi::CString::new(destination)?;

        let mut msg_ptr: *mut ffi::MessageHandle = std::ptr::null_mut();
        ffi::data_node_send_request_message_sync(
            self.node_ptr,
            destination.as_ptr(),
            data.len(),
            data.as_ptr(),
            &mut msg_ptr as *mut *mut ffi::MessageHandle,
        );
        match ffi::error() {
            ffi::ErrorCode::Ok => Ok(Message { msg_ptr }),
            _ => Err(get_nos_error()),
        }
    }

    /// This function sends a reply to a message that was originally sent using `DataNode::send_request_message()`.
    /// This function uses raw pointers instead of the `DataNode` and `Message` types because this
    /// function is usually needed from within a callback, and inside the callback, the Rust
    /// structs are not available.
    ///
    /// # Arguments
    ///
    /// * `node`: Pointer to a data node, which was received as an argument in a callback
    /// * `message`: The message which is being replied to. Also received as an argument to a callback.
    /// * `data`: Bytes to be sent in the reply
    ///
    /// # Examples
    ///
    /// See [`nosengine-rust::client`](../client/index.html#examples).
    pub fn send_reply_message_ptr(
        node: *mut ffi::DataNodeHandle,
        message: *mut ffi::MessageHandle,
        data: &[u8],
    ) {
        ffi::data_node_send_reply_message_sync(node, message, data.len(), data.as_ptr());
    }

    /// This function replies to a message that was originally sent using `DataNode::send_request_message`.
    /// If this function is used to respond to a message that was not sent using that function, an
    /// error will occur.
    ///
    /// # Arguments
    /// * `message`: The message to which you are responding
    /// * `data`: Data to be sent
    ///
    /// # Examples
    ///
    /// See [`nosengine-rust::client`](../client/index.html#examples)
    pub fn send_reply_message(&self, message: &Message, data: &[u8]) {
        DataNode::send_reply_message_ptr(self.node_ptr, message.msg_ptr, data);
    }

    /// This function blocks until a message is received.
    ///
    /// # Examples
    ///
    /// See [`nosengine-rust::client`](../client/index.html#examples)
    pub fn receive_message(&self) -> Result<Message, NosError> {
        let msg_ptr = ffi::data_node_receive_message_sync(self.node_ptr);
        match ffi::error() {
            ffi::ErrorCode::Ok => Ok(Message { msg_ptr }),
            _ => Err(get_nos_error()),
        }
    }

    /// Sets a callback which will be called each time this node receives a message.
    /// Due to limitations of the NOSEngine C API, this callback cannot be a closure, and must
    /// receive inputs as raw pointers. To interact with these pointers, use functions in
    /// this module:
    ///
    /// * To get data from message: [`Message::get_contents_from_ptr`](struct.Message.html#method.get_contents_from_ptr)
    /// * To reply to a request message: [`DataNode::send_reply_message_ptr`](struct.DataNode.html#method.send_reply_message_ptr)
    ///
    /// # Safety
    ///
    /// Do NOT call `ffi::destroy_message` on `msg_pointer`, or a memory error will occur. NOSEngine
    /// handles the deallocation of this message pointer.
    pub fn set_message_callback(
        &self,
        func: extern "C" fn(node_ptr: *mut ffi::DataNodeHandle, msg_ptr: *mut ffi::MessageHandle),
    ) {
        ffi::data_node_set_message_received_callback(self.node_ptr, func);
    }
}

impl Drop for DataNode {
    fn drop(&mut self) {
        if !self.node_ptr.is_null() {
            ffi::destroy_data_node(
                self.bus_ptr.bus_ptr,
                &mut self.node_ptr as *mut *mut ffi::DataNodeHandle,
            );
        }
    }
}

unsafe impl Send for DataNode {}
