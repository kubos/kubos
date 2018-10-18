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

#[derive(Debug, Fail)]
pub enum AppError {
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
}