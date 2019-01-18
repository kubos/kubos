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

#![deny(missing_docs)]
#![deny(warnings)]

//! Kubos Shell Protocol
//!
//! This crate contains structures and functions used by the Kubos shell service
//! and shell client to send and receive messages using the shell protocol.

/// Shell protocol errors
pub mod error;

/// Shell protocol messages
pub mod messages;

mod process;
mod protocol;

pub use crate::error::ProtocolError;
pub use crate::messages::parse_message;
pub use crate::messages::Message as ShellMessage;
pub use crate::process::ProcessHandler;
pub use crate::protocol::Protocol as ShellProtocol;

/// Default chunk size used by shell protocol
pub const CHUNK_SIZE: u32 = 4096;

/// Default port used by shell protocol
pub const PORT: &str = "8080";
