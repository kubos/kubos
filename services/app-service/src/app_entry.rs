/*
 * Copyright (C) 2018 Kubos Corporation
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
use serde_derive::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use toml;

/// The high level metadata of an application
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AppMetadata {
    /// A unique name for the application (usually the same as the name of the binary)
    pub name: String,
    /// The version of this application
    pub version: String,
    /// The author of the application
    pub author: String,
}
/// Kubos App struct
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct App {
    /// The generated UUID for the application
    pub uuid: String,
    /// The process ID of the application, if it's currently running (0 otherwise)
    pub pid: u32,
    /// The absolute path to the application binary
    pub path: String,
    /// The associated metadata of the application
    pub metadata: AppMetadata,
}
/// AppRegistryEntry
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AppRegistryEntry {
    /// Whether or not this application is the active installation
    pub active_version: bool,
    /// The app itself
    pub app: App,
}

impl AppRegistryEntry {
    // Fetch a registered apps entry information
    pub fn from_dir(dir: &PathBuf) -> Result<AppRegistryEntry, AppError> {
        let mut app_toml = dir.clone();
        app_toml.push("app.toml");
        if !app_toml.exists() {
            return Err(AppError::FileError {
                err: "No app.toml file found".to_owned(),
            });
        }

        let app_entry = fs::read_to_string(app_toml)?;

        match toml::from_str::<AppRegistryEntry>(&app_entry) {
            Ok(entry) => Ok(entry),
            Err(error) => Err(AppError::ParseError {
                entity: "app.toml".to_owned(),
                err: error.to_string(),
            }),
        }
    }

    // Create or update a registered apps entry information
    pub fn save(&self) -> Result<(), AppError> {
        let mut app_toml = PathBuf::from(self.app.path.clone());
        app_toml.set_file_name("app.toml");

        let mut file = fs::File::create(app_toml)?;
        let toml_str = match toml::to_string(&self) {
            Ok(toml) => toml,
            Err(error) => {
                return Err(AppError::ParseError {
                    entity: "app entry".to_owned(),
                    err: error.to_string(),
                });
            }
        };

        file.write_all(&toml_str.into_bytes())?;
        Ok(())
    }
}
