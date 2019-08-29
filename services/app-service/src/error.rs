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

use failure::Fail;

#[derive(Debug, Fail, PartialEq)]
pub enum AppError {
    /// An error was encountered while interacting with the app registry
    #[fail(display = "Registry Error: {}", err)]
    RegistryError {
        /// Underlying error encountered
        err: String,
    },
    /// An error was encountered while interacting with a file
    #[fail(display = "File Error: {}", err)]
    FileError {
        /// Underlying error encountered
        err: String,
    },
    /// An error was encountered while registering an application
    #[fail(display = "Failed to register app: {}", err)]
    RegisterError {
        /// Underlying error encountered
        err: String,
    },
    /// An error was encountered while registering an application
    #[fail(display = "Failed to uninstall app: {}", err)]
    UninstallError {
        /// Underlying error encountered
        err: String,
    },
    /// An error was encountered while starting an application
    #[fail(display = "Failed to start app: {}", err)]
    StartError {
        /// Underlying error encountered
        err: String,
    },
    /// An error was encountered while parsing data
    #[fail(display = "Failed to parse {}: {}", entity, err)]
    ParseError {
        /// Item being parsed
        entity: String,
        /// Underlying error encountered
        err: String,
    },
    /// An I/O error was thrown by the kernel
    #[fail(display = "IO Error: {}", description)]
    IoError {
        /// The underlying error type
        cause: ::std::io::ErrorKind,
        /// Error description
        description: String,
    },
    /// A catch-all error for the service
    #[fail(display = "{}", err)]
    SystemError {
        /// Underlying error encountered
        err: String,
    },
    /// An error was encountered while monitoring an application
    #[fail(display = "Error while monitoring app: {}", err)]
    MonitorError {
        /// Underlying error encountered
        err: String,
    },
    /// An error was encountered while killing an application
    #[fail(display = "Failed to kill app: {}", err)]
    KillError {
        /// Underlying error encountered
        err: String,
    },
}

impl From<::std::io::Error> for AppError {
    fn from(error: ::std::io::Error) -> Self {
        AppError::IoError {
            cause: error.kind(),
            description: error.to_string(),
        }
    }
}
