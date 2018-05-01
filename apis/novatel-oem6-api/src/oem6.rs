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

use byteorder::{LittleEndian, WriteBytesExt};
use crc32::*;
use messages::*;
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
    /// let connection = Connection::new("/dev/ttyS5");
    /// let oem = OEM6::new(connection);
    /// # Ok(())
    /// # }
    /// ```
    ///
    pub fn new(conn: Connection) -> OEM6 {
        OEM6 { conn }
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
    /// # let connection = Connection::new("/dev/ttyS5");
    /// let oem = OEM6::new(connection);
    ///
    /// let mut array = [0; 8];
    /// array[0] = 0x90; // SYNC byte 1
    /// array[1] = 0xEB; // SYNC byte 2
    /// array[2] = 0x0;  // Data_len byte 1
    /// array[3] = 0x0;  // Data_len byte 2
    /// array[4] = 0x5A; // Msg_id
    /// array[5] = 0x0;  // Addr
    /// array[6] = 0x00; // CRC byte 1
    /// array[7] = 0x00; // CRC byte 2
    ///
    /// oem.passthrough(&array)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`OEMError`]: enum.OEMError.html
    pub fn passthrough(&self, msg: &[u8]) -> OEMResult<()> {
        Ok(self.conn.write(msg)?)
    }

    fn send_message<T: Message>(&self, msg: T) -> OEMResult<()> {
        let mut raw = msg.serialize();

        // Get the calculated CRC
        let crc = CRC32::calc_crc(&CRC32(0), &raw);
        raw.write_u32::<LittleEndian>(crc.0).unwrap();

        self.conn.write(raw.as_slice())?;
        Ok(())
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
    /// # let connection = Connection::new("/dev/ttyS5");
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
    pub fn get_message() {
        unimplemented!();
    }

    pub fn get_response() {
        unimplemented!();
    }
}

/// Common Error for OEM Actions
#[derive(Fail, Display, Debug, Clone, PartialEq)]
pub enum OEMError {
    /// Catch-all error
    #[display(fmt = "Generic Error")]
    GenericError,
    /// Software attempted to send a command packet which was not exactly 40 characters long
    #[display(fmt = "Bad Command Length")]
    BadCommandLen,
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
