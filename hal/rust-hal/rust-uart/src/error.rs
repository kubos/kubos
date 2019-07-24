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

use failure::Fail;
#[cfg(feature = "nos3")]
use nosengine_rust::client::uart;
use std::error::Error;
#[cfg(feature = "nos3")]
use std::sync;
#[cfg(feature = "nos3")]
use toml;

/// Custom errors for UART actions
#[derive(Fail, Debug, Clone, PartialEq)]
pub enum UartError {
    /// Catch-all error case
    #[fail(display = "Generic Error")]
    GenericError,
    /// A read/write call was made while another call was already in-progress
    #[fail(display = "Serial port already in-use")]
    PortBusy,
    /// An I/O error was thrown by the kernel
    #[fail(display = "IO Error: {}", description)]
    IoError {
        /// The underlying error type
        cause: std::io::ErrorKind,
        /// Error description
        description: String,
    },
    /// An error was thrown by the serial driver
    #[fail(display = "Serial Error: {}", description)]
    SerialError {
        /// The underlying error type
        cause: serial::ErrorKind,
        /// Error description
        description: String,
    },
    /// A poison error from the nosengine-rust uart client
    #[cfg(feature = "nos3")]
    #[fail(display = "Mutex Poison Error")]
    MutexPoisonError,
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

#[cfg(feature = "nos3")]
impl From<uart::UARTError> for UartError {
    fn from(_error: uart::UARTError) -> Self {
        UartError::GenericError
    }
}

#[cfg(feature = "nos3")]
impl From<toml::ser::Error> for UartError {
    fn from(_error: toml::ser::Error) -> Self {
        UartError::GenericError
    }
}

#[cfg(feature = "nos3")]
impl From<toml::de::Error> for UartError {
    fn from(_error: toml::de::Error) -> Self {
        UartError::GenericError
    }
}

#[cfg(feature = "nos3")]
impl From<sync::mpsc::RecvError> for UartError {
    fn from(_error: sync::mpsc::RecvError) -> Self {
        UartError::GenericError
    }
}

#[cfg(feature = "nos3")]
impl From<std::sync::PoisonError<std::sync::MutexGuard<'_, nosengine_rust::client::uart::UART>>>
    for UartError
{
    fn from(
        _error: std::sync::PoisonError<
            std::sync::MutexGuard<'_, nosengine_rust::client::uart::UART>,
        >,
    ) -> Self {
        UartError::MutexPoisonError
    }
}

/// Errors that occur while reading from and writing to stream
pub type UartResult<T> = Result<T, UartError>;
