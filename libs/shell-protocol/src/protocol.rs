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
use std::time::Duration;

pub struct Protocol {
    pub channel_protocol: ChannelProtocol,
    pub process: Box<Option<ProcessHandler>>,
    channel_id: u32,
}

/// Shell Protocol structure used in the shell service
impl Protocol {
    pub fn new(host_ip: &str, remote_addr: &str, channel_id: u32) -> Self {
        // Set up the full connection info
        Protocol {
            channel_protocol: ChannelProtocol::new(host_ip, remote_addr, 4096),
            process: Box::new(None),
            channel_id: channel_id,
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
        F: Fn(Duration) -> Result<ChannelMessage, ProtocolError>,
    {
        loop {
            if let Some(process) = self.process.as_mut() {
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

    pub fn process_message(&mut self, message: ChannelMessage) -> Result<(), ProtocolError> {
        let parsed_message = messages::parse_message(message)?;

        match parsed_message {
            messages::Message::Spawn {
                channel_id,
                command,
                args,
            } => {
                info!(
                    "<- {{ {}, spawn, {}, {{ args = {:?} }} }}",
                    channel_id, command, args
                );

                self.process = Box::new(Some(ProcessHandler::spawn(command, args)?));
                if let Some(process) = self.process.as_ref() {
                    self.channel_protocol
                        .send(messages::pid::to_cbor(self.channel_id, process.id()?)?)?;
                }
            }
            messages::Message::Stdin { channel_id, data } => {
                info!("-< {{ {}, stdin, {:?} }}", channel_id, data);
                if let Some(process) = self.process.as_mut() {
                    match data {
                        Some(data) => process.write_stdin(&data.as_bytes())?,
                        None => process.close_stdin()?,
                    }
                }
            }
            messages::Message::Stdout { channel_id, data } => {
                info!("<- {{ {}, stdout, {:?} }}", channel_id, data);
            }
            messages::Message::Stderr { channel_id, data } => {
                info!("<- {{ {}, stderr, {:?} }}", channel_id, data);
            }
            messages::Message::Pid { channel_id, pid } => {
                info!("<- {{ {}, pid, {} }}", channel_id, pid);
            }
            messages::Message::Exit {
                channel_id,
                code,
                signal,
            } => {
                info!("-< {{ {}, exit, {}, {} }}", channel_id, code, signal);
            }
        }

        Ok(())
    }
}
