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

use comms;
use radio_api::Connection;
use std::io;

pub struct SerialConnection;

impl Connection for SerialConnection {
    fn send(&self, cmd: &str) -> Result<(), String> {
        match serial_send(cmd) {
            Ok(_) => Ok(()),
            Err(e) => Err(String::from("Error receiving")),
        }
    }

    fn receive(&self) -> Result<Vec<u8>, String> {
        match serial_receive() {
            Ok(d) => Ok(d),
            Err(e) => Err(String::from("Error receiving")),
        }
    }
}

pub fn serial_send(cmd: &str) -> io::Result<()> {
    use std::io::prelude::*;
    use serial::prelude::*;

    let mut ret_msg = Vec::<u8>::new();

    let mut port = try!(serial::open("/dev/ttyUSB0"));

    try!(port.reconfigure(&|settings| {
        settings.set_baud_rate(serial::Baud38400);
        settings.set_char_size(serial::Bits8);
        settings.set_parity(serial::ParityNone);
        settings.set_stop_bits(serial::Stop1);
        settings.set_flow_control(serial::FlowNone);
        Ok(())
    }));

    let send_buf: Vec<u8> = cmd.as_bytes().to_vec();

    try!(port.write(&send_buf[..]));
    Ok(())
}

pub fn serial_receive() -> io::Result<Vec<u8>> {
    use std::io::prelude::*;
    use serial::prelude::*;

    let mut ret_msg = Vec::<u8>::new();

    let mut port = try!(serial::open("/dev/ttyUSB0"));

    try!(port.reconfigure(&|settings| {
        settings.set_baud_rate(serial::Baud38400);
        settings.set_char_size(serial::Bits8);
        settings.set_parity(serial::ParityNone);
        settings.set_stop_bits(serial::Stop1);
        settings.set_flow_control(serial::FlowNone);
        Ok(())
    }));

    try!(port.read(&mut ret_msg[..]));
    Ok(ret_msg)
}
