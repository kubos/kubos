#![deny(missing_docs)]
#![deny(warnings)]

//!
//! Hardware service to allow for ethernet debugging. This service starts up a communication
//! service to allow communication over an ethernet cable to the satellite.
//!
//! Telemetry queries will be added as desired for testing.
//!

extern crate comms_service;
extern crate failure;

use comms::*;
use comms_service::*;
use failure::Error;
use std::net::{Ipv4Addr, UdpSocket};
use std::str::FromStr;
use std::sync::{Arc, Mutex};

mod comms;

// Path to configuration file.
const CONFIG_PATH: &'static str = "comms.toml";
// Read port for the socket used in the 'read' function.
const READ_PORT: u16 = 13000;
// Write port for the socket used in the 'write' function.
const WRITE_PORT: u16 = 13001;

// Return type for the ethernet service.
type EthernetServiceResult<T> = Result<T, Error>;

fn main() -> EthernetServiceResult<()> {
    // Read configuration from config file.
    let config = CommsConfig::new("ethernet-service", CONFIG_PATH.to_string());

    // Create socket to mock reading from a radio.
    let read_conn = Arc::new(UdpSocket::bind((config.satellite_ip.as_str(), READ_PORT))?);

    // Create socket to mock writing to a radio.
    let write_conn = Arc::new(UdpSocket::bind((config.satellite_ip.as_str(), WRITE_PORT))?);

    // Control block to configure communication service.
    let controls = CommsControlBlock {
        read: Some(Arc::new(read)),
        write: vec![Arc::new(write)],
        read_conn,
        write_conn,
        handler_port_min: config.handler_port_min,
        handler_port_max: config.handler_port_max,
        timeout: config.timeout,
        ground_ip: Ipv4Addr::from_str(&config.ground_ip)?,
        satellite_ip: Ipv4Addr::from_str(&config.satellite_ip)?,
        downlink_ports: config.downlink_ports,
        ground_port: config.ground_port,
    };

    // Initialize new `CommsTelemetry` object.
    let telem = Arc::new(Mutex::new(CommsTelemetry::default()));

    // Start communication service.
    CommsService::start(controls, telem)?;

    // We will eventually start the GraphQL service here.
    loop {}
}
