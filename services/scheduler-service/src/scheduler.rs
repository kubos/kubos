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

use crate::config::get_mode_configs;
use crate::mode::get_active_mode;
use kubos_service::Config;
use log::{error, info};
use std::fs;
use std::path::Path;
use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use tokio::prelude::future::lazy;
use tokio::prelude::*;
use tokio::runtime::Runtime;

pub static DEFAULT_SCHEDULES_DIR: &str = "/home/system/etc/schedules";

#[derive(Clone)]
pub struct Scheduler {
    // Path to directory where schedules are stored
    pub scheduler_dir: String,
    // Handle to thread running scheduler runtime
    scheduler_thread: Arc<Mutex<Option<thread::JoinHandle<()>>>>,
    // Channel sender for stopping scheduler
    scheduler_stopper: Arc<Mutex<Option<Sender<()>>>>,
}

impl Scheduler {
    // Create new Scheduler
    pub fn new(sched_dir: &str) -> Scheduler {
        if !Path::new(&sched_dir).is_dir() {
            if let Err(e) = fs::create_dir(&sched_dir) {
                error!("Failed to create schedule dir: {}", e);
                panic!("Failed to create schedule dir: {}", e)
            }
        }

        Scheduler {
            scheduler_dir: sched_dir.to_owned(),
            scheduler_thread: Arc::new(Mutex::new(None)),
            scheduler_stopper: Arc::new(Mutex::new(None)),
        }
    }

    // Iterate through the active schedule file and kick off scheduling tasks
    pub fn start(&self) -> Result<(), String> {
        let apps_service_config = Config::new("app-service")
            .map_err(|err| format!("Failed to load app service config: {:?}", err))?;

        let apps_service_url = apps_service_config
            .hosturl()
            .ok_or_else(|| "Failed to fetch app service url".to_owned())?;

        let active_mode = get_active_mode(&self.scheduler_dir)?;

        let (tx, rx) = channel::<()>();
        let handle = thread::spawn(move || {
            let mut runner = Runtime::new().unwrap_or_else(|e| {
                error!("Failed to create timer runtime: {}", e);
                panic!("Failed to create timer runtime: {}", e);
            });

            for sched in get_mode_configs(&active_mode.path).unwrap() {
                let task_app_url = apps_service_url.clone();
                runner.spawn(lazy(move || {
                    for task in sched.tasks {
                        info!("Scheduling init task: {}", &task.name);
                        // tokio::spawn(schedule_task(&task, task_app_url.clone()));
                        tokio::spawn(task.schedule(task_app_url.clone()));
                    }
                    Ok(())
                }));
            }

            // Wait on the stop message before ending the runtime
            rx.recv().unwrap_or_else(|e| {
                error!("Failed to received thread stop: {:?}", e);
                panic!("Failed to received thread stop: {:?}", e);
            });
            runner.shutdown_now().wait().unwrap_or_else(|e| {
                error!("Failed to wait on runtime shutdown: {:?}", e);
                panic!("Failed to wait on runtime shutdown: {:?}", e);
            })
        });

        let mut my_handle = self.scheduler_thread.lock().unwrap();
        *my_handle = Some(handle);

        let mut my_stopper = self.scheduler_stopper.lock().unwrap();
        *my_stopper = Some(tx);

        Ok(())
    }

    // Send signal to scheduler runtime to stop
    pub fn stop(&self) -> Result<(), String> {
        let stopper_guard = self.scheduler_stopper.lock().unwrap();
        if let Some(stopper) = stopper_guard.as_ref() {
            stopper
                .send(())
                .map_err(|e| format!("Failed to send stop to scheduler: {}", e))?;
        }

        Ok(())
    }
}
