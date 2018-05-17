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
use transports::*;

use std::net::{SocketAddr, UdpSocket};

fn main() {
    let radio_transport = nsl_serial::Transport::new();
    let pressure = 0;

    loop {
        print!(".");
        io::stdout().flush().unwrap();

        match radio_transport.read() {
            Ok(data) => match data {
                Some(packet) => {
                    // use udp packet here
                    let mut socket =
                        UdpSocket::bind(format!("127.0.0.1:{}", packet.source)).unwrap();

                    socket.set_read_timeout(Some(::std::time::Duration::from_millis(100)));
                    socket.set_write_timeout(Some(::std::time::Duration::from_millis(100)));

                    let dest = SocketAddr::from(([127, 0, 0, 1], packet.dest));

                    socket.send_to(&packet.data, &dest);

                    thread::sleep(Duration::from_millis(100));

                    let mut buf = vec![0u8; 1024];
                    if let Ok((amt, src)) = socket.recv_from(&mut buf) {
                        let pressure = pressure + 1;
                        if pressure == 2 {
                            print!("pause");
                            socket.send_to(&vec![0x01], src);
                        }
                        println!("Received from {:?}\n{:?}", src, &buf[0..amt]);
                        println!(
                            "Sending over radio {:?}",
                            radio_transport.write(udp::UdpData {
                                source: packet.dest,
                                dest: packet.source,
                                data: buf[0..amt].to_vec(),
                                checksum: false,
                            })
                        );
                        let pressure = pressure - 1;
                        if pressure == 1 {
                            print!("resume");
                            socket.send_to(&vec![0x02], src);
                        }
                    }
                }
                None => (),
            },
            Err(e) => println!("Read err {:?}", e),
        };
        thread::sleep(Duration::from_millis(500));
    }
}
