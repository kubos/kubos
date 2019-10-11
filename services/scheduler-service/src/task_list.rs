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
//! Definitions and functions concerning the manipulation of task lists
//!

use crate::error::SchedulerError;
use crate::scheduler::SchedulerHandle;
use crate::task::Task;
use chrono::{DateTime, Utc};
use juniper::GraphQLObject;
use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::thread;
use tokio::prelude::future::lazy;
use tokio::prelude::*;
use tokio::runtime::Runtime;

// Task list's contents
#[derive(Debug, GraphQLObject, Serialize, Deserialize)]
struct ListContents {
    pub tasks: Vec<Task>,
}

// Task list's metadata
#[derive(Debug, GraphQLObject)]
pub struct TaskList {
    pub tasks: Vec<Task>,
    pub path: String,
    pub name: String,
    pub time_imported: String,
}

impl TaskList {
    pub fn from_path(path_obj: &Path) -> Result<TaskList, SchedulerError> {
        let path = path_obj
            .to_str()
            .map(|path| path.to_owned())
            .ok_or_else(|| SchedulerError::TaskListParseError {
                err: "Failed to convert path".to_owned(),
                name: "".to_owned(),
            })?;

        let name = path_obj
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| SchedulerError::TaskListParseError {
                err: "Failed to read task list name".to_owned(),
                name: path.to_owned(),
            })?
            .to_owned();

        let data = path_obj
            .metadata()
            .map_err(|e| SchedulerError::TaskListParseError {
                err: format!("Failed to read file metadata: {}", e),
                name: name.to_owned(),
            })?;

        let time_imported: DateTime<Utc> = data
            .modified()
            .map_err(|e| SchedulerError::TaskListParseError {
                err: format!("Failed to get modified time: {}", e),
                name: name.to_owned(),
            })?
            .into();
        let time_imported = time_imported.format("%Y-%m-%d %H:%M:%S").to_string();

        let list_contents =
            fs::read_to_string(&path_obj).map_err(|e| SchedulerError::TaskListParseError {
                err: format!("Failed to read task list: {}", e),
                name: name.to_owned(),
            })?;

        let list_contents: ListContents = serde_json::from_str(&list_contents).map_err(|e| {
            SchedulerError::TaskListParseError {
                err: format!("Failed to parse json: {}", e),
                name: name.to_owned(),
            }
        })?;

        let tasks = list_contents.tasks;

        Ok(TaskList {
            path,
            name,
            tasks,
            time_imported,
        })
    }

    // Schedules the tasks contained in this task list
    pub fn schedule_tasks(&self, app_service_url: &str) -> Result<SchedulerHandle, SchedulerError> {
        let (stopper, receiver) = channel::<()>();
        let service_url = app_service_url.to_owned();
        let tasks = self.tasks.to_vec();
        let thread_handle = thread::spawn(move || {
            let mut runner = Runtime::new().unwrap_or_else(|e| {
                error!("Failed to create timer runtime: {}", e);
                panic!("Failed to create timer runtime: {}", e);
            });

            runner.spawn(lazy(move || {
                for task in tasks {
                    info!("Scheduling task '{}'", &task.name);
                    tokio::spawn(task.schedule(service_url.clone()));
                }
                Ok(())
            }));

            // Wait on the stop message before ending the runtime
            receiver.recv().unwrap_or_else(|e| {
                error!("Failed to received thread stop: {:?}", e);
                panic!("Failed to received thread stop: {:?}", e);
            });
            runner.shutdown_now().wait().unwrap_or_else(|e| {
                error!("Failed to wait on runtime shutdown: {:?}", e);
                panic!("Failed to wait on runtime shutdown: {:?}", e);
            })
        });
        let thread_handle = Arc::new(Mutex::new(thread_handle));
        Ok(SchedulerHandle {
            thread_handle,
            stopper,
        })
    }
}

// Copy a task list into a mode directory
pub fn import_task_list(
    scheduler_dir: &str,
    raw_name: &str,
    path: &str,
    raw_mode: &str,
) -> Result<(), SchedulerError> {
    let name = raw_name.to_lowercase();
    let mode = raw_mode.to_lowercase();
    info!(
        "Importing task list '{}': {} into mode '{}'",
        name, path, mode
    );
    let schedule_dest = format!("{}/{}/{}.json", scheduler_dir, mode, name);

    if !Path::new(&format!("{}/{}", scheduler_dir, mode)).is_dir() {
        return Err(SchedulerError::ImportError {
            err: "Mode not found".to_owned(),
            name: name.to_owned(),
        });
    }

    fs::copy(path, &schedule_dest).map_err(|e| SchedulerError::ImportError {
        err: e.to_string(),
        name: name.to_owned(),
    })?;

    let list_path = Path::new(&schedule_dest);
    if let Err(e) = TaskList::from_path(&list_path) {
        let _ = fs::remove_file(&schedule_dest);
        return Err(e);
    }

    Ok(())
}

// Remove an existing task list from the mode's directory
pub fn remove_task_list(scheduler_dir: &str, name: &str, mode: &str) -> Result<(), SchedulerError> {
    let name = name.to_lowercase();
    let mode = mode.to_lowercase();
    info!("Removing task list '{}'", name);
    let sched_path = format!("{}/{}/{}.json", scheduler_dir, mode, name);

    if !Path::new(&format!("{}/{}", scheduler_dir, mode)).is_dir() {
        return Err(SchedulerError::RemoveError {
            err: "Mode not found".to_owned(),
            name: name.to_owned(),
        });
    }

    if !Path::new(&sched_path).is_file() {
        return Err(SchedulerError::RemoveError {
            err: "File not found".to_owned(),
            name: name.to_owned(),
        });
    }

    fs::remove_file(&sched_path).map_err(|e| SchedulerError::RemoveError {
        err: e.to_string(),
        name: name.to_owned(),
    })?;

    info!("Removed task list '{}'", name);
    Ok(())
}

// Retrieve list of the task lists in a mode's directory
pub fn get_mode_task_lists(mode_path: &str) -> Result<Vec<TaskList>, SchedulerError> {
    let mut schedules = vec![];

    let mut files_list: Vec<PathBuf> = fs::read_dir(mode_path)
        .map_err(|e| SchedulerError::GenericError {
            err: format!("Failed to read mode dir: {}", e),
        })?
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
        match TaskList::from_path(&path) {
            Ok(sched) => schedules.push(sched),
            Err(e) => warn!("Failed to parse task list {:?}: {}", path, e),
        }
    }

    Ok(schedules)
}
