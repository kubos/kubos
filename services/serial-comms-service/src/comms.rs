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

use comms_service::CommsResult;
use kiss;
use rust_uart::Connection;
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub struct SerialComms {
    conn: Connection,
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

        SerialComms { conn }
    }

    // Function to allow reading a whole udp packet from a serial socket
    // using KISS framing
    pub fn read(&self) -> CommsResult<Vec<u8>> {
        let mut packet_buf = Vec::<u8>::new();
        let mut tries = 0;
        loop {
            let mut buf = match self.conn.read(1, Duration::from_millis(1)) {
                Ok(buf) => buf,
                Err(_e) => {
                    break;
                }
            };
            packet_buf.append(&mut buf);
            if tries > 409 {
                bail!("Failed to parse");
            }
            match kiss::decode(&packet_buf) {
                Ok(parsed) => {
                    return Ok(parsed);
                }
                Err(_e) => {}
            }
            tries += 1;
        }

        match kiss::decode(&packet_buf) {
            Ok(parsed) => {
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
