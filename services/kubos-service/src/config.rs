//
// Copyright (C) 2017 Kubos Corporation
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
use serde_json;
use std::fs::File;
use std::io::prelude::*;
use serde_json::Value;

static DEFAULT_PATH: &str = "/home/system/etc/config.json";

#[derive(Debug, Deserialize)]
pub struct Config {
    ip: String,
    port: u16,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            ip: "127.0.0.1".to_string(),
            port: 8080,
        }
    }
}

impl Config {
    pub fn hosturl(&self) -> String {
        format!("{}:{}", self.ip, self.port)
    }
}

fn get_config_path() -> String {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optopt("c", "config", "path to config file", "CONFIG");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };
    match matches.opt_str("c") {
        Some(s) => s,
        None => DEFAULT_PATH.to_string(),
    }
}

fn parse_config(name: &str, path: String) -> Config {
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(_) => return Config::default(),
    };
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(s) => s,
        Err(_) => return Config::default(),
    };
    let data: Value = match serde_json::from_str(&contents) {
        Ok(d) => d,
        Err(_) => return Config::default(),
    };

    match data.get(name) {
        Some(v) => serde_json::from_str(&v.to_string()).unwrap_or(Config::default()),
        None => Config::default(),
    }
}

pub fn config(name: &str) -> Config {
    parse_config(name, get_config_path())
}
