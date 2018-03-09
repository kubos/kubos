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

use failure::Error;
use serial_comm::Connection;

const SYNC: u16 = 0xEB90;
const HDR_SZ: u8 = 6;
const FRAME_SZ: u8 = HDR_SZ + 2;

#[repr(C, packed)]
struct MessageHeader {
    sync: u16,
    data_len: u16,
    msg_id: u8,
    addr: u8,
}

#[repr(C, packed)]
struct GetInfoMessage {
    hdr: MessageHeader,
    crc: u16,
}

pub struct MAI400 {
    conn: Connection,
}

pub fn hello_world() {
    println!("Hello, world!");
}

impl MAI400 {
    /// Constructor for MAI400 structure
    pub fn new(conn: Connection) -> MAI400 {
        MAI400 { conn }
    }

    pub fn init_tx() {}

    pub fn init_rx() {}

    pub fn terminate() {

        //close(tx);
        //close(rx);
    }

    pub fn reset() {
        //REQUEST_RESET
        //CONFIRM_RESET
    }

    pub fn set_mode() {}

    pub fn get_info(&self) -> ADCSResult<()> {
        //Create packet
        let mut packet = GetInfoMessage {
            hdr: {
                sync: SYNC
            },
        };

        //send packet
        self.conn.write(b"hello")?;
        Ok(())
    }

    fn send_message() {}
}

/// Common Error for ADCS Actions
#[derive(Debug, Fail)]
pub enum ADCSError {
    #[fail(display = "Parse error: {}", message)]
    /// There was a problem parsing the result data
    ParseError {
        /// The message from original error
        message: String,
    },
}

/// Custom error type for adcs operations.
pub type ADCSResult<T> = Result<T, Error>;
