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

use crate::objects::Schedule;
use log::{error, info, warn};
use std::fs;
use std::os::unix::fs::symlink;
use std::path::{Path, PathBuf};

pub static DEFAULT_SCHEDULES_DIR: &str = "/home/system/etc/schedules";

#[derive(Clone)]
pub struct Scheduler {
    scheduler_dir: String,
}

impl Scheduler {
    pub fn new(sched_dir: &str) -> Scheduler {
        if !Path::new(&sched_dir).is_dir() {
            if let Err(e) = fs::create_dir(&sched_dir) {
                error!("Failed to create schedule dir: {}", e);
                panic!("Failed to create schedule dir: {}", e)
            }
        }

        Scheduler {
            scheduler_dir: sched_dir.to_owned(),
        }
    }

    pub fn register_schedule(&self, path: &str, name: &str) -> Result<(), String> {
        info!("Registering new schedule '{}': {}", name, path);
        let schedule_dest = format!("{}/{}.json", self.scheduler_dir, name);
        fs::copy(path, schedule_dest).map_err(|e| format!("Schedule copy failed: {}", e))?;
        Ok(())
    }

    pub fn activate_schedule(&self, name: &str) -> Result<(), String> {
        info!("Activating schedule {}", name);
        let sched_path = format!("{}/{}.json", self.scheduler_dir, name);
        let active_path = format!("{}/active.json", self.scheduler_dir);

        if !Path::new(&sched_path).is_file() {
            return Err(format!("Schedule {}.json not found", name));
        }

        if Path::new(&active_path).is_file() {
            fs::remove_file(&active_path)
                .map_err(|e| format!("Failed to remove active symlink: {}", e))?;
        }

        symlink(sched_path, active_path)
            .map_err(|e| format!("Failed to create active symlink: {}", e))?;
        info!("Activated schedule {}", name);
        Ok(())
    }

    pub fn remove_schedule(&self, name: &str) -> Result<(), String> {
        info!("Removing schedule {}", name);
        let sched_path = format!("{}/{}.json", self.scheduler_dir, name);

        fs::remove_file(&sched_path)
            .map_err(|e| format!("Failed to remove schedule {}.json: {}", name, e))?;

        info!("Removed schedule {}", name);
        Ok(())
    }

    pub fn get_active_schedule(&self) -> Option<Schedule> {
        let active_path = fs::read_link(format!("{}/active.json", &self.scheduler_dir)).ok()?;

        match Schedule::from_path(&active_path) {
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

    pub fn get_registered_schedules(&self, name: Option<String>) -> Result<Vec<Schedule>, String> {
        let mut schedules: Vec<Schedule> = vec![];

        let active_path: Option<PathBuf> =
            fs::read_link(format!("{}/active.json", &self.scheduler_dir)).ok();

        for path in fs::read_dir(&self.scheduler_dir)
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

            match Schedule::from_path(&path) {
                Ok(mut sched) => {
                    sched.active = active;
                    schedules.push(sched);
                }
                Err(e) => warn!("Error loading schedule: {}", e),
            }
        }

        Ok(schedules)
    }
}
