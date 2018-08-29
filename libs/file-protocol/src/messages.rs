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

use super::storage;
use serde_cbor::{ser, Value};
use serde_cbor::error::Error;

// Create export message
pub fn export_request(
    channel_id: u32,
    hash: &str,
    target_path: &str,
    mode: u32,
) -> Result<Vec<u8>, Error> {
    info!(
        "-> {{ {}, export, {}, {}, {} }}",
        channel_id, hash, target_path, mode
    );

    Ok(ser::to_vec_packed(&(
        channel_id,
        "export",
        hash,
        target_path,
        mode,
    ))?)
}

// Create import message
pub fn import_request(channel_id: u32, source_path: &str) -> Result<Vec<u8>, Error> {
    info!("-> {{ import, {} }}", source_path);
    Ok(ser::to_vec_packed(&(channel_id, "import", source_path))?)
}

// Create sync message
pub fn metadata(hash: &str, num_chunks: u32) -> Result<Vec<u8>, Error> {
    info!("-> {{ {}, {} }}", hash, num_chunks);
    Ok(ser::to_vec_packed(&(hash, num_chunks))?)
}

// Generate ACK or NAK depending on state of received file
pub fn file_status(hash: &str, num_chunks: Option<u32>) -> Result<Vec<u8>, Error> {
    let (result, chunks) = storage::validate_file(hash, num_chunks).unwrap();

    info!("-> {{ {}, {:?}, {:?} }}", hash, result, chunks);
    let mut vec = ser::to_vec_packed(&(hash, result)).unwrap();
    // Make the array indefinite-length
    vec[0] |= 0x1F;

    for chunk in chunks.iter() {
        // Add the chunk number to the end of the CBOR array
        vec.append(&mut ser::to_vec_packed(&chunk).unwrap());
    }

    // Add the array break character
    vec.push(0xFF);
    Ok(vec)
}

// Create chunk message
pub fn chunk(hash: &str, index: u32, chunk: &[u8]) -> Result<Vec<u8>, Error> {
    let chunk_bytes = Value::Bytes(chunk.to_vec());
    info!("-> {{ {}, {}, chunk_data", hash, index);
    Ok(ser::to_vec_packed(&(hash, index, chunk_bytes))?)
}

// Create succesful import request response message
pub fn import_setup_success(
    channel_id: u64,
    hash: &str,
    num_chunks: u32,
    mode: u32,
) -> Result<Vec<u8>, Error> {
    info!(
        "-> {{ {}, true, {}, {}, {} }}",
        channel_id, hash, num_chunks, mode
    );
    Ok(ser::to_vec_packed(&(
        channel_id,
        true,
        hash,
        num_chunks,
        mode,
    ))?)
}

// Create successful export request response message
pub fn export_complete_success(channel_id: u64) -> Result<Vec<u8>, Error> {
    info!("-> {{ {}, true }}", channel_id);
    Ok(ser::to_vec_packed(&(channel_id, true))?)
}

// Create an operation failure response message
pub fn failure(channel_id: u64, error: &str) -> Result<Vec<u8>, Error> {
    info!("-> {{ {}, false, {} }}", channel_id, error);
    Ok(ser::to_vec_packed(&(channel_id, false, error))?)
}
