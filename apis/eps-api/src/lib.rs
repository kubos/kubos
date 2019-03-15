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

//! High level EPS API functions
//!
//! This crate contains high level types and functions for use
//! by other crates implementing EPS APIs.

use failure::Fail;
use std::error::Error;
use std::io;

/// EpsError
///
/// Describes various errors which may result from using EPS APIs
#[derive(Debug, Eq, Fail, PartialEq)]
#[fail(display = "Eps Error")]
pub enum EpsError {
    /// Generic error condition
    #[fail(display = "Generic Error")]
    GenericError,
    /// Error resulting from underlying Io functions
    #[fail(display = "IO Error: {}", description)]
    IoError {
        /// Underlying cause captured from io function
        cause: std::io::ErrorKind,
        /// Error description
        description: String,
    },
    /// Error resulting from receiving invalid data from EPS
    #[fail(display = "Parsing failed: {}", source)]
    ParsingFailure {
        /// Source where invalid data was received
        source: String,
    },
    /// Error resulting from a failure with an EPS command
    #[fail(display = "Failure in Eps command: {}", command)]
    CommandFailure {
        /// EPS command which failed
        command: String,
    },
}

impl EpsError {
    /// Convience function for creating an EpsError::ParsingFailure
    ///
    /// # Arguments
    /// - source - Source of parsing failure
    pub fn parsing_failure(source: &str) -> EpsError {
        EpsError::ParsingFailure {
            source: String::from(source),
        }
    }
}

/// Convience converter from io::Error to EpsError
impl From<io::Error> for EpsError {
    fn from(error: std::io::Error) -> Self {
        EpsError::IoError {
            cause: error.kind(),
            description: error.description().to_owned(),
        }
    }
}

/// Universal return type for EPS api functions
pub type EpsResult<T> = Result<T, EpsError>;
