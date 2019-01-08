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
// Example radio hardware service
//
// This service is an example implementation of the communications service framework.
// It initializes logging and the UART port which will be used for the connection to the "radio".
// The other end of the "radio" is provided by the `uart-comms-client` program in the parent folder.
//
// The service initializes logging and the serial port connection, then kicks of the communications
// logic, and finally starts up the standard GraphQL interface that all services provide.

#[macro_use]
extern crate failure;
#[macro_use]
extern crate log;
extern crate log4rs;
extern crate log4rs_syslog;

// Return type for this service.
type ServiceResult<T> = Result<T, Error>;

mod comms;

use comms_service::*;
use failure::Error;
use std::sync::{Arc, Mutex};

const BUS: &str = "/dev/ttyS2";
const CONFIG_PATH: &str = "comms.toml";

// Initialize logging for the service
// All messages will be routed to syslog and echoed to the console
fn log_init() -> ServiceResult<()> {
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
                "uart-comms-service",
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

fn main() -> ServiceResult<()> {
    // Initialize logging for the program
    log_init()?;

    // Initialize the serial port
    let conn = comms::serial_init(BUS)?;

    // Set up the comms configuration
    // In this instance, reading and writing are done over the same connection,
    // so we'll just clone the UART port connection
    let read_conn = conn.clone();
    let write_conn = conn;

    let config = CommsConfig::new("uart-comms-service", CONFIG_PATH.to_string());
    let control = CommsControlBlock::new(
        Some(Arc::new(comms::read)),
        vec![Arc::new(comms::write)],
        read_conn,
        write_conn,
        config,
    );

    /*
    let config = CommsControlBlock {
        read: Some(Arc::new(read)),
        write: vec![Arc::new(write)],
        read_conn,
        write_conn,
        handler_port_min: 9000,
        handler_port_max: 9100,
        timeout: 500,
        ground_ip: Ipv4Addr::new(192, 168, 8, 40),
        satellite_ip: Ipv4Addr::new(0, 0, 0, 0),
        downlink_ports: None,
        ground_port: Some(9001)
    };
    */

    // Start the comms service thread
    CommsService::start(control, Arc::new(Mutex::new(CommsTelemetry::default())))?;

    // TODO: Start the GraphQL service
    loop {}
}
