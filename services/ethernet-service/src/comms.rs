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
// Contributed by: William Greer (wgreer184@gmail.com) and Sam Justice (sam.justice1@gmail.com)
//

use comms_service::{CommsConfig, CommsResult};
use std::net::UdpSocket;
use std::sync::Arc;

use super::CONFIG_PATH;

// Function to allow reading from a UDP socket.
pub fn read(socket: Arc<UdpSocket>) -> CommsResult<Vec<u8>> {
    let mut buf = [0; 4096];
    let (size, _) = socket.recv_from(&mut buf)?;
    Ok(buf[0..size].to_vec())
}

// Function to allow writing over a UDP socket.
pub fn write(socket: Arc<UdpSocket>, data: &[u8]) -> CommsResult<()> {
    let config = CommsConfig::new("ethernet-service", CONFIG_PATH.to_string());
    socket.send_to(
        data,
        (&*config.ground_ip, config.ground_port.unwrap_or_default()),
    )?;
    Ok(())
}
