extern crate nsl_duplex_d2;

#[macro_use]
extern crate log;
extern crate simplelog;

use simplelog::*;

use std::thread;
use std::time::Duration;

mod codecs;
mod transports;

use std::io::{self, Write};

fn main() {
    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Warn, Config::default()).unwrap(),
        TermLogger::new(LevelFilter::Info, Config::default()).unwrap(),
    ]).unwrap();

    info!("Starting communications service");

    let radio_transport = transports::nsl_serial::Transport::new();
    let mut udp_transport = transports::udp::Transport::new();

    loop {
        io::stdout().flush().unwrap();

        match radio_transport.read() {
            Ok(data) => match data {
                Some(packet) => {
                    let dest = packet.dest;
                    if let Err(e) = udp_transport.write(packet, dest) {
                        warn!("udp_transport failed write {:?}", e);
                    }

                    if let Ok(res) = udp_transport.read(dest) {
                        match res {
                            Some(packet) => if let Err(e) = radio_transport.write(packet) {
                                warn!("radio_transport failed write {:?}", e);
                            },
                            None => (),
                        }
                    } else {
                        warn!("udp_transport failed read");
                    }
                }
                None => (),
            },
            Err(e) => warn!("radio_transport failed read {:?}", e),
        };
        thread::sleep(Duration::from_millis(500));
    }
}
