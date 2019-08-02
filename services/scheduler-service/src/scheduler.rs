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

pub static DEFAULT_SCHEDULES_DIR: &str = "/home/system/etc/schedules";

#[derive(GraphQLObject)]
pub struct Schedule {
    pub contents: String,
    pub path: String,
    pub name: String,
    pub time_registered: String,
    pub active: bool,
}

#[derive(Clone)]
pub struct Scheduler {
    scheduler_dir: String,
}

impl Scheduler {
    pub fn new(sched_dir: &str) -> Scheduler {
        Scheduler {
            scheduler_dir: sched_dir.to_owned(),
        }
    }
}
