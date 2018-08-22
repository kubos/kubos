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

extern crate serde_cbor;

use serde_cbor::de;
use std::net::{SocketAddr, UdpSocket};
use std::time::Duration;

pub struct Protocol {
    pub handle: UdpSocket,
}

impl Protocol {
    pub fn new(host_url: String) -> Self {
        Self {
            handle: UdpSocket::bind(host_url.parse::<SocketAddr>().unwrap()).unwrap(),
        }
    }

    pub fn send_message(&self, message: &[u8], host: &str, port: u16) -> Result<(), String> {
        // TODO: If paused, just queue up the message
        let dest: SocketAddr = format!("{}:{}", host, port).parse().unwrap();
        // let mut e = Encoder::from_memory();
        //e.encode(&message).unwrap();
        //let mut payload = e.as_bytes().to_vec();
        let mut payload = vec![];
        payload.extend(message);
        payload.insert(0, 0);

        self.handle.send_to(&payload, &dest).unwrap();
        Ok(())
    }

    pub fn resume(&self) -> Result<(), String> {
        // if !paused {return;}
        // paused = false
        // while !paused && Some(write_queue.next()) {
        //   pull chunks out of the queue and write them
        // }
        unimplemented!();
    }

    // Not actually used by anything in the original Lua?
    pub fn send_pause(&self, host: &str, port: u16) -> Result<(), String> {
        println!("-> pause");
        let dest: SocketAddr = format!("{}:{}", host, port).parse().unwrap();
        let payload = vec![1];
        self.handle.send_to(&payload, &dest).unwrap();
        Ok(())
    }

    // Not actually used by anything in the original Lua?
    pub fn send_resume(&self, host: &str, port: u16) -> Result<(), String> {
        println!("-> resume");
        let dest: SocketAddr = format!("{}:{}", host, port).parse().unwrap();
        let payload = vec![2];
        self.handle.send_to(&payload, &dest).unwrap();
        Ok(())
    }

    pub fn recv_message(&self) -> Result<Option<serde_cbor::Value>, String> {
        let mut buf = [0; 4136];
        let (size, _peer) = self
            .handle
            .recv_from(&mut buf)
            .map_err(|err| format!("Failed to receive a message: {}", err))?;

        println!("Received {} bytes", size);

        self.recv_start(&buf[0..size])
    }

    pub fn recv_message_peer(&self) -> Result<(SocketAddr, Option<serde_cbor::Value>), String> {
        let mut buf = [0; 4136];
        let (size, peer) = self
            .handle
            .recv_from(&mut buf)
            .map_err(|err| format!("Failed to receive a message: {}", err))?;

        println!("Received {} bytes", size);

        let message = self.recv_start(&buf[0..size])?;
        Ok((peer, message))
    }

    pub fn recv_message_timeout(
        &self,
        timeout: Duration,
    ) -> Result<Option<serde_cbor::Value>, Option<String>> {
        // Set the timeout for this particular receive
        self.handle
            .set_read_timeout(Some(timeout))
            .map_err(|err| format!("Failed to set timeout: {}", err))?;

        // Max message size:
        // - 4096 - Max chunk size TODO: Make this configurable
        // -   32 - Hash string
        // -    8 - Chunk number
        let mut buf = [0; 4136];
        let result = self.handle.recv_from(&mut buf);

        // Reset the timeout for future calls
        // TODO: Decide what should happen if this fails...
        let _ = self.handle.set_read_timeout(None);

        let (size, _peer) = match result {
            Ok(data) => data,
            Err(err) => match err.kind() {
                ::std::io::ErrorKind::WouldBlock => return Err(None), // For some reason, UDP recv returns WouldBlock for timeouts
                _ => return Err(Some(format!("Failed to receive a message: {:?}", err))),
            },
        };

        //println!("Received {} bytes", size);

        self.recv_start(&buf[0..size]).map_err(|err| Some(err))
    }

    // Parse the received CBOR message
    pub fn recv_start(&self, data: &[u8]) -> Result<Option<serde_cbor::Value>, String> {
        // TODO: error processing?
        if data.len() == 0 {
            return Ok(None);
        }

        let result: Option<serde_cbor::Value> = match data[0] {
            0 => {
                let message: serde_cbor::Value = de::from_slice(&data[1..])
                    .map_err(|err| format!("Failed to parse data: {:?}", err))?;
                //println!("<- {:?}", message);

                if message.is_array() {
                    Some(message)
                } else {
                    return Err(format!("Failed to parse data: Body not an array"));
                }
            }
            1 => {
                println!("<- pause");
                //TODO: paused = true
                None
            }
            2 => {
                println!("<- resume");
                // TODO: This might need to be a channel message/signal
                self.resume().unwrap();
                None
            }
            x => {
                eprintln!("Ignoring unknown control frame: {}", x);
                None
            }
        };

        Ok(result)
    }
}
