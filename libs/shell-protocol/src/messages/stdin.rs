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
use serde_cbor::ser;

/// CBOR -> Message::Stdin
pub fn from_cbor(message: &ChannelMessage) -> Result<Message, ProtocolError> {
    let data = match message.payload.get(0) {
        Some(Value::String(data)) => data,
        _ => {
            return Err(ProtocolError::MessageParseError {
                err: "No stdin data found".to_owned(),
            })
        }
    };

    Ok(Message::Stdin {
        channel_id: message.channel_id,
        data: data.to_owned(),
    })
}

/// Stdin -> CBOR
pub fn to_cbor(channel_id: u32, data: Option<&str>) -> Result<Vec<u8>, ProtocolError> {
    info!("-> {{ {}, stdin, '{:?}' }}", channel_id, data);

    Ok(
        ser::to_vec_packed(&(channel_id, "stdin", data)).map_err(|err| {
            ProtocolError::MessageCreationError {
                message: "stdin".to_owned(),
                err,
            }
        })?,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use channel_protocol::parse_message;
    use serde_cbor::de;

    #[test]
    fn create_parse_stdin_message() {
        let channel_id = 13;
        let data = "hello world";

        let raw = to_cbor(channel_id, Some(data)).unwrap();
        let parsed = parse_message(de::from_slice(&raw).unwrap()).unwrap();
        let msg = from_cbor(&parsed);

        assert_eq!(
            msg.unwrap(),
            Message::Stdin {
                channel_id: channel_id,
                data: data.to_owned(),
            }
        );
    }
}
