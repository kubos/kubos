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

use byteorder::{LittleEndian, WriteBytesExt};
use crc32::*;
use messages::*;
use nom;
use rust_uart::UartError;
use rust_uart::*;
//use std::io;
use serial;
use std::sync::mpsc::{Receiver, SyncSender};
use std::sync::{Arc, Mutex};
use std::time::Duration;

const CHAR_SIZE: serial::CharSize = serial::Bits8;
const PARITY: serial::Parity = serial::ParityNone;
const STOP_BITS: serial::StopBits = serial::Stop1;
const FLOW_CONTROL: serial::FlowControl = serial::FlowNone;
const TIMEOUT: Duration = Duration::from_millis(60);

pub fn read_thread(
    rx_conn: Arc<Mutex<Connection>>,
    log_send: SyncSender<(Header, Vec<u8>)>,
    response_send: SyncSender<(Header, Vec<u8>)>,
) {
    //TODO: try_lock? or just error handling

    loop {
        {
            // Read SYNC bytes
            let conn = rx_conn.lock().unwrap();
            let mut message = match conn.read(3, Duration::from_secs(2)) {
                Ok(v) => v,
                Err(_err) => continue, //TODO: actual error handling. Loop on timeout
            };

            if message != SYNC {
                println!("SYNC mismatch: {:?} {:?}", message, SYNC);
                continue;
            }

            // Read the rest of the header
            message.append(&mut conn.read(25, TIMEOUT).unwrap());

            let hdr = match Header::parse(&message) {
                Some(v) => v,
                None => {
                    println!("failed to parse header");
                    continue;
                }
            };

            // Read body + CRC bytes
            message.append(&mut conn.read((hdr.msg_len + 4) as usize, TIMEOUT).unwrap());

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
}

/// Structure for OEM6 device instance
pub struct OEM6 {
    /// Device connection structure
    pub conn: Arc<Mutex<Connection>>,
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
    pub fn new(
        bus: &str,
        baud_rate: serial::BaudRate,
        log_recv: Receiver<(Header, Vec<u8>)>,
        response_recv: Receiver<(Header, Vec<u8>)>,
    ) -> OEMResult<OEM6> {
        let settings = serial::PortSettings {
            baud_rate,
            char_size: CHAR_SIZE,
            parity: PARITY,
            stop_bits: STOP_BITS,
            flow_control: FLOW_CONTROL,
        };

        let conn = Arc::new(Mutex::new(Connection::from_path(bus, settings, TIMEOUT)?));

        Ok(OEM6 {
            conn,
            log_recv,
            response_recv,
        })
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

    /// Request BestXYZ position log/s from the device
    ///
    /// # Errors
    ///
    /// If this function encounters any errors, an [`MAIError`] variant will be returned.
    ///
    /// # Examples
    ///
    ///
    /// [`MAIError`]: enum.MAIError.html
    pub fn request_position(&self, interval: f64, offset: f64, hold: bool) -> OEMResult<()> {
        let trigger = if interval == 0.0 {
            LogTrigger::Once
        } else {
            LogTrigger::OnTime
        };

        let request = LogCmd::new(
            Port::COM1 as u32,
            MessageID::BestXYZ as u16,
            trigger as u32,
            interval,
            offset,
            hold,
        );

        // Send request
        self.send_message(request)?;

        // Get request response
        self.get_response(MessageID::Log)
    }

    /// Request that the device send error messages as they occur
    ///
    /// # Errors
    ///
    /// If this function encounters any errors, an [`MAIError`] variant will be returned.
    ///
    /// # Examples
    ///
    ///
    /// [`MAIError`]: enum.MAIError.html
    pub fn request_errors(&self) -> OEMResult<()> {
        let request = LogCmd::new(
            Port::COM1 as u32,
            MessageID::RxStatusEvent as u16,
            LogTrigger::OnChanged as u32,
            0.0,
            0.0,
            false,
        );

        // Send request
        self.send_message(request)?;

        // Get request response
        self.get_response(MessageID::Log)
    }

    pub fn request_unlog(&self, id: MessageID) -> OEMResult<()> {
        let request = UnlogCmd::new(Port::COM1 as u32, id as u16);

        // Send request
        self.send_message(request)?;

        // Get request response
        self.get_response(MessageID::Unlog)
    }

    pub fn request_unlog_all(&self, hold: bool) -> OEMResult<()> {
        let request = UnlogAllCmd::new(Port::COM1 as u32, hold);

        // Send request
        self.send_message(request)?;

        // Get request response
        self.get_response(MessageID::UnlogAll)
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
        println!("Passthrough taking lock");
        Ok(self.conn.lock().unwrap().write(msg)?)
    }

    fn send_message<T: Message>(&self, msg: T) -> OEMResult<()> {
        let mut raw = msg.serialize();

        // Get the calculated CRC
        let crc = calc_crc(&raw);
        raw.write_u32::<LittleEndian>(crc).unwrap();

        println!("Send message taking lock");
        Ok(self.conn.lock().unwrap().write(raw.as_slice())?)
    }

    // TODO: how to deal with async log messages being interspersed?
    // Probably set up a read thread with two channels: one for responses and one for logs
    fn get_response(&self, id: MessageID) -> OEMResult<()> {
        println!("Waiting for response");
        let (hdr, body) = self.response_recv
            .recv_timeout(Duration::from_millis(500))
            .map_err(|_| OEMError::NoResponse)?;

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

        match Log::new(hdr.msg_id, body) {
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
    #[display(fmt = "{}", cause)]
    /// An error was thrown by the serial driver
    UartError {
        #[fail(cause)]
        cause: UartError,
    },
}

impl From<UartError> for OEMError {
    fn from(error: UartError) -> Self {
        OEMError::UartError { cause: error }
    }
}

/// Custom error type for OEM6 operations.
pub type OEMResult<T> = Result<T, OEMError>;
