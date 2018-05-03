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
use rust_uart::*;
use std::io;
use serial;

/// Structure for OEM6 device instance
pub struct OEM6 {
    /// Device connection structure
    pub conn: Connection,
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
    pub fn new(conn: Connection) -> OEM6 {
        OEM6 { conn }
    }

    /// Get the system version information
    ///
    /// # Errors
    ///
    /// If this function encounters any errors, an [`MAIError`] variant will be returned.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mai400_api::*;
    /// # fn func() -> OEMResult<()> {
    /// # let connection = Connection::new("/dev/ttyS4");
    /// let mai = MAI400::new(connection);
    /// let result = mai.get_version()?;
    /// TODO
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`MAIError`]: enum.MAIError.html
    pub fn get_version(&self) -> OEMResult<()> {
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
        let result = self.get_response()?;

        // Get the version message

        // TODO: remove this. Test code only
        Ok(())
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
    /// ```
    /// # use OEM6_api::*;
    /// # fn func() -> OEMResult<()> {
    /// # let connection = Connection::new("/dev/ttyS4");
    /// let oem = OEM6::new(connection);
    ///
    /// let mut array = [0; 8];
    /// TODO
    ///
    /// oem.passthrough(&array)?;
    /// # Ok(())
    /// # }
    /// ```
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

        self.conn.write(raw.as_slice())?;
        self.get_response()
    }

    /// Wait for and read a message set from the OEM6.
    ///
    /// # Errors
    ///
    /// If this function encounters any errors, an [`OEMError`] variant will be returned.
    ///
    /// # Examples
    ///
    /// ```
    /// # use OEM6_api::*;
    /// # fn func() -> OEMResult<()> {
    /// # let connection = Connection::new("/dev/ttyS4");
    /// let oem = OEM6::new(connection);
    /// let (std, imu, irehs) = oem.get_message()?;
    ///
    /// if let Some(telem) = std {
    ///     println!("Num successful commands: {}", telem.cmd_valid_cntr);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`OEMError`]: enum.OEMError.html
    pub fn get_message(&self) -> OEMResult<()> {
        unimplemented!();
    }

    pub fn get_response(&self) -> OEMResult<()> {
        // Read response header
        let mut message = self.conn.read(RESP_HDR_LEN.into())?;

        if &message[0..3] != SYNC {
            println!("SYNC bytes invalid: {:?}", &message[0..3]);
            throw!(OEMError::BadSync);
        }

        // Pull out message length
        let msg_len = message[3];

        // Read body
        message.append(&mut self.conn.read(msg_len.into())?);

        // Read CRC
        let crc = nom::le_u32(self.conn.read(4)?.as_slice()).unwrap().1;

        // Verify CRC
        let calc = calc_crc(&message);
        if calc != crc {
            // TODO: remove debugging line
            println!("CRC Mismatch: {:X} {:X}", calc, crc);
            throw!(OEMError::BadCRC);
        }

        // Get response ID
        // Parse message? Or pass back to caller?

        // TODO: how to deal with async log messages being interspersed?
        // Probably set up a read thread with two channels: one for responses and one for logs

        // TODO: remove this
        Ok(())
    }
}

/// Common Error for OEM Actions
#[derive(Fail, Display, Debug, Clone, PartialEq)]
pub enum OEMError {
    /// Catch-all error
    #[display(fmt = "Generic Error")]
    GenericError,
    /// A message was received, but the CRC didn't match what was expected
    #[display(fmt = "Bad Response CRC")]
    BadCRC,
    /// A message was received, but didn't start with the correct sync bytes
    #[display(fmt = "Bad Response Sync Bytes")]
    BadSync,
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
