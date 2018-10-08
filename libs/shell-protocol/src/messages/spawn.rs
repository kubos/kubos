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
use channel_protocol::ChannelMessage;
use error::ProtocolError;
use serde_cbor::{ser, ObjectKey};
use std::collections::BTreeMap;
use std::process::{Child, Command, Stdio};

/// CBOR -> Message::Spawn
pub fn from_cbor(message: &ChannelMessage) -> Result<Message, ProtocolError> {
    let mut args: Option<Vec<String>> = None;

    let command = match message.payload.get(0) {
        Some(Value::String(command)) => command,
        _ => {
            return Err(ProtocolError::MessageParseError {
                err: "No spawn command found".to_owned(),
            })
        }
    };

    // Parse out options
    match message.payload.get(1) {
        Some(Value::Object(raw_options)) => {
            // Parse out command arguments
            args = match raw_options.get(&ObjectKey::String("args".to_owned())) {
                Some(Value::Array(args)) => Some(
                    args.to_vec()
                        .iter()
                        .map(|s| s.as_string().unwrap().to_owned())
                        .collect(),
                ),
                _ => None,
            };
        }
        _ => {}
    };

    Ok(Message::Spawn {
        channel_id: message.channel_id,
        command: command.to_owned(),
        args: args,
    })
}

/// Spawn -> CBOR
pub fn to_cbor(channel_id: u32, command: &str, args: Option<&[String]>) -> Result<Vec<u8>, String> {
    info!("-> {{ {}, spawn, {} }}", channel_id, command);
    let mut options = BTreeMap::new();
    if let Some(args) = args {
        let args_vec = args
            .to_vec()
            .iter()
            .map(|s| Value::String(s.to_owned()))
            .collect();
        options.insert(ObjectKey::String("args".to_owned()), Value::Array(args_vec));
    }

    ser::to_vec_packed(&(channel_id, "spawn", command, options))
        .map_err(|_err| "Error creating spawn message".to_owned())
}

/// Perform a spawn action
pub fn run(command: &str) -> Child {
    Command::new(command)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use channel_protocol::parse_message;
    use serde_cbor::de;

    #[test]
    fn create_parse_spawn_message() {
        let channel_id = 10;
        let command = "/bin/pwd";

        let raw = to_cbor(channel_id, command, None).unwrap();
        let parsed = parse_message(de::from_slice(&raw).unwrap()).unwrap();
        let msg = from_cbor(&parsed);

        assert_eq!(
            msg.unwrap(),
            Message::Spawn {
                channel_id: channel_id,
                command: command.to_owned(),
                args: None
            }
        );
    }

    #[test]
    fn create_parse_spawn_single_arg() {
        let channel_id = 10;
        let command = "/bin/sleep";
        let args: Vec<String> = vec!["100".to_owned()];

        let raw = to_cbor(channel_id, command, Some(&args)).unwrap();
        let parsed = parse_message(de::from_slice(&raw).unwrap()).unwrap();
        let msg = from_cbor(&parsed);

        assert_eq!(
            msg.unwrap(),
            Message::Spawn {
                channel_id: channel_id,
                command: command.to_owned(),
                args: Some(args)
            }
        );
    }

    #[test]
    fn create_parse_spawn_multi_args() {
        let channel_id = 10;
        let command = "/usr/bin/echo";
        let args: Vec<String> = vec!["hello".to_owned(), "world".to_owned()];

        let raw = to_cbor(channel_id, command, Some(&args)).unwrap();
        let parsed = parse_message(de::from_slice(&raw).unwrap()).unwrap();
        let msg = from_cbor(&parsed);

        assert_eq!(
            msg.unwrap(),
            Message::Spawn {
                channel_id: channel_id,
                command: command.to_owned(),
                args: Some(args)
            }
        );
    }
}
