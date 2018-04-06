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

use byteorder::{LittleEndian, ReadBytesExt};
use mai400::MAIResult;
//use std::io;
use std::time::Duration;
use serial;
use std::io::prelude::*;
use serial::prelude::*;
//use std::cell::RefCell;
use messages::*;
use std::io::Cursor;

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

    /// Wait for and then return the next message received on the bus
    pub fn read(&self) -> MAIResult<Vec<u8>> {
        self.stream.read()
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

        //TODO: I don't like closing this after every read. how likely is it that this will cause us to miss messages?
        let mut port = serial::open(self.bus.as_str())?;

        port.configure(&self.settings)?;

        let mut ret_msg: Vec<u8>;

        loop {
            ret_msg = Vec::new();

            // We just want to sit and wait until a message starts to come in
            port.set_timeout(Duration::new(0, 0))?;

            let mut sync: [u8; 2] = [0; 2];
            port.read(&mut sync)?;

            let mut wrapper = Cursor::new(sync.to_vec());
            if wrapper.read_u16::<LittleEndian>()? == SYNC {
                ret_msg.append(&mut sync.to_vec());
            } else {
                // Odds are that we magically ended up in the middle of a message,
                // so just loop so we can get all of the bytes out of the buffer
                continue;
            }

            port.set_timeout(Duration::new(0, 1))?;

            // We got the SYNC bytes, so we know we're at the start of a message.
            // Get the rest of the header
            let mut hdr: [u8; HDR_SZ - 2] = [0; HDR_SZ - 2];
            if port.read(&mut hdr)? == 0 {
                // We timed out. Throw out what we've got and start over
                continue;
            }

            // Pull out the data_len value so we know how many more bytes we need to read
            // (Add 2 to account for the CRC bytes)
            let mut wrapper = Cursor::new(hdr[0..2].to_vec());
            let len = wrapper.read_u16::<LittleEndian>()? + 2;

            // Add the rest of the header to our return message
            ret_msg.append(&mut hdr.to_vec());

            let mut data: Vec<u8> = vec![0; len as usize];
            if port.read(&mut data[..])? == 0 {
                // We timed out. Throw out what we've got and start over
                continue;
            }

            ret_msg.append(&mut data);
            break;
        }

        Ok(ret_msg)
    }
}
