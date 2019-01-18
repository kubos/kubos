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
// Contributed by: William Greer (wgreer184@gmail.com) and Sam Justice (sam.justice1@gmail.com)
//

#![deny(missing_docs)]
#![deny(warnings)]

//!
//! Hardware service to allow for ethernet debugging. This service starts up a communication
//! service to allow communication over an ethernet cable to the satellite.
//!
//! Telemetry queries will be added as desired for testing.
//!

use crate::comms::*;
use comms_service::*;
use failure::Error;
use std::net::UdpSocket;
use std::sync::{Arc, Mutex};
use syslog::Facility;

mod comms;

// Read port for the socket used in the 'read' function.
const READ_PORT: u16 = 13000;
// Write port for the socket used in the 'write' function.
const WRITE_PORT: u16 = 13001;

// Return type for the ethernet service.
type EthernetServiceResult<T> = Result<T, Error>;

fn main() -> EthernetServiceResult<()> {
    // Setup new system logging for ethernet service.
    syslog::init(
        Facility::LOG_DAEMON,
        log::LevelFilter::Debug,
        Some("ethernet-service"),
    )
    .unwrap();

    // Get the main service configuration from the system's config.toml file
    let service_config = kubos_system::Config::new("ethernet-service");
    
    // Pull out our communication settings
    let config = CommsConfig::new(service_config);
    
    let satellite_ip = config.satellite_ip.clone().unwrap();
    
    // Create socket to mock reading from a radio.
    let read_conn = Arc::new(UdpSocket::bind((satellite_ip.as_str(), READ_PORT))?);

    // Create socket to mock writing to a radio.
    let write_conn = Arc::new(UdpSocket::bind((satellite_ip.as_str(), WRITE_PORT))?);

    // Control block to configure communication service.
    let controls = CommsControlBlock::new(
        Some(Arc::new(read)),
        vec![Arc::new(write)],
        read_conn,
        write_conn,
        config,
    );

    // Initialize new `CommsTelemetry` object.
    let telem = Arc::new(Mutex::new(CommsTelemetry::default()));

    // Start communication service.
    CommsService::start(controls, telem)?;

    // We will eventually start the GraphQL service here.
    loop {}
}
