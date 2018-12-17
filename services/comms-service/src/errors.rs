//
// Copyright (C) 2018 Kubos Corporation
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
// Contributed by: William Greer (wgreer184@gmail.com) and Sam Justice (sam.justice1@gmail.com)
//

use failure::Error;

/// This enum defines all errors that can occur within the `comms-service`.
#[derive(Fail, Debug, PartialEq)]
pub enum CommsServiceError {
    /// The mutex guarding the telemetry cache has been poisoned.
    #[fail(display = "The mutex guarding the telemetry cache has been poisoned.")]
    MutexPoisoned,
    /// A UDP header was unable to be correctly parsed.
    #[fail(display = "A UDP header was unable to be correctly parsed.")]
    HeaderParsing,
    /// The length of a UDP packet does not match the length found in the header.
    #[fail(display = "The length of a UDP packet does not match the length found in the header.")]
    InvalidPacketLength,
    /// The checksum of a UDP packet does not match the one found in the header.
    #[fail(display = "The checksum of a UDP packet does not match the one found in the header.")]
    InvalidChecksum,
    /// The number of `write` methods and the number of downlink ports are not the same.
    #[fail(
        display = "The number of write methods and the number of downlink ports are not the same."
    )]
    ParameterLengthMismatch,
    /// The read thread could not be started because a no `write()` method was specified.
    #[fail(
        display = "The read thread could not be started because no write method was specified."
    )]
    MissingWriteMethod,
}

/// Result returned by the `comms-service`.
pub type CommsResult<T> = Result<T, Error>;
