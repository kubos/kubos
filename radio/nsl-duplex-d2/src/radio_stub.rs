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

use std::io;
use std::io::prelude::*;
use serial::prelude::*;

pub const RESP_HEADER: &'static str = "GU";

#[cfg(any(target_arch = "x86_64"))]
pub fn send_command(cmd: &str) -> Result<Vec<u8>, String> {
    let mut ret_msg = Vec::<u8>::new();

    // Almost all responses are preceded by 0x4755 or 'GU'
    ret_msg.extend(RESP_HEADER.as_bytes().iter().cloned());

    match cmd {
        // Returns uploaded file count as
        // 4 byte unsigned integer
        comms::GET_UPLOADED_FILE_COUNT => {
            ret_msg.push(1 as u8);
            ret_msg.push(0 as u8);
            ret_msg.push(0 as u8);
            ret_msg.push(0 as u8);
            Ok(ret_msg)
        }
        // Returns uploaded message as
        // 3 ascii digits - file name size
        // 6 ascii digits - file size
        // N ascii digits - file name
        // N bytes - data
        // 2 bytes - CRC
        comms::GET_UPLOADED_FILE => {
            let name_size = String::from("008");
            let size = String::from("000004");
            let name = String::from("test.txt");
            let data = String::from("test");
            let crc = String::from("44");

            ret_msg.extend(name_size.as_bytes().iter().cloned());
            ret_msg.extend(size.as_bytes().iter().cloned());
            ret_msg.extend(name.as_bytes().iter().cloned());
            ret_msg.extend(data.as_bytes().iter().cloned());
            ret_msg.extend(crc.as_bytes().iter().cloned());
            Ok(ret_msg)
        }
        &_ => Err(String::from("Command not recognized")),
    }
}

#[cfg(any(target_arch = "arm"))]
pub fn send_command(cmd: &str) -> io::Result<Vec<u8>> {
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
    try!(port.read(&mut ret_msg[..]));
    Ok(ret_msg)
}
