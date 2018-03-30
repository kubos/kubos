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

#[macro_use]
extern crate failure;

/// Errors for ADCS devices
#[derive(Fail, Display, Debug, PartialEq)]
pub enum AdcsError {
    /// Generic error
    #[display(fmt = "Generic error")]
    Generic,
    /// Configuration error
    #[display(fmt = "Configuration error")]
    Config,
    /// No response received from subsystem
    #[display(fmt = "No response received from subsystem")]
    NoResponse,
    /// An error was thrown by the subsystem
    #[display(fmt = "An error was thrown by the subsystem")]
    Internal,
    /// Mutex-related error
    #[display(fmt = "Mutex-related error")]
    Mutex,
    /// Requested function has not been implemented
    #[display(fmt = "Requested function has not been implemented")]
    NotImplemented,
}

/// ADCS specific result type
pub type AdcsResult<T> = Result<T, AdcsError>;
