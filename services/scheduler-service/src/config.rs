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
//! Definitions and functions concerning the manipulation of schedule configs
//!

use crate::task::ScheduleTask;
use chrono::{DateTime, Utc};
use juniper::GraphQLObject;
use log::info;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

// Schedule config's contents
#[derive(Debug, GraphQLObject, Serialize, Deserialize)]
struct ConfigContents {
    pub tasks: Vec<ScheduleTask>,
}

// Schedule config's metadata
#[derive(Debug, GraphQLObject)]
pub struct ScheduleConfig {
    pub tasks: Vec<ScheduleTask>,
    pub path: String,
    pub name: String,
    pub time_imported: String,
}

impl ScheduleConfig {
    pub fn from_path(path_obj: &Path) -> Result<ScheduleConfig, String> {
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

        let config = fs::read_to_string(&path_obj)
            .map_err(|e| format!("Failed to read schedule contents: {}", e))?;

        let config: ConfigContents = serde_json::from_str(&config)
            .map_err(|e| format!("Failed to parse schedule config: {}", e))?;

        let tasks = config.tasks;

        Ok(ScheduleConfig {
            path,
            name,
            tasks,
            time_imported,
        })
    }
}

// Copy a new schedule config into a mode directory
pub fn import_config(
    scheduler_dir: &str,
    path: &str,
    name: &str,
    mode: &str,
) -> Result<(), String> {
    info!(
        "Importing new config '{}': {} into mode '{}'",
        name, path, mode
    );
    let schedule_dest = format!("{}/{}/{}.json", scheduler_dir, mode, name);
    fs::copy(path, schedule_dest).map_err(|e| format!("Schedule copy failed: {}", e))?;
    Ok(())
}

// Remove an existing schedule config from the mode's directory
pub fn remove_config(scheduler_dir: &str, name: &str, mode: &str) -> Result<(), String> {
    info!("Removing config {}", name);
    let sched_path = format!("{}/{}/{}.json", scheduler_dir, mode, name);

    if !Path::new(&format!("{}/{}", scheduler_dir, mode)).is_dir() {
        return Err(format!("Mode {} not found", mode));
    }

    if !Path::new(&sched_path).is_file() {
        return Err(format!("Config file {}.json not found", name));
    }

    fs::remove_file(&sched_path)
        .map_err(|e| format!("Failed to remove config {}.json: {}", name, e))?;

    info!("Removed config {}", name);
    Ok(())
}

// Get all the schedules in a mode's directory
pub fn get_mode_configs(mode_path: &str) -> Result<Vec<ScheduleConfig>, String> {
    let mut schedules = vec![];

    let mut files_list: Vec<PathBuf> = fs::read_dir(mode_path)
        .map_err(|e| format!("Failed to read mode dir: {}", e))?
        // Filter out invalid entries
        .filter_map(|x| x.ok())
        // Convert DirEntry -> PathBuf
        .map(|entry| entry.path())
        // Filter out non-directories
        .filter(|entry| entry.is_file())
        .collect();
    // Sort into predictable order
    files_list.sort();

    for path in files_list {
        schedules.push(ScheduleConfig::from_path(&path)?);
    }

    Ok(schedules)
}
