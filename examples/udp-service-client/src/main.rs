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

#[macro_use]
extern crate nix;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate toml;

use std::net::{SocketAddr, UdpSocket};
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::os::unix::io::AsRawFd;
use std::time::Duration;

const FIONREAD: u16 = 0x541B;
ioctl!(bad read available with FIONREAD; usize);

#[derive(Debug, Deserialize)]
pub struct Address {
    ip: String,
    port: u16,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    host: Address,
    service: Address,
    query_file: String,
}

fn configure_client() -> (SocketAddr, SocketAddr, String) {
    let args: Vec<String> = env::args().collect();

    let config: Config = if args.len() == 2 {
        let mut raw = String::new();
        let filename = &args[1];
        match File::open(filename)
            .map(|mut f| f.read_to_string(&mut raw))
            .map(|_| toml::from_str(&raw)) {
            Ok(toml) => toml.unwrap(),
            _ => {
                panic!("Couldn't open config file");
            }
        }
    } else {
        panic!("No config file");
    };

    let listen_addr = format!("{}:{}", config.host.ip, config.host.port)
        .parse::<SocketAddr>()
        .unwrap();

    let send_addr = format!("{}:{}", config.service.ip, config.service.port)
        .parse::<SocketAddr>()
        .unwrap();

    let query: String = {
        let mut raw = String::new();
        match File::open(config.query_file).map(|mut f| f.read_to_string(&mut raw)) {
            Ok(_) => raw,
            _ => {
                println!("Using default query");
                "{ping}".to_owned()
            }
        }
    };

    (listen_addr, send_addr, query)
}

fn main() {

    // Get the host IP, remote IP, and query string from the configuration file
    let (client_addr, service_addr, query) = configure_client();

    // Get a socket for the host IP
    let socket = match UdpSocket::bind(client_addr) {
        Ok(sock) => {
            println!("Bound socket to {}", client_addr);
            sock
        }
        Err(err) => {
            println!("Could not bind: {}", err);
            return;
        }
    };

    // Send our query to the requested service IP
    let result = socket.send_to(&query.as_bytes(), &service_addr);
    match result {
        Ok(amt) => println!("Sent {} bytes", amt),
        Err(err) => {
            println!("Write error: {}", err);
            return;
        }
    }

    // Wait for a response, but don't actually read it yet.
    // If we don't get a reply within one second, the service probably
    // isn't actually running.
    socket.set_read_timeout(Some(Duration::new(1, 0))).unwrap();
    let mut buf: [u8; 1] = [0];
    let result = socket.peek_from(&mut buf);
    if let Err(err) = result {
        println!("Read error: {}", err);
        return;
    }

    // Get the number of bytes in the response so we know how big to make our buffer
    let mut len: usize = 0;
    unsafe {
        match available(socket.as_raw_fd(), &mut len) {
            Ok(_) => {}
            Err(err) => {
                println!("Failed to read bytes available: {}", err);
                return;
            }
        }
    }

    // Allocate the buffer and read the response
    let mut buf = vec![0u8; len].into_boxed_slice();
    let result = socket.recv_from(&mut buf);
    match result {
        Ok((amt, src)) => {
            println!("Received {} bytes from {}", amt, src);
            let mut v: serde_json::Value = serde_json::from_slice(&buf[0..(amt)]).unwrap();
            // If there's  'msg' field in the returned JSON, then our query went (atleast mostly) successfully.
            // Extract the query response JSON from the field
            match serde_json::from_value::<String>(v["msg"].clone()) {
                Ok(msg) => {
                    v = serde_json::from_str(&msg).unwrap();
                }
                // Otherwise, we'll just print the raw returned JSON
                _ => println!("Errors returned from query"),
            }
            println!("{}", serde_json::to_string_pretty(&v).unwrap());
        }
        Err(err) => {
            println!("Read error: {}", err);
            return;
        }
    }
}
