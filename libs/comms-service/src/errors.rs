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

use failure::{Error, Fail};

/// This enum defines all errors that can occur within the `comms-service`.
#[derive(Fail, Debug, PartialEq, Eq)]
pub enum CommsServiceError {
    /// A component of the service's configuration was incorrect
    #[fail(display = "Config error: {}", _0)]
    ConfigError(String),
    /// The mutex guarding the telemetry cache has been poisoned.
    #[fail(display = "The mutex guarding the telemetry cache has been poisoned.")]
    MutexPoisoned,
    /// A UDP header was unable to be correctly parsed.
    #[fail(display = "A UDP header was unable to be correctly parsed.")]
    HeaderParsing,
    /// The checksum of a UDP packet does not match the one found in the header.
    #[fail(display = "The checksum of a UDP packet does not match the one found in the header.")]
    InvalidChecksum,
    /// The number of `write` methods and the number of downlink ports are not the same.
    #[fail(
        display = "The number of write methods and the number of downlink ports are not the same."
    )]
    ParameterLengthMismatch,
    /// All of the ports allocated for handling packets are binded and unable to be used.
    #[fail(display = "All of the ports allocated for handling packets are binded.")]
    NoAvailablePorts,
    /// No data available for reading
    #[fail(display = "No data available for reading")]
    NoReadData,
    /// An error was encountered when parsing a packet
    #[fail(display = "Parsing error {}", _0)]
    ParsingError(String),
    /// Generic error encountered
    #[fail(display = "Error encountered {}", _0)]
    GenericError(String),
    /// Unknown payload type encountered
    #[fail(display = "Unknown payload type encountered: {}", _0)]
    UnknownPayloadType(u16),
}

/// Result returned by the `comms-service`.
pub type CommsResult<T> = Result<T, Error>;
