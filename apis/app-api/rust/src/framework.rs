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

/// Helper macro to set up the logger for the program
///
/// All log messages will be sent to rsyslog using the User facility.
/// Additionally, they will also be echoed to ``stdout``
///
/// # Arguments
///
/// * `name` - The application name which should be used for all log messages
/// * `level` - The minimum logging level which should be recorded (Default: Debug)
///
/// # Examples
///
/// ```
/// use kubos_app::logging_setup;
/// // Initialize logging at default Debug level, using identifier "my_app"
/// logging_setup!("my_app");
/// ```
///
/// ```
/// use kubos_app::logging_setup;
/// use log::*;
///
/// // Initialize logging at Info level, using the crate name for the identifier
/// logging_setup!(env!("CARGO_PKG_NAME"), log::LevelFilter::Info);
/// ```
#[macro_export]
macro_rules! logging_setup {
    ($name:expr) => {{
        logging_setup!($name, log::LevelFilter::Debug)
    }};
    ($name:expr, $level:path) => {{
        use kubos_app::setup_log;
        setup_log($name, $level)
    }};
}

/// Set up the logger for the program
///
/// All log messages will be sent to rsyslog using the User facility.
/// Additionally, they will also be echoed to ``stdout``
///
/// # Arguments
///
/// * `name` - The application name which should be used for all log messages
/// * `log_level` - The minimum logging level which should be recorded
pub fn setup_log(name: &str, log_level: log::LevelFilter) -> Result<(), Error> {
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

    Ok(())
}
