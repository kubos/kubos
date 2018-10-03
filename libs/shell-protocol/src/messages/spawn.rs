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
use error::ProtocolError;
use serde_cbor::{ser, Value};
use std::process::{Child, Command, Stdio};

/// cbor -> spawn
pub fn from_cbor(message: Value) -> Result<Option<Message>, ProtocolError> {
    let data = match message {
        Value::Array(val) => val.to_owned(),
        _ => {
            return Err(ProtocolError::MessageParseError {
                err: "Data not an array".to_owned(),
            })
        }
    };
    let mut pieces = data.iter();
    let first_param: Value = pieces
        .next()
        .ok_or(ProtocolError::MessageParseError {
            err: "No contents".to_owned(),
        })?.to_owned();
    if let Value::U64(channel_id) = first_param {
        let channel_id = channel_id as u32;
        if let Some(Value::String(_message)) = pieces.next() {
            if let Some(Value::String(command)) = pieces.next() {
                return Ok(Some(Message::Spawn(channel_id, command.to_owned())));
            }
        }
    }
    Ok(None)
}

/// spawn -> cbor
pub fn to_cbor(channel_id: u32, command: &str) -> Result<Vec<u8>, String> {
    info!("-> {{ {}, spawn, {} }}", channel_id, command);
    ser::to_vec_packed(&(channel_id, "spawn", command))
        .map_err(|_err| "Error creating spawn message".to_owned())
}

/// do a spawn
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
    use serde_cbor::de;

    #[test]
    fn create_parse_spawn_message() {
        let channel_id = 10;
        let command = "/bin/pwd";

        let raw = to_cbor(channel_id, command).unwrap();
        let msg = from_cbor(de::from_slice(&raw).unwrap());

        assert_eq!(msg.unwrap(), Message::Spawn(channel_id, command.to_owned()));
    }
}
