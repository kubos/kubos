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

/// The high level metadata of an application derived from the `manifest.toml` file during
/// registration
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AppMetadata {
    /// A unique name for the application
    pub name: String,
    /// Optional. The path of the file which should be called to kick off execution.
    /// If not specified, ``name`` will be used.
    pub executable: Option<String>,
    /// The version of this application
    pub version: String,
    /// The author of the application
    pub author: String,
    /// The custom configuration file which should be passed to the application when it is started
    pub config: Option<String>,
}
/// Kubos App struct
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct App {
    /// The name of the application
    pub name: String,
    /// The absolute path to the application executable
    pub executable: String,
    /// The version of this instance of the application
    pub version: String,
    /// The author of the application
    pub author: String,
    /// Configuration file to be passed to the application
    pub config: String,
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
        let mut app_toml = PathBuf::from(self.app.executable.clone());
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
