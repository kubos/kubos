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
// Example client program for interacting with a Kubos Service using UDP
//

extern crate getopts;
extern crate kubos_service;

use std::cell::RefCell;
use std::env;
use std::net::SocketAddr;
use std::str;

use kubos_service::{FrameHandler, KissUdpSocket};
use getopts::Options;

struct Handler<'a> {
    socket: RefCell<Option<KissUdpSocket<'a>>>,
}

impl<'a> FrameHandler for Handler<'a> {
    fn handle_frame(&self, socket: &KissUdpSocket, frame: &[u8], src_addr: SocketAddr) {
        println!("{}", str::from_utf8(frame).unwrap());
        let s = self.socket.borrow();
        match *s {
            Some(ref s) => { s.looping.replace(false); },
            None => {},
        }
    }
}

pub fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} [dest] [graphql-query]", args[0]);
        return;
    }

    let mut handler = Handler {
        socket: RefCell::new(None)
    };

    handler.socket.replace(
        KissUdpSocket::bind("0.0.0.0:0".parse::<SocketAddr>().unwrap(), &handler)
    );

    let s = handler.socket.borrow();
    match *s {
        Some(ref sock) => {
            let dest = args[1].parse::<SocketAddr>().unwrap();
            let query = &args[2];
            sock.send_to(query.as_bytes(), dest);
            sock.recv_loop();
        },
        None => {}
    }
}
