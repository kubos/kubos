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
use log::*;
use std::os::unix::process::ExitStatusExt;
use std::process::Child;
use std::sync::{Arc, Mutex};

/// Apps which have been started and are being monitored until they finish
#[derive(Debug, GraphQLObject)]
pub struct MonitorEntry {
    name: String,
    version: String,
    pid: i32,
    run_level: String,
    args: Option<Vec<String>>,
    config: Option<String>,
}

pub fn monitor_app(
    registry: Arc<Mutex<Vec<MonitorEntry>>>,
    mut child: Child,
    name: String,
    version: String,
    run_level: String,
    args: Option<Vec<String>>,
    config: Option<String>,
) {
    // Add the app information to our monitoring registry
    insert_entry(
        registry.clone(),
        name.clone(),
        version,
        child.id() as i32,
        run_level,
        args,
        config,
    );

    // Wait for the application to finish running
    let status = child
        .wait()
        .map_err(|err| AppError::MonitorError {
            err: format!("Failed to wait for {} to finish: {:?}", name, err),
        })
        .unwrap();

    // Parse and log the result
    if status.success() {
        info!("App {} completed successfully", name);
    } else if let Some(code) = status.code() {
        error!("App {} failed. RC: {}", name, code);
    } else if let Some(signal) = status.signal() {
        warn!("App {} terminated by signal {}", name, signal);
    } else {
        warn!("App {} terminated for unknown reasons", name);
    }

    // Remove the app from our monitoring registry
    remove_entry(registry, name);
}

fn insert_entry(
    registry: Arc<Mutex<Vec<MonitorEntry>>>,
    name: String,
    version: String,
    pid: i32,
    run_level: String,
    args: Option<Vec<String>>,
    config: Option<String>,
) {
    registry
        .lock()
        .map_err(|err| AppError::MonitorError {
            err: format!(
                "Failed to add {} to monitoring. Couldn't get entries mutex: {:?}",
                name, err
            ),
        })
        .unwrap()
        .push(MonitorEntry {
            name,
            version,
            pid,
            run_level,
            args,
            config,
        });
}

fn remove_entry(registry: Arc<Mutex<Vec<MonitorEntry>>>, name: String) {
    let mut entries = registry
        .lock()
        .map_err(|err| AppError::MonitorError {
            err: format!(
                "Failed to remove {} from monitoring. Couldn't get entries mutex: {:?}",
                name, err
            ),
        })
        .unwrap();
    if let Some(index) = entries.iter().position(|ref e| e.name == name) {
        entries.remove(index);
    } else {
        // This would only happen if we were somehow trying to doubly free a monitor entry,
        // which shouldn't ever happen
        error!("Failed to unmonitor {}", name);
    }
}
