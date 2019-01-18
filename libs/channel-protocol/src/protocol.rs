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

use cbor_protocol::Protocol as CborProtocol;
use crate::error::ProtocolError;
use crate::parsers::parse_message;
use serde_cbor::Value;
use std::cell::Cell;
use std::net::SocketAddr;
use std::time::Duration;

/// Channel message structure
#[derive(Clone, Debug)]
pub struct Message {
    /// Channel ID
    pub channel_id: u32,
    /// Message name
    pub name: String,
    /// Message payload
    pub payload: Vec<Value>,
}

/// Channel protocol structure
pub struct Protocol {
    cbor_proto: CborProtocol,
    remote_addr: Cell<SocketAddr>,
}

impl Protocol {
    /// Create a new channel protocol instance using an automatically assigned UDP socket
    ///
    /// # Arguments
    ///
    /// * host_ip - The local IP address
    /// * remote_addr - The remote IP and port to communicate with
    /// * data_len - Max payload length
    ///
    /// # Errors
    ///
    /// If this function encounters any errors, it will panic
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use channel_protocol::*;
    ///
    /// let channel_protocol = ChannelProtocol::new("0.0.0.0", "192.168.0.1:7000", 4096);
    /// ```
    ///
    pub fn new(host_ip: &str, remote_addr: &str, data_len: u32) -> Self {
        // Get a local UDP socket (Bind)
        let c_protocol = CborProtocol::new(&format!("{}:0", host_ip), data_len as usize);

        // Set up the full connection info
        Protocol {
            cbor_proto: c_protocol,
            remote_addr: Cell::new(remote_addr.parse::<SocketAddr>().unwrap()),
        }
    }

    /// Set new remote address on existing channel procotol structure
    ///
    /// # Arguments
    ///
    /// * remote - New remote address
    ///
    pub fn set_remote(&mut self, remote: SocketAddr) {
        self.remote_addr.set(remote);
    }

    /// Send CBOR packet to the destination port
    ///
    /// # Arguments
    ///
    /// * vec - CBOR packet to send
    ///
    /// # Errors
    ///
    /// If this function encounters any errors, it will return an error message string
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use channel_protocol::*;
    /// use serde_cbor::ser;
    ///
    /// let c_protocol = ChannelProtocol::new("0.0.0.0", "0.0.0.0:7000", 4096);
    /// let message = ser::to_vec_packed(&"ping").unwrap();
    ///
    /// c_protocol.send(&message);
    /// ```
    ///
    pub fn send(&self, vec: &[u8]) -> Result<(), ProtocolError> {
        self.cbor_proto.send_message(&vec, self.remote_addr.get())?;
        Ok(())
    }

    /// Receive a raw cbor message message
    ///
    /// # Arguments
    ///
    /// * timeout - Maximum time to wait for a reply. If `None`, will block indefinitely
    ///
    /// # Errors
    ///
    /// - If this function times out, it will return `Err(ProtocolError::ReceiveTimeout)`
    /// - If this function encounters any errors, it will return an error message string
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use channel_protocol::*;
    /// use std::time::Duration;
    ///
    /// let c_protocol = ChannelProtocol::new("0.0.0.0", "0.0.0.0:7000", 4096);
    ///
    /// let message = match c_protocol.recv_raw(Some(Duration::from_secs(1))) {
    ///     Ok(data) => data,
    ///     Err(ProtocolError::ReceiveTimeout) =>  {
    ///         println!("Timeout waiting for message");
    ///         return;
    ///     }
    ///     Err(err) => panic!("Failed to receive message: {}", err),
    /// };
    /// ```
    ///
    pub fn recv_raw(&self, timeout: Option<Duration>) -> Result<Value, ProtocolError> {
        match timeout {
            Some(value) => Ok(self.cbor_proto.recv_message_timeout(value)?),
            None => Ok(self.cbor_proto.recv_message()?),
        }
    }

    /// Receive a parsed channel procotol message
    ///
    /// # Arguments
    ///
    /// * timeout - Maximum time to wait for a reply. If `None`, will block indefinitely
    ///
    /// # Errors
    ///
    /// - If this function times out, it will return `Err(ProtocolError::ReceiveTimeout)`
    /// - If this function encounters any errors, it will return an error message string
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use channel_protocol::*;
    /// use std::time::Duration;
    ///
    /// let c_protocol = ChannelProtocol::new("0.0.0.0", "0.0.0.0:7000", 4096);
    ///
    /// let message = match c_protocol.recv_message(Some(Duration::from_secs(1))) {
    ///     Ok(data) => data,
    ///     Err(ProtocolError::ReceiveTimeout) =>  {
    ///         println!("Timeout waiting for message");
    ///         return;
    ///     }
    ///     Err(err) => panic!("Failed to receive message: {}", err),
    /// };
    /// ```
    ///
    pub fn recv_message(&self, timeout: Option<Duration>) -> Result<Message, ProtocolError> {
        let raw = self.recv_raw(timeout)?;
        Ok(parse_message(raw)?)
    }
}
