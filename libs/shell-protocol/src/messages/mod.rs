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

use channel_protocol::ChannelMessage;
use error::ProtocolError;
use serde_cbor::Value;

#[derive(Debug, Eq, PartialEq)]
pub enum Message {
    Spawn {
        channel_id: u32,
        command: String,
        args: Option<Vec<String>>,
        // TODO: Add these options:
        // - pty - boolean specifying need for a pty
        // - env - list of environment variables
        // - cwd - current working directory of process
        // - uid - uid of process
        // - gid - gid of processs
        // - detached - boolean specifying if child process should be detached
    },
}

pub mod spawn;

pub fn parse_message(message: ChannelMessage) -> Result<Message, ProtocolError> {
    if message.name == "spawn" {
        Ok(spawn::from_cbor(&message)?)
    } else {
        Err(ProtocolError::MessageParseError {
            err: "No message found".to_owned(),
        })
    }
}
