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

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

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
            let mut task_delay: Vec<&str> = delay.split(':').collect();
            let delay_s: u64 = task_delay.pop().unwrap_or("").parse().unwrap_or(0);
            let delay_m: u64 = task_delay.pop().unwrap_or("").parse().unwrap_or(0) * 60;
            let delay_h: u64 = task_delay.pop().unwrap_or("").parse().unwrap_or(0) * 3600;
            Ok(Duration::from_secs(delay_s + delay_m + delay_h))
        } else {
            Err("No delay defined for task".to_owned())
        }
    }
}

// Schedule configuration information - the guts of the actual schedule
#[derive(Debug, Serialize, Deserialize)]
pub struct ScheduleConfig {
    pub init: Option<HashMap<String, ScheduleTask>>,
    pub one_time: Option<HashMap<String, ScheduleTask>>,
}
