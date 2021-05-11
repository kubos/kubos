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
use log::info;
use serde_cbor::{ser, Value};

// Create export message
pub fn export_request(
    channel_id: u32,
    hash: &str,
    target_path: &str,
    mode: u32,
) -> Result<Vec<u8>, ProtocolError> {
    info!(
        "-> {{ {}, export, {}, {}, {} }}",
        channel_id, hash, target_path, mode
    );

    ser::to_vec_packed(&(channel_id, "export", hash, target_path, mode)).map_err(|err| {
        ProtocolError::MessageCreationError {
            message: "export".to_owned(),
            err,
        }
    })
}

// Create import message
pub fn import_request(channel_id: u32, source_path: &str) -> Result<Vec<u8>, ProtocolError> {
    info!("-> {{ import, {} }}", source_path);
    ser::to_vec_packed(&(channel_id, "import", source_path)).map_err(|err| {
        ProtocolError::MessageCreationError {
            message: "import".to_owned(),
            err,
        }
    })
}

// Create sync message
pub fn metadata(channel_id: u32, hash: &str, num_chunks: u32) -> Result<Vec<u8>, ProtocolError> {
    info!("-> {{ {}, {}, {} }}", channel_id, hash, num_chunks);
    ser::to_vec_packed(&(channel_id, hash, num_chunks)).map_err(|err| {
        ProtocolError::MessageCreationError {
            message: "metadata".to_owned(),
            err,
        }
    })
}

// Send an acknowledge to the remote address
pub fn ack(channel_id: u32, hash: &str, num_chunks: Option<u32>) -> Result<Vec<u8>, ProtocolError> {
    info!("-> {{ {}, {}, true, {:?} }}", channel_id, hash, num_chunks);
    ser::to_vec_packed(&(channel_id, hash, true, num_chunks)).map_err(|err| {
        ProtocolError::MessageCreationError {
            message: "ACK".to_owned(),
            err,
        }
    })
}

// Sends a nak with ranges of missing chunks
pub fn nak(channel_id: u32, hash: &str, missing_chunks: &[u32]) -> Result<Vec<u8>, ProtocolError> {
    let chunks = if missing_chunks.len() > 20 {
        &missing_chunks[0..20]
    } else {
        &missing_chunks
    };

    info!("-> {{ {}, {}, false, {:?} }}", channel_id, hash, chunks);
    let mut vec = ser::to_vec_packed(&(channel_id, hash, false)).map_err(|err| {
        ProtocolError::MessageCreationError {
            message: "NAK".to_owned(),
            err,
        }
    })?;

    // Make the array indefinite-length
    vec[0] |= 0x1F;

    for chunk in chunks.iter() {
        // Add the chunk number to the end of the CBOR array
        vec.append(&mut ser::to_vec_packed(&chunk).map_err(|err| {
            ProtocolError::MessageCreationError {
                message: "NAK".to_owned(),
                err,
            }
        })?);
    }

    // Add the array break character
    vec.push(0xFF);
    Ok(vec)
}

// Create chunk message
pub fn chunk(
    channel_id: u32,
    hash: &str,
    index: u32,
    chunk: &[u8],
) -> Result<Vec<u8>, ProtocolError> {
    let chunk_bytes = Value::Bytes(chunk.to_vec());
    info!("-> {{ {}, {}, {}, chunk_data }}", channel_id, hash, index);
    ser::to_vec_packed(&(channel_id, hash, index, chunk_bytes)).map_err(|err| {
        ProtocolError::MessageCreationError {
            message: "chunk".to_owned(),
            err,
        }
    })
}

// Create succesful import request response message
pub fn import_setup_success(
    channel_id: u32,
    hash: &str,
    num_chunks: u32,
    mode: u32,
) -> Result<Vec<u8>, ProtocolError> {
    info!(
        "-> {{ {}, true, {}, {}, {} }}",
        channel_id, hash, num_chunks, mode
    );

    ser::to_vec_packed(&(channel_id, true, hash, num_chunks, mode)).map_err(|err| {
        ProtocolError::MessageCreationError {
            message: "import success".to_owned(),
            err,
        }
    })
}

// Create successful export request response message
pub fn operation_success(channel_id: u32, hash: &str) -> Result<Vec<u8>, ProtocolError> {
    info!("-> {{ {}, true, {} }}", channel_id, hash);
    ser::to_vec_packed(&(channel_id, true, hash)).map_err(|err| {
        ProtocolError::MessageCreationError {
            message: "operation success".to_owned(),
            err,
        }
    })
}

// Create an operation failure response message
pub fn operation_failure(channel_id: u32, error: &str) -> Result<Vec<u8>, ProtocolError> {
    info!("-> {{ {}, false, {} }}", channel_id, error);
    ser::to_vec_packed(&(channel_id, false, error)).map_err(|err| {
        ProtocolError::MessageCreationError {
            message: "operation failure".to_owned(),
            err,
        }
    })
}

// Create sync message
#[allow(dead_code)]
pub fn sync(channel_id: u32, hash: &str) -> Result<Vec<u8>, ProtocolError> {
    info!("-> {{ {}, {} }}", channel_id, hash);
    ser::to_vec_packed(&(channel_id, hash)).map_err(|err| ProtocolError::MessageCreationError {
        message: "sync".to_owned(),
        err,
    })
}

pub fn cleanup(channel_id: u32, hash: Option<String>) -> Result<Vec<u8>, ProtocolError> {
    info!("-> {{ {}, cleanup, {:?}, }}", channel_id, hash);
    ser::to_vec_packed(&(channel_id, "cleanup", hash)).map_err(|err| {
        ProtocolError::MessageCreationError {
            message: "cleanup".to_owned(),
            err,
        }
    })
}
