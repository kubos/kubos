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

use super::*;
use std::error::Error;

/// Custom errors for UART actions
#[derive(Fail, Display, Debug, Clone, PartialEq)]
pub enum UartError {
    /// Catch-all error case
    #[display(fmt = "Generic Error")]
    GenericError,
    /// A read/write call was made while another call was already in-progress
    #[display(fmt = "Serial port already in-use")]
    PortBusy,
    /// An I/O error was thrown by the kernel
    #[display(fmt = "IO Error: {}", description)]
    IoError {
        /// The underlying error type
        cause: std::io::ErrorKind,
        /// Error description
        description: String,
    },
    /// An error was thrown by the serial driver
    #[display(fmt = "Serial Error: {}", description)]
    SerialError {
        /// The underlying error type
        cause: serial::ErrorKind,
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
