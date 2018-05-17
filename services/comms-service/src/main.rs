#[macro_use]
extern crate nom;

extern crate nsl_duplex_d2;

use nsl_duplex_d2::DuplexD2;
use nsl_duplex_d2::File;
use nsl_duplex_d2::serial_connection;

use std::thread;
use std::time::Duration;

mod codecs;
mod transports;

use codecs::kiss;
use codecs::udp;
use std::io::{self, Write};

use std::net::{SocketAddr, UdpSocket};

fn main() {
    let radio_transport = transports::nsl_serial::Transport::new();
    let mut udp_transport = transports::udp::Transport::new();
    let pressure = 0;

    loop {
        print!(".");
        io::stdout().flush().unwrap();

        match radio_transport.read() {
            Ok(data) => match data {
                Some(packet) => {
                    if let Err(e) = udp_transport.write(packet) {
                        println!("udp_transport failed write {:?}", e);
                    }

                    if let Ok(res) = udp_transport.read() {
                        match res {
                            Some(packet) => if let Err(e) = radio_transport.write(packet) {
                                println!("radio_transport failed write {:?}", e)
                            },
                            None => (),
                        }
                    } else {
                        println!("udp_transport failed read");
                    }
                }
                None => (),
            },
            Err(e) => println!("Read err {:?}", e),
        };
        thread::sleep(Duration::from_millis(500));
    }
}
