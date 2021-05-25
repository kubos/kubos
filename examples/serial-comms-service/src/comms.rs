//
// Copyright (C) 2019 Kubos Corporation
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

//!
//! Serial communications functionality for use in conjunction
//! with the communications service library. KISS framing is
//! implemented for data integrity over the serial link.
//!

use crate::kiss;
use crate::SerialServiceResult;
use rust_uart::Connection;
use std::cell::RefCell;
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub struct SerialComms {
    conn: Connection,
    buffer: RefCell<Vec<u8>>,
}

impl SerialComms {
    pub fn new(path: &str) -> Self {
        let serial_settings = serial::PortSettings {
            baud_rate: serial::Baud115200,
            char_size: serial::Bits8,
            parity: serial::ParityNone,
            stop_bits: serial::Stop1,
            flow_control: serial::FlowNone,
        };

        let conn = Connection::from_path(path, serial_settings, Duration::from_millis(1)).unwrap();

        SerialComms {
            conn,
            buffer: RefCell::new(vec![]),
        }
    }

    // Function to allow reading a whole UDP packet from a serial socket
    // using KISS framing
    pub fn read(&self) -> SerialServiceResult<Vec<u8>> {
        let mut buffer = self.buffer.borrow_mut();
        while let Ok(mut buf) = self.conn.read(1, Duration::from_millis(1)) {
            buffer.append(&mut buf);
            if buffer.len() > 4096 {
                break;
            }
        }

        match kiss::decode(&buffer) {
            Ok(kiss::DecodedData {
                frame,
                mut pre_data,
                mut post_data,
            }) => {
                buffer.clear();
                buffer.append(&mut pre_data);
                buffer.append(&mut post_data);
                Ok(frame)
            }
            Err(e) => {
                bail!("Parse err {:?}", e);
            }
        }
    }

    // Function to allow writing over a UDP socket.
    pub fn write(&self, data: &[u8]) -> SerialServiceResult<()> {
        let wrapped = kiss::encode(data);
        self.conn.write(&wrapped)?;
        Ok(())
    }
}

pub fn read_ser(socket: &Arc<Mutex<SerialComms>>) -> SerialServiceResult<Vec<u8>> {
    if let Ok(socket) = socket.lock() {
        return socket.read();
    }
    bail!("Failed to lock socket");
}

pub fn write_ser(socket: &Arc<Mutex<SerialComms>>, data: &[u8]) -> SerialServiceResult<()> {
    if let Ok(socket) = socket.lock() {
        socket.write(data)?;
    }
    Ok(())
}
