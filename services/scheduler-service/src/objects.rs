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

// Generic GraphQL Response
#[derive(Debug, GraphQLObject, Deserialize)]
pub struct GenericResponse {
    pub success: bool,
    pub errors: String,
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

// Configuration used for execution of an app
#[derive(Debug, Serialize, Deserialize)]
pub struct ScheduleApp {
    pub name: String,
    pub args: Option<Vec<String>>,
    pub config: Option<String>,
    pub run_level: Option<String>,
}

// Configuration used to schedule app execution
#[derive(Debug, Serialize, Deserialize)]
pub struct ScheduleTask {
    pub delay: String,
    pub app: ScheduleApp,
}

impl ScheduleTask {
    // TODO: When our threads panic, can we capture and put into our logs
    pub fn schedule(&self, app_service_url: String) -> Result<(), String> {
        let service_url = app_service_url.clone();
        let delay = self.delay.clone();
        let name = self.app.name.clone();
        let config = self.app.config.clone();
        let args = self.app.args.clone();
        let runlevel = self.app.run_level.clone().unwrap_or("onBoot".to_owned());
        std::thread::spawn(move || {
            let mut task_delay: Vec<&str> = delay.split(':').collect();
            let delay_s: u64 = task_delay.pop().unwrap_or("").parse().unwrap_or(0);
            let delay_m: u64 = task_delay.pop().unwrap_or("").parse().unwrap_or(0) * 60;
            let delay_h: u64 = task_delay.pop().unwrap_or("").parse().unwrap_or(0) * 3600;
            let when = Instant::now() + Duration::from_secs(delay_s + delay_m + delay_h);

            let task = Delay::new(when)
                .map_err(|e| {
                    error!("Task delay error: {}", e);
                    panic!("Task delay error: {}", e)
                })
                .and_then(move |_| {
                    info!("Start app {}", name);

                    let mut query_args = format!("runLevel: \"{}\", name: \"{}\"", runlevel, name);
                    if let Some(config) = config {
                        query_args.push_str(&format!(", config: \"{}\"", config));
                    }
                    if let Some(args) = args {
                        let app_args: Vec<String> =
                            args.iter().map(|x| format!("\"{}\"", x)).collect();

                        let app_args = app_args.join(",");
                        query_args.push_str(&format!(", args: [{}]", app_args));
                    }
                    let query = format!(
                        r#"mutation {{ startApp({}) {{ success, errors }} }}"#,
                        query_args
                    );
                    match service_query(&query, &service_url) {
                        Err(e) => {
                            error!("Sending app start query failed: {}", e);
                        }
                        Ok(result) => {
                            if result.data.start_app.success {
                                info!("App started sucessfully");
                            } else {
                                error!("App failed to start: {}", result.data.start_app.errors);
                            }
                        }
                    }
                    Ok(())
                });

            tokio::run(task);
        });
        Ok(())
    }
}

// Schedule configuration information - the guts of the actual schedule
#[derive(Debug, Serialize, Deserialize)]
pub struct ScheduleConfig {
    pub init: Option<HashMap<String, ScheduleTask>>,
}

// Descriptive information about a Schedule File
#[derive(Debug, GraphQLObject)]
pub struct ScheduleFile {
    pub contents: String,
    pub path: String,
    pub name: String,
    pub time_imported: String,
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

        let time_imported: DateTime<Utc> = data
            .modified()
            .map_err(|e| format!("Failed to get modified time: {}", e))?
            .into();
        let time_imported = time_imported.format("%Y-%m-%d %H:%M:%S").to_string();

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
            time_imported,
            active: false,
        })
    }
}
