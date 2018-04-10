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
//TODO: remove before publishing
#![allow(unused)]

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use crc16::*;
use failure::Error;
use messages::*;
use serial;
use serial_comm::Connection;
use std::io;
use std::io::Cursor;

//TODO: Create container trait? Or just for the serial connection part...

pub struct MAI400 {
    pub conn: Connection,
}

impl MAI400 {
    /// Constructor for MAI400 structure
    pub fn new(conn: Connection) -> MAI400 {
        MAI400 { conn }
    }

    pub fn reset() {
        //REQUEST_RESET
        //CONFIRM_RESET
    }

    //SetAcsMode
    pub fn set_mode(
        &self,
        mode: u8,
        sec_vec: i32,
        pri_axis: i32,
        sec_axis: i32,
        qbi_cmd4: i32,
    ) -> MAIResult<()> {
        let request = SetAcsMode {
            mode,
            sec_vec,
            pri_axis,
            sec_axis,
            qbi_cmd4,
            ..Default::default()
        };

        self.send_message(request.serialize())
    }

    pub fn set_gps_time(&self) -> MAIResult<()> {
        unimplemented!()
    }

    pub fn set_rv(&self) -> MAIResult<()> {
        unimplemented!()
    }

    pub fn get_info(&self) -> MAIResult<()> {
        self.send_message(GetInfo::default().serialize())
    }
    //Option 2
    //Don't actually merge this. Need to figure out which way is preferable
    /*
    pub fn get_info_alt(&self) -> MAIResult<()> {
        //Create packet
        let packet = GetInfoMessage::default();
        let slice = unsafe {
            ::std::mem::transmute::<GetInfoMessage, [u8; ::std::mem::size_of::<GetInfoMessage>()]>(
                packet,
            )
        };

        //send packet
        self.conn.write(&slice)?;
        Ok(())
    }
    */

    fn send_message(&self, mut msg: Vec<u8>) -> MAIResult<()> {
        let crc = State::<AUG_CCITT>::calculate(&msg[2..]);
        msg.write_u16::<LittleEndian>(crc).unwrap();

        //send packet
        self.conn.write(msg.as_slice())?;
        Ok(())
    }

    pub fn get_message(&self) -> MAIResult<Response> {
        let mut msg = vec![];
        let mut response: Response;

        loop {
            println!("Staring read");
            msg = self.conn.read()?;

            // Get the expected CRC
            let len = msg.len();
            println!("Len: {}", len);

            //            let mut s = String::new();

            let mut raw = msg.split_off(len - 3);
            //println!("Raw: {:?}", raw);
            let mut crc = Cursor::new(raw.to_vec());
            let crc = crc.read_u16::<LittleEndian>()?;
            //println!("CRC: {}", crc);

            // Get the calculated CRC
            let calc = State::<AUG_CCITT>::calculate(&msg[1..]);

            // Make sure they match
            // If not, pretend this never happened and go get another message
            if calc != crc {
                //println!("CRC mismatch: {} {}", calc, crc);
                continue;
            }

            // Put the CRC bytes back and go process the message
            msg.append(&mut raw);

            let id = msg[5];

            // Identify message type and convert to usable structure
            match id {
                1 => {
                    println!("got telemetry message");
                    let info = StandardTelemetry::new(&msg[..]);
                    response = Response::StdTelem(telem);
                    break;
                }
                6 => {
                    println!("got info message");
                    let info = ConfigInfo::new(&msg[..]);
                    response = Response::Config(info);
                    break;
                }
                _ => {
                    println!("Got other message");
                    throw!(MAIError::GenericError);
                }
            }
        }

        Ok(response)

    }
}

/* 
TODO: Deal with the fact that you can't clone io::error,
but double requires the ability to clone errors 

#[derive(Fail, Display, Debug)]
pub enum MAIError {
    #[display(fmt = "Generic Error")]
    GenericError,
    #[display(fmt = "Serial Error: {}", cause)]
    /// There was a problem parsing the result data
    SerialError { #[fail(cause)] cause: serial::Error },
    #[display(fmt = "IO Error: {}", cause)]
    /// There was a problem parsing the result data
    IoError {
        #[fail(cause)]
        cause: io::Error,
    },
}

impl From<io::Error> for MAIError {
    fn from(error: io::Error) -> Self {
        MAIError::IoError { cause: error }
    }
}

impl From<serial::Error> for MAIError {
    fn from(error: serial::Error) -> Self {
        MAIError::SerialError { cause: error }
    }
}
*/

/// Common Error for MAI Actions
#[derive(Fail, Display, Debug, Clone, PartialEq)]
pub enum MAIError {
    #[display(fmt = "Generic Error")]
    GenericError,
    #[display(fmt = "Serial Error: {}", cause)]
    /// There was a problem parsing the result data
    SerialError { cause: String },
    #[display(fmt = "IO Error: {}", cause)]
    /// There was a problem parsing the result data
    IoError { cause: String },
}

impl From<io::Error> for MAIError {
    fn from(error: io::Error) -> Self {
        MAIError::IoError { cause: format!("{}", error) }
    }
}

impl From<serial::Error> for MAIError {
    fn from(error: serial::Error) -> Self {
        MAIError::SerialError { cause: format!("{}", error) }
    }
}

/// Custom error type for MAI400 operations.
pub type MAIResult<T> = Result<T, MAIError>;
