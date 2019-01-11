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

// #![deny(missing_docs)]
#![deny(warnings)]

//!
//! Hardware service to allow for ethernet debugging. This service starts up a communication
//! service to allow communication over an ethernet cable to the satellite.
//!
//! Telemetry queries will be added as desired for testing.
//!

extern crate comms_service;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate log;
extern crate kubos_system;
extern crate rust_uart;
extern crate serial;
extern crate simplelog;
extern crate syslog;

use comms::*;
use comms_service::*;
use failure::Error;
use simplelog::*;
use std::sync::{Arc, Mutex};
//use syslog::Facility;

mod comms;
mod kiss;

// Path to configuration file.
const CONFIG_PATH: &'static str = "comms.toml";

// Return type for the ethernet service.
type SerialServiceResult<T> = Result<T, Error>;

fn main() -> SerialServiceResult<()> {
    // Setup new system logging for serial service.
    // syslog::init(
    //     Facility::LOG_DAEMON,
    //     log::LevelFilter::Debug,
    //     Some("serial-comms-service"),
    // )
    //     .unwrap();

    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Info, Config::default()).unwrap()
    ])
    .unwrap();

    let bus = "/dev/ttyUSB0";

    let _service_config = kubos_system::Config::new("serial-comms-service");

    // Read configuration from config file.
    let comms_config = CommsConfig::new("serial-comms-service", CONFIG_PATH.to_string());

    // Open serial port
    let serial_comms = Arc::new(Mutex::new(SerialComms::new(bus)));

    info!("comms config {:?}", comms_config);

    // Control block to configure communication service.
    let controls = CommsControlBlock::new(
        Some(Arc::new(read_ser)),
        vec![Arc::new(write_ser)],
        serial_comms.clone(),
        serial_comms,
        comms_config,
    );

    // Initialize new `CommsTelemetry` object.
    let telem = Arc::new(Mutex::new(CommsTelemetry::default()));

    info!("Serial Communications Service starting on {}", bus);
    // Start communication service.
    CommsService::start(controls, telem)?;

    // We will eventually start the GraphQL service here.
    loop {}
}
