extern crate nsl_duplex_d2;

#[macro_use]
extern crate log;
extern crate simplelog;

use simplelog::*;

use std::thread;
use std::time::Duration;

mod codecs;
mod transports;

use transports::Transport;

fn transport_comms(mut t1: Box<Transport>, mut t2: Box<Transport>) -> Result<(), String> {
    loop {
        match t1.read() {
            Ok(data) => match data {
                Some(packet) => {
                    info!("t1 --> {:?} --> t2", packet);
                    if let Err(e) = t2.write(packet) {
                        warn!("transport2 failed write {:?}", e);
                    }
                }
                None => (),
            },
            Err(e) => warn!("transport1 failed read {:?}", e),
        };

        match t2.read() {
            Ok(data) => match data {
                Some(packet) => {
                    info!("t2 --> {:?} --> t1", packet);
                    if let Err(e) = t1.write(packet) {
                        warn!("transport1 failed write {:?}", e);
                    }
                }
                None => (),
            },
            Err(e) => warn!("transport2 failed read {:?}", e),
        };
        thread::sleep(Duration::from_millis(100));
    }
}

fn main() {
    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Warn, Config::default()).unwrap(),
        TermLogger::new(LevelFilter::Info, Config::default()).unwrap(),
    ]).unwrap();

    info!("Starting communications service");

    let radio_transport = Box::new(transports::nsl_serial::NslSerial::new());
    //let echo_transport = Box::new(transports::echo::Echo::new());
    let mut udp_transport = Box::new(transports::udp::Udp::new());

    // if let Err(e) = udp_transport.expose_ports(&vec![6000, 7000]) {
    //     warn!("failed to exposed udp ports {:?}", e);
    // }

    if let Err(e) = transport_comms(radio_transport, udp_transport) {
        warn!("transport comms err {:?}", e);
    }

    // if let Err(e) = transport_comms(radio_transport, udp_transport) {
    //     warn!("transport comms err {:?}", e);
    // }
}
