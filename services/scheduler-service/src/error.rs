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
//! Scheduler specific errors
//!

use failure::Fail;

/// Errors which occur when using the scheduler
#[derive(Debug, Eq, Fail, PartialEq)]
pub enum SchedulerError {
    /// An error was raised while activating a mode
    #[fail(display = "Failed to activate '{}': {}", name, err)]
    ActivateError {
        /// The specific error encountered
        err: String,
        /// Mode which failed activation
        name: String,
    },
    #[fail(display = "Failed to create '{}': {}", path, err)]
    CreateError { err: String, path: String },
    /// An error was raised while importing a config file
    #[fail(display = "Failed to import '{}': {}", name, err)]
    ImportError {
        /// The specific import error
        err: String,
        // Path of config file which failed to import
        name: String,
    },
    /// An error was raised when removing a mode or task file
    #[fail(display = "Failed to remove '{}': {}", name, err)]
    RemoveError {
        /// Specific removal error
        err: String,
        /// Name of task or mode removed
        name: String,
    },
    // A generic scheduler error
    #[fail(display = "Scheduler error encountered: {}", err)]
    GenericError {
        /// Generic error encountered
        err: String,
    },
    #[fail(display = "Failed to parse duration '{}': {}", field, err)]
    DurationParseError {
        /// Error encountered
        err: String,
        /// Delay or time field parsed
        field: String,
    },
    #[fail(display = "Failed to import config file '{}': {}", name, err)]
    ConfigParseError { err: String, name: String },
    #[fail(display = "Failed to load mode {}: {}", path, err)]
    LoadModeError { err: String, path: String },
    #[fail(display = "Scheduler query failed: {}", err)]
    QueryError { err: String },
    #[fail(display = "Scheduler failed to start: {}", err)]
    StartError { err: String },
}

impl From<String> for SchedulerError {
    fn from(err: String) -> Self {
        SchedulerError::GenericError { err }
    }
}
