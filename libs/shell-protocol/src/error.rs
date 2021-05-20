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
use std::io;

/// Errors which occur when using ShellProtocol
#[derive(Debug, Fail)]
pub enum ProtocolError {
    /// An error was raised by the cbor protocol
    #[fail(display = "Cbor Error: {}", err)]
    CborError {
        /// The specific CBOR protocol error
        err: cbor_protocol::ProtocolError,
    },
    /// An error was encountered in the channel protocol
    #[fail(display = "Channel protocol error: {}", err)]
    ChannelError {
        /// The specific channel protocol error
        err: channel_protocol::ProtocolError,
    },
    /// An error was encountered when killing a process
    #[fail(display = "Kill error: {}", err)]
    KillError {
        /// Underlying error encountered
        err: nix::Error,
    },
    /// An error was encountered when creating a message
    #[fail(display = "Unable to create message {}: {}", message, err)]
    MessageCreationError {
        /// Message which was being created
        message: String,
        /// Underlying serde_cbor error
        err: serde_cbor::error::Error,
    },
    /// A general error was encountered when parsing a message
    #[fail(display = "Unable to parse message: {}", err)]
    MessageParseError {
        /// Underlying error encountered
        err: String,
    },
    /// A general error was raised by the process
    #[fail(display = "Process error when {}: {}", action, err)]
    ProcesssError {
        /// Action which caused error
        action: String,
        /// Underlying error
        err: io::Error,
    },
    /// A timeout occurred when receiving data
    #[fail(display = "A receive timeout was encountered")]
    ReceiveTimeout,
    /// An error was encountered when receiving a message
    #[fail(display = "Failure receiving message: {}", err)]
    ReceiveError {
        /// Underlying error encountered
        err: String,
    },
    /// An error was encountered when spawning a process
    #[fail(display = "Error spawning command {}: {}", cmd, err)]
    SpawnError {
        /// Command spawned
        cmd: String,
        /// Underlying error
        err: io::Error,
    },
    /// A timeout was encountered when reading data
    #[fail(display = "Timeout was encountered when reading data")]
    ReadTimeout,
}

impl From<cbor_protocol::ProtocolError> for ProtocolError {
    fn from(error: cbor_protocol::ProtocolError) -> Self {
        match error {
            cbor_protocol::ProtocolError::Timeout => ProtocolError::ReceiveTimeout,
            err => ProtocolError::CborError { err },
        }
    }
}

impl From<channel_protocol::ProtocolError> for ProtocolError {
    fn from(error: channel_protocol::ProtocolError) -> Self {
        match error {
            channel_protocol::ProtocolError::ReceiveTimeout => ProtocolError::ReceiveTimeout,
            err => ProtocolError::ChannelError { err },
        }
    }
}
