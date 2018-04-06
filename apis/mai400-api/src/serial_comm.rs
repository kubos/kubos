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

use mai400::MAIResult;
//use std::io;
use std::time::Duration;
use serial;
use std::io::prelude::*;
use serial::prelude::*;
//use std::cell::RefCell;

/// A connection is like a stream, but allowed parsed reads with properly buffered
/// input data.
pub struct Connection {
    pub stream: Box<Stream>,
    //buffer: RefCell<Vec<u8>>,
}

impl Connection {
    /// Convenience constructor to create connection from stream.
    pub fn new(
        bus: String,
        baud_rate: serial::BaudRate,
        char_size: serial::CharSize,
        parity: serial::Parity,
        stop_bits: serial::StopBits,
        flow_control: serial::FlowControl,
    ) -> Connection {

        Connection {
            stream: Box::new(SerialStream {
                bus,
                settings: serial::PortSettings {
                    baud_rate,
                    char_size,
                    parity,
                    stop_bits,
                    flow_control,
                },
            }),
            //buffer: RefCell::new(Vec::new()),
        }
    }

    /// Write out raw bytes to the underlying stream.
    pub fn write(&self, data: &[u8]) -> MAIResult<()> {
        println!("Writing: {:?}", data);
        self.stream.write(data)
    }

    /// Read the next object using provided parser.
    //TODO: Make listener function rather than poller
    pub fn read<T>(&self) -> MAIResult<()> {
        Ok(())
    }
}

/// Connections expect a struct instance with this trait to represent streams.
pub trait Stream {
    /// Write raw bytes to the stream.
    fn write(&self, data: &[u8]) -> MAIResult<()>;
    /// Read raw bytes from the stream.
    fn read(&self) -> MAIResult<Vec<u8>>;
}

struct SerialStream {
    bus: String,
    settings: serial::PortSettings,
}

impl Stream for SerialStream {
    //TODO: Encapsulate the possible IO errors into appropriate MAIError values
    fn write(&self, data: &[u8]) -> MAIResult<()> {
        //But why don't you just make 'port' a field of SerialStream and then you
        //only have to open the connection once, during new?
        //
        //Because the write and read functions require port to be mutable (for...reasons),
        //so you'd end up doing this massive chain of (&mut self) definitions in all your
        //functions and that seems silly
        let mut port = serial::open(self.bus.as_str())?;

        port.configure(&self.settings)?;

        port.set_timeout(Duration::from_secs(1))?;

        port.flush()?;
        port.write(data)?;

        Ok(())
    }

    fn read(&self) -> MAIResult<Vec<u8>> {
        //TODO: This will have to change to listening
        let mut ret_msg: Vec<u8> = Vec::new();
        let mut port = serial::open(self.bus.as_str())?;

        port.configure(&self.settings)?;

        port.set_timeout(Duration::from_millis(100))?;

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
}
