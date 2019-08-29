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
use std::process::{Child, ExitStatus};
use std::sync::{Arc, Mutex};

/// Apps which have been started and are being monitored until they finish
#[derive(Clone, Debug, GraphQLObject)]
pub struct MonitorEntry {
    pub name: String,
    pub version: String,
    pub run_level: String,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub running: bool,
    pub pid: Option<i32>,
    pub last_rc: Option<i32>,
    pub last_signal: Option<i32>,
    pub args: Option<Vec<String>>,
    pub config: String,
}

// Check if any version of the application is running with the requested run level
pub fn find_running(
    registry: &Arc<Mutex<Vec<MonitorEntry>>>,
    name: &str,
    run_level: &str,
) -> Result<Option<MonitorEntry>, AppError> {
    let entries = registry.lock().map_err(|err| AppError::MonitorError {
        err: format!("Failed get entries mutex: {:?}", err),
    })?;

    Ok(entries.iter().find_map(|entry| {
        if entry.name == name && entry.run_level == run_level && entry.running {
            Some(entry.clone())
        } else {
            None
        }
    }))
}

// Find a monitoring registry entry by its PID
pub fn find_by_pid(
    registry: &Arc<Mutex<Vec<MonitorEntry>>>,
    pid: i32,
) -> Result<Option<MonitorEntry>, AppError> {
    let entries = registry.lock().map_err(|err| AppError::MonitorError {
        err: format!("Failed to get entries mutex: {:?}", err),
    })?;

    Ok(entries.iter().find_map(|entry| {
        if entry.pid == Some(pid) {
            Some(entry.clone())
        } else {
            None
        }
    }))
}

pub fn monitor_app(
    registry: Arc<Mutex<Vec<MonitorEntry>>>,
    mut process_handle: Child,
    name: &str,
    version: &str,
    run_level: &str,
) -> Result<(), AppError> {
    // Wait for the application to finish running
    let status = process_handle
        .wait()
        .map_err(|err| AppError::MonitorError {
            err: format!("Failed to wait for {} to finish: {:?}", name, err),
        })?;

    finish_entry(&registry, &name, &version, &run_level, status)
}

// Update/add an entry to denote the start of a new execution of an app
pub fn start_entry(
    registry: &Arc<Mutex<Vec<MonitorEntry>>>,
    new_entry: &MonitorEntry,
) -> Result<(), AppError> {
    let mut entries = registry.lock().map_err(|err| AppError::MonitorError {
        err: format!(
            "Failed to add {} to monitoring. Couldn't get entries mutex: {:?}",
            new_entry.name, err
        ),
    })?;

    // If an entry already exists for this name/version/run_level combo, update it
    // Otherwise, add the entry to the registry
    if let Some(index) = entries.iter().position(|ref e| {
        e.name == new_entry.name
            && e.version == new_entry.version
            && e.run_level == new_entry.run_level
    }) {
        debug!(
            "Updating existing entry for {} {} {}",
            new_entry.name, new_entry.version, new_entry.run_level
        );
        entries[index] = (*new_entry).clone();
    } else {
        debug!("Adding new entry: {:?}", new_entry);
        entries.push((*new_entry).clone());
    }

    Ok(())
}

// An app has finished running. Update its entry with the end time and RC or signal
pub fn finish_entry(
    registry: &Arc<Mutex<Vec<MonitorEntry>>>,
    name: &str,
    version: &str,
    run_level: &str,
    status: ExitStatus,
) -> Result<(), AppError> {
    let mut last_rc = None;
    let mut last_signal = None;
    let end_time = Utc::now();

    // Parse and log the result
    if status.success() {
        info!("App {} completed successfully", name);
        last_rc = Some(0);
    } else if let Some(code) = status.code() {
        error!("App {} failed. RC: {}", name, code);
        last_rc = Some(code);
    } else if let Some(signal) = status.signal() {
        warn!("App {} terminated by signal {}", name, signal);
        last_signal = Some(signal);
    } else {
        warn!("App {} terminated for unknown reasons", name);
    }

    let mut entries = registry.lock().map_err(|err| AppError::MonitorError {
        err: format!(
            "Failed to remove {} from monitoring. Couldn't get entries mutex: {:?}",
            name, err
        ),
    })?;

    // Find the monitoring entry and update it with the return code/exit signal and the end time
    if let Some(index) = entries
        .iter()
        .position(|ref e| e.name == name && e.version == version && e.run_level == run_level)
    {
        entries[index].running = false;
        entries[index].last_rc = last_rc;
        entries[index].last_signal = last_signal;
        entries[index].pid = None;
        entries[index].end_time = Some(end_time);
    } else {
        warn!(
            "Unable to find entry for {} {} {}",
            name, version, run_level
        );
    }

    Ok(())
}

// Remove an entry from the monitoring registry
// Used when uninstalling a version of an application from the master app registry
pub fn remove_entry(
    registry: &Arc<Mutex<Vec<MonitorEntry>>>,
    name: &str,
    version: &str,
    run_level: &str,
) -> Result<(), AppError> {
    let mut entries = registry.lock().map_err(|err| AppError::MonitorError {
        err: format!(
            "Failed to remove {} from monitoring. Couldn't get entries mutex: {:?}",
            name, err
        ),
    })?;
    if let Some(index) = entries
        .iter()
        .position(|ref e| e.name == name && e.version == version && e.run_level == run_level)
    {
        entries.remove(index);
    }

    Ok(())
}

// Remove all entries corresponding to a particular app from the monitoring registry
// Used when uninstalling an entire application from the master app registry
pub fn remove_entries(
    registry: &Arc<Mutex<Vec<MonitorEntry>>>,
    name: &str,
) -> Result<(), AppError> {
    let mut entries = registry.lock().map_err(|err| AppError::MonitorError {
        err: format!(
            "Failed to remove {} from monitoring. Couldn't get entries mutex: {:?}",
            name, err
        ),
    })?;

    // `drain_filter` is currently a nightly-only function, so instead we'll just keep
    // the apps that don't have the name we're trying to remove...
    entries.retain(|entry| entry.name != name);

    Ok(())
}
