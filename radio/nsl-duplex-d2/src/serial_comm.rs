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

use radio_api::Connection;
use std::io;
use std::time::Duration;
use serial;

/// Connection for communicating with actual
/// Duplex-D2 hardware
pub struct SerialConnection;

impl Connection for SerialConnection {
    fn send(&self, data: &[u8]) -> Result<(), String> {
        match serial_send(data) {
            Ok(_) => Ok(()),
            Err(_) => Err(String::from("Error sending")),
        }
    }

    fn receive(&self) -> Result<Vec<u8>, String> {
        match serial_receive() {
            Ok(d) => Ok(d),
            Err(_) => Err(String::from("Error receiving")),
        }
    }
}

pub fn serial_send(data: &[u8]) -> io::Result<()> {
    use std::io::prelude::*;
    use serial::prelude::*;

    let mut port = try!(serial::open("/dev/ttyUSB0"));
    let settings: serial::PortSettings = serial::PortSettings {
        baud_rate: serial::Baud38400,
        char_size: serial::Bits8,
        parity: serial::ParityNone,
        stop_bits: serial::Stop1,
        flow_control: serial::FlowNone,
    };
    try!(port.configure(&settings));

    try!(port.set_timeout(Duration::from_secs(1)));

    let be_data = {
        let mut v = Vec::<u8>::new();
        for i in 0..data.len() {
            v.push(data[i].to_be());
        }
        v
    };

    try!(port.flush());
    try!(port.write(&be_data[..]));

    Ok(())
}

pub fn serial_receive() -> io::Result<Vec<u8>> {
    use std::io::prelude::*;
    use serial::prelude::*;

    let mut ret_msg: Vec<u8> = Vec::new();
    let mut port = try!(serial::open("/dev/ttyUSB0"));

    let settings: serial::PortSettings = serial::PortSettings {
        baud_rate: serial::Baud38400,
        char_size: serial::Bits8,
        parity: serial::ParityNone,
        stop_bits: serial::Stop1,
        flow_control: serial::FlowNone,
    };
    try!(port.configure(&settings));

    try!(port.set_timeout(Duration::from_millis(100)));

    let mut tries = 0;

    loop {
        let mut read_buffer: Vec<u8> = vec![0; 1];

        match port.read(&mut read_buffer[..]) {
            Ok(c) => {
                if c > 0 {
                    ret_msg.extend(read_buffer);
                } else {
                    tries = tries + 1;
                }
            }
            Err(_) => break,
        };
        if tries > 5 {
            break;
        }
    }

    Ok(ret_msg)
}
