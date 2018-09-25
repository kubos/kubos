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

use super::Message;
use super::ProtocolError;
use serde_cbor::Value;
use std::slice::Iter;

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

pub fn parse_message(message: Value) -> Result<Message, ProtocolError> {
    let raw = match message {
        Value::Array(val) => val.to_owned(),
        _ => {
            return Err(ProtocolError::MessageParseError {
                err: "Data not an array".to_owned(),
            })
        }
    };

    let mut pieces = raw.iter();

    let channel_param: Value = pieces
        .next()
        .ok_or(ProtocolError::MessageParseError {
            err: "No contents".to_owned(),
        })?
        .to_owned();

    if let Value::U64(channel) = channel_param {
        let channel_id = channel as u32;
        if let Some(msg) = parse_export_request(channel_id, pieces.to_owned())? {
            return Ok(msg);
        }
        if let Some(msg) = parse_import_request(channel_id, pieces.to_owned())? {
            return Ok(msg);
        }
        if let Some(msg) = parse_success_receive(channel_id, pieces.to_owned())? {
            return Ok(msg);
        }
        if let Some(msg) = parse_success_transmit(channel_id, pieces.to_owned())? {
            return Ok(msg);
        }
        if let Some(msg) = parse_bad_op(channel_id, pieces.to_owned())? {
            return Ok(msg);
        }
        if let Some(msg) = parse_ack(channel_id, pieces.to_owned())? {
            return Ok(msg);
        }
        if let Some(msg) = parse_nak(channel_id, pieces.to_owned())? {
            return Ok(msg);
        }
        if let Some(msg) = parse_chunk(channel_id, pieces.to_owned())? {
            return Ok(msg);
        }
        if let Some(msg) = parse_sync(channel_id, pieces.to_owned())? {
            return Ok(msg);
        }
    }

    return Err(ProtocolError::MessageParseError {
        err: "No message found".to_owned(),
    });
}

// Parse out export request
// { channel_id, "export", hash, path, [, mode] }
pub fn parse_export_request(
    channel_id: u32,
    mut pieces: Iter<Value>,
) -> Result<Option<Message>, ProtocolError> {
    if let Some(Value::String(op)) = pieces.next() {
        if op == "export" {
            let hash = match pieces.next().ok_or(ProtocolError::MissingParam(
                "export".to_owned(),
                "hash".to_owned(),
            ))? {
                Value::String(val) => val,
                _ => {
                    return Err(ProtocolError::InvalidParam(
                        "export".to_owned(),
                        "hash".to_owned(),
                    ))
                }
            };

            let path = match pieces.next().ok_or(ProtocolError::MissingParam(
                "export".to_owned(),
                "path".to_owned(),
            ))? {
                Value::String(val) => val,
                _ => {
                    return Err(ProtocolError::InvalidParam(
                        "export".to_owned(),
                        "path".to_owned(),
                    ))
                }
            };

            let mode = match pieces.next() {
                Some(Value::U64(num)) => Some(*num as u32),
                _ => None,
            };

            return Ok(Some(Message::ReqReceive(
                channel_id,
                hash.to_owned(),
                path.to_owned(),
                mode,
            )));
        }
    }

    return Ok(None);
}

// Parse out import request
// { channel_id, "import", path }
pub fn parse_import_request(
    channel_id: u32,
    mut pieces: Iter<Value>,
) -> Result<Option<Message>, ProtocolError> {
    if let Some(Value::String(op)) = pieces.next() {
        if op == "import" {
            let path = match pieces.next().ok_or(ProtocolError::MissingParam(
                "export".to_owned(),
                "hash".to_owned(),
            ))? {
                Value::String(val) => val,
                _ => {
                    return Err(ProtocolError::InvalidParam(
                        "export".to_owned(),
                        "hash".to_owned(),
                    ))
                }
            };
            return Ok(Some(Message::ReqTransmit(
                channel_id as u32,
                path.to_owned(),
            )));
        }
    }

    return Ok(None);
}

// Parse out success received message
// { channel_id, true }
pub fn parse_success_receive(
    channel_id: u32,
    mut pieces: Iter<Value>,
) -> Result<Option<Message>, ProtocolError> {
    if let Some(Value::Bool(true)) = pieces.next() {
        // Good - { channel_id, true, ...values }
        if let None = pieces.next() {
            return Ok(Some(Message::SuccessReceive(channel_id)));
        }
    }

    return Ok(None);
}

// Parse out success transmit message
// { channel_id, "true", ..values }
pub fn parse_success_transmit(
    channel_id: u32,
    mut pieces: Iter<Value>,
) -> Result<Option<Message>, ProtocolError> {
    if let Some(Value::Bool(true)) = pieces.next() {
        // Good - { channel_id, true, ...values }
        if let Some(piece) = pieces.next() {
            // It's a good result after an 'import' operation
            let hash = match piece {
                Value::String(val) => val,
                _ => {
                    return Err(ProtocolError::InvalidParam(
                        "success".to_owned(),
                        "hash".to_owned(),
                    ))
                }
            };

            let num_chunks = match pieces.next().ok_or(ProtocolError::MissingParam(
                "success".to_owned(),
                "num chunks".to_owned(),
            ))? {
                Value::U64(val) => *val,
                _ => {
                    return Err(ProtocolError::InvalidParam(
                        "success".to_owned(),
                        "num chunks".to_owned(),
                    ))
                }
            };

            let mode = match pieces.next() {
                Some(Value::U64(val)) => Some(*val as u32),
                _ => None,
            };

            // Return the file info
            return Ok(Some(Message::SuccessTransmit(
                channel_id,
                hash.to_string(),
                num_chunks as u32,
                mode,
            )));
        }
    }

    return Ok(None);
}

// Parse out bad
// { channel_id, "false", ..values }
pub fn parse_bad_op(
    channel_id: u32,
    mut pieces: Iter<Value>,
) -> Result<Option<Message>, ProtocolError> {
    if let Some(Value::Bool(false)) = pieces.next() {
        let error = match pieces.next().ok_or(ProtocolError::MissingParam(
            "failure".to_owned(),
            "error".to_owned(),
        ))? {
            Value::String(val) => val,
            _ => {
                return Err(ProtocolError::InvalidParam(
                    "failure".to_owned(),
                    "error".to_owned(),
                ))
            }
        };

        return Ok(Some(Message::Failure(channel_id, error.to_owned())));
    }

    return Ok(None);
}

// Parse out ack
// { hash, true, num_chunks }
pub fn parse_ack(
    channel_id: u32,
    mut pieces: Iter<Value>,
) -> Result<Option<Message>, ProtocolError> {
    if let Some(Value::String(hash)) = pieces.next() {
        if let Some(Value::Bool(true)) = pieces.next() {
            // It's an ACK: { hash, true, num_chunks }
            // Our data transfer (export) completed successfully
            // self.stop_push(&hash)?;

            //TODO: Do something with the third param? (num_chunks)
            // Doesn't look like we do anything with num_chunks
            return Ok(Some(Message::ACK(channel_id, hash.to_owned())));
        }
    }

    return Ok(None);
}

// Parse out nak
// { hash, false, ..missing_chunks }
pub fn parse_nak(
    channel_id: u32,
    mut pieces: Iter<Value>,
) -> Result<Option<Message>, ProtocolError> {
    if let Some(Value::String(hash)) = pieces.next() {
        if let Some(Value::Bool(false)) = pieces.next() {
            let mut remaining_chunks: Vec<(u32, u32)> = vec![];
            let mut chunk_nums: Vec<u32> = vec![];
            for entry in pieces {
                if let Value::U64(chunk_num) = entry {
                    chunk_nums.push(*chunk_num as u32);
                }
            }

            for chunk in chunk_nums.chunks(2) {
                let first = chunk[0];
                let last = chunk[1];
                remaining_chunks.push((first, last));
            }

            return Ok(Some(Message::NAK(
                channel_id,
                hash.to_owned(),
                Some(remaining_chunks),
            )));
        }
    }

    return Ok(None);
}

// Parse out chunk
// { hash, chunk_index, data }
pub fn parse_chunk(
    channel_id: u32,
    mut pieces: Iter<Value>,
) -> Result<Option<Message>, ProtocolError> {
    if let Some(Value::String(hash)) = pieces.next() {
        if let Some(Value::U64(num)) = pieces.next() {
            if let Some(third_param) = pieces.next() {
                if let Value::Bytes(data) = third_param {
                    return Ok(Some(Message::ReceiveChunk(
                        channel_id,
                        hash.to_owned(),
                        *num as u32,
                        data.to_vec(),
                    )));
                } else {
                    return Err(ProtocolError::InvalidParam(
                        "chunk".to_owned(),
                        "chunk data".to_owned(),
                    ));
                }
            }
        }
    }

    return Ok(None);
}

// Parse out sync
// { hash, num_chunks }
// or
// { hash }
pub fn parse_sync(
    channel_id: u32,
    mut pieces: Iter<Value>,
) -> Result<Option<Message>, ProtocolError> {
    if let Some(Value::String(hash)) = pieces.next() {
        if let Some(second_param) = pieces.next() {
            if let Value::U64(num) = second_param {
                if let None = pieces.next() {
                    // It's a sync message: { hash, num_chunks }
                    return Ok(Some(Message::Metadata(
                        channel_id,
                        hash.to_owned(),
                        *num as u32,
                    )));
                }
            }
        } else {
            // It's a sync message: { hash }
            return Ok(Some(Message::Sync(channel_id, hash.to_owned())));
        }
    }

    return Ok(None);
}
