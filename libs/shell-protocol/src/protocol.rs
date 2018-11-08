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

use channel_protocol::{ChannelMessage, ChannelProtocol};
use error::ProtocolError;
use messages;
use process::ProcessHandler;
use std::net::SocketAddr;
use std::time::Duration;

/// Shell Service Protocol structure
///
/// This structure is only intended for usage inside of the
/// shell service. It is not required for usage by shell clients.
pub struct Protocol {
    channel_protocol: ChannelProtocol,
    process: Box<ProcessHandler>,
    channel_id: u32,
}

impl Protocol {
    /// Create new instance of shell protocol structure
    ///
    /// # Arguments
    ///
    /// * channel_protocol - Instance of ChannelProtocol
    /// * channel_id - Channel ID of shell session
    /// * process - Instance of ProcessHandler
    pub fn new(
        channel_protocol: ChannelProtocol,
        channel_id: u32,
        process: Box<ProcessHandler>,
    ) -> Self {
        // Set up the full connection info
        Protocol {
            channel_protocol,
            process,
            channel_id,
        }
    }

    /// Listen for and process shell protocol messages
    ///
    /// # Arguments
    ///
    /// * pump - Function which returns the next message for processing
    /// * timeout - Maximum time to listen for a single message
    ///
    /// # Errors
    ///
    /// If this function encounters any errors, it will return an error message string
    ///
    pub fn message_engine<F>(&mut self, pump: F, timeout: Duration) -> Result<(), ProtocolError>
    where
        F: Fn(Duration) -> Result<(ChannelMessage, SocketAddr), ProtocolError>,
    {
        loop {
            {
                let process = self.process.as_mut();
                // Check if process has stdout output
                if process.stdout_reader.is_some() {
                    match process.read_stdout() {
                        Ok(Some(data)) => {
                            self.channel_protocol
                                .send(messages::stdout::to_cbor(self.channel_id, Some(&data))?)?;
                        }
                        Err(ProtocolError::ReadTimeout) => {}
                        _ => {
                            self.channel_protocol
                                .send(messages::stdout::to_cbor(self.channel_id, None)?)?;
                            process.stdout_reader = None;
                        }
                    }
                }

                // Check if process has stderr output
                if process.stderr_reader.is_some() {
                    match process.read_stderr() {
                        Ok(Some(data)) => {
                            self.channel_protocol
                                .send(messages::stderr::to_cbor(self.channel_id, Some(&data))?)?;
                        }
                        Err(ProtocolError::ReadTimeout) => {}
                        _ => {
                            self.channel_protocol
                                .send(messages::stderr::to_cbor(self.channel_id, None)?)?;
                            process.stderr_reader = None;
                        }
                    }
                }

                // When the process ends we will start to get `None` on stdout/stderr
                // Once we have closed those pipes we can check for the status code
                // and clean up. Other wise we might miss output
                if process.stdout_reader.is_none() && process.stderr_reader.is_none() {
                    // Check if process has exited
                    if let Some((code, signal)) = process.status()? {
                        self.channel_protocol.send(messages::exit::to_cbor(
                            self.channel_id,
                            code,
                            signal,
                        )?)?;
                        // If the process is done then we can exit this loop
                        return Ok(());
                    }
                }
            }
            // Check for new messages from the client
            let (message, remote) = match pump(timeout) {
                Ok(message) => message,
                Err(ProtocolError::ReceiveTimeout) => {
                    // TODO when do we end this?
                    continue;
                }
                Err(e) => return Err(e),
            };

            // Update the remote so that responses go to the
            // last client that we had contact with
            self.channel_protocol.set_remote(remote);

            self.process_message(message)?;
        }
    }

    fn process_message(&mut self, message: ChannelMessage) -> Result<(), ProtocolError> {
        let parsed_message = messages::parse_message(message)?;

        match parsed_message {
            messages::Message::Stdin { channel_id, data } => {
                info!("<- {{ {}, stdin, {:?} }}", channel_id, data);
                {
                    let process = self.process.as_mut();
                    match data {
                        Some(data) => process.write_stdin(&data.as_bytes())?,
                        None => process.close_stdin()?,
                    }
                }
            }
            messages::Message::Kill { channel_id, signal } => {
                info!("<- {{ {}, kill, {:?} }}", channel_id, signal);
                {
                    let process = self.process.as_mut();
                    process.kill(signal)?;
                }
            }
            message => warn!("Shell service received unexpected message: {:?}", message),
        }

        Ok(())
    }
}
