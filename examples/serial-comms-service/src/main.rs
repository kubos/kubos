//
// Copyright (C) 2019 Kubos Corporation
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

#![deny(missing_docs)]
#![deny(warnings)]

//!
//! Hardware service to allow for serial communications. This service starts up a communication
//! service to allow transfer of udp packets over a serial link to the satellite.
//!

extern crate comms_service;
#[macro_use]
extern crate failure;
extern crate kubos_service;
extern crate kubos_system;
#[macro_use]
extern crate juniper;
extern crate rust_uart;
extern crate serial;
#[macro_use]
extern crate log;
extern crate log4rs;
extern crate log4rs_syslog;

mod comms;
mod kiss;
mod model;
mod schema;

use crate::comms::*;
use crate::model::Subsystem;
use crate::schema::{MutationRoot, QueryRoot};
use comms_service::*;
use failure::Error;
use kubos_service::{Config, Service};
use std::sync::{Arc, Mutex};

// Return type for the ethernet service.
type SerialServiceResult<T> = Result<T, Error>;

// Initialize logging for the service
// All messages will be routed to syslog and echoed to the console
fn log_init() -> SerialServiceResult<()> {
    use log4rs::append::console::ConsoleAppender;
    use log4rs::encode::pattern::PatternEncoder;
    use log4rs_syslog::SyslogAppender;
    // Use custom PatternEncoder to avoid duplicate timestamps in logs.
    let syslog_encoder = Box::new(PatternEncoder::new("{m}"));
    // Set up logging which will be routed to syslog for processing
    let syslog = Box::new(
        SyslogAppender::builder()
            .encoder(syslog_encoder)
            .openlog(
                "serial-comms-service",
                log4rs_syslog::LogOption::LOG_PID | log4rs_syslog::LogOption::LOG_CONS,
                log4rs_syslog::Facility::Daemon,
            )
            .build(),
    );

    // Set up logging which will be routed to stdout
    let stdout = Box::new(ConsoleAppender::builder().build());

    // Combine the loggers into one master config
    let config = log4rs::config::Config::builder()
        .appender(log4rs::config::Appender::builder().build("syslog", syslog))
        .appender(log4rs::config::Appender::builder().build("stdout", stdout))
        .build(
            log4rs::config::Root::builder()
                .appender("syslog")
                .appender("stdout")
                // Set the minimum logging level to record
                .build(log::LevelFilter::Debug),
        )?;

    // Start the logger
    log4rs::init_config(config)?;

    Ok(())
}

fn main() -> SerialServiceResult<()> {
    log_init()?;

    let service_config = Config::new("serial-comms-service")?;

    let bus = service_config
        .get("bus")
        .expect("No 'bus' parameter in config.toml")
        .as_str()
        .unwrap()
        .to_owned();

    // Read configuration from config file.
    let comms_config = CommsConfig::new(service_config.clone())?;

    // Open serial port
    let serial_comms = Arc::new(Mutex::new(SerialComms::new(&bus)));

    // Control block to configure communication service.
    let controls = CommsControlBlock::new(
        Some(Arc::new(read_ser)),
        vec![Arc::new(write_ser)],
        serial_comms.clone(),
        serial_comms,
        comms_config,
    )?;

    // Initialize new `CommsTelemetry` object.
    let telem = Arc::new(Mutex::new(CommsTelemetry::default()));

    // Start communication service.
    info!("Serial Communications Service starting on {}", bus);
    CommsService::start::<Arc<Mutex<SerialComms>>, SpacePacket>(controls, &telem.clone())?;

    let subsystem = Subsystem::new(telem);
    Service::new(service_config, subsystem, QueryRoot, MutationRoot).start();

    Ok(())
}
