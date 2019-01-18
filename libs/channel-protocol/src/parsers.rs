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

use crate::error::ProtocolError;
use crate::protocol::Message;
use serde_cbor::Value;

/// Parse out just the channel ID from a message
pub fn parse_channel_id(message: &Value) -> Result<u32, ProtocolError> {
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
        })?
        .to_owned();

    if let Value::U64(channel_id) = first_param {
        Ok(channel_id as u32)
    } else {
        Err(ProtocolError::MessageParseError {
            err: "No channel ID found".to_owned(),
        })
    }
}

/// Parses raw cbor message into ChannelMessage
pub fn parse_message(message: Value) -> Result<Message, ProtocolError> {
    let data = match message {
        Value::Array(val) => val.to_owned(),
        _ => {
            return Err(ProtocolError::MessageParseError {
                err: "Data not an array".to_owned(),
            })
        }
    };
    let channel_id = *(match data.get(0) {
        Some(Value::U64(channel_id)) => channel_id,
        _ => {
            return Err(ProtocolError::MessageParseError {
                err: "No channel ID found".to_owned(),
            })
        }
    }) as u32;
    let name = match data.get(1) {
        Some(Value::String(name)) => name,
        _ => {
            return Err(ProtocolError::MessageParseError {
                err: "No message name found".to_owned(),
            })
        }
    }
    .to_owned();
    Ok(Message {
        channel_id,
        name,
        payload: data[2..].to_owned(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_cbor::{de, ser};

    #[test]
    fn test_parse_channel_id() {
        let raw = ser::to_vec_packed(&(1, 2, 3, 4, 5)).unwrap();
        let channel_id = parse_channel_id(&de::from_slice(&raw).unwrap()).unwrap();;

        assert_eq!(channel_id, 1);
    }

    #[test]
    fn test_parse_channel_message() {
        let raw = ser::to_vec_packed(&(10, "test", "data", 12)).unwrap();
        let message = parse_message(de::from_slice(&raw).unwrap()).unwrap();

        assert_eq!(message.channel_id, 10);
        assert_eq!(message.name, "test".to_owned());
        assert_eq!(message.payload.len(), 2);
        assert_eq!(
            message.payload.get(0),
            Some(&Value::String("data".to_owned()))
        );
        assert_eq!(message.payload.get(1), Some(&Value::U64(12)));
    }
}
