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
use crate::error::ProtocolError;
use channel_protocol::ChannelMessage;
use log::info;
use serde_cbor::ser;

/// CBOR -> Message::Exit
pub fn from_cbor(message: &ChannelMessage) -> Result<Message, ProtocolError> {
    let code = *(match message.payload.get(0) {
        Some(Value::U64(data)) => data,
        _ => {
            return Err(ProtocolError::MessageParseError {
                err: "No exit code found".to_owned(),
            });
        }
    }) as u32;

    let signal = *(match message.payload.get(1) {
        Some(Value::U64(data)) => data,
        _ => {
            return Err(ProtocolError::MessageParseError {
                err: "No exit signal found".to_owned(),
            });
        }
    }) as u32;

    Ok(Message::Exit {
        channel_id: message.channel_id,
        code,
        signal,
    })
}

/// Exit -> CBOR
pub fn to_cbor(channel_id: u32, code: u32, signal: u32) -> Result<Vec<u8>, ProtocolError> {
    info!("-> {{ {}, exit, {}, {} }}", channel_id, code, signal);

    ser::to_vec_packed(&(channel_id, "exit", code, signal)).map_err(|err| {
            ProtocolError::MessageCreationError {
                message: "exit".to_owned(),
                err,
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use channel_protocol;
    use serde_cbor::de;

    #[test]
    fn create_parse_message() {
        let channel_id = 13;
        let code = 0;
        let signal = 9;

        let raw = to_cbor(channel_id, code, signal).unwrap();
        let parsed = channel_protocol::parse_message(de::from_slice(&raw).unwrap()).unwrap();
        let msg = parse_message(&parsed);

        assert_eq!(
            msg.unwrap(),
            Message::Exit {
                channel_id,
                code,
                signal
            }
        );
    }
}
