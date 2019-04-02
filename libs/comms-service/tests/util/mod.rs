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

use comms_service::*;
use std::cell::RefCell;
use std::str;
use std::sync::{Arc, Barrier, Mutex};
use std::thread;
use warp::{self, Buf, Filter};

// This MockComms structure allows for easy integration testing
// of the comms_service crate by giving direct access to the
// "radio" buffers that ground/flight read and write to.
#[derive(Debug)]
pub struct MockComms {
    pub read_buff: RefCell<Vec<Vec<u8>>>,
    pub write_buff: RefCell<Vec<Vec<u8>>>,
}

impl MockComms {
    pub fn new() -> Self {
        MockComms {
            read_buff: RefCell::new(Vec::<Vec<u8>>::with_capacity(50)),
            write_buff: RefCell::new(Vec::<Vec<u8>>::with_capacity(50)),
        }
    }

    // Used by comms service to read from radio
    pub fn read(&self) -> CommsResult<Vec<u8>> {
        let mut buffer = self.read_buff.borrow_mut();

        if let Some(data) = buffer.pop() {
            return Ok(data);
        }
        bail!("Failed to get data");
    }

    // Used by comms service to write to radio
    pub fn write(&self, data: &[u8]) -> CommsResult<()> {
        let mut buffer = self.write_buff.borrow_mut();
        buffer.push(data.to_vec());
        Ok(())
    }

    // Push data into the radio's read buffer
    // "Ground has sent a packet to be read"
    pub fn push_read(&self, data: &[u8]) {
        let mut buffer = self.read_buff.borrow_mut();
        buffer.push(data.to_vec());
    }

    // Remove a packet from the radio's write buffer
    // "Ground is reading a packet from the radio"
    pub fn pop_write(&self) -> Option<Vec<u8>> {
        let mut buffer = self.write_buff.borrow_mut();
        let ret_data = buffer.pop();
        ret_data
    }
}

// Read fn for CommsControlBlock
pub fn read(socket: &Arc<Mutex<MockComms>>) -> CommsResult<Vec<u8>> {
    if let Ok(socket) = socket.lock() {
        socket.read()
    } else {
        bail!("Failed to lock socket");
    }
}

// Write fn for CommsControlBlock
pub fn write(socket: &Arc<Mutex<MockComms>>, data: &[u8]) -> CommsResult<()> {
    if let Ok(socket) = socket.lock() {
        socket.write(data).unwrap();
        Ok(())
    } else {
        bail!("Failed to lock socket");
    }
}

// Convenience config generator
pub fn comms_config(
    sat_ip: &str,
    ground_ip: &str,
    ground_port: u16,
    downlink_port: u16,
) -> CommsConfig {
    CommsConfig {
        max_num_handlers: Some(10),
        downlink_ports: Some(vec![downlink_port]),
        timeout: Some(1000),
        ground_ip: ground_ip.to_owned(),
        ground_port: Some(ground_port),
        satellite_ip: sat_ip.to_owned(),
    }
}

pub fn spawn_http_server(
    payload: Vec<u8>,
    thread_data: Arc<Mutex<Vec<u8>>>,
    service_ip: &str,
    barrier: Arc<Barrier>,
) {
    let routes = warp::post2()
        .and(warp::any())
        .and(warp::body::concat())
        .map(move |mut body: warp::body::FullBody| {
            let mut data = vec![];
            let mut remaining = body.remaining();
            while remaining != 0 {
                let cnt = body.bytes().len();
                let mut body_bytes = body.bytes().to_vec();
                data.append(&mut body_bytes);
                body.advance(cnt);
                remaining -= cnt;
            }

            if let Ok(mut thread_data_handle) = thread_data.lock() {
                thread_data_handle.append(&mut data);
            }

            barrier.wait();

            // Send a response back to the ground via the handler port
            str::from_utf8(&payload).unwrap().to_owned()
        });
    let service_ip: std::net::SocketAddrV4 = service_ip.parse().unwrap();
    thread::spawn(move || warp::serve(routes).run(service_ip));
}
