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

use failure::{bail, Error};
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

/// A helper macro which detects the requested run level and calls the appropriate handler function.
/// Logging will be set up automatically for the application.
/// The log name will be taken from the package name specified in the application's `Cargo.toml` file.
/// Note: Any hyphens will be automatically converted to underscores (`test-app` -> `test_app`)
///
/// # Arguments
///
/// * `handler` - A reference to an object which implements the run level handler functions
/// * `level` - Default: `log::LevelFilter::Debug`. The minimum log level to record
///
/// # Examples
///
/// ```
/// use failure::Error;
/// use kubos_app::{AppHandler, app_main};
///
/// struct MyApp;
///
/// impl AppHandler for MyApp {
///   fn on_boot(&self, _args: Vec<String>) -> Result<(), Error> {
///     println!("OnBoot logic");
///     Ok(())
///   }
///   fn on_command(&self, _args: Vec<String>) -> Result<(), Error> {
///     println!("OnCommand logic");
///     Ok(())
///   }
/// }
///
/// fn main() -> Result<(), Error> {
///     let app = MyApp { };
///     app_main!(&app)?;
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! app_main {
    ($handler:expr) => {{
        app_main!($handler, log::LevelFilter::Debug)
    }};
    ($handler:expr, $level:expr) => {{
        let name = env!("CARGO_PKG_NAME");
        kubos_app::app_start($handler, name, $level)
    }};
}

/// The entry point for all KubOS applications. The preferred way to use this application
/// is through the `app_main!` macro
pub fn app_start(
    handler: &dyn AppHandler,
    name: &str,
    log_level: log::LevelFilter,
) -> Result<(), Error> {
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
                name,
                log4rs_syslog::LogOption::LOG_PID | log4rs_syslog::LogOption::LOG_CONS,
                log4rs_syslog::Facility::User,
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
                .build(log_level),
        )?;

    // Start the logger
    log4rs::init_config(config)?;

    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflagopt(
        "r",
        "run",
        "Run level which should be executed",
        "RUN_LEVEL",
    );
    // This option will be processed by the system-api crate when a service query is run
    opts.optflagopt(
        "c",
        "config",
        "System config file which should be used",
        "CONFIG",
    );
    opts.optflag("h", "help", "Print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(r) => r,
        Err(f) => panic!(f.to_string()),
    };

    if matches.opt_present("h") {
        let brief = format!("Usage: {} [options]", program);
        print!("{}", opts.usage(&brief));
        return Ok(());
    }

    let run_level = matches
        .opt_str("r")
        .unwrap_or_else(|| "OnCommand".to_owned());

    match run_level.as_ref() {
        "OnBoot" => handler.on_boot(matches.free),
        "OnCommand" => handler.on_command(matches.free),
        level => {
            bail!(
                "Error: Unknown run level was requested - {}. Available run levels: OnBoot, OnCommand", level
            );
        }
    }
}
