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

use serde_derive::Deserialize;

// Default values for control block configurations.
/// Default message handler starting port
pub const DEFAULT_HANDLER_START: u16 = 13100;
/// Default message handler ending port
pub const DEFAULT_HANDLER_END: u16 = 13149;
/// Default message handler timeout
pub const DEFAULT_TIMEOUT: u64 = 1500;

/// A struct that holds useful configuration options to use in a `comms-service` implementation.
/// Created by parsing a configuration file in the `toml` file format.
#[derive(Clone, Debug, Deserialize)]
pub struct CommsConfig {
    /// Starting port used to define a range of ports that are used in the message handlers
    /// that handle messages received from the ground. Default: 13100
    pub handler_port_min: Option<u16>,
    /// Ending port used to define a range of ports that are used in the message handlers
    /// that handle messages received from the ground. Default: 13149
    pub handler_port_max: Option<u16>,
    /// Optional list of ports used by downlink endpoints that send messages to the ground.
    /// Each port in the list will be used by one downlink endpoint.
    pub downlink_ports: Option<Vec<u16>>,
    /// Timeout for the completion of GraphQL operations within message handlers (in milliseconds).
    /// Default: 1500
    pub timeout: Option<u64>,
    /// Required. IP address of the ground gateway.
    pub ground_ip: String,
    /// Required if downlink_ports is not `None`. Specifies the port to which the ground gateway is bound.
    pub ground_port: Option<u16>,
    /// Required. Satellite's IP address.
    pub satellite_ip: String,
}

impl CommsConfig {
    /// Builds a new configuration for a specific `comms-service`.
    /// Configuration parameters are read from the service's `config.toml` file.
    pub fn new(service_config: kubos_system::Config) -> Self {
        let config = service_config
            .get("comms")
            .and_then(|raw| raw.try_into().unwrap());
            
        let config: CommsConfig = config.unwrap();
        
        if config.downlink_ports.is_some() {
            assert!(config.ground_port.is_some(), "Config ground_port parameter is required when downlink_ports is used");
        }
        
        config
    }
}
