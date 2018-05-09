/*
 * Copyright (C) 2018 Kubos Corporation
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
#![allow(dead_code)]
#![allow(unused_variables)]

use byteorder::{LittleEndian, WriteBytesExt};
use crc32::*;
use messages::*;
use nom;
use std::io;
use serial;
use serial_comm::*;
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::thread;
use std::time::Duration;

pub fn read_thread(
    bus: &str,
    log_send: SyncSender<(Header, Vec<u8>)>,
    response_send: SyncSender<(Header, Vec<u8>)>,
) {
    let conn = Connection::new(bus);
    println!("Read thread started");
    loop {
        let mut message = conn.read().unwrap();
        println!("Got message");

        let hdr = match Header::parse(&message) {
            Some(v) => v,
            None => {
                println!("failed to parse header");
                continue;
            }
        };

        if hdr.sync != SYNC {
            println!("SYNC bytes invalid: {:?}", hdr.sync);
            continue;
        }

        // Read body
        //message.append(&mut self.conn.read(hdr.msg_len as usize)?);
        let len = message.len();

        // Read CRC
        let crc = nom::le_u32(message.split_off(len - 4).as_slice())
            .unwrap()
            .1;

        // Verify CRC
        let calc = calc_crc(&message);
        if calc != crc {
            // TODO: remove debugging line
            println!("CRC Mismatch: {:X} {:X}", calc, crc);
            continue;
        }

        let body = message.split_off(HDR_LEN.into());

        match hdr.msg_type & 0x80 == 0x80 {
            true => response_send.try_send((hdr, body)).unwrap(),
            false => log_send.try_send((hdr, body)).unwrap(),
        }
    }
}

/// Structure for OEM6 device instance
pub struct OEM6 {
    /// Device connection structure
    pub conn: Connection,
    pub log_recv: Receiver<(Header, Vec<u8>)>,
    pub response_recv: Receiver<(Header, Vec<u8>)>,
}

impl OEM6 {
    /// Constructor for OEM6 structure
    ///
    /// # Arguments
    ///
    /// * conn - The underlying connection stream to use for communication with the device
    ///
    /// # Examples
    ///
    /// ```
    /// use OEM6_api::*;
    ///
    /// # fn func() -> OEMResult<()> {
    /// let connection = Connection::new("/dev/ttyS4");
    /// let oem = OEM6::new(connection);
    /// # Ok(())
    /// # }
    /// ```
    ///
    pub fn new(
        conn: Connection,
        log_recv: Receiver<(Header, Vec<u8>)>,
        response_recv: Receiver<(Header, Vec<u8>)>,
    ) -> OEM6 {
        OEM6 {
            conn,
            log_recv,
            response_recv,
        }
        //TODO: Turn off RXSTATUSEVENTA messages (UNLOG)
    }

    /// Get the system version information
    ///
    /// # Errors
    ///
    /// If this function encounters any errors, an [`MAIError`] variant will be returned.
    ///
    /// # Examples
    ///
    ///
    /// [`MAIError`]: enum.MAIError.html
    pub fn request_version(&self) -> OEMResult<()> {
        let request = LogCmd::new(
            Port::COM1 as u32,
            MessageID::Version as u16,
            LogTrigger::Once as u32,
            0.0,
            0.0,
            false,
        );

        // Send request
        self.send_message(request)?;

        // Get request response
        self.get_response(MessageID::Log)
    }

    /// Get the position data ONCE (TODO: change to ONTIME trigger)
    ///
    /// # Errors
    ///
    /// If this function encounters any errors, an [`MAIError`] variant will be returned.
    ///
    /// # Examples
    ///
    ///
    /// [`MAIError`]: enum.MAIError.html
    pub fn request_position(&self) -> OEMResult<()> {
        let request = LogCmd::new(
            Port::COM1 as u32,
            MessageID::BestXYZ as u16,
            LogTrigger::OnTime as u32,
            1.0,
            0.0,
            false,
        );

        // Send request
        self.send_message(request)?;

        // Get request response
        self.get_response(MessageID::Log)
    }

    /// Directly send a message without formatting or checksum calculation
    ///
    /// # Arguments
    ///
    /// * msg - Message to send
    ///
    /// # Errors
    ///
    /// If this function encounters any errors, an [`OEMError`] variant will be returned.
    ///
    /// # Examples
    ///
    ///
    /// [`OEMError`]: enum.OEMError.html
    pub fn passthrough(&self, msg: &[u8]) -> OEMResult<()> {
        // TODO: get_response()?
        Ok(self.conn.write(msg)?)
    }

    fn send_message<T: Message>(&self, msg: T) -> OEMResult<()> {
        let mut raw = msg.serialize();

        // Get the calculated CRC
        let crc = calc_crc(&raw);
        raw.write_u32::<LittleEndian>(crc).unwrap();

        Ok(self.conn.write(raw.as_slice())?)
    }

    /// Wait for and read a message from the OEM6.
    ///
    /// # Errors
    ///
    /// If this function encounters any errors, an [`OEMError`] variant will be returned.
    ///
    /// # Examples
    ///
    ///
    /// [`OEMError`]: enum.OEMError.html
    fn get_message(&self) -> OEMResult<(Header, Vec<u8>)> {
        //loop {
        {
            // Read header
            // TODO: Failure processing. If it's just that we weren't able
            // to read the requested number of bytes, then try again
            let mut message = self.conn.read()?;

            let hdr = match Header::parse(&message) {
                Some(v) => v,
                None => {
                    println!("failed to parse header");
                    throw!(OEMError::GenericError);
                    //continue;
                }
            };

            if hdr.sync != SYNC {
                println!("SYNC bytes invalid: {:?}", hdr.sync);
                throw!(OEMError::GenericError);
                //continue;
            }

            // Read body
            //message.append(&mut self.conn.read(hdr.msg_len as usize)?);
            let len = message.len();

            // Read CRC
            let crc = nom::le_u32(message.split_off(len - 4).as_slice())
                .unwrap()
                .1;

            // Verify CRC
            let calc = calc_crc(&message);
            if calc != crc {
                // TODO: remove debugging line
                println!("CRC Mismatch: {:X} {:X}", calc, crc);
                throw!(OEMError::GenericError);
            } else {
                let body = message.split_off(HDR_LEN.into());
                return Ok((hdr, body));
            }
        }
    }

    // TODO: how to deal with async log messages being interspersed?
    // Probably set up a read thread with two channels: one for responses and one for logs
    fn get_response(&self, id: MessageID) -> OEMResult<()> {
        println!("Waiting for response");
        let (hdr, body) = self.response_recv
            .recv_timeout(Duration::from_millis(500))
            .unwrap();

        println!("Got response");

        // Make sure we got specifically a response message
        if hdr.msg_type & 0x80 != 0x80 {
            println!("Response bit not set");
            throw!(OEMError::NoResponse);
        }

        let resp = match Response::new(body) {
            Some(v) => v,
            None => {
                println!("failed to parse response");
                throw!(OEMError::NoResponse);
            }
        };

        if hdr.msg_id != id {
            println!("ID mismatch: {:?} {:?}", hdr.msg_id, id);
            throw!(OEMError::ResponseMismatch);
        }

        if resp.resp_id != ResponseID::Ok {
            println!("Error response: {:?} {}", resp.resp_id, resp.resp_string);
            throw!(OEMError::CommandError {
                id: resp.resp_id,
                description: resp.resp_string.clone(),
            });
        }

        Ok(())
    }

    pub fn get_log(&self) -> OEMResult<Log> {
        println!("Waiting for log");
        let (hdr, body) = self.log_recv.recv_timeout(Duration::from_secs(5)).unwrap();
        println!("Got log");

        // Make sure it's not a response message
        if hdr.msg_type & 0x80 == 0x80 {
            println!("Response bit not set");
            throw!(OEMError::GenericError);
        }

        let resp = match Log::new(hdr.msg_id, body) {
            Some(v) => return Ok(v),
            None => {
                println!("failed to parse response");
                throw!(OEMError::GenericError);
            }
        };
    }
}

/// Common Error for OEM Actions
#[derive(Fail, Display, Debug, Clone, PartialEq)]
pub enum OEMError {
    /// Catch-all error
    #[display(fmt = "Generic Error")]
    GenericError,
    /// A response message was received, but the ID doesn't match the command that was sent
    #[display(fmt = "Response ID Mistmatch")]
    ResponseMismatch,
    /// A command was sent, but we were unable to get the response
    #[display(fmt = "Failed to get command response")]
    NoResponse,
    /// A response was recieved and indicates an error with the previously sent command
    #[display(fmt = "Command Error({:?}): {}", id, description)]
    CommandError {
        /// The underlying error
        id: ResponseID,
        description: String,
    },
    /// Received a valid message, but the message ID doesn't match any known message type
    #[display(fmt = "Unknown Message Received: {:X}", id)]
    UnknownMessage {
        /// ID of message received
        id: u16,
    },
    #[display(fmt = "Serial Error: {}", cause)]
    /// An error was thrown by the serial driver
    SerialError {
        /// The underlying error
        cause: String,
    },
    #[display(fmt = "IO Error: {}", cause)]
    /// An I/O error was thrown by the kernel
    IoError {
        /// The underlying error
        cause: String,
    },
}

impl From<io::Error> for OEMError {
    fn from(error: io::Error) -> Self {
        OEMError::IoError {
            cause: format!("{}", error),
        }
    }
}

impl From<serial::Error> for OEMError {
    fn from(error: serial::Error) -> Self {
        OEMError::SerialError {
            cause: format!("{}", error),
        }
    }
}

/// Custom error type for OEM6 operations.
pub type OEMResult<T> = Result<T, OEMError>;
