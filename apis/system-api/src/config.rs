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
use getopts::Options;
use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use toml;
use toml::Value;

/// The default conifguration file path
pub static DEFAULT_PATH: &str = "/home/system/etc/config.toml";
/// The default IP address for service bindings
pub static DEFAULT_IP: &str = "127.0.0.1";
/// The default port for service bindings
pub const DEFAULT_PORT: u16 = 8080;

#[derive(Debug, Deserialize)]
pub struct Address {
    ip: Option<String>,
    port: Option<u16>,
}

impl Default for Address {
    fn default() -> Self {
        Address {
            ip: Some(DEFAULT_IP.to_string()),
            port: Some(DEFAULT_PORT),
        }
    }
}

impl Address {
    pub fn ip(&self) -> &str {
        match self.ip.as_ref() {
            Some(ref ip) => ip,
            None => DEFAULT_IP
        }
    }

    pub fn port(&self) -> u16 {
        self.port.unwrap_or(DEFAULT_PORT)
    }
}

/// KubOS config used by either Apps or Services. KubOS config files use the TOML format, and can
/// may contain multiple named Categories. Typically each category corresponds to an App or Service
/// name. This allows one config file to store configuration for multiple Apps / Services at a
/// time.
///
/// Example KubOS config files for a Service called `my-service` with an IP/port binding
/// ```toml
/// [my-service]
/// my-property = "value"
///
/// [my-service.addr]
/// ip = 0.0.0.0
/// port = 8181
/// ```
///
/// When `addr`, `addr.ip`, or `addr.port` are not provided in the config file, the default IP 
/// `"127.0.0.1"` and default port `8080` are used instead.
#[derive(Debug)]
pub struct Config {
    addr: Address,
    raw: Value,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            addr: Address::default(),
            raw: Value::String("".to_string()),
        }
    }
}

impl Config {
    /// Creates and parses configuration data from the system configuration
    /// file or the path passed as the '-c' or '--config' option to this
    /// executable.
    ///
    /// # Arguments
    /// `name` - Category name used as a key in the config file
    pub fn new(name: &str) -> Self {
        Self::new_from_path(name, get_config_path())
    }

    /// Creates and parses configuration data from the passed in configuration
    /// path.
    /// # Arguments
    /// `name` - Category name used as a key in the config file
    /// `path` - Path to configuration file
    pub fn new_from_path(name: &str, path: String) -> Self {
        parse_config_file(name, path).unwrap_or(Config::default())
    }

    /// Creates and parses configuration data from the passed in configuration
    /// string.
    /// # Arguments
    /// `name` - Category name used as a key in the config
    /// `config` - Config data as a string
    pub fn new_from_str(name: &str, config: &str) -> Self {
        parse_config_str(name, config).unwrap_or(Config::default())
    }

    /// Returns the configured hosturl string in the following
    /// format (using IPv4 addresses) - 0.0.0.0:0000
    pub fn hosturl(&self) -> String {
        format!("{}:{}", self.addr.ip(), self.addr.port())
    }

    /// Returns the category's configuration information
    /// in the `toml::Value` format.
    /// This will contain the ip/port if provided, along with any other
    /// configuration information found in the config file.
    ///
    /// ### Examples
    ///
    /// ```rust,no_run
    /// use kubos_system::Config;
    ///
    /// let config = Config::new("example-service");
    /// let raw = config.raw();
    /// let bus = raw["bus"].as_str();
    /// ```
    pub fn raw(&self) -> toml::Value {
        self.raw.clone()
    }

    /// Performs a get on the raw config data
    ///
    /// # Arguments
    /// `key` - Key of value to get from config
    pub fn get(&self, key: &str) -> Option<toml::Value> {
        match self.raw.get(key) {
            Some(v) => Some(v.clone()),
            None => None,
        }
    }
}

fn get_config_path() -> String {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optopt("c", "config", "Path to config file", "CONFIG");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(_) => {
            // suppress errors so applications using Config can have their own Options
            return DEFAULT_PATH.to_string();
        }
    };
    match matches.opt_str("c") {
        Some(s) => s,
        None => DEFAULT_PATH.to_string(),
    }
}

fn get_file_data(path: String) -> Result<String, io::Error> {
    let mut contents = String::new();
    let mut file = File::open(path)?;
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn parse_config_file(name: &str, path: String) -> Result<Config, toml::de::Error> {
    let contents = get_file_data(path).unwrap_or("".to_string());
    parse_config_str(name, &contents)
}

fn parse_config_str(name: &str, contents: &str) -> Result<Config, toml::de::Error> {
    let data: Value = toml::from_str(&contents)?;
    let mut config = Config::default();

    if let Some(data) = data.get(name) {
        if let Some(address) = data.get("addr") {
            config.addr = address.clone().try_into()?;
        }
        config.raw = data.clone();
    }

    Ok(config)
}
