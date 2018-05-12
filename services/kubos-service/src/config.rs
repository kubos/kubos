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
use toml;
use std::fs::File;
use std::io::prelude::*;
use toml::Value;
use std::io;

static PATH: &str = "/home/system/etc/config.toml";
static IP: &str = "127.0.0.1";
const PORT: u16 = 8080;

#[derive(Debug, Deserialize)]
pub struct Address {
    ip: String,
    port: u16,
}

impl Default for Address {
    fn default() -> Self {
        Address {
            ip: IP.to_string(),
            port: PORT,
        }
    }
}

/// Service configuration structure
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
    /// Creates and parses configuration data. Service name is used
    /// as a key in the config file.
    pub fn new(name: &str) -> Self {
        parse_config(name, get_config_path()).unwrap_or(Config::default())
    }

    /// Returns the configured hosturl string in the following
    /// format (using IPv4 addresses) - 0.0.0.0:0000
    pub fn hosturl(&self) -> String {
        format!("{}:{}", self.addr.ip, self.addr.port)
    }

    /// Returns the service's configuration information
    /// in the `toml::Value` format.
    /// This will contain the ip/port if provided, along with any other
    /// configuration information found in the config file.
    ///
    /// ### Examples
    ///
    /// ```rust,no_run
    /// use kubos_service::Config;
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
        Err(f) => panic!(f.to_string()),
    };
    match matches.opt_str("c") {
        Some(s) => s,
        None => PATH.to_string(),
    }
}

fn get_file_data(path: String) -> Result<String, io::Error> {
    let mut contents = String::new();
    let mut file = File::open(path)?;
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn parse_config(name: &str, path: String) -> Result<Config, toml::de::Error> {
    let contents = get_file_data(path).unwrap_or("".to_string());
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
