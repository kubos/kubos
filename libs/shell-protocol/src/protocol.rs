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
use std::process::{Command, Stdio};
use std::time::Duration;

pub struct Protocol {
    pub channel_protocol: ChannelProtocol,
}

impl Protocol {
    pub fn new(host_ip: &str, remote_addr: &str) -> Self {
        // Set up the full connection info
        Protocol {
            channel_protocol: ChannelProtocol::new(host_ip, remote_addr, 4096),
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
    /// # Examples
    ///
    /// ```no_run
    /// extern crate shell_protocol;
    ///
    /// use shell_protocol::*;
    /// use std::time::Duration;
    ///
    /// let s_protocol = ShellProtocol::new("0.0.0.0", "0.0.0.0:7000");
    ///
    /// s_protocol.message_engine(
    ///     |d| Ok(s_protocol.channel_protocol.recv_message(Some(d))?),
    ///     Duration::from_millis(10),
    /// );
    /// ```
    ///
    pub fn message_engine<F>(&self, pump: F, timeout: Duration) -> Result<(), ProtocolError>
    where
        F: Fn(Duration) -> Result<ChannelMessage, ProtocolError>,
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

    pub fn process_message(&self, message: ChannelMessage) -> Result<(), ProtocolError> {
        let parsed_message = messages::parse_message(message)?;

        match parsed_message {
            messages::Message::Spawn {
                channel_id,
                command,
                args,
            } => {
                info!("{}: spawning command {} {:?}", channel_id, command, args);
                self.spawn(command, args)?;
            }
        }

        Ok(())
    }

    fn spawn(&self, command: String, args: Option<Vec<String>>) -> Result<(), ProtocolError> {
        match Command::new(command.to_owned())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .args(args.unwrap_or(vec![]))
            .spawn()
        {
            Ok(_) => Ok(()),
            Err(err) => Err(ProtocolError::SpawnError { cmd: command, err }),
        }
    }
}
