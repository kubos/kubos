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
#![deny(missing_docs)]
#![deny(warnings)]

use failure::Error;
use getopts::Options;
use std::env;
use std::fmt;

/// The different ways an application can be started
#[derive(Clone, Debug, PartialEq)]
pub enum RunLevel {
    /// Logic intended to be run if the application is started at system boot time
    OnBoot,
    /// Logic intended to be run if the application is started manually
    OnCommand,
}

impl fmt::Display for RunLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RunLevel::OnBoot => write!(f, "OnBoot"),
            RunLevel::OnCommand => write!(f, "OnCommand"),
        }
    }
}

/// Common trait which is used to ensure handlers for all required run levels are defined
pub trait AppHandler {
    /// Called when the application is started at system boot time
    fn on_boot(&self, args: Vec<String>) -> Result<(), Error>;

    /// Called when the application is started on-demand through the `start_app` GraphQL mutation
    fn on_command(&self, args: Vec<String>) -> Result<(), Error>;
}

/// A helper macro which detects the requested run level and calls the appropriate handler function
///
/// # Arguments
///
/// * `handler` - A reference to an object which implements the run level handler functions
///
/// # Examples
///
/// ```
/// #[macro_use]
/// extern crate kubos_app;
///
/// use kubos_app::AppHandler;
///
/// struct MyApp;
///
/// impl AppHandler for MyApp {
///   fn on_boot(&self, _args: Vec<String>) -> Result<(), Error> {
///     println!("OnBoot logic");
///   }
///   fn on_command(&self, _args: Vec<String>) -> Result<(), Error> {
///     println!("OnCommand logic");
///   }
/// }
///
/// fn main() {
///     let app = MyApp { };
///     app_main!(&app);
/// }
/// ```
#[macro_export]
macro_rules! app_main {
    ($handler:expr) => {{
        kubos_app::app_start(std::process::id(), $handler)
    }};
}

/// The entry point for all KubOS applications. The preferred way to use this application
/// is through the `app_main!` macro
pub fn app_start(_pid: u32, handler: &AppHandler) -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflagopt(
        "r",
        "run",
        "Run level which should be executed",
        "RUN_LEVEL",
    );
    opts.optflag("h", "help", "Print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(r) => r,
        Err(f) => panic!(f.to_string()),
    };

    if matches.opt_present("h") {
        let brief = format!("Usage: {} [options]", program);
        print!("{}", opts.usage(&brief));
        return Ok(())
    }

    let _uuid = env::var_os("KUBOS_APP_UUID");
    let run_level = matches.opt_str("r").unwrap_or("OnCommand".to_owned());

    match run_level.as_ref() {
        "OnBoot" => handler.on_boot(args),
        "OnCommand" => handler.on_command(args),
        level => {
            bail!(
                "Error: Unknown run level was requested - {}. Available run levels: OnBoot, OnCommand", level
            );
        }
    }
}
