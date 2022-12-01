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

// Check that the user data partition is still writeable and readable.
// If it's not, send a distress beacon over the radio

use super::*;
use std::fs::{self, File};
use std::io::prelude::*;
use std::net::UdpSocket;

pub fn check_fs() -> Result<(), Error> {
    // Check that the user data partition is still writeable.
    let test_file = "/home/obc-test-file";
    let test_string = "I'm a test string";

    if check_write(test_file, test_string)
        .and(check_read(test_file, test_string))
        .is_err()
        && !COMMS_SERVICE.is_empty()
    {
        let _ = send_error();
    }

    // Cleanup
    let _ = fs::remove_file(test_file);
    Ok(())
}

fn check_write(file_name: &str, test_string: &str) -> Result<(), Error> {
    let mut file = File::create(file_name)?;
    file.write_all(test_string.as_bytes())?;
    file.sync_all()?;

    Ok(())
}

fn check_read(file_name: &str, test_string: &str) -> Result<(), Error> {
    let mut file = File::open(file_name)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    if contents != test_string {
        bail!("Failed to verify test file");
    }

    Ok(())
}

fn send_error() -> Result<(), Error> {
    // Get the comms service IP address
    let config = ServiceConfig::new(COMMS_SERVICE)?;
    let host = config
        .hosturl()
        .ok_or_else(|| failure::format_err!("Unable to fetch addr for comms service"))?;
    let mut host_parts = host.split(':').map(|val| val.to_owned());
    let comms_ip = host_parts.next().unwrap_or_else(|| {
        error!("Failed to lookup comms service IP. Using default 0.0.0.0");
        "0.0.0.0".to_owned()
    });
    let downlink = format!("{}:{}", comms_ip, DOWNLINK_PORT);

    // Bind a socket for the host IP
    let local_socket = UdpSocket::bind("0.0.0.0:0")?;

    // Send our distress message
    // Try to log the occurance, even though logging probably isn't working right now
    match local_socket.send_to(b"USER PARTITION CORRUPTED", downlink) {
        Ok(_) => error!("Sent distress beacon"),
        Err(err) => error!("Failed to send distress beacon: {:?}", err),
    }

    Ok(())
}
