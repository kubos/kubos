use codecs::*;
use std::net::{SocketAddr, UdpSocket};
use std::time::Duration;

pub struct Transport {
    pub socket: Option<UdpSocket>,
}

impl Transport {
    pub fn new() -> Self {
        Self { socket: None }
    }

    pub fn read(&mut self) -> Result<Option<udp::UdpData>, String> {
        //println!("-udp_read-");
        match self.socket {
            Some(ref mut socket) => {
                println!("-settimeout-");
                if let Err(e) = socket.set_read_timeout(Some(Duration::from_millis(100))) {
                    return Err(format!("Failed to set udp read timeout {:?}", e));
                }
                let mut buffer = vec![0u8; 4069];
                println!("-udp-recv-");
                if let Ok((amount, source)) = socket.recv_from(&mut buffer) {
                    return Ok(Some(udp::UdpData {
                        source: 7000,
                        dest: 7000,
                        data: buffer[0..amount].to_vec(),
                        checksum: false,
                    }));
                }
                return Ok(None);
            }
            None => Ok(None),
        }
    }

    pub fn write(&mut self, data: udp::UdpData) -> Result<(), String> {
        //        println!("-udp_write-");
        if let None = self.socket {
            let mut socket = match UdpSocket::bind(format!("127.0.0.1:{}", data.source)) {
                Ok(s) => s,
                Err(e) => return Err(format!("Failed to bind UdpSocket {:?}", e)),
            };

            if let Err(e) = socket.set_write_timeout(Some(Duration::from_millis(100))) {
                return Err(format!("Failed to set udp write timeout {:?}", e));
            }

            self.socket = Some(socket);
        }

        let dest = SocketAddr::from(([127, 0, 0, 1], data.dest));

        if let Some(ref socket) = self.socket {
            if let Err(e) = socket.send_to(&data.data, &dest) {
                return Err(format!("Failed to send udp packet {:?}", e));
            }
        }

        Ok(())
    }
}
