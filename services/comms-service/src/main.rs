extern crate nsl_duplex_d2;

use std::thread;
use std::time::Duration;

mod codecs;
mod transports;

use std::io::{self, Write};

fn main() {
    let radio_transport = transports::nsl_serial::Transport::new();
    let mut udp_transport = transports::udp::Transport::new();

    loop {
        print!(".");
        io::stdout().flush().unwrap();

        match radio_transport.read() {
            Ok(data) => match data {
                Some(packet) => {
                    let dest = packet.dest;
                    if let Err(e) = udp_transport.write(packet, dest) {
                        println!("udp_transport failed write {:?}", e);
                    }

                    if let Ok(res) = udp_transport.read(dest) {
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
