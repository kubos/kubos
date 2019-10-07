/*
 * Copyright (C) 2019 Kubos Corporation
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

//!
//! Structures and functions concerning the actual running of a schedule
//!

use crate::config::{get_mode_configs, ScheduleConfig};
use crate::error::SchedulerError;
use crate::mode::{
    activate_mode, create_mode, get_active_mode, get_available_modes, is_mode_active,
};
use log::{error, info};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use std::thread;

pub static DEFAULT_SCHEDULES_DIR: &str = "/home/system/etc/schedules";
pub static SAFE_MODE: &str = "safe";

// Handle to primitives controlling scheduler runtime context
#[derive(Clone)]
pub struct SchedulerHandle {
    // Handle to thread running scheduler runtime
    pub thread_handle: Arc<Mutex<thread::JoinHandle<()>>>,
    // Sender for stopping scheduler runtime/thread
    pub stopper: Sender<()>,
}

#[derive(Clone)]
pub struct Scheduler {
    // Path to directory where schedules are stored
    pub scheduler_dir: String,
    // URL of App Service - for start app queries
    app_service_url: String,
    // Map of active config names and scheduler handles. This allows us to
    // start/stop tasks associated with individual config files
    scheduler_map: Arc<Mutex<HashMap<String, SchedulerHandle>>>,
}

impl Scheduler {
    // Create new Scheduler
    pub fn new(sched_dir: &str, app_service_url: &str) -> Scheduler {
        Scheduler {
            scheduler_dir: sched_dir.to_owned(),
            scheduler_map: Arc::new(Mutex::new(HashMap::<String, SchedulerHandle>::new())),
            app_service_url: app_service_url.to_owned(),
        }
    }

    // Ensure that conditions are good for starting the scheduler
    pub fn init(&self) -> Result<(), SchedulerError> {
        if !Path::new(&self.scheduler_dir).is_dir() {
            if let Err(e) = fs::create_dir(&self.scheduler_dir) {
                return Err(SchedulerError::CreateError {
                    err: e.to_string(),
                    path: self.scheduler_dir.to_owned(),
                });
            }
        }

        if get_active_mode(&self.scheduler_dir)?.is_none() {
            if get_available_modes(&self.scheduler_dir, Some(SAFE_MODE.to_owned()))?.is_empty() {
                create_mode(&self.scheduler_dir, SAFE_MODE)?;
            }
            activate_mode(&self.scheduler_dir, SAFE_MODE)?;
        }
        Ok(())
    }

    // Checks if config is in active mode and schedules tasks if needed
    pub fn check_start_config_tasks(&self, name: &str, mode: &str) -> Result<(), SchedulerError> {
        if is_mode_active(&self.scheduler_dir, &mode) {
            let config_path = format!("{}/{}/{}.json", self.scheduler_dir, mode, name);
            let config_path = Path::new(&config_path);
            let config = ScheduleConfig::from_path(&config_path)?;

            Ok(self.start_config_tasks(config)?)
        } else {
            Ok(())
        }
    }

    // Schedules tasks associated with config
    fn start_config_tasks(&self, config: ScheduleConfig) -> Result<(), SchedulerError> {
        let mut schedules_map = self.scheduler_map.lock().unwrap();

        let scheduler_handle = config.schedule_tasks(&self.app_service_url)?;

        schedules_map.insert(config.name, scheduler_handle);

        Ok(())
    }

    // Iterate through the active schedule file and kick off scheduling tasks
    pub fn start(&self) -> Result<(), SchedulerError> {
        if let Some(active_mode) = get_active_mode(&self.scheduler_dir)? {
            for config in get_mode_configs(&active_mode.path)? {
                self.start_config_tasks(config)?;
            }
            Ok(())
        } else {
            panic!("Failed to find active mode");
        }
    }

    // Stops all running tasks and clears of list of scheduler handles
    pub fn stop(&self) -> Result<(), SchedulerError> {
        let mut schedules_map = self.scheduler_map.lock().unwrap();
        for (name, handle) in schedules_map.drain().take(1) {
            info!("Stopping {}'s tasks", name);
            if let Err(e) = handle.stopper.send(()) {
                error!("Failed to send stop to {}'s tasks: {}", name, e);
            }
        }
        Ok(())
    }

    // Checks if a config exists in an active mode and stops scheduler if needed
    pub fn check_stop_config_tasks(&self, name: &str, mode: &str) -> Result<(), SchedulerError> {
        if is_mode_active(&self.scheduler_dir, mode) {
            Ok(self.stop_config_tasks(name)?)
        } else {
            Ok(())
        }
    }

    // Attempts to stop scheduler associated with config
    fn stop_config_tasks(&self, name: &str) -> Result<(), SchedulerError> {
        let mut schedules_map = self.scheduler_map.lock().unwrap();
        if let Some(handle) = schedules_map.remove(name) {
            info!("Stopping {}'s tasks", name);
            if let Err(e) = handle.stopper.send(()) {
                error!("Failed to send stop to {}'s tasks: {}", name, e);
            }
        }
        Ok(())
    }
}
