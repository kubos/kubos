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

//! High level Eps API functions
//!
//! This crate contains high level types and functions for use
//! by other crates implementing Eps APIs.

#[macro_use]
extern crate failure;

use std::io;

/// EpsError
///
/// Describes various errors which may result from using Eps APIs
#[derive(Debug, Display, Eq, Fail, PartialEq)]
#[display(fmt = "Eps Error")]
pub enum EpsError {
    /// Error resulting from underlying Io functions
    #[display(fmt = "IO Error: {}", cause)]
    IoError {
        /// Underlying cause captured from io function
        cause: String,
    },
    /// Error resulting from receiving invalid data from Eps
    #[display(fmt = "Received invalid data: {}", data)]
    InvalidData {
        /// Invalid data which was received
        data: String,
    },
    /// Error resulting from a failure with an Eps command
    #[display(fmt = "Failure in Eps command: {}", command)]
    CommandFailure {
        /// Eps command which failed
        command: String,
    },
}

impl EpsError {
    /// Convience function for creating an EpsError::InvalidData
    ///
    /// # Arguments
    /// - data - Byte array to store in data parameter of enum
    pub fn invalid_data(data: &[u8]) -> EpsError {
        EpsError::InvalidData {
            data: String::from_utf8(data.to_vec()).unwrap(),
        }
    }
}

/// Convience converter from io::Error to EpsError
impl From<io::Error> for EpsError {
    fn from(error: io::Error) -> Self {
        EpsError::IoError {
            cause: error.to_string(),
        }
    }
}

/// Universal return type for Eps api functions
pub type EpsResult<T> = Result<T, EpsError>;
