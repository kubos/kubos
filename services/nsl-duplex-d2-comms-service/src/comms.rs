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
use crate::NslDuplexCommsResult;
use std::cell::RefCell;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use nsl_duplex_d2::{DuplexD2, serial_connection, File};

pub struct DuplexComms {
    radio: DuplexD2,
    buffer: RefCell<Vec<u8>>,
}

impl DuplexComms {
    pub fn new(path: &str) -> Self {
        let serial_conn = serial_connection(path);
        let radio = DuplexD2::new(serial_conn);

        DuplexComms {
            radio,
            buffer: RefCell::new(vec![]),
        }
    }

    // Function to allow reading a whole UDP packet from a serial socket
    // using KISS framing
    pub fn read(&self) -> NslDuplexCommsResult<Vec<u8>> {
        let mut buffer = self.buffer.borrow_mut();
        let file = self.radio.get_uploaded_file().unwrap();

        match kiss::decode(&file.body) {
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
    pub fn write(&self, data: &[u8]) -> NslDuplexCommsResult<()> {
        let wrapped = kiss::encode(data);
        let file = File::new("", data);
        self.radio.put_download_file(&file).unwrap();
        Ok(())
    }
}

pub fn read_ser(socket: &Arc<Mutex<DuplexComms>>) -> NslDuplexCommsResult<Vec<u8>> {
    if let Ok(socket) = socket.lock() {
        return Ok(socket.read()?);
    }
    bail!("Failed to lock socket");
}

pub fn write_ser(socket: &Arc<Mutex<DuplexComms>>, data: &[u8]) -> NslDuplexCommsResult<()> {
    if let Ok(socket) = socket.lock() {
        socket.write(data)?;
    }
    Ok(())
}
