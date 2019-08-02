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
//! Definitions and functions concerning the manipulation of schedule files
//!

use chrono::{DateTime, Utc};
use log::{info, warn};
use std::fs;
use std::os::unix::fs::symlink;
use std::path::{Path, PathBuf};

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

// Copy a new schedule file into the schedules directory
pub fn import_schedule(scheduler_dir: &str, path: &str, name: &str) -> Result<(), String> {
    info!("Importing new schedule '{}': {}", name, path);
    let schedule_dest = format!("{}/{}.json", scheduler_dir, name);
    fs::copy(path, schedule_dest).map_err(|e| format!("Schedule copy failed: {}", e))?;
    Ok(())
}

// Make an existing schedule file the active schedule file
// TODO: Should this kill the schedule and rerun scheduler.start()?
// TODO: What if this is the active schedule?? What would happen?
pub fn activate_schedule(scheduler_dir: &str, name: &str) -> Result<(), String> {
    info!("Activating schedule {}", name);
    let sched_path = format!("{}/{}.json", scheduler_dir, name);
    let active_path = format!("{}/active.json", scheduler_dir);
    let new_active_path = format!("{}/new_active.json", scheduler_dir);

    if !Path::new(&sched_path).is_file() {
        return Err(format!("Schedule {}.json not found", name));
    }
    symlink(sched_path, &new_active_path)
        .map_err(|e| format!("Failed to create active symlink: {}", e))?;

    fs::rename(&new_active_path, &active_path)
        .map_err(|e| format!("Failed to copy over new active symlink: {}", e))?;

    info!("Activated schedule {}", name);
    Ok(())
}

// Remove an existing schedule file from the schedules directory
// TODO: What happens if this is the active schedule?
pub fn remove_schedule(scheduler_dir: &str, name: &str) -> Result<(), String> {
    info!("Removing schedule {}", name);
    let sched_path = format!("{}/{}.json", scheduler_dir, name);

    fs::remove_file(&sched_path)
        .map_err(|e| format!("Failed to remove schedule {}.json: {}", name, e))?;

    info!("Removed schedule {}", name);
    Ok(())
}

// Retrieve information on the active schedule file
pub fn get_active_schedule(scheduler_dir: &str) -> Option<ScheduleFile> {
    let active_path = fs::read_link(format!("{}/active.json", scheduler_dir)).ok()?;

    match ScheduleFile::from_path(&active_path) {
        Ok(mut s) => {
            s.active = true;
            Some(s)
        }
        Err(e) => {
            warn!("Failed to parse active schedule: {}", e);
            None
        }
    }
}

// Retrieve information on all schedule files in the schedules directory
pub fn get_available_schedules(
    scheduler_dir: &str,
    name: Option<String>,
) -> Result<Vec<ScheduleFile>, String> {
    let mut schedules: Vec<ScheduleFile> = vec![];

    let active_path: Option<PathBuf> = fs::read_link(format!("{}/active.json", scheduler_dir)).ok();

    for path in fs::read_dir(scheduler_dir)
        .map_err(|e| format!("Failed to read schedules dir: {}", e))?
        // Filter out invalid entries
        .filter_map(|x| x.ok())
        // Convert DirEntry -> PathBuf
        .map(|entry| entry.path())
        // Filter out non-files
        .filter(|entry| entry.is_file())
        // Filter out active.json
        .filter(|path| !path.ends_with("active.json"))
        // Filter on name if specified
        .filter(|path| {
            if let Some(name_str) = &name {
                path.ends_with(format!("{}.json", name_str))
            } else {
                true
            }
        })
    {
        let active = if let Some(active_sched) = active_path.clone() {
            active_sched == path
        } else {
            false
        };

        match ScheduleFile::from_path(&path) {
            Ok(mut sched) => {
                sched.active = active;
                schedules.push(sched);
            }
            Err(e) => warn!("Error loading schedule: {}", e),
        }
    }

    Ok(schedules)
}
