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
//! Definitions and parsing instructions for Schedule Configurations
//!

use log::error;
use serde::{Deserialize, Serialize};
use std::time::Duration;

// Configuration used for execution of an app
#[derive(Debug, GraphQLObject, Serialize, Deserialize)]
pub struct ScheduleApp {
    pub name: String,
    pub args: Option<Vec<String>>,
    pub config: Option<String>,
    pub run_level: Option<String>,
}

// Configuration used to schedule app execution
#[derive(Debug, GraphQLObject, Serialize, Deserialize)]
pub struct ScheduleTask {
    // Descriptive name of task
    pub name: String,
    // Start delay specified in HH:mm:ss format
    // Used by init and recurring tasks
    pub delay: Option<String>,
    // Details of the app to be executed
    pub app: ScheduleApp,
}

impl ScheduleTask {
    // Parse delay duration from delay field
    pub fn get_duration(&self) -> Result<Duration, String> {
        if let Some(delay) = &self.delay {
            let delay_parts: Vec<String> = delay.split(' ').map(|s| s.to_owned()).collect();
            let mut task_delay: u64 = 0;
            for mut delay in delay_parts {
                let unit: Option<char> = delay.pop();
                let num: Result<u64, _> = delay.parse();
                if let Ok(num) = num {
                    match unit {
                        Some('s') => {
                            task_delay += num;
                        }
                        Some('m') => {
                            task_delay += num * 60;
                        }
                        Some('h') => {
                            task_delay += num * 60 * 60;
                        }
                        _ => {
                            error!("Failed to parse delay for task");
                            return Err("Failed to parse delay for task".to_owned());
                        }
                    }
                }
            }
            dbg!(task_delay);
            Ok(Duration::from_secs(task_delay))
        } else {
            Err("No delay defined for task".to_owned())
        }
    }
}

// Schedule configuration information - the guts of the actual schedule
#[derive(Debug, GraphQLObject, Serialize, Deserialize)]
pub struct ScheduleConfig {
    pub init: Option<Vec<ScheduleTask>>,
    pub one_time: Option<Vec<ScheduleTask>>,
}
