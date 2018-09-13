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

//! Kubos CBOR over UDP communication crate
//!
//! # Examples
//!
//! ```
//! extern crate cbor_protocol;
//! extern crate serde_cbor;
//!
//! use cbor_protocol::*;
//! use serde_cbor::ser;
//! use std::time::Duration;
//!
//! let cbor_connection = Protocol::new("0.0.0.0:8000".to_owned());
//! let message = ser::to_vec_packed(&"ping").unwrap();
//!
//! cbor_connection.send_message(&message, "0.0.0.0:8001".parse().unwrap()).unwrap();
//!
//! match cbor_connection.recv_message_peer_timeout(Duration::from_millis(10)) {
//!     Ok((source, message)) => {
//!         if let Some(msg) = message {
//!             println!("Received message from {:?}: {:?}", source, msg);
//!            }
//!        }
//!        Err(None) => println!("Timed out waiting for reply"),
//!     Err(Some(err)) => eprintln!("Failed to receive message: {}", err)
//! }
//! ```
//!

#![deny(missing_docs)]
#![deny(warnings)]

extern crate serde_cbor;

use serde_cbor::de;
use serde_cbor::Value;
use std::net::{SocketAddr, UdpSocket};
use std::time::Duration;

// Was 4136
// Somewhere we are sending packets bigger than this...
const MSG_SIZE: usize = 4500;

/// Parse and return the channel_id for a message
pub fn peek_channel_id(message: &Option<serde_cbor::Value>) -> Result<Option<u32>, String> {
    let data = match message {
        Some(Value::Array(val)) => val.to_owned(),
        _ => return Err("Unable to parse message: Data not an array".to_owned()),
    };

    let mut pieces = data.iter();

    let first_param: Value = pieces
        .next()
        .ok_or(format!("Unable to parse message: No contents"))?
        .to_owned();

    if let Value::U64(channel_id) = first_param {
        Ok(Some(channel_id as u32))
    } else {
        Ok(None)
    }
}

/// CBOR protocol communication structure
pub struct Protocol {
    handle: UdpSocket,
}

impl Protocol {
    /// Binds a UDP listener socket and saves it in a new protocol instance
    ///
    /// # Arguments
    ///
    /// * host_url - The IP address and port to bind
    ///
    /// # Errors
    ///
    /// If this function encounters any errors, it will panic
    ///
    /// # Examples
    ///
    /// ```
    /// use cbor_protocol::*;
    ///
    /// let cbor_connection = Protocol::new("0.0.0.0:8000".to_owned());
    /// ```
    ///
    pub fn new(host_url: String) -> Self {
        Self {
            handle: UdpSocket::bind(host_url.parse::<SocketAddr>().unwrap()).unwrap(),
        }
    }

    /// Creates a protocol instance using an existing socket
    ///
    /// # Arguments
    ///
    /// * handle - The socket to use for future communication
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate cbor_protocol;
    ///
    /// use cbor_protocol::*;
    /// use std::net::UdpSocket;
    ///
    /// let socket = UdpSocket::bind("0.0.0.0:8000").unwrap();
    ///
    /// let cbor_connection = Protocol::new_from_socket(socket);
    /// ```
    ///
    pub fn new_from_socket(handle: UdpSocket) -> Self {
        Self { handle }
    }

    /// Send a CBOR packet to a specified UDP socket destination
    ///
    /// # Arguments
    ///
    /// * message - CBOR packet to send
    /// * dest - UDP socket destination
    ///
    /// # Errors
    ///
    /// If this function encounters any errors, it will return an error message string
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate cbor_protocol;
    /// extern crate serde_cbor;
    ///
    /// use cbor_protocol::*;
    /// use serde_cbor::ser;
    ///
    /// let cbor_connection = Protocol::new("0.0.0.0:8000".to_owned());
    /// let message = ser::to_vec_packed(&"ping").unwrap();
    ///
    /// cbor_connection.send_message(&message, "0.0.0.0:8001".parse().unwrap());
    /// ```
    ///
    pub fn send_message(&self, message: &[u8], dest: SocketAddr) -> Result<(), String> {
        // TODO: If paused, just queue up the message

        let mut payload = vec![];
        payload.extend(message);
        payload.insert(0, 0);

        self.handle
            .send_to(&payload, &dest)
            .map_err(|err| format!("Failed to send message to {:?}: {}", dest, err))?;
        Ok(())
    }

    /// Send a pause message to a specified UDP socket destination
    ///
    /// # Arguments
    ///
    /// * dest - UDP socket destination
    ///
    /// # Errors
    ///
    /// If this function encounters any errors, it will return an error message string
    ///
    /// # Examples
    ///
    /// ```
    /// use cbor_protocol::*;
    ///
    /// let cbor_connection = Protocol::new("0.0.0.0:8000".to_owned());
    ///
    /// cbor_connection.send_pause("0.0.0.0:8001".parse().unwrap());
    /// ```
    ///
    pub fn send_pause(&self, dest: SocketAddr) -> Result<(), String> {
        println!("-> pause");

        let payload = vec![1];
        self.handle
            .send_to(&payload, &dest)
            .map_err(|err| format!("Failed to send message to {:?}: {}", dest, err))?;
        Ok(())
    }

    /// Send a resume message to a specified UDP socket destination
    ///
    /// # Arguments
    ///
    /// * dest - UDP socket destination
    ///
    /// # Errors
    ///
    /// If this function encounters any errors, it will return an error message string
    ///
    /// # Examples
    ///
    /// ```
    /// use cbor_protocol::*;
    ///
    /// let cbor_connection = Protocol::new("0.0.0.0:8000".to_owned());
    ///
    /// cbor_connection.send_resume("0.0.0.0:8001".parse().unwrap());
    /// ```
    ///
    pub fn send_resume(&self, dest: SocketAddr) -> Result<(), String> {
        println!("-> resume");

        let payload = vec![2];
        self.handle
            .send_to(&payload, &dest)
            .map_err(|err| format!("Failed to send message to {:?}: {}", dest, err))?;
        Ok(())
    }

    /// Receive a UDP message (no timeout)
    ///
    /// # Errors
    ///
    /// If this function encounters any errors, it will return an error message string
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use cbor_protocol::*;
    ///
    /// let cbor_connection = Protocol::new("0.0.0.0:8000".to_owned());
    ///
    /// let message = cbor_connection.recv_message().unwrap();
    /// ```
    ///
    pub fn recv_message(&self) -> Result<Option<serde_cbor::Value>, Option<String>> {
        let mut buf = [0; MSG_SIZE];
        let (size, _peer) = self
            .handle
            .recv_from(&mut buf)
            .map_err(|err| Some(format!("Failed to receive a message: {}", err)))?;

        self.recv_start(&buf[0..size]).map_err(|err| Some(err))
    }

    /// Peek at the sender information for the next message in the UDP receive buffer
    ///
    /// # Errors
    ///
    /// If this function encounters any errors, it will return an error message string
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use cbor_protocol::*;
    ///
    /// let cbor_connection = Protocol::new("0.0.0.0:8000".to_owned());
    ///
    /// let source = cbor_connection.peek_peer();
    /// ```
    ///
    pub fn peek_peer(&self) -> Result<SocketAddr, String> {
        let mut buf = [0; MSG_SIZE];

        let (_size, peer) = self
            .handle
            .peek_from(&mut buf)
            .map_err(|err| format!("Failed to receive a message: {}", err))?;

        Ok(peer)
    }

    /// Receive a UDP message and take note of the sender (no timeout)
    ///
    /// # Errors
    ///
    /// If this function encounters any errors, it will return an error message string
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use cbor_protocol::*;
    ///
    /// let cbor_connection = Protocol::new("0.0.0.0:8000".to_owned());
    ///
    /// let (source, message) = cbor_connection.recv_message_peer().unwrap();
    /// ```
    ///
    pub fn recv_message_peer(&self) -> Result<(SocketAddr, Option<serde_cbor::Value>), String> {
        let mut buf = [0; MSG_SIZE];
        let (size, peer) = self
            .handle
            .recv_from(&mut buf)
            .map_err(|err| format!("Failed to receive a message: {}", err))?;

        let message = self.recv_start(&buf[0..size])?;
        Ok((peer, message))
    }

    /// Receive a UDP message and take note of the sender (with timeout)
    ///
    /// # Arguments
    ///
    /// * timeout - Maximum amount of time to wait for a UDP packet
    ///
    /// # Errors
    ///
    /// - If this function times out, it will return Err(None)
    /// - If this function encounters any errors, it will return an error message string
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate cbor_protocol;
    ///
    /// use cbor_protocol::*;
    /// use std::time::Duration;
    ///
    /// let cbor_connection = Protocol::new("0.0.0.0:8000".to_owned());
    ///
    /// let (source, message) = match cbor_connection.recv_message_peer_timeout(Duration::from_secs(1)) {
    ///     Ok(data) => data,
    ///     Err(None) => {
    ///            println!("Timeout waiting for message");
    ///            return;
    ///        }
    ///     Err(Some(err)) => panic!("Failed to receive message: {}", err),
    /// };
    /// ```
    ///
    pub fn recv_message_peer_timeout(
        &self,
        timeout: Duration,
    ) -> Result<(SocketAddr, Option<serde_cbor::Value>), Option<String>> {
        // Set the timeout for this particular receive
        self.handle
            .set_read_timeout(Some(timeout))
            .map_err(|err| format!("Failed to set timeout: {}", err))?;

        let mut buf = [0; MSG_SIZE];

        let result = self.handle.recv_from(&mut buf);

        // Reset the timeout for future calls
        // TODO: Decide what should happen if this fails...
        let _ = self.handle.set_read_timeout(None);

        let (size, peer) = match result {
            Ok(data) => data,
            Err(err) => match err.kind() {
                ::std::io::ErrorKind::WouldBlock => return Err(None), // For some reason, UDP recv returns WouldBlock for timeouts
                _ => return Err(Some(format!("Failed to receive a message: {:?}", err))),
            },
        };

        let message = self.recv_start(&buf[0..size]).map_err(|err| Some(err))?;
        Ok((peer, message))
    }

    /// Receive a UDP message (with timeout)
    ///
    /// # Arguments
    ///
    /// * timeout - Maximum amount of time to wait for a UDP packet
    ///
    /// # Errors
    ///
    /// - If this function times out, it will return Err(None)
    /// - If this function encounters any errors, it will return an error message string
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate cbor_protocol;
    ///
    /// use cbor_protocol::*;
    /// use std::time::Duration;
    ///
    /// let cbor_connection = Protocol::new("0.0.0.0:9000".to_owned());
    ///
    /// let message = match cbor_connection.recv_message_timeout(Duration::from_secs(1)) {
    ///     Ok(data) => data,
    ///     Err(None) => {
    ///            println!("Timeout waiting for message");
    ///            return;
    ///        }
    ///     Err(Some(err)) => panic!("Failed to receive message: {}", err),
    /// };
    /// ```
    ///
    pub fn recv_message_timeout(
        &self,
        timeout: Duration,
    ) -> Result<Option<serde_cbor::Value>, Option<String>> {
        // Set the timeout for this particular receive
        self.handle
            .set_read_timeout(Some(timeout))
            .map_err(|err| format!("Failed to set timeout: {}", err))?;

        let mut buf = [0; MSG_SIZE];

        let result = self.handle.recv_from(&mut buf);

        // Reset the timeout for future calls
        // TODO: Decide what should happen if this fails...
        let _ = self.handle.set_read_timeout(None);

        let (size, _peer) = match result {
            Ok(data) => data,
            Err(err) => match err.kind() {
                ::std::io::ErrorKind::WouldBlock => return Err(None), // For some reason, UDP recv returns WouldBlock for timeouts
                _ => return Err(Some(format!("Failed to receive a message: {:?}", err))),
            },
        };

        self.recv_start(&buf[0..size]).map_err(|err| Some(err))
    }

    // Parse the received CBOR message
    fn recv_start(&self, data: &[u8]) -> Result<Option<serde_cbor::Value>, String> {
        if data.len() == 0 {
            return Ok(None);
        }

        let result: Option<serde_cbor::Value> = match data[0] {
            0 => {
                let message: serde_cbor::Value = de::from_slice(&data[1..])
                    .map_err(|err| format!("Failed to parse data: {:?}", err))?;

                if message.is_array() {
                    Some(message)
                } else {
                    return Err(format!("Failed to parse data: Body not an array"));
                }
            }
            1 => {
                println!("<- pause");
                //TODO: self.pause()?;
                None
            }
            2 => {
                println!("<- resume");
                // TODO: self.resume()?;
                None
            }
            x => {
                eprintln!("Ignoring unknown control frame: {}", x);
                None
            }
        };

        Ok(result)
    }
}
