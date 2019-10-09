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
//! Definitions and functions concerning the manipulation of schedule modes
//!

use crate::error::SchedulerError;
use crate::scheduler::SAFE_MODE;
use crate::task_list::{get_mode_task_lists, TaskList};
use chrono::offset::TimeZone;
use chrono::{DateTime, Utc};
use juniper::GraphQLObject;
use log::{info, warn};
use std::fs;
use std::os::unix::fs::symlink;
use std::path::{Path, PathBuf};

// Descriptive information about a Schedule Mode
#[derive(Debug, GraphQLObject)]
pub struct ScheduleMode {
    pub name: String,
    pub path: String,
    pub last_revised: String,
    pub schedule: Vec<TaskList>,
    pub active: bool,
}

impl ScheduleMode {
    pub fn from_path(path_obj: &Path) -> Result<ScheduleMode, SchedulerError> {
        let path = path_obj
            .to_str()
            .map(|path| path.to_owned())
            .ok_or_else(|| SchedulerError::LoadModeError {
                err: "Failed to convert mode path".to_owned(),
                path: "".to_owned(),
            })?;

        let name = path_obj
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| SchedulerError::LoadModeError {
                err: "Failed to read mode name".to_owned(),
                path: path.to_owned(),
            })?
            .to_owned();

        let schedule = get_mode_task_lists(&path)?;

        let mut last_revised: DateTime<Utc> = Utc.ymd(1970, 1, 1).and_hms(0, 0, 0);
        for s in &schedule {
            let sched_time: DateTime<Utc> = Utc
                .datetime_from_str(&s.time_imported, "%Y-%m-%d %H:%M:%S")
                .unwrap();
            if sched_time > last_revised {
                last_revised = sched_time;
            }
        }

        let last_revised = last_revised.format("%Y-%m-%d %H:%M:%S").to_string();

        let active = false;

        Ok(ScheduleMode {
            name,
            path,
            last_revised,
            schedule,
            active,
        })
    }
}

// Retrieve information on the active scheduler mode
pub fn get_active_mode(scheduler_dir: &str) -> Result<Option<ScheduleMode>, SchedulerError> {
    match fs::read_link(format!("{}/active", scheduler_dir)) {
        Ok(active_path) => {
            let mut active_mode = ScheduleMode::from_path(&active_path)?;
            active_mode.active = true;
            Ok(Some(active_mode))
        }
        Err(e) => {
            warn!("Unable to read active mode link: {}", e);
            Ok(None)
        }
    }
}

pub fn is_mode_active(scheduler_dir: &str, name: &str) -> bool {
    let name = name.to_lowercase();
    if let Ok(Some(active_mode)) = get_active_mode(scheduler_dir) {
        name == active_mode.name
    } else {
        false
    }
}

pub fn get_available_modes(
    scheduler_dir: &str,
    name: Option<String>,
) -> Result<Vec<ScheduleMode>, SchedulerError> {
    let mut modes: Vec<ScheduleMode> = vec![];

    let active_path: Option<PathBuf> = fs::read_link(format!("{}/active", scheduler_dir)).ok();
    let mut modes_list: Vec<PathBuf> = fs::read_dir(scheduler_dir)
        .map_err(|e| SchedulerError::LoadModeError {
            err: format!("Failed to read schedules dir: {}", e),
            path: "".to_owned(),
        })?
        // Filter out invalid entries
        .filter_map(|x| x.ok())
        // Convert DirEntry -> PathBuf
        .map(|entry| entry.path())
        // Filter out non-directories
        .filter(|entry| entry.is_dir())
        // Filter out active directory
        .filter(|path| !path.ends_with("active"))
        // Filter on name if specified
        .filter(|path| {
            if let Some(name_str) = &name {
                path.ends_with(name_str.to_lowercase())
            } else {
                true
            }
        })
        .collect();
    // Sort into predictable order
    modes_list.sort();

    for path in modes_list {
        let active = if let Some(active_mode) = active_path.clone() {
            active_mode == path
        } else {
            false
        };

        match ScheduleMode::from_path(&path) {
            Ok(mut mode) => {
                mode.active = active;
                modes.push(mode);
            }
            Err(e) => warn!("Error loading mode: {}", e),
        }
    }

    Ok(modes)
}

pub fn create_mode(scheduler_dir: &str, name: &str) -> Result<(), SchedulerError> {
    let name = name.to_lowercase();
    let mode_dir = format!("{}/{}", scheduler_dir, name);
    Ok(
        fs::create_dir(mode_dir).map_err(|e| SchedulerError::CreateError {
            err: e.to_string(),
            path: name.to_owned(),
        })?,
    )
}

pub fn remove_mode(scheduler_dir: &str, name: &str) -> Result<(), SchedulerError> {
    let name = name.to_lowercase();

    if name == SAFE_MODE {
        return Err(SchedulerError::RemoveError {
            err: "The safe mode cannot be removed".to_owned(),
            name: name.to_owned(),
        });
    }

    if let Ok(Some(active_mode)) = get_active_mode(&scheduler_dir) {
        if name == active_mode.name {
            return Err(SchedulerError::RemoveError {
                err: "Cannot remove active mode".to_owned(),
                name: name.to_owned(),
            });
        }
    }

    let mode_dir = format!("{}/{}", scheduler_dir, name);
    Ok(
        fs::remove_dir_all(mode_dir).map_err(|e| SchedulerError::RemoveError {
            err: e.to_string(),
            name: name.to_owned(),
        })?,
    )
}

pub fn activate_mode(scheduler_dir: &str, name: &str) -> Result<(), SchedulerError> {
    let name = name.to_lowercase();
    info!("Activating mode {}", name);
    let sched_path = format!("{}/{}", scheduler_dir, name);
    let active_path = format!("{}/active", scheduler_dir);
    let new_active_path = format!("{}/new_active", scheduler_dir);

    if !Path::new(&sched_path).is_dir() {
        warn!("Attempted to activate non-existant mode. Falling back to safe mode.");
        activate_mode(scheduler_dir, SAFE_MODE)?;
        return Err(SchedulerError::ActivateError {
            err: "Mode not found".to_owned(),
            name: name.to_owned(),
        });
    }
    symlink(sched_path, &new_active_path).map_err(|e| SchedulerError::ActivateError {
        err: e.to_string(),
        name: name.to_owned(),
    })?;

    fs::rename(&new_active_path, &active_path).map_err(|e| SchedulerError::ActivateError {
        err: e.to_string(),
        name: name.to_owned(),
    })?;

    info!("Activated mode {}", name);
    Ok(())
}
