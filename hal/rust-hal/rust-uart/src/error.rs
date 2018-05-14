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

use std::error::Error;
use super::*;

/// Custom errors for UART actions
#[derive(Fail, Display, Debug, Clone, PartialEq)]
pub enum UartError {
    /// Catch-all error case
    #[display(fmt = "Generic Error")]
    GenericError,
    #[display(fmt = "Serial Error: {}", description)]
    /// An error was thrown by the serial driver
    SerialError {
        /// The underlying error type
        cause: serial::ErrorKind,
        /// Error description
        description: String,
    },
    #[display(fmt = "IO Error: {}", description)]
    /// An I/O error was thrown by the kernel
    IoError {
        /// The underlying error type
        cause: std::io::ErrorKind,
        /// Error description
        description: String,
    },
}

impl From<std::io::Error> for UartError {
    fn from(error: std::io::Error) -> Self {
        UartError::IoError {
            cause: error.kind(),
            description: error.description().to_owned(),
        }
    }
}

impl From<serial::Error> for UartError {
    fn from(error: serial::Error) -> Self {
        UartError::SerialError {
            cause: error.kind(),
            description: error.description().to_owned(),
        }
    }
}

/// Errors that occur while reading from and writing to stream
pub type UartResult<T> = Result<T, UartError>;
