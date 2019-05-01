//
// Copyright (C) 2019 Kubos Corporation
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

use failure::{Error, Fail};

/// This enum defines all errors that can occur within the `comms-service`.
#[derive(Fail, Debug, PartialEq)]
pub enum CommsServiceError {
    /// A component of the service's configuration was incorrect
    #[fail(display = "Config error: {}", _0)]
    ConfigError(String),
    /// The mutex guarding the telemetry cache has been poisoned.
    #[fail(display = "The mutex guarding the telemetry cache has been poisoned.")]
    MutexPoisoned,
    #[fail(display = "Reading data failed {}", _0)]
    ReadFailed(String),
    #[fail(display = "Writing data failed {}", _0)]
    WriteFailed(String)
}

/// Result returned by the `comms-service`.
pub type CommsResult<T> = Result<T, Error>;
