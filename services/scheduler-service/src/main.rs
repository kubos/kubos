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

//! Service for scheduling tasks in the KubOS system.

// #![deny(warnings)]
// #![deny(missing_docs)]

#[macro_use]
extern crate juniper;

mod config;
mod file;
mod mode;
mod scheduler;
mod schema;

use kubos_service::{Config, Service};
use log::{error, info};
use scheduler::{Scheduler, DEFAULT_SCHEDULES_DIR};
use schema::{MutationRoot, QueryRoot};

// Initialize logging for the service
// All messages will be routed to syslog and echoed to the console
fn log_init() -> Result<(), String> {
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
                "kubos-scheduler-service",
                log4rs_syslog::LogOption::LOG_PID | log4rs_syslog::LogOption::LOG_CONS,
                log4rs_syslog::Facility::Daemon,
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
                .build(log::LevelFilter::Debug),
        )
        .map_err(|e| format!("Logging configuration failed: {}", e))?;

    // Start the logger
    log4rs::init_config(config).map_err(|e| format!("Logging setup failed: {}", e))?;
    Ok(())
}

fn main() -> Result<(), String> {
    log_init()?;

    let config = Config::new("scheduler-service").map_err(|err| {
        error!("Failed to load service config: {:?}", err);
        format!("Failed to load service config: {}", err)
    })?;

    let scheduler_dir = if let Some(s_dir) = config.get("schedules_dir") {
        String::from(
            s_dir
                .as_str()
                .ok_or_else(|| "Error parsing scheduler dir path".to_owned())?,
        )
    } else {
        String::from(DEFAULT_SCHEDULES_DIR)
    };

    let scheduler = Scheduler::new(&scheduler_dir);

    info!("Starting scheduler-service - {:?}", scheduler_dir);

    // For now we will only kick off scheduling when the scheduler comes up
    if let Err(e) = scheduler.start() {
        error!("Failed to schedule tasks: {:?}", e);
    }

    Service::new(config, scheduler, QueryRoot, MutationRoot).start();

    Ok(())
}
