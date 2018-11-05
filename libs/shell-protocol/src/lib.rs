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

extern crate cbor_protocol;
extern crate channel_protocol;
#[macro_use]
extern crate failure;
extern crate libc;
#[macro_use]
extern crate log;
extern crate nix;
extern crate rand;
extern crate serde_cbor;
extern crate timeout_readwrite;

pub mod error;
pub mod messages;
mod process;
mod protocol;

pub use error::ProtocolError;
pub use messages::parse_message;
pub use messages::Message as ShellMessage;
pub use process::ProcessHandler;
pub use protocol::Protocol as ShellProtocol;

/// Default chunk size used by shell protocol
pub const CHUNK_SIZE: u32 = 4096;

/// Default port used by shell protocol
pub const PORT: &str = "6000";
