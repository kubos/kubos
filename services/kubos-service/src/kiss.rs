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

use std::cell::RefCell;
use std::io;
use std::net::{SocketAddr, UdpSocket};

///Frame delimiter code, used to represent start and end of frames.
pub const FEND: u8 = 0xC0;

///Frame escape code, used to escape FESC and FEND codes if they are found in byte stream
pub const FESC: u8 = 0xDB;

///Escaped FEND value
pub const TFEND: u8 = 0xDC;

///Escaped FESC value
pub const TFESC: u8 = 0xDD;

pub trait FrameHandler {
    fn handle_frame(&self, socket: &KissUdpSocket, frame: &[u8], src_addr: SocketAddr) { }
}

pub struct KissUdpSocket<'a> {
    pub addr: SocketAddr,
    socket: UdpSocket,
    // TODO: implement a per-client frame
    current_frame: RefCell<Vec<u8>>,
    handler: &'a FrameHandler,
    pub looping: RefCell<bool>,
}

impl<'a> KissUdpSocket<'a> {
    pub fn bind(addr: SocketAddr, handler: &'a FrameHandler) -> Option<Self> {
        match UdpSocket::bind(&addr) {
            Ok(socket) => {
                //println!("Listening on {}", addr);
                Some(Self {
                    addr: addr,
                    socket: socket,
                    current_frame: RefCell::new(Vec::new()),
                    handler: handler,
                    looping: RefCell::new(false),
                })
            },
            Err(_) => None
        }
    }

    pub fn handle(&self, chunk: &[u8], src_addr: SocketAddr) {
        //println!("Received {} bytes", chunk.len());
        let mut current_frame = self.current_frame.borrow_mut();
        current_frame.extend(chunk);

        let mut begin: usize = 0;
        let mut state = State::OutsideFrame;

        #[derive(PartialEq)]
        enum State {
            OutsideFrame,
            InsideFrame,
        }

        for index in 0..current_frame.len() {
            match current_frame[index] {
                FEND => match state {
                    State::OutsideFrame => {
                        state = State::InsideFrame;
                        begin = index;
                        continue;
                    },
                    _ => {
                        if state == State::InsideFrame {
                            self.handle_frame(&current_frame[begin..index+1], src_addr);
                            current_frame.drain(..index+1);
                        }
                        state = State::OutsideFrame;
                        continue;
                    },
                },
                _ => {},
            }
        }
    }

    fn handle_frame(&self, frame: &[u8], src_addr: SocketAddr) {
        match self.decode(frame) {
            Ok(decoded) => {
                self.handler.handle_frame(&self, &decoded, src_addr);
            },
            Err(e) => { eprintln!("Warning, handed invalid frame: {:?}", e); }
        }
    }

    fn escape(&self, buf: &[u8]) -> Vec<u8> {
        let mut new_buf = vec![];
        for (_, e) in buf.iter().enumerate() {
            match e {
                0xC0 => {
                    new_buf.push(0xDB);
                    new_buf.push(0xDC);
                }
                0xDB => {
                    new_buf.push(0xDB);
                    new_buf.push(0xDD);
                }
                _ => new_buf.push(*e),
            };
        }
        new_buf
    }

    fn unescape(&self, buf: &[u8]) -> (Vec<u8>, bool) {
        let mut new_buf = vec![];

        let mut i = 0;
        while i < buf.len() {
            let e = buf[i];
            match e {
                0xDB => {
                    new_buf.push(match buf.get(i + 1) {
                        Some(0xDC) => 0xC0,
                        Some(0xDD) => 0xDB,
                        _ => {
                            return (vec![], false);
                        }
                    });
                    i += 1;
                }
                _ => new_buf.push(e),
            }
            i += 1;
        }

        (new_buf, true)
    }

    pub fn encode(&self, frame: &[u8]) -> Result<Vec<u8>, String> {
        let mut buff = vec![0xC0, 0x00];

        buff.extend(self.escape(frame).iter().clone());
        buff.push(0xC0);

        Ok(buff)
    }

    pub fn decode(&self, chunk: &[u8]) -> Result<Vec<u8>, String> {
        let mut frame = vec![];
        let mut index_l = 0;
        let mut valid = false;

        while !valid {
            frame.clear();
            let mut index_a = 0;
            let mut index_b;

            // Search for first full kiss frame
            for (i, e) in chunk.iter().skip(index_l).enumerate() {
                if *e == 0xC0 {
                    if chunk[i + 1] == 0x00 {
                        index_a = i + index_l + 1;
                        break;
                    }
                }
            }
            if index_a == 0 {
                return Err(String::from("Kiss frame start not found"));
            }

            index_b = 0;
            // Search for end sequence?
            for (i, e) in chunk.iter().skip(index_a).enumerate() {
                if *e == 0xC0 {
                    index_b = i + index_a + 1;
                    break;
                }
            }
            if index_b == 0 {
                return Err(String::from("Kiss frame end not found"));
            }

            // Extract the frame payload
            frame.extend(chunk[index_a + 1..index_b - 1].iter().clone());
            index_l = index_b;

            // Unescape KISS control characters
            let (un_frame, check) = self.unescape(&frame);
            valid = check;
            frame = un_frame;
        }

        Ok(frame)
    }

    pub fn send_to(&self, frame: &[u8], dest: SocketAddr) -> Result<usize, io::Error> {
        let encoded = match self.encode(frame) {
            Ok(e) => e,
            Err(msg) => return Err(io::Error::new(io::ErrorKind::Other, msg))
        };

        let chunk_iter = encoded.chunks(1024);
        let mut total_written = 0;

        for chunk in chunk_iter {
            let mut chunk_written = 0;
            let mut start = 0;
            'chunk_loop: loop {
                match self.socket.send_to(&chunk[start..], &dest) {
                    Ok(written) => {
                        //println!("Wrote {} bytes", written);
                        total_written += written;
                        chunk_written += written;
                        start += written;
                    },
                    Err(err) => {
                        return Err(err);
                    }
                }

                if chunk_written >= chunk.len() {
                    break 'chunk_loop;
                }
            }
        }
        Ok(total_written)
    }

    pub fn recv_loop(&self) {
        let mut buf = [0; 4096];
        *self.looping.borrow_mut() = true;
        while *self.looping.borrow() {
            // Wait for an incoming message
            match self.socket.recv_from(&mut buf) {
                Ok((size, peer)) => {
                    self.handle(&buf[0..size], peer);
                },
                Err(e) => {
                    eprintln!("Error receiving data: {:?}", e);
                    continue;
                }
            }
        }
    }
}




