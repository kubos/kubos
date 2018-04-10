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
    pub fn new(bus: String) -> Connection {
        Connection {
            stream: Box::new(SerialStream {
                bus,
                settings: serial::PortSettings {
                    baud_rate: serial::Baud115200,
                    char_size: serial::Bits8,
                    parity: serial::ParityNone,
                    stop_bits: serial::Stop1,
                    flow_control: serial::FlowNone,
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
        /*        {
            use termios::{CREAD, CLOCAL}; // cflags
            use termios::{ICANON, ECHO, ECHOE, ECHOK, ECHONL, ISIG, IEXTEN}; // lflags
            use termios::OPOST; // oflags
            use termios::{INLCR, IGNCR, ICRNL, IGNBRK}; // iflags
            use termios::{VMIN, VTIME}; // c_cc indexes

            let mut termios = match termios::Termios::from_fd(self.fd) {
                Ok(t) => t,
                Err(e) => return Err(super::error::from_io_error(e)),
            };

            // setup TTY for binary serial port access
            termios.c_cflag |= CREAD | CLOCAL;
            termios.c_lflag &= !(ICANON | ECHONL);
            //termios.c_oflag &= !OPOST;
            termios.c_iflag &= !(INLCR | IGNCR | ICRNL | IGNBRK);

            termios.c_cc[VMIN] = 0;
            termios.c_cc[VTIME] = 0;

        } */

        let mut ret_msg: Vec<u8> = Vec::new();
        loop {
            ret_msg.clear();

            // Messages should be coming out every 250 msec,
            // so giving the timeout a little bit of wiggle room
            port.set_timeout(Duration::new(0, 500))?;

            let mut sync: [u8; 2] = [0; 2];
            //println!("First read");
            let mut len = match port.read(&mut sync) {
                Ok(v) => v,
                Err(_err) => continue, //TODO: process timeout
            };

            let mut wrapper = Cursor::new(sync.to_vec());
            let check = wrapper.read_u16::<LittleEndian>()?;
            if check == SYNC {
                ret_msg.append(&mut sync.to_vec());
            } else {
                // Odds are that we magically ended up in the middle of a message,
                // so just loop so we can get all of the bytes out of the buffer
                //println!("Got unknown: {:?}", sync);
                continue;
            }

            //port.set_timeout(Duration::new(0, 1))?;
            while len < 200 {
                let mut data: Vec<u8> = vec![0; 46];
                //println!("Third read");
                let temp = match port.read(&mut data[..]) {
                    Ok(v) => v,
                    Err(_err) => continue, //TODO: process timeout
                };

                len += temp;
                ret_msg.append(&mut data[0..temp].to_vec());
            }

            println!("Read len: {}", len);
            break;
        }

        println!("Returning");
        Ok(ret_msg)
    }
}
