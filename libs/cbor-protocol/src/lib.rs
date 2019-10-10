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
//! ```no_run
//! use cbor_protocol::*;
//! use serde_cbor::ser;
//! use std::time::Duration;
//!
//! let cbor_connection = Protocol::new(&"0.0.0.0:8000".to_owned(), 4096);
//! let message = ser::to_vec_packed(&("hello", "world")).unwrap();
//!
//! cbor_connection.send_message(&message, "0.0.0.0:8001".parse().unwrap()).unwrap();
//!
//! match cbor_connection.recv_message_peer_timeout(Duration::from_millis(10)) {
//!     Ok((source, message)) => {
//!         println!("Received message from {:?}: {:?}", source, message);
//!     }
//!     Err(ProtocolError::Timeout) => println!("Timed out waiting for reply"),
//!     Err(err) => eprintln!("Failed to receive message: {}", err)
//! }
//! ```
//!

#![deny(missing_docs)]
#![deny(warnings)]

use failure::Fail;
use log::error;
use serde_cbor::de;
use std::io;
use std::net::{SocketAddr, UdpSocket};
use std::time::Duration;

/// An error generated during protocol execution
#[derive(Debug, Fail)]
pub enum ProtocolError {
    /// Indicates a timeout when sending or receiving
    #[fail(display = "Protocol timed out")]
    Timeout,
    /// Indicates no data was received
    #[fail(display = "No valid data received")]
    NoDataReceived,
    /// Indicates a failure to receive
    #[fail(display = "Failed to receive message from: {}", err)]
    ReceiveFailed {
        /// Cause of receive failure
        err: io::Error,
    },
    /// Indicates a failure to send
    #[fail(display = "Failed to send message to {}: {}", dest, err)]
    SendFailed {
        /// Intended send destination
        dest: SocketAddr,
        /// Cause of send failure
        err: io::Error,
    },
    #[fail(display = "Encountered io Error: {}", err)]
    /// Indicates a non-send/received Io error
    IoError {
        /// Root Io Error
        err: io::Error,
    },
    /// Indicates a failure to parse a message
    #[fail(display = "Failed to parse data: {:?}", err)]
    ParseFail {
        /// Cause of parsing failure
        err: String,
    },
}

/// CBOR protocol communication structure
pub struct Protocol {
    handle: UdpSocket,
    msg_size: usize,
}

impl Protocol {
    /// Binds a UDP listener socket and saves it in a new protocol instance
    ///
    /// # Arguments
    ///
    /// * host_url - The IP address and port to bind
    /// * data_size - Expected max size of payload in messages
    ///
    /// # Errors
    ///
    /// If this function encounters any errors, it will panic
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use cbor_protocol::*;
    ///
    /// let cbor_connection = Protocol::new(&"0.0.0.0:8000".to_owned(), 4096);
    /// ```
    ///
    pub fn new(host_url: &str, data_size: usize) -> Self {
        Self {
            handle: UdpSocket::bind(
                host_url
                    .parse::<SocketAddr>()
                    .map_err(|err| {
                        error!("Failed to parse host_url: {:?}", err);
                        err
                    })
                    .unwrap(),
            )
            .map_err(|err| {
                error!("Failed to bind socket for {}: {:?}", host_url, err);
                err
            })
            .unwrap(),
            msg_size: data_size + 50,
        }
    }

    /// Send a CBOR packet to a specified UDP socket destination
    ///
    /// # Arguments
    ///
    /// * message - CBOR packet to send. Packet must be a serialized array or tuple.
    /// * dest - UDP socket destination
    ///
    /// # Errors
    ///
    /// If this function encounters any errors, it will return an error message string
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use cbor_protocol::*;
    /// use serde_cbor::ser;
    ///
    /// let cbor_connection = Protocol::new(&"0.0.0.0:8000".to_owned(), 4096);
    /// let message = ser::to_vec_packed(&["ping"]).unwrap();
    ///
    /// cbor_connection.send_message(&message, "0.0.0.0:8001".parse().unwrap());
    /// ```
    ///
    /// ```no_run
    /// use cbor_protocol::*;
    /// use serde_cbor::ser;
    ///
    /// let cbor_connection = Protocol::new(&"0.0.0.0:8000".to_owned(), 4096);
    /// let message = ser::to_vec_packed(&("hello", "world")).unwrap();
    ///
    /// cbor_connection.send_message(&message, "0.0.0.0:8001".parse().unwrap());
    /// ```
    ///
    pub fn send_message(&self, message: &[u8], dest: SocketAddr) -> Result<(), ProtocolError> {
        // TODO: If paused, just queue up the message

        let mut payload = vec![];
        payload.extend(message);
        payload.insert(0, 0);

        self.handle
            .send_to(&payload, &dest)
            .map_err(|err| ProtocolError::SendFailed { dest, err })?;
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
    /// ```no_run
    /// use cbor_protocol::*;
    ///
    /// let cbor_connection = Protocol::new(&"0.0.0.0:8000".to_owned(), 4096);
    ///
    /// cbor_connection.send_pause("0.0.0.0:8001".parse().unwrap());
    /// ```
    ///
    pub fn send_pause(&self, dest: SocketAddr) -> Result<(), ProtocolError> {
        println!("-> pause");

        let payload = vec![1];
        self.handle
            .send_to(&payload, &dest)
            .map_err(|err| ProtocolError::SendFailed { dest, err })?;
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
    /// ```no_run
    /// use cbor_protocol::*;
    ///
    /// let cbor_connection = Protocol::new(&"0.0.0.0:8000".to_owned(), 4096);
    ///
    /// cbor_connection.send_resume("0.0.0.0:8001".parse().unwrap());
    /// ```
    ///
    pub fn send_resume(&self, dest: SocketAddr) -> Result<(), ProtocolError> {
        println!("-> resume");

        let payload = vec![2];
        self.handle
            .send_to(&payload, &dest)
            .map_err(|err| ProtocolError::SendFailed { dest, err })?;
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
    /// let cbor_connection = Protocol::new(&"0.0.0.0:8000".to_owned(), 4096);
    ///
    /// let message = cbor_connection.recv_message().unwrap();
    /// ```
    ///
    pub fn recv_message(&self) -> Result<serde_cbor::Value, ProtocolError> {
        let mut buf = vec![0; self.msg_size];
        let (size, _peer) = self
            .handle
            .recv_from(&mut buf)
            .map_err(|err| ProtocolError::ReceiveFailed { err })?;

        self.recv_start(&buf[0..size])
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
    /// let cbor_connection = Protocol::new(&"0.0.0.0:8000".to_owned(), 4096);
    ///
    /// let source = cbor_connection.peek_peer();
    /// ```
    ///
    pub fn peek_peer(&self) -> Result<SocketAddr, ProtocolError> {
        let mut buf = vec![0; self.msg_size];

        let (_size, peer) = self
            .handle
            .peek_from(&mut buf)
            .map_err(|err| ProtocolError::ReceiveFailed { err })?;

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
    /// let cbor_connection = Protocol::new(&"0.0.0.0:8000".to_owned(), 4096);
    ///
    /// let (source, message) = cbor_connection.recv_message_peer().unwrap();
    /// ```
    ///
    pub fn recv_message_peer(&self) -> Result<(SocketAddr, serde_cbor::Value), ProtocolError> {
        let mut buf = vec![0; self.msg_size];
        let (size, peer) = self
            .handle
            .recv_from(&mut buf)
            .map_err(|err| ProtocolError::ReceiveFailed { err })?;

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
    /// ```no_run
    /// use cbor_protocol::*;
    /// use std::time::Duration;
    ///
    /// let cbor_connection = Protocol::new(&"0.0.0.0:8000".to_owned(), 4096);
    ///
    /// let (source, message) = match cbor_connection.recv_message_peer_timeout(Duration::from_secs(1)) {
    ///     Ok(data) => data,
    ///     Err(ProtocolError::Timeout) => {
    ///         println!("Timeout waiting for message");
    ///         return;
    ///     }
    ///     Err(err) => panic!("Failed to receive message: {}", err),
    /// };
    /// ```
    ///
    pub fn recv_message_peer_timeout(
        &self,
        timeout: Duration,
    ) -> Result<(SocketAddr, serde_cbor::Value), ProtocolError> {
        // Set the timeout for this particular receive
        self.handle
            .set_read_timeout(Some(timeout))
            .map_err(|err| ProtocolError::IoError { err })?;

        let mut buf = vec![0; self.msg_size];

        let result = self.handle.recv_from(&mut buf);

        // Reset the timeout for future calls
        // TODO: Decide what should happen if this fails...
        let _ = self.handle.set_read_timeout(None);

        let (size, peer) = match result {
            Ok(data) => data,
            Err(err) => match err.kind() {
                // For some reason, UDP recv returns WouldBlock for timeouts
                io::ErrorKind::WouldBlock => return Err(ProtocolError::Timeout),
                _ => return Err(ProtocolError::ReceiveFailed { err }),
            },
        };

        let message = self.recv_start(&buf[0..size])?;
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
    /// ```no_run
    /// use cbor_protocol::*;
    /// use std::time::Duration;
    ///
    /// let cbor_connection = Protocol::new(&"0.0.0.0:9000".to_owned(), 4096);
    ///
    /// let message = match cbor_connection.recv_message_timeout(Duration::from_secs(1)) {
    ///     Ok(data) => data,
    ///     Err(ProtocolError::Timeout) => {
    ///         println!("Timeout while waiting for message");
    ///         return;
    ///     }
    ///     Err(err) => panic!("Failed to receive message: {}", err),
    /// };
    /// ```
    ///
    pub fn recv_message_timeout(
        &self,
        timeout: Duration,
    ) -> Result<serde_cbor::Value, ProtocolError> {
        // Set the timeout for this particular receive
        self.handle
            .set_read_timeout(Some(timeout))
            .map_err(|err| ProtocolError::IoError { err })?;

        let mut buf = vec![0; self.msg_size];

        let result = self.handle.recv_from(&mut buf);

        // Reset the timeout for future calls
        // TODO: Decide what should happen if this fails...
        let _ = self.handle.set_read_timeout(None);

        let (size, _peer) = match result {
            Ok(data) => data,
            Err(err) => match err.kind() {
                // For some reason, UDP recv returns WouldBlock for timeouts
                ::std::io::ErrorKind::WouldBlock => return Err(ProtocolError::Timeout),
                _ => return Err(ProtocolError::ReceiveFailed { err }),
            },
        };

        Ok(self.recv_start(&buf[0..size])?)
    }

    // Parse the received CBOR message
    fn recv_start(&self, data: &[u8]) -> Result<serde_cbor::Value, ProtocolError> {
        if data.is_empty() {
            return Err(ProtocolError::NoDataReceived);
        }

        let result = match data[0] {
            0 => {
                let message: serde_cbor::Value =
                    de::from_slice(&data[1..]).map_err(|err| ProtocolError::ParseFail {
                        err: format!("{:?}", err),
                    })?;

                if message.is_array() {
                    message
                } else {
                    return Err(ProtocolError::ParseFail {
                        err: "Body is not an array".to_owned(),
                    });
                }
            }
            1 => {
                println!("<- pause");
                //TODO: Evaluate whether to keep/use pause & resume
                //TODO: self.pause()?;
                return Err(ProtocolError::NoDataReceived);
            }
            2 => {
                println!("<- resume");
                // TODO: self.resume()?;
                return Err(ProtocolError::NoDataReceived);
            }
            x => {
                eprintln!("Ignoring unknown control frame: {}", x);
                return Err(ProtocolError::NoDataReceived);
            }
        };

        Ok(result)
    }
}
