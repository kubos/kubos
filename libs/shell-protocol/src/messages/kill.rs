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

/// CBOR -> Message::Kill
pub fn from_cbor(message: &ChannelMessage) -> Result<Message, ProtocolError> {
    let signal: Option<u32> = message
        .payload
        .get(0)
        .and_then(|v| v.as_u64())
        .and_then(|n| Some(n as u32));

    Ok(Message::Kill {
        channel_id: message.channel_id,
        signal,
    })
}

/// Kill -> CBOR
pub fn to_cbor(channel_id: u32, signal: Option<u32>) -> Result<Vec<u8>, ProtocolError> {
    info!("-> {{ {}, kill, {:?} }}", channel_id, signal);

    Ok(
        ser::to_vec_packed(&(channel_id, "kill", signal)).map_err(|err| {
            ProtocolError::MessageCreationError {
                message: "kill".to_owned(),
                err,
            }
        })?,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use channel_protocol;
    use serde_cbor::de;

    #[test]
    fn create_parse_message() {
        let channel_id = 13;
        let signal = 9;

        let raw = to_cbor(channel_id, Some(signal)).unwrap();
        let parsed = channel_protocol::parse_message(de::from_slice(&raw).unwrap()).unwrap();
        let msg = parse_message(&parsed);

        assert_eq!(
            msg.unwrap(),
            Message::Kill {
                channel_id: channel_id,
                signal: Some(signal)
            }
        );
    }

    #[test]
    fn create_parse_message_no_signal() {
        let channel_id = 22;

        let raw = to_cbor(channel_id, None).unwrap();
        let parsed = channel_protocol::parse_message(de::from_slice(&raw).unwrap()).unwrap();
        let msg = parse_message(&parsed);

        assert_eq!(
            msg.unwrap(),
            Message::Kill {
                channel_id: channel_id,
                signal: None
            }
        );
    }
}
