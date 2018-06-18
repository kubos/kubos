/*
 * Copyright (C) 2018 Kubos Corporation
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

//! A simple API to make standalone Rust applications with high-level hooks
//! for mission life-cycle management
//#![deny(missing_docs)]
#![deny(warnings)]

extern crate uuid;
extern crate getopts;

extern crate kubos_system;

#[macro_use]
extern crate serde_derive;
extern crate toml;

use std::env;

use getopts::Options;

pub mod registry;
use self::registry::*;

pub type App = registry::App;
pub type RunLevel = registry::RunLevel;

/// AppHandler
#[allow(unused_variables)]
pub trait AppHandler {
    ///
    fn on_boot(&self) {
    }

    ///
    fn on_command(&self) {
    }

    /// app process shutting down
    fn on_shutdown(&self) {
    }
}

#[macro_export]
macro_rules! app_main {
    ($handler:expr) => {{
        let name: Option<&'static str> = option_env!("CARGO_PKG_NAME");
        let version: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
        let authors: Option<&'static str> = option_env!("CARGO_PKG_AUTHORS");

         App::main(name.unwrap_or("Unknown"),
                   version.unwrap_or("Unknown"),
                   authors.unwrap_or("Unknown"),
                   std::process::id(),
                   $handler)
    }};
}

impl App {
    pub fn main(name: &str, version: &str, authors: &str, _pid: u32, handler: &AppHandler) {
        let args: Vec<String> = env::args().collect();
        let program = args[0].clone();

        let mut opts = Options::new();
        opts.optflag("m", "metadata", "Print app metadata and immediately exit");
        opts.optflag("h", "help", "Print this help menu");

        let matches = match opts.parse(&args[1..]) {
            Ok(m) => m,
            Err(f) => panic!(f.to_string())
        };

        if matches.opt_present("h") {
            let brief = format!("Usage: {} [options]", program);
            print!("{}", opts.usage(&brief));
            return;
        }

        if matches.opt_present("M") {
            println!("{}", toml::to_string(
                &AppMetadata::new(name, version, authors)).unwrap());
            return;
        }

        let _uuid = env::var_os("KUBOS_APP_UUID");
        let run_level = env::var_os("KUBOS_APP_RUN_LEVEL");

        match run_level {
            Some(level) => {
                if &level == "OnBoot" {
                    handler.on_boot();
                } else {
                    handler.on_command();
                }
            },
            None => { handler.on_command(); },
        }
    }
}
