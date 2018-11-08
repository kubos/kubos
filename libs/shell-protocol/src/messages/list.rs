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
use std::collections::HashMap;

/// CBOR -> Message::List
pub fn from_cbor(message: &ChannelMessage) -> Result<Message, ProtocolError> {
    let mut process_list: Option<HashMap<u32, (String, u32)>> = None;

    // Parse out options
    match message.payload.get(0) {
        Some(Value::Object(raw_list)) => {
            process_list = Some(
                raw_list
                    .into_iter()
                    // Map and filter on channel and path/pid array as Some
                    .map(|(channel, data)| (channel.as_u64(), data.as_array()))
                    .filter(|(channel, data)| channel.is_some() && data.is_some())
                    // Extract path/pid
                    .map(|(channel, data)| {
                        let path = data.unwrap().get(0).and_then(|v| v.as_string());
                        let pid = data.unwrap().get(1).and_then(|v| v.as_u64());
                        (channel, path, pid)
                    })
                    // Check if path/pid are Some
                    .filter(|(_channel, path, pid)| path.is_some() && pid.is_some())
                    // Combine
                    .map(|(channel, path, pid)| {
                        (
                            channel.unwrap() as u32,
                            (path.unwrap().to_owned(), pid.unwrap() as u32),
                        )
                    })
                    .collect::<HashMap<u32, (String, u32)>>(),
            )
        }
        _ => {}
    };

    Ok(Message::List {
        channel_id: message.channel_id,
        process_list: process_list,
    })
}

/// Stdout -> CBOR
pub fn to_cbor(
    channel_id: u32,
    process_list: Option<HashMap<u32, (String, u32)>>,
) -> Result<Vec<u8>, ProtocolError> {
    info!("-> {{ {}, list, '{:?}' }}", channel_id, process_list);

    Ok(
        ser::to_vec_packed(&(channel_id, "list", process_list)).map_err(|err| {
            ProtocolError::MessageCreationError {
                message: "list".to_owned(),
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
        let mut process_list: HashMap<u32, (String, u32)> = HashMap::new();
        process_list.insert(10, ("/bin/bash".to_owned(), 99));
        process_list.insert(12, ("ls".to_owned(), 1132));

        let raw = to_cbor(channel_id, Some(process_list.to_owned())).unwrap();
        let parsed = channel_protocol::parse_message(de::from_slice(&raw).unwrap()).unwrap();
        let msg = parse_message(parsed);

        assert_eq!(
            msg.unwrap(),
            Message::List {
                channel_id: channel_id,
                process_list: Some(process_list),
            }
        );
    }

    #[test]
    fn create_parse_message_empty() {
        let channel_id = 13;

        let raw = to_cbor(channel_id, None).unwrap();
        let parsed = channel_protocol::parse_message(de::from_slice(&raw).unwrap()).unwrap();
        let msg = parse_message(parsed);

        assert_eq!(
            msg.unwrap(),
            Message::List {
                channel_id: channel_id,
                process_list: None,
            }
        );
    }
}
