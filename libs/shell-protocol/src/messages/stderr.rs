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

/// CBOR -> Message::Stderr
pub fn from_cbor(message: &ChannelMessage) -> Result<Message, ProtocolError> {
    if let Some(Value::String(data)) = message.payload.get(0) {
        Ok(Message::Stderr {
            channel_id: message.channel_id,
            data: Some(data.to_owned()),
        })
    } else {
        Ok(Message::Stderr {
            channel_id: message.channel_id,
            data: None,
        })
    }
}

/// Stderr -> CBOR
pub fn to_cbor(channel_id: u32, data: Option<&str>) -> Result<Vec<u8>, ProtocolError> {
    info!("-> {{ {}, stderr, '{:?}' }}", channel_id, data);

    ser::to_vec_packed(&(channel_id, "stderr", data)).map_err(|err| {
        ProtocolError::MessageCreationError {
            message: "stderr".to_owned(),
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
        let data = "hello world";

        let raw = to_cbor(channel_id, Some(data)).unwrap();
        let parsed = channel_protocol::parse_message(de::from_slice(&raw).unwrap()).unwrap();
        let msg = parse_message(&parsed);

        assert_eq!(
            msg.unwrap(),
            Message::Stderr {
                channel_id,
                data: Some(data.to_owned()),
            }
        );
    }

    #[test]
    fn create_parse_message_empty() {
        let channel_id = 13;

        let raw = to_cbor(channel_id, None).unwrap();
        let parsed = channel_protocol::parse_message(de::from_slice(&raw).unwrap()).unwrap();
        let msg = parse_message(&parsed);

        assert_eq!(
            msg.unwrap(),
            Message::Stderr {
                channel_id,
                data: None,
            }
        );
    }
}
