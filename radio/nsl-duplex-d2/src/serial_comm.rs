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

extern crate serial;

use radio_api::Connection;
use std::io;
use std::time::Duration;
use std::thread;

pub struct SerialConnection;

impl Connection for SerialConnection {
    fn send(&self, data: Vec<u8>) -> Result<(), String> {
        match serial_send(&data) {
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

    println!("Open send port");
    let mut port = try!(serial::open("/dev/ttyUSB0"));
    println!("Configure send port");
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

    println!("Serial sending {:?}", be_data);

    try!(port.flush());
    println!("Port flushed");
    let _count = try!(port.write(&be_data[..]));

    println!("Wrote {}", _count);

    Ok(())
}

pub fn serial_receive() -> io::Result<Vec<u8>> {
    use std::io::prelude::*;
    use serial::prelude::*;

    let mut ret_msg: Vec<u8> = Vec::new();
    println!("Open receive port");

    let mut port = try!(serial::open("/dev/ttyUSB0"));

    println!("Configure port");
    let settings: serial::PortSettings = serial::PortSettings {
        baud_rate: serial::Baud38400,
        char_size: serial::Bits8,
        parity: serial::ParityNone,
        stop_bits: serial::Stop1,
        flow_control: serial::FlowNone,
    };
    try!(port.configure(&settings));

    try!(port.set_timeout(Duration::from_millis(100)));

    let mut amount = 0;
    let mut tries = 0;

    println!("Attempt to read");

    loop {
        let mut read_buffer: Vec<u8> = vec![0; 1];

        match port.read(&mut read_buffer[..]) {
            Ok(c) => {
                if c > 0 {
                    println!("Read in {} bytes", c);
                    println!("Serial received {:?}", read_buffer);
                    ret_msg.extend(read_buffer);
                } else {
                    tries = tries + 1;
                }
            },
            Err(_) => break
        };
        if tries > 5 {
            break;
        }
    }

    println!("Final received {:?}", ret_msg);

    Ok(ret_msg)
}
