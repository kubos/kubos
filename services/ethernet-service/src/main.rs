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

// #![deny(missing_docs)]
// #![deny(warnings)]

//!
//! Local comms service for testing against "flight commms".
//! Creates a single listening UDP interface
//! which accepts SpacePackets and sends them along to the service.
//!
//! Telemetry queries will be added as desired for testing.
//!

#[macro_use]
extern crate failure;

#[macro_use]
extern crate log;

use crate::comms::*;
use comms_service::*;
use failure::Error;
use std::net::UdpSocket;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
// use syslog::Facility;

mod comms;

// Return type for the ethernet service.
type EthernetCommsServiceResult<T> = Result<T, Error>;

// Initialize logging for the service
// All messages will be routed to syslog and echoed to the console
fn log_init() -> EthernetCommsServiceResult<()> {
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
                "ethernet-comms-service",
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

fn main() -> EthernetCommsServiceResult<()> {
    // Setup new system logging for ethernet service.
    log_init().unwrap();

    // Get the main service configuration from the system's config.toml file
    let service_config = kubos_system::Config::new("ethernet-comms-service");

    let gateway_ip = service_config
        .get("gateway_ip")
        .expect("No 'gateway_ip' parameter in config")
        .as_str()
        .unwrap()
        .to_owned();

    let gateway_port = service_config
        .get("gateway_port")
        .expect("No 'gateway_port' parameter in config")
        .as_integer()
        .unwrap() as u16;

    // Pull out our communication settings
    let config = CommsConfig::new(service_config)?;

    let ground_ip = "127.0.0.1";
    let ground_port = config
        .ground_port
        .expect("No 'ground-port' parameter in config");

    // // Create socket to mock reading from a radio.
    // let read_conn = Arc::new(UdpSocket::bind((ground_ip.as_str(), ground_port))?);

    // // Create socket to mock writing to a radio.
    // let write_conn = Arc::new(UdpSocket::bind((ground_ip.as_str(), 0))?);

    let socket = UdpSocket::bind((ground_ip, ground_port))?;

    let conn = Arc::new(Mutex::new(LocalComms {
        socket,
        gateway_ip,
        gateway_port
    }));

    // Control block to configure communication service.
    let controls = CommsControlBlock::new(
        Some(Arc::new(read)),
        vec![Arc::new(write)],
        conn.clone(),
        conn,
        config,
    )?;

    // Initialize new `CommsTelemetry` object.
    let telem = Arc::new(Mutex::new(CommsTelemetry::default()));

    // Start communication service.
    CommsService::start(controls, &telem)?;

    // We will eventually start the GraphQL service here.
    loop {
        thread::sleep(Duration::from_millis(1))
    }
}
