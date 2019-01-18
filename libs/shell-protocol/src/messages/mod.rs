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
use crate::error::ProtocolError;
use serde_cbor::Value;
use std::collections::HashMap;

/// Messages available in shell protocol
#[derive(Debug, Eq, PartialEq)]
pub enum Message {
    /// This message is sent by the shell service when a process exits
    Exit {
        /// Channel ID of shell session
        channel_id: u32,
        /// Exit code
        code: u32,
        /// Exit signal
        signal: u32,
    },
    /// This message is sent when an error occurs within the shell protocol
    Error {
        /// Channel ID of shell session
        channel_id: u32,
        /// Error condition encountered
        message: String,
    },
    /// This message is sent to the shell service to send a kill signal to the child process
    Kill {
        /// Channel ID of shell session
        channel_id: u32,
        /// Optional signal to use. Default is SIGKILL
        signal: Option<u32>,
    },
    /// This message is used to request and respond with the lists of processes
    /// currently running under the shell service.
    List {
        /// Channel ID of shell session
        channel_id: u32,
        /// Optional list of processes. No list is sent when
        /// a request is sent.
        process_list: Option<HashMap<u32, (String, u32)>>,
    },
    /// This message is sent by the shell service after a process is spawned
    /// to indicate the process' PID
    Pid {
        /// Channel ID of shell session
        channel_id: u32,
        /// PID of remote process
        pid: u32,
    },
    /// This message is sent to the shell service to request a child process to be spawned.
    Spawn {
        /// Channel ID of shell session
        channel_id: u32,
        /// Process command to spawn
        command: String,
        /// Optional arguments to pass into command when spawning
        args: Option<Vec<String>>,
        // TODO: Add these options:
        // - pty - boolean specifying need for a pty
        // - env - list of environment variables
        // - cwd - current working directory of process
        // - uid - uid of process
        // - gid - gid of processs
        // - detached - boolean specifying if child process should be detached
    },
    /// This message is sent by the shell service when a process has produced stdout data.
    /// The shell service will send this message with no data when the stdout pipe is closed.
    Stdout {
        /// Channel ID of shell session
        channel_id: u32,
        /// Optional stdout data
        data: Option<String>,
    },
    /// This message is sent by the shell service when a process has produced stderr data.
    /// The shell service will send this message with no data when the stderr pipe is closed.
    Stderr {
        /// Channel ID of shell session
        channel_id: u32,
        /// Optional stdout data
        data: Option<String>,
    },
    /// This message is sent by the shell client with stdin for a shell process.
    /// If sent without any data the shell service will close the stdin pipe.
    Stdin {
        /// Channel ID of shell session
        channel_id: u32,
        /// Optional stdin data
        data: Option<String>,
    },
}

/// Helper functions for Message::Error
pub mod error;
/// Helper functions for Message::Exit
pub mod exit;
/// Helper functions for Message::Kill
pub mod kill;
/// Helper functions for Message::List
pub mod list;
/// Helper functions for Message::Pid
pub mod pid;
/// Helper functions for Message::Spawn
pub mod spawn;
/// Helper functions for Message::Stderr
pub mod stderr;
/// Helper functions for Message::Stdin
pub mod stdin;
/// Helper functions for Message::Stdout
pub mod stdout;

/// Parse a ChannelMessage into a ShellMessage
pub fn parse_message(message: &ChannelMessage) -> Result<Message, ProtocolError> {
    match message.name.as_ref() {
        "exit" => Ok(exit::from_cbor(&message)?),
        "error" => Ok(error::from_cbor(&message)?),
        "kill" => Ok(kill::from_cbor(&message)?),
        "list" => Ok(list::from_cbor(&message)?),
        "pid" => Ok(pid::from_cbor(&message)?),
        "spawn" => Ok(spawn::from_cbor(&message)?),
        "stderr" => Ok(stderr::from_cbor(&message)?),
        "stdin" => Ok(stdin::from_cbor(&message)?),
        "stdout" => Ok(stdout::from_cbor(&message)?),
        _ => Err(ProtocolError::MessageParseError {
            err: "No message found".to_owned(),
        }),
    }
}
