extern crate file_protocol;
extern crate file_service_rust;
extern crate kubos_system;
#[macro_use]
extern crate log;
extern crate simplelog;

use file_service_rust::*;
use kubos_system::Config as ServiceConfig;
use simplelog::*;
use std::thread;

use file_protocol::{CborProtocol, FileProtocol, Message};

fn main() {
    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Info, Config::default()).unwrap(),
    ]).unwrap();

    let config = ServiceConfig::new("file-transfer-service");

    info!("Starting file transfer service");

    match recv_loop(config) {
        Ok(()) => warn!("Service listener loop exited successfully?"),
        Err(err) => error!("Service listener exited early: {}", err),
    }
}
