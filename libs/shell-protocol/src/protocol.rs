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
use error::ProtocolError;
use messages;
use rand::{self, Rng};
use serde_cbor::Value;
use std::cell::Cell;
use std::net::SocketAddr;
use std::process::{Command, Stdio};
use std::time::Duration;

pub struct Protocol {
    cbor_proto: CborProtocol,
    remote_addr: Cell<SocketAddr>,
}

impl Protocol {
    pub fn new(host_ip: &str, remote_addr: &str) -> Self {
        // Get a local UDP socket (Bind)

        let c_protocol = CborProtocol::new(format!("{}:0", host_ip), 4096);

        // Set up the full connection info
        Protocol {
            cbor_proto: c_protocol,
            remote_addr: Cell::new(remote_addr.parse::<SocketAddr>().unwrap()),
        }
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
    /// ```
    /// extern crate file_protocol;
    /// extern crate serde_cbor;
    ///
    /// use file_protocol::*;
    /// use serde_cbor::ser;
    ///
    /// let config = FileProtocolConfig::new(None, 4096, 5);
    /// let f_protocol = FileProtocol::new("0.0.0.0", "0.0.0.0:7000", config);
    /// let message = ser::to_vec_packed(&"ping").unwrap();
    ///
    /// f_protocol.send(message);
    /// ```
    ///
    pub fn send(&self, vec: Vec<u8>) -> Result<(), ProtocolError> {
        self.cbor_proto.send_message(&vec, self.remote_addr.get())?;
        Ok(())
    }

    /// Receive a file protocol message
    ///
    /// # Arguments
    ///
    /// * timeout - Maximum time to wait for a reply. If `None`, will block indefinitely
    ///
    /// # Errors
    ///
    /// - If this function times out, it will return `Err(None)`
    /// - If this function encounters any errors, it will return an error message string
    ///
    ///
    /// # Examples
    ///
    /// ```no_run
    /// extern crate file_protocol;
    ///
    /// use file_protocol::*;
    /// use std::time::Duration;
    ///
    /// let config = FileProtocolConfig::new(None, 4096, 5);
    /// let f_protocol = FileProtocol::new("0.0.0.0", "0.0.0.0:7000", config);
    ///
    /// let message = match f_protocol.recv(Some(Duration::from_secs(1))) {
    ///     Ok(data) => data,
    ///     Err(ProtocolError::ReceiveTimeout) =>  {
    ///         println!("Timeout waiting for message");
    ///         return;
    ///     }
    ///     Err(err) => panic!("Failed to receive message: {}", err),
    /// };
    /// ```
    ///
    pub fn recv(&self, timeout: Option<Duration>) -> Result<Value, ProtocolError> {
        match timeout {
            Some(value) => Ok(self.cbor_proto.recv_message_timeout(value)?),
            None => Ok(self.cbor_proto.recv_message()?),
        }
    }

    /// Generates a new random channel ID for use when initiating a
    /// file transfer.
    ///
    /// # Errors
    ///
    /// If this function encounters any errors, it will return an error message string
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use file_protocol::*;
    ///
    /// let config = FileProtocolConfig::new(None, 4096, 5);
    /// let f_protocol = FileProtocol::new("0.0.0.0", "0.0.0.0:7000", config);
    ///
    /// let channel_id = f_protocol.generate_channel();
    /// ```
    ///
    pub fn generate_channel(&self) -> Result<u32, ProtocolError> {
        let mut rng = rand::thread_rng();
        let channel_id: u32 = rng.gen_range(100000, 999999);
        Ok(channel_id)
    }

    /// Listen for and process file protocol messages
    ///
    /// # Arguments
    ///
    /// * pump - Function which returns the next message for processing
    /// * timeout - Maximum time to listen for a single message
    /// * start_state - Current transaction state
    ///
    /// # Errors
    ///
    /// If this function encounters any errors, it will return an error message string
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate file_protocol;
    ///
    /// use file_protocol::*;
    /// use std::time::Duration;
    ///
    /// let config = FileProtocolConfig::new(None, 4096, 5);
    /// let f_protocol = FileProtocol::new("0.0.0.0", "0.0.0.0:7000", config);
    ///
    /// f_protocol.message_engine(
    ///     |d| f_protocol.recv(Some(d)),
    ///     Duration::from_millis(10),
    ///     State::Transmitting
    /// );
    /// ```
    ///
    pub fn message_engine<F>(&self, pump: F, timeout: Duration) -> Result<(), ProtocolError>
    where
        F: Fn(Duration) -> Result<Value, ProtocolError>,
    {
        loop {
            let message = match pump(timeout) {
                Ok(message) => message,
                Err(ProtocolError::ReceiveTimeout) => {
                    // TODO when do we end this?
                    continue;
                }
                Err(e) => return Err(e),
            };

            self.process_message(message)?;
        }
    }

    pub fn process_message(&self, message: Value) -> Result<(), ProtocolError> {
        let parsed_message = messages::parse_message(message)?;

        match parsed_message {
            messages::Message::Spawn(channel_id, command) => {
                info!("{}: spawning command {}", channel_id, command);
                self.spawn(command)?;
            }
        }

        Ok(())
    }

    fn spawn(&self, command: String) -> Result<(), ProtocolError> {
        match Command::new(command.to_owned())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
        {
            Ok(_) => Ok(()),
            Err(err) => Err(ProtocolError::SpawnError { cmd: command, err }),
        }
    }
}
