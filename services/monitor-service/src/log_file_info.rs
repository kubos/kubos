//
// Copyright (C) 2020 Kubos Corporation
//
// Licensed under the Apache License, Version 2.0 (the "License")
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

use failure;
use log::warn;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

static DEFAULT_LOG_PATH: &str = "/var/log";

#[derive(Clone, Debug)]
pub struct LogFileInfo {
    pub kubos_mod_time: Option<f64>,
    pub app_mod_time: Option<f64>,
}

impl LogFileInfo {
    pub fn from_disk(path: Option<&str>) -> Result<LogFileInfo, failure::Error> {
        Ok(LogFileInfo {
            kubos_mod_time: LogFileInfo::get_mtime(
                path.unwrap_or(DEFAULT_LOG_PATH),
                "kubos-warn.log",
            )?,
            app_mod_time: LogFileInfo::get_mtime(path.unwrap_or(DEFAULT_LOG_PATH), "app-warn.log")?,
        })
    }

    fn get_mtime(path: &str, file_name: &str) -> Result<Option<f64>, failure::Error> {
        let full_path = format!("{}/{}", path, file_name);
        if Path::new(&full_path).exists() {
            fs::metadata(&full_path)?
                .modified()?
                .duration_since(SystemTime::UNIX_EPOCH)
                .map(|duration| Some(duration.as_secs_f64()))
                .map_err(|err| err.into())
        } else {
            warn!("Unable to find log file {} on disk", full_path);
            Ok(None)
        }
    }
}
