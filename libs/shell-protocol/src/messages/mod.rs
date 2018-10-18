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
    Exit {
        channel_id: u32,
        code: u32,
        signal: u32,
    },
    Pid {
        channel_id: u32,
        pid: u32,
    },
    /// This message is sent to the shell service to request a child process to be spawned.
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
    /// This message is sent from the shell service when a process has produced data via stdout.
    Stdout {
        channel_id: u32,
        data: Option<String>,
    },
    Stderr {
        channel_id: u32,
        data: Option<String>,
    },
    Stdin {
        channel_id: u32,
        data: String,
    },
}

pub mod exit;
pub mod pid;
pub mod spawn;
pub mod stderr;
pub mod stdin;
pub mod stdout;

pub fn parse_message(message: ChannelMessage) -> Result<Message, ProtocolError> {
    if message.name == "exit" {
        Ok(exit::from_cbor(&message)?)
    } else if message.name == "pid" {
        Ok(pid::from_cbor(&message)?)
    } else if message.name == "spawn" {
        Ok(spawn::from_cbor(&message)?)
    } else if message.name == "stderr" {
        Ok(stderr::from_cbor(&message)?)
    } else if message.name == "stdin" {
        Ok(stdin::from_cbor(&message)?)
    } else if message.name == "stdout" {
        Ok(stdout::from_cbor(&message)?)
    } else {
        Err(ProtocolError::MessageParseError {
            err: "No message found".to_owned(),
        })
    }
}
