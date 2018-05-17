use codecs::*;
use std::net::{SocketAddr, UdpSocket};
use std::time::Duration;

use std::collections::HashMap;

pub struct Transport {
    pub sockets: HashMap<u16, UdpSocket>,
}

impl Transport {
    pub fn new() -> Self {
        Self {
            sockets: HashMap::new(),
        }
    }

    // Attempts to grab socket for destination port from socket map
    // Creates a new socket for port if one doesn't exist already
    fn init_socket(&mut self, dest_port: u16) -> Result<(), String> {
        let socket = match UdpSocket::bind("127.0.0.1:0") {
            Ok(s) => s,
            Err(e) => return Err(format!("Failed to bind UdpSocket {:?}", e)),
        };

        if let Err(e) = socket.set_write_timeout(Some(Duration::from_millis(100))) {
            return Err(format!("Failed to set udp write timeout {:?}", e));
        }

        if let Err(e) = socket.set_read_timeout(Some(Duration::from_millis(100))) {
            return Err(format!("Failed to set udp read timeout {:?}", e));
        }

        self.sockets.insert(dest_port, socket);
        Ok(())
    }

    fn grab_socket(&mut self, dest_port: u16) -> Result<&UdpSocket, String> {
        if !self.sockets.contains_key(&dest_port) {
            self.init_socket(dest_port)?;
        };

        match self.sockets.get(&dest_port) {
            Some(ref socket) => Ok(socket),
            None => Err(format!("Failed to locate socket {}", dest_port)),
        }
    }

    pub fn read(&mut self, dest_port: u16) -> Result<Option<udp::UdpData>, String> {
        let socket = self.grab_socket(dest_port)?;

        let mut buffer = vec![0u8; 4069];
        println!("-udp-recv-");
        if let Ok((amount, _source)) = socket.recv_from(&mut buffer) {
            return Ok(Some(udp::UdpData {
                source: 7000,
                dest: 7000,
                data: buffer[0..amount].to_vec(),
                checksum: false,
            }));
        }
        return Ok(None);
    }

    pub fn write(&mut self, data: udp::UdpData, dest_port: u16) -> Result<(), String> {
        let socket = self.grab_socket(dest_port)?;

        let dest = SocketAddr::from(([127, 0, 0, 1], data.dest));

        if let Err(e) = socket.send_to(&data.data, &dest) {
            return Err(format!("Failed to send udp packet {:?}", e));
        }

        Ok(())
    }
}
