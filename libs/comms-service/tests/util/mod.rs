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

use comms_service::*;
use std::cell::RefCell;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct MockComms {
    pub msg_buff: RefCell<Vec<Vec<u8>>>,
    pub read_buff: RefCell<Vec<Vec<u8>>>,
    pub write_buff: RefCell<Vec<Vec<u8>>>,
}

impl MockComms {
    pub fn new() -> Self {
        MockComms {
            msg_buff: RefCell::new(vec![]),
            read_buff: RefCell::new(vec![]),
            write_buff: RefCell::new(vec![]),
        }
    }

    pub fn read_data(&self, data: &[u8]) {
        let mut buffer = self.read_buff.borrow_mut();
        buffer.push(data.to_vec());
    }

    pub fn read(&self) -> CommsResult<Vec<u8>> {
        let mut buffer = self.read_buff.borrow_mut();

        if let Some(data) = buffer.pop() {
            return Ok(data);
        }
        bail!("Failed to get data");
    }

    pub fn write(&self, data: &[u8]) -> CommsResult<()> {
        let mut buffer = self.write_buff.borrow_mut();
        buffer.push(data.to_vec());
        Ok(())
    }
}

pub fn read(socket: &Arc<Mutex<MockComms>>) -> CommsResult<Vec<u8>> {
    if let Ok(socket) = socket.lock() {
        socket.read()
    } else {
        bail!("Failed to lock socket");
    }
}

pub fn write(socket: &Arc<Mutex<MockComms>>, data: &[u8]) -> CommsResult<()> {
    if let Ok(socket) = socket.lock() {
        socket.write(data).unwrap();
        Ok(())
    } else {
        bail!("Failed to lock socket");
    }
}

pub fn comms_config(
    sat_ip: &str,
    ground_ip: &str,
    ground_port: u16,
    downlink_port: u16,
) -> CommsConfig {
    CommsConfig {
        handler_port_min: Some(18000),
        handler_port_max: Some(18100),
        downlink_ports: Some(vec![downlink_port]),
        timeout: Some(1000),
        ground_ip: ground_ip.to_owned(),
        ground_port: Some(ground_port),
        satellite_ip: sat_ip.to_owned(),
    }
}
