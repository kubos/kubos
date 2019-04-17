//! This crate contains wrappers around the NOSEngine API. This crate can only be built in a
//! system that has NOS3 installed.
//!
//! The tests in this crate will not run properly unless the NOSEngine server is running on
//! `tcp://localhost:12001`. This is the way it is configured to run in the NOS3 VM.
//!
//! # Example Usage
//!
//! ### Simple send and receive
//!
//! ```
//! # extern crate nosengine_rust;
//! # use nosengine_rust::client;
//! # use nosengine_rust::ffi;
//! let bus = client::Bus::new("testbus", "tcp://localhost:12001").unwrap();
//! let node1 = client::DataNode::new(&bus, "node1").unwrap();
//! let node2 = client::DataNode::new(&bus, "node2").unwrap();
//!
//! node1
//!     .send_message("node2", &[1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10])
//!     .unwrap();
//! let result = node2.receive_message().unwrap();
//! assert_eq!(result.get_contents(), &[1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
//! ```
//!
//! ### Using callbacks
//!
//! ```
//! # extern crate nosengine_rust;
//! # use nosengine_rust::client;
//! # use nosengine_rust::ffi;
//! let bus = client::Bus::new("testbus", "tcp://localhost:12001").unwrap();
//! let node3 = client::DataNode::new(&bus, "node3").unwrap();
//! let node4 = client::DataNode::new(&bus, "node4").unwrap();
//!
//! extern "C" fn callback(
//!     _data_node: *mut ffi::DataNodeHandle,
//!     msg_ptr: *mut ffi::MessageHandle,
//! ) {
//!     println!("Received message in callback: {:?}", unsafe {
//!         client::Message::get_contents_from_ptr(msg_ptr)
//!     });
//! }
//!
//! node4.set_message_callback(callback);
//!
//! node3.send_message("node4", &[1u8, 2, 3, 4, 5]).unwrap();
//! ```
//!
//! ### UART
//!
//! ```
//! # extern crate nosengine_rust;
//! # use nosengine_rust::client::uart::*;
//! let uart1 = UART::new("uart10", "tcp://localhost:12001", "testuart", 15).unwrap();
//! let mut uart2 = UART::new("uart11", "tcp://localhost:12001", "testuart", 15).unwrap();
//!
//! uart2.set_callback(move |data: &[u8]|{
//!     assert_eq!(data, &[1u8, 2, 3, 4]);
//! });
//!
//! uart1.write(&[1u8, 2, 3, 4]);
//! ```

#![deny(missing_docs)]

extern crate libc;

pub mod client;
pub mod ffi;

#[cfg(test)]
#[allow(missing_docs)]
mod tests {
    use super::client;

    #[test]
    fn callback_test() {
        let bus = client::Bus::new("testbus", "tcp://localhost:12001").unwrap();
        let node3 = client::DataNode::new(&bus, "node3").unwrap();
        let node4 = client::DataNode::new(&bus, "node4").unwrap();

        extern "C" fn callback(
            _data_node: *mut super::ffi::DataNodeHandle,
            msg_ptr: *mut super::ffi::MessageHandle,
        ) {
            println!("Received message in callback: {:?}", unsafe {
                client::Message::get_contents_from_ptr(msg_ptr)
            });
        }

        node4.set_message_callback(callback);

        node3.send_message("node4", &[1u8, 2, 3, 4, 5]).unwrap();
    }

    #[test]
    fn request_message_test() {
        let bus = client::Bus::new("testbus", "tcp://localhost:12001").unwrap();
        let node5 = client::DataNode::new(&bus, "node5").unwrap();
        let node6 = client::DataNode::new(&bus, "node6").unwrap();

        extern "C" fn callback(
            node_ptr: *mut super::ffi::DataNodeHandle,
            msg_ptr: *mut super::ffi::MessageHandle,
        ) {
            client::DataNode::send_reply_message_ptr(node_ptr, msg_ptr, &[5u8, 6, 7, 8]);
        }

        node6.set_message_callback(callback);

        let response = node5
            .send_request_message("node6", &[1u8, 2, 3, 4])
            .unwrap();
        assert_eq!(response.get_contents(), &[5u8, 6, 7, 8]);
    }

    #[test]
    fn expect_error() {
        let bus = client::Bus::new("testbus", "tcp://localhost:12001").unwrap();
        let node7 = client::DataNode::new(&bus, "node7").unwrap();
        let node8 = client::DataNode::new(&bus, "node8").unwrap();

        extern "C" fn callback(
            node_ptr: *mut super::ffi::DataNodeHandle,
            msg_ptr: *mut super::ffi::MessageHandle,
        ) {
            client::DataNode::send_reply_message_ptr(node_ptr, msg_ptr, &[5u8, 6, 7, 8]);
        }

        node8.set_message_callback(callback);

        let response = node7.send_request_message("nowhere", &[1u8, 2, 3, 4]);
        match response {
            Err(client::NosError::NosEngineError {
                error_code: _,
                description: _,
            }) => {}
            _ => panic!("Expected invalid destination error."),
        }
    }

    #[test]
    fn spi_test() {
        use client::spi::*;
        use ffi::spi;
        use std::slice;

        extern "C" fn callback(dir: spi::SPIDirection, buffer: *mut u8, len: usize) -> usize {
            let data = unsafe { slice::from_raw_parts_mut(buffer, len) };
            match dir {
                spi::SPIDirection::Write => {
                    println!("Wrote {} bytes: {:?}", len, data);
                    len
                }
                spi::SPIDirection::Read => {
                    println!("Reading {} bytes.", len);
                    for i in 0..len {
                        data[i] = i as u8;
                    }
                    len
                }
            }
        }

        let master = SPIMaster::new("tcp://localhost:12001", "spia").unwrap();
        let _slave = SPISlave::new(1u8, "tcp://localhost:12001", "spia", callback).unwrap();

        master.chip_select(1u8);
        master.write(&[1u8, 2u8, 3u8, 4u8]).unwrap();
        let data = master.read(4).unwrap();
        assert_eq!(data, &[0u8, 1, 2, 3]);
    }
}
