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

use crate::error::*;
use chrono::{DateTime, Utc};
use log::*;
use std::os::unix::process::ExitStatusExt;
use std::process::Child;
use std::sync::{Arc, Mutex};

/// Apps which have been started and are being monitored until they finish
#[derive(Clone, Debug, GraphQLObject)]
pub struct MonitorEntry {
    pub start_time: DateTime<Utc>,
    pub name: String,
    pub version: String,
    pub pid: i32,
    pub run_level: String,
    pub args: Option<Vec<String>>,
    pub config: Option<String>,
}

pub fn monitor_app(
    registry: Arc<Mutex<Vec<MonitorEntry>>>,
    mut process_handle: Child,
    app: MonitorEntry,
) {
    // Add the app information to our monitoring registry
    insert_entry(registry.clone(), &app);

    // Wait for the application to finish running
    let status = process_handle
        .wait()
        .map_err(|err| AppError::MonitorError {
            err: format!("Failed to wait for {} to finish: {:?}", app.name, err),
        })
        .unwrap();

    // Parse and log the result
    if status.success() {
        info!("App {} completed successfully", app.name);
    } else if let Some(code) = status.code() {
        error!("App {} failed. RC: {}", app.name, code);
    } else if let Some(signal) = status.signal() {
        warn!("App {} terminated by signal {}", app.name, signal);
    } else {
        warn!("App {} terminated for unknown reasons", app.name);
    }

    // Remove the app from our monitoring registry
    remove_entry(registry, &app.name, &app.run_level);
}

fn insert_entry(registry: Arc<Mutex<Vec<MonitorEntry>>>, entry: &MonitorEntry) {
    registry
        .lock()
        .map_err(|err| AppError::MonitorError {
            err: format!(
                "Failed to add {} to monitoring. Couldn't get entries mutex: {:?}",
                entry.name, err
            ),
        })
        .unwrap()
        .push(entry.clone());
}

fn remove_entry(registry: Arc<Mutex<Vec<MonitorEntry>>>, name: &str, run_level: &str) {
    let mut entries = registry
        .lock()
        .map_err(|err| AppError::MonitorError {
            err: format!(
                "Failed to remove {} from monitoring. Couldn't get entries mutex: {:?}",
                name, err
            ),
        })
        .unwrap();
    if let Some(index) = entries
        .iter()
        .position(|ref e| e.name == name && e.run_level == run_level)
    {
        entries.remove(index);
    } else {
        // This would only happen if we were somehow trying to doubly free a monitor entry,
        // which shouldn't ever happen
        error!("Failed to unmonitor {}", name);
    }
}

pub fn find_entry(
    registry: &Arc<Mutex<Vec<MonitorEntry>>>,
    name: &str,
    run_level: &str,
) -> Result<bool, AppError> {
    let entries = registry.lock().map_err(|err| AppError::MonitorError {
        err: format!("Failed get entries mutex: {:?}", err),
    })?;

    Ok(entries
        .iter()
        .any(|entry| entry.name == name && entry.run_level == run_level))
}
