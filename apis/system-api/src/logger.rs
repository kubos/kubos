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

//!
//! Common structure and functions for setting up service logging
//!

use failure::{bail, Error};
use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::config::Config;
use log4rs::encode::pattern::PatternEncoder;
use log4rs_syslog::SyslogAppender;
use std::env;

/// Initialize logging for the service
/// All messages will be routed to syslog and optionally echoed to the console
pub fn init(service_name: &str) -> Result<(), Error> {
    let stdout_flag = get_stdout_flag();
    let log_level = get_log_level()?;

    // Use custom PatternEncoder to avoid duplicate timestamps in logs.
    let syslog_encoder = Box::new(PatternEncoder::new("{m}"));
    // Set up logging which will be routed to syslog for processing
    let syslog = Box::new(
        SyslogAppender::builder()
            .encoder(syslog_encoder)
            .openlog(
                service_name,
                log4rs_syslog::LogOption::LOG_PID | log4rs_syslog::LogOption::LOG_CONS,
                log4rs_syslog::Facility::Daemon,
            )
            .build(),
    );

    // Set up logging which will be routed to stdout
    let stdout_appender = Box::new(ConsoleAppender::builder().build());

    let config_builder = Config::builder();
    let config_builder =
        config_builder.appender(log4rs::config::Appender::builder().build("syslog", syslog));
    let config_builder = if stdout_flag {
        config_builder
            .appender(log4rs::config::Appender::builder().build("stdout", stdout_appender))
    } else {
        config_builder
    };

    let root_builder = log4rs::config::Root::builder();
    let root_builder = root_builder.appender("syslog");
    let root_builder = if stdout_flag {
        root_builder.appender("stdout")
    } else {
        root_builder
    };
    let root_config = root_builder.build(log_level);

    let config = config_builder.build(root_config)?;

    // Start the logger
    log4rs::init_config(config)?;
    Ok(())
}

fn get_log_level() -> Result<LevelFilter, Error> {
    // Manually check for a "-l {log-level}" command line argument specifying a log level
    // Doing it this way so that entities which use this module (apps, services) can have any
    // number of additional command arguments
    let mut args = env::args();

    // Navigate to the "-l" option
    let config_arg_pos = args.position(|arg| arg == "-l");

    if let Some(_pos) = config_arg_pos {
        // The config path will be the arg immediately after "-l"
        match args.next() {
            Some(path) => match path.as_str() {
                "error" => Ok(LevelFilter::Error),
                "warn" => Ok(LevelFilter::Warn),
                "info" => Ok(LevelFilter::Info),
                "debug" => Ok(LevelFilter::Debug),
                "trace" => Ok(LevelFilter::Trace),
                _ => bail!("The '-l' arg was specified, but an invalid log level was provided"),
            },
            None => bail!("The '-l' arg was specified, but no log level was provided"),
        }
    } else {
        // The "-l" arg wasn't specified, so we can go ahead with the default
        Ok(LevelFilter::Debug)
    }
}

fn get_stdout_flag() -> bool {
    // Manually check for a "--stdout" command line argument requesting logging via stdout
    // Doing it this way so that entities which use this module (apps, services) can have any
    // number of additional command arguments
    let mut args = env::args();

    // Navigate to the "--stdout" option
    let config_arg_pos = args.position(|arg| arg == "--stdout");

    if let Some(_pos) = config_arg_pos {
        true
    } else {
        // The "--stdout" arg wasn't specified, so we can go ahead with the default
        false
    }
}
