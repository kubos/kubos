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

#![deny(missing_docs)]

//! High level ADCS interfaces

use failure::Fail;

/// Errors for ADCS devices
#[derive(Fail, Debug, PartialEq)]
pub enum AdcsError {
    /// Generic error
    #[fail(display = "Generic error")]
    Generic,
    /// Configuration error
    #[fail(display = "Configuration error")]
    Config,
    /// No response received from subsystem
    #[fail(display = "No response received from subsystem")]
    NoResponse,
    /// An error was thrown by the subsystem
    #[fail(display = "An error was thrown by the subsystem")]
    Internal,
    /// Mutex-related error
    #[fail(display = "Mutex-related error")]
    Mutex,
    /// Requested function has not been implemented
    #[fail(display = "Requested function has not been implemented")]
    NotImplemented,
}

/// ADCS specific result type
pub type AdcsResult<T> = Result<T, AdcsError>;
