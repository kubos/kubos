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

use chrono::{DateTime, Utc};
use std::fs;

use std::path::Path;

#[derive(GraphQLObject)]
pub struct GenericResponse {
    pub success: bool,
    pub errors: String,
}

#[derive(GraphQLObject)]
pub struct Schedule {
    pub contents: String,
    pub path: String,
    pub name: String,
    pub time_registered: String,
    pub active: bool,
}

impl Schedule {
    pub fn from_path(path_obj: &Path) -> Result<Schedule, String> {
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

        Ok(Schedule {
            path,
            name,
            contents,
            time_registered,
            active: false,
        })
    }
}
