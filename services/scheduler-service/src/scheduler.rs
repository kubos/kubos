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

use crate::config::{ScheduleConfig, ScheduleTask};
use crate::file::get_active_schedule;
use crate::schema::GenericResponse;
use kubos_service::Config;
use log::{error, info};
use reqwest::Client;
use serde::Deserialize;
use serde_json::from_str;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;
use tokio::prelude::future::lazy;
use tokio::prelude::*;
use tokio::runtime::Runtime;
use tokio::timer::Delay;
use std::time::Duration;

pub static DEFAULT_SCHEDULES_DIR: &str = "/home/system/etc/schedules";

// Generates a timer future for tokio to schedule and execute
fn schedule_task(
    task: &ScheduleTask,
    app_service_url: String,
) -> Box<dyn Future<Item = (), Error = ()> + Send> {
    let service_url = app_service_url.clone();
    let name = task.app.name.clone();
    let config = task.app.config.clone();
    let args = task.app.args.clone();
    let runlevel = task
        .app
        .run_level
        .clone()
        .unwrap_or_else(|| "onBoot".to_owned());

    let duration = match task.get_duration() {
        Ok(d) => d,
        Err(e) => {
            error!("Failed to parse task delay duration: {}", e);
            return Box::new(future::err::<(), ()>(()));
        }
    };

    let when = Instant::now() + duration;

    Box::new(
        Delay::new(when)
            .and_then(move |_| {
                info!("Start app {}", name);
                let mut query_args = format!("runLevel: \"{}\", name: \"{}\"", runlevel, name);
                if let Some(config) = config {
                    query_args.push_str(&format!(", config: \"{}\"", config));
                }
                if let Some(args) = args {
                    let app_args: Vec<String> = args.iter().map(|x| format!("\"{}\"", x)).collect();

                    let app_args = app_args.join(",");
                    query_args.push_str(&format!(", args: [{}]", app_args));
                }
                let query = format!(
                    r#"mutation {{ startApp({}) {{ success, errors }} }}"#,
                    query_args
                );
                match service_query(&query, &service_url) {
                    Err(e) => {
                        error!("Failed to send start app query: {}", e);
                        panic!("Failed to send start app query: {}", e);
                    }
                    Ok(resp) => {
                        if !resp.data.start_app.success {
                            error!(
                                "Failed to start scheduled app: {}",
                                resp.data.start_app.errors
                            );
                            panic!(
                                "Failed to start scheduled app: {}",
                                resp.data.start_app.errors
                            );
                        }
                    }
                }
                Ok(())
            })
            .map_err(|e| {
                error!("Task delay errored; err={:?}", e);
                panic!("Task delay errored; err={:?}", e)
            }),
    )
}

#[derive(Debug, Deserialize)]
pub struct StartAppResponse {
    #[serde(rename = "startApp")]
    pub start_app: GenericResponse,
}

#[derive(Debug, Deserialize)]
pub struct StartAppGraphQL {
    pub data: StartAppResponse,
}

// Helper function for sending query to app service
pub fn service_query(query: &str, hosturl: &str) -> Result<StartAppGraphQL, String> {
    let client = Client::builder()
        .timeout(Duration::from_millis(100))
        .build()
        .map_err(|e| format!("Failed to create client: {:?}", e))?;
    let mut map = HashMap::new();
    map.insert("query", query);
    let url = format!("http://{}", hosturl);

    let mut res = client
        .post(&url)
        .json(&map)
        .send()
        .map_err(|e| format!("Failed to send query: {:?}", e))?;

    Ok(from_str(
        &res.text()
            .map_err(|e| format!("Failed to get result text: {:?}", e))?,
    )
    .map_err(|e| format!("Failed to convert http result to json: {}", e))?)
}

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

        let active_schedule = get_active_schedule(&self.scheduler_dir)
            .ok_or_else(|| "Failed to fetch active schedule".to_owned())?;
        let active_config: ScheduleConfig = serde_json::from_str(&active_schedule.contents)
            .map_err(|e| format!("Failed to parse config: {}", e))?;

        let (tx, rx) = channel::<()>();
        let handle = thread::spawn(move || {
            let mut runner = Runtime::new().unwrap();
            runner.spawn(lazy(move || {
                if let Some(init) = active_config.init {
                    for (name, task) in init {
                        let task_app_url = apps_service_url.clone();
                        info!("Scheduling {}", name);
                        tokio::spawn(schedule_task(&task, task_app_url));
                    }
                }
                Ok(())
            }));
            // Wait on the stop message before ending the runtime
            rx.recv().unwrap();
            runner.shutdown_now().wait().unwrap();
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
