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

use crate::util::service_query;
use chrono::{DateTime, Utc};
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::time::{Duration, Instant};
use tokio::prelude::*;
use tokio::timer::Delay;
use chrono::prelude::*;

// Generic GraphQL Response
#[derive(GraphQLObject)]
pub struct GenericResponse {
    pub success: bool,
    pub errors: String,
}

// Configuration used for execution of an app
#[derive(Debug, Serialize, Deserialize)]
pub struct ScheduleApp {
    pub name: String,
    pub args: Option<String>,
    pub config: Option<String>,
}

// Configuration used to schedule app execution
#[derive(Debug, Serialize, Deserialize)]
pub struct ScheduleTask {
    // Start delay specified in HH:mm:ss format
    // Used by init and recurring tasks
    pub delay: Option<String>,
    // Start time specified in yyyy-mm-dd hh:mm:ss format
    // Used by onetime tasks
    pub time: Option<String>,
    // Details of the app to be executed
    pub app: ScheduleApp,
}

impl ScheduleTask {
    pub fn schedule(&self, app_service_url: String) -> Result<(), String> {
        let service_url = app_service_url.clone();
        let delay = self.delay.clone();
        let time = self.time.clone();
        let name = self.app.name.clone();
        std::thread::spawn(move || {
            let when = if let Some(delay) = delay {
                let mut task_delay: Vec<&str> = delay.split(':').collect();
                let delay_s: u64 = task_delay.pop().unwrap_or("").parse().unwrap_or(0);
                let delay_m: u64 = task_delay.pop().unwrap_or("").parse().unwrap_or(0) * 60;
                let delay_h: u64 = task_delay.pop().unwrap_or("").parse().unwrap_or(0) * 3600;
                Instant::now() + Duration::from_secs(delay_s + delay_m + delay_h)
            } else if let Some(time) = time {
                let dt = match Utc.datetime_from_str(&time, "%Y-%m-%d %H:%M:%S") {
                    Ok(dt) => dt,
                    Err(e) => {
                        error!("Failed to parse one_time starttime: {:?}", e);
                        return;
                    }
                };
                let now: chrono::DateTime<Utc> = chrono::Utc::now();
                info!("Scheduling onetime task for {:?}", dt);
                let diff = match (dt - now).to_std() {
                    Ok(diff) => diff,
                    Err(e) => {
                        error!("Failed to create one_time time diff: {:?}", e);
                        return;
                    }
                };
                info!("Time diff {:?}", diff);
                Instant::now() + diff
            } else {
                error!("No delay or start time defined");
                return;
            };

            let task = Delay::new(when)
                .and_then(move |_| {
                    info!("Start app {}", name);
                    let query =
                        format!(r#"mutation {{ startApp(name: "{}") {{ success }} }}"#, name);
                    service_query(&query, &service_url);
                    Ok(())
                })
                .map_err(|e| panic!("delay errored; err={:?}", e));

            tokio::run(task);
        });
        Ok(())
    }
}

// Schedule configuration information - the guts of the actual schedule
#[derive(Debug, Serialize, Deserialize)]
pub struct ScheduleConfig {
    pub init: Option<HashMap<String, ScheduleTask>>,
    pub one_time: Option<HashMap<String, ScheduleTask>>,
}

// Descriptive information about a Schedule File
#[derive(Debug, GraphQLObject)]
pub struct ScheduleFile {
    pub contents: String,
    pub path: String,
    pub name: String,
    pub time_registered: String,
    pub active: bool,
}

impl ScheduleFile {
    pub fn from_path(path_obj: &Path) -> Result<ScheduleFile, String> {
        let path = path_obj
            .to_str()
            .map(|path| path.to_owned())
            .ok_or_else(|| "Failed to convert path".to_owned())?;

        let data = path_obj
            .metadata()
            .map_err(|e| format!("Failed to read file metadata: {}", e))?;

        let time_registered: DateTime<Utc> = data
            .modified()
            .map_err(|e| format!("Failed to get modified time: {}", e))?
            .into();
        let time_registered = time_registered.format("%Y-%m-%d %H:%M:%S").to_string();

        let name = path_obj
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| "Failed to read schedule name".to_owned())?
            .to_owned();

        let contents = fs::read_to_string(&path_obj)
            .map_err(|e| format!("Failed to read schedule contents: {}", e))?;

        Ok(ScheduleFile {
            path,
            name,
            contents,
            time_registered,
            active: false,
        })
    }
}
