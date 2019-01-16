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

use comms_service::CommsResult;
use kiss;
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

    // Function to allow reading a whole udp packet from a serial socket
    // using KISS framing
    pub fn read(&self) -> CommsResult<Vec<u8>> {
        let mut buffer = self.buffer.borrow_mut();
        loop {
            let mut buf = match self.conn.read(1, Duration::from_millis(1)) {
                Ok(buf) => buf,
                Err(_e) => {
                    break;
                }
            };
            buffer.append(&mut buf);
            if buffer.len() > 4096 {
                break;
            }
        }

        match kiss::decode(&buffer) {
            Ok((parsed, mut pre, mut post)) => {
                buffer.clear();
                buffer.append(&mut pre);
                buffer.append(&mut post);
                return Ok(parsed);
            }
            Err(e) => {
                bail!("parse err {:?}", e);
            }
        }
    }

    // Function to allow writing over a UDP socket.
    pub fn write(&self, data: &[u8]) -> CommsResult<()> {
        let wrapped = kiss::encode(data).unwrap();
        self.conn.write(&wrapped)?;
        Ok(())
    }
}

pub fn read_ser(socket: Arc<Mutex<SerialComms>>) -> CommsResult<Vec<u8>> {
    if let Ok(socket) = socket.lock() {
        return Ok(socket.read()?);
    }
    bail!("Failed to lock socket");
}

pub fn write_ser(socket: Arc<Mutex<SerialComms>>, data: &[u8]) -> CommsResult<()> {
    if let Ok(socket) = socket.lock() {
        socket.write(data)?;
    }
    Ok(())
}
