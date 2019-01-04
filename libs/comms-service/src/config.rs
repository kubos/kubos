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

//! TOML parser for the `comms-service`. This module parses a `toml` file and returns a
//! struct containing configuration information for a `comms-service`.

use failure::Error;
use std::fs::File;
use std::io::Read;
use toml::*;

// Default values for control block configurations.
const DEFAULT_HANDLER_START: u16 = 13100;
const DEFAULT_HANDLER_END: u16 = 13149;
const DEFAULT_TIMEOUT: u64 = 1500;
static DEFAULT_GROUND_IP: &str = "192.168.8.1";
static DEFAULT_SATELLITE_IP: &str = "192.168.8.2";

/// A struct that holds useful configuration options to use in a `comms-service` implementation.
/// Created by parsing a configuration file in the `toml` file format.
#[derive(Debug)]
pub struct CommsConfig {
    /// Starting port used to define a range of ports that are used in the message handlers
    /// that handle messages received from the ground.
    pub handler_port_min: u16,
    /// Ending port used to define a range of ports that are used in the message handlers
    /// that handle messages received from the ground.
    pub handler_port_max: u16,
    /// Optional list of ports used by downlink endpoints that send messages to the ground.
    /// Each port in the list will be used by one downlink endpoint.
    pub downlink_ports: Option<Vec<u16>>,
    /// Timeout for the completion of GraphQL operations within message handlers (in milliseconds).
    pub timeout: u64,
    /// IP address of the ground gateway.
    pub ground_ip: String,
    /// Specifies the port to which the ground gateway is bound.
    pub ground_port: Option<u16>,
    /// Satellite's IP address.
    pub satellite_ip: String,
}

// Implementation of `Default` trait for `CommsConfig`.
impl Default for CommsConfig {
    fn default() -> Self {
        CommsConfig {
            handler_port_min: DEFAULT_HANDLER_START,
            handler_port_max: DEFAULT_HANDLER_END,
            downlink_ports: None,
            ground_port: None,
            timeout: DEFAULT_TIMEOUT,
            ground_ip: DEFAULT_GROUND_IP.to_string(),
            satellite_ip: DEFAULT_SATELLITE_IP.to_string(),
        }
    }
}

impl CommsConfig {
    /// Builds a new configuration for a specific `comms-service`.
    pub fn new(name: &str, path: String) -> Self {
        Self::parse_file(name, path).unwrap_or(CommsConfig::default())
    }

    // Function used to parse configuration files.
    fn parse_file(name: &str, path: String) -> ConfigResult<CommsConfig> {
        // Read file to string.
        let mut contents = String::new();
        let mut file = File::open(path)?;
        file.read_to_string(&mut contents)?;

        // Read contents and place into TOML table.
        let table: Value = toml::from_str(&contents)?;
        let mut config = CommsConfig::default();

        // Go through TOML table and replace default if provided.
        if let Some(service) = table.get(name) {
            // Get the integer values from TOML file.
            if let Ok(num) = get_number_u16("handler-port-min", &service) {
                config.handler_port_min = num;
            }
            if let Ok(num) = get_number_u16("handler-port-max", &service) {
                config.handler_port_max = num;
            }
            if let Ok(num) = get_number_u64("timeout", &service) {
                config.timeout = num;
            }
            if let Some(num) = get_number_u16("ground-port", &service).ok() {
                config.ground_port = Some(num);
            }

            // Get the `downlink_ports` TOML array.
            if let Some(array) = service.get("downlink-ports") {
                if let Some(vector) = array.as_array() {
                    let mut ports = vec![];
                    for item in vector {
                        if let Some(val) = item.as_integer() {
                            ports.push(val as u16);
                        }
                    }
                    config.downlink_ports = Some(ports);
                }
            }

            // Pull strings from the TOML file.
            if let Some(ground_ip) = service.get("ground-ip") {
                if let Some(val) = ground_ip.as_str() {
                    config.ground_ip = val.to_string();
                }
            }
            if let Some(satellite_ip) = service.get("satellite-ip") {
                if let Some(val) = satellite_ip.as_str() {
                    config.satellite_ip = val.to_string()
                }
            }
        }
        Ok(config)
    }
}

// Helper function to get a `u16` from a TOML table.
fn get_number_u16(key: &str, table: &Value) -> ConfigResult<u16> {
    if let Some(val) = table.get(key) {
        if let Some(num) = val.as_integer() {
            return Ok(num as u16);
        }
    }
    Err(ConfigError::IntegerParsing.into())
}

// Helper function to get a `u64` from a TOML table.
fn get_number_u64(key: &str, table: &Value) -> ConfigResult<u64> {
    if let Some(val) = table.get(key) {
        if let Some(num) = val.as_integer() {
            return Ok(num as u64);
        }
    }
    Err(ConfigError::IntegerParsing.into())
}

/// Result returned when creating a `CommsConfig` struct.
pub type ConfigResult<T> = Result<T, Error>;

/// This enum defines all errors that can occur during configuration parsing.
#[derive(Fail, Debug, PartialEq)]
pub enum ConfigError {
    /// Failure occured during the parsing of integers from the TOML table.
    #[fail(display = "Unable to parse integers from the configuration file")]
    IntegerParsing,
}
