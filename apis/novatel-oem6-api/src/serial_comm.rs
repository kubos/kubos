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
use oem6::{OEMError, OEMResult};
use messages::*;
use std::io::Cursor;
use std::io::prelude::*;
use std::time::Duration;
use serial;
use serial::prelude::*;

/// Wrapper structure for underlying stream
#[derive(Clone)]
pub struct Connection {
    /// UART stream to interact with
    /// It's wrapped in a Box so that it can be easily mocked for unit tests
    pub stream: Box<Stream>,
}

impl Connection {
    /// Convenience constructor to create connection from stream.
    pub fn new(bus: &str) -> Connection {
        Connection {
            stream: Box::new(SerialStream {
                bus: bus.to_owned(),
                settings: serial::PortSettings {
                    baud_rate: serial::Baud9600,
                    char_size: serial::Bits8,
                    parity: serial::ParityNone,
                    stop_bits: serial::Stop1,
                    flow_control: serial::FlowNone,
                },
            }),
        }
    }

    /// Write out raw bytes to the underlying stream.
    pub fn write(&self, data: &[u8]) -> OEMResult<()> {
        /*
        print!("Sending:");
        for &elem in data.iter() {
            print!(" {:X}", elem);
        }
        println!("");
        */

        self.stream.write(data)
    }

    /// Wait for and then return the next message received on the bus
    pub fn read(&self) -> OEMResult<Vec<u8>> {
        self.stream.read()
    }
}

/// Connections expect a struct instance with this trait to represent streams.
pub trait Stream: StreamClone {
    /// Write raw bytes to the stream.
    fn write(&self, data: &[u8]) -> OEMResult<()>;
    /// Read raw bytes from the stream.
    fn read(&self) -> OEMResult<Vec<u8>>;
}

pub trait StreamClone {
    fn clone_box(&self) -> Box<Stream>;
}

impl<T> StreamClone for T
where
    T: 'static + Stream + Clone,
{
    fn clone_box(&self) -> Box<Stream> {
        Box::new(self.clone())
    }
}

// We can now implement Clone manually by forwarding to clone_box.
impl Clone for Box<Stream> {
    fn clone(&self) -> Box<Stream> {
        self.clone_box()
    }
}

#[derive(Clone)]
struct SerialStream {
    bus: String,
    settings: serial::PortSettings,
}

impl Stream for SerialStream {
    fn write(&self, data: &[u8]) -> OEMResult<()> {
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

    fn read(&self) -> OEMResult<Vec<u8>> {
        //TODO: I don't like closing this after every read. how likely is it that this will cause us to miss messages?
        let mut port = serial::open(self.bus.as_str())?;

        port.configure(&self.settings)?;

        let mut ret_msg: Vec<u8> = Vec::new();

        loop {
            ret_msg.clear();

            port.set_timeout(Duration::new(0, 10))?;

            let mut sync: [u8; 3] = [0; 3];
            match port.read(&mut sync) {
                Ok(len) => {
                    if len != 3 {
                        continue;
                    }
                }
                Err(err) => {
                    match err.kind() {
                        ::std::io::ErrorKind::TimedOut => continue, //TODO: Govern with a master timer? Or will the set_timeout call be enough? Needs to be tested
                        _ => throw!(err),
                    }
                }
            }

            if sync == SYNC {
                ret_msg.extend_from_slice(&sync);
            } else {
                // Odds are that we magically ended up in the middle of a message,
                // so just loop so we can get all of the bytes out of the buffer
                continue;
            }

            let mut data: [u8; 25] = [0; 25];
            match port.read(&mut data) {
                Ok(v) => v,            //println!("Got {} bytes", v),
                Err(_err) => continue, //TODO: process timeout
            };

            let mut len = data[5] as isize;
            len += 4; //CRC bytes
                      //println!("Message len: {}", len);

            ret_msg.extend_from_slice(&data);

            //let mut len = 0;

            while len > 0 {
                let mut data: Vec<u8> = vec![0; len as usize];
                let temp = match port.read(&mut data[..]) {
                    Ok(v) => {
                        //println!("Got {} bytes", v);
                        v
                    }
                    Err(_err) => {
                        //println!("Read error. Looping");
                        continue;
                    } //TODO: process timeout
                };

                len -= temp as isize;
                ret_msg.append(&mut data[0..temp].to_vec());
            }

            /*
            let mut data: Vec<u8> = vec![0; len as usize];
            match port.read(&mut data[..]) {
                Ok(v) => //println!("Got {} bytes", v),
                Err(err) => {
                    //println!("Body read failed: {:?}", err);
                    break; //TODO: process timeout
                }
            };

            ret_msg.extend_from_slice(&data);
            */

            break;
        }

        /*
        print!("Received({}):", ret_msg.len());
        for elem in ret_msg.iter() {
            print!(" {:X}", elem);
        }
        println!("");
        */

        Ok(ret_msg)
    }
}
