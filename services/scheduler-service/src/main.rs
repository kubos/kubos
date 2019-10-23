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

#![deny(warnings)]
#![deny(missing_docs)]

mod app;
mod error;
mod mode;
mod scheduler;
mod schema;
mod task;
mod task_list;

use crate::error::SchedulerError;
use kubos_service::{Config, Logger, Service};
use log::{error, info};
use scheduler::{Scheduler, DEFAULT_SCHEDULES_DIR};
use schema::{MutationRoot, QueryRoot};

fn main() -> Result<(), SchedulerError> {
    Logger::init("kubos-scheduler-service").unwrap();

    let config = Config::new("scheduler-service").map_err(|err| {
        error!("Failed to load service config: {:?}", err);
        SchedulerError::StartError {
            err: format!("Failed to load service config: {}", err),
        }
    })?;

    let scheduler_dir = if let Some(s_dir) = config.get("schedules_dir") {
        String::from(s_dir.as_str().ok_or_else(|| SchedulerError::StartError {
            err: "Error parsing scheduler dir path".to_owned(),
        })?)
    } else {
        String::from(DEFAULT_SCHEDULES_DIR)
    };

    let apps_service_config =
        Config::new("app-service").map_err(|err| SchedulerError::StartError {
            err: format!("Failed to load app service config: {:?}", err),
        })?;

    let apps_service_url =
        apps_service_config
            .hosturl()
            .ok_or_else(|| SchedulerError::StartError {
                err: "Failed to fetch app service url".to_owned(),
            })?;

    let scheduler = Scheduler::new(&scheduler_dir, &apps_service_url)?;

    info!("Starting scheduler-service - {:?}", scheduler.scheduler_dir);

    scheduler.init()?;

    // For now we will only kick off scheduling when the scheduler comes up
    if let Err(e) = scheduler.start() {
        error!("Failed to schedule tasks: {:?}", e);
    }

    Service::new(config, scheduler, QueryRoot, MutationRoot).start();

    Ok(())
}
