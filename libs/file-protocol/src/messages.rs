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
use serde_cbor::{error::Error, ser, Value};

// Perform local export and create approriate message from results
pub fn local_export(
    channel_id: u64,
    hash: &str,
    path: &str,
    mode: Option<u32>,
) -> Result<Vec<u8>, Error> {
    match storage::local_export(hash, path, mode) {
        Ok(results) => {
            // TODO: Results might need to be unpacked from tuple
            info!("-> {{ {}, true, {:?} }}", channel_id, results);
            Ok(ser::to_vec_packed(&(channel_id, true, results))?)
        }
        Err(error) => {
            info!("-> {{ {}, false, {} }}", channel_id, error);
            Ok(ser::to_vec_packed(&(channel_id, false, error))?)
        }
    }
}

// Perform local import and create appropriate message from results
pub fn local_import(channel_id: u64, path: &str) -> Result<Vec<u8>, Error> {
    match storage::local_import(path) {
        Ok((hash, num_chunks, mode)) => {
            // TODO: Results might need to be unpacked from tuple
            info!(
                "-> {{ {}, true, {}, {}, {} }}",
                channel_id, hash, num_chunks, mode
            );
            Ok(ser::to_vec_packed(&(
                channel_id, true, hash, num_chunks, mode,
            ))?)
        }
        Err(error) => {
            info!("-> {{ {}, false, {} }}", channel_id, error);
            Ok(ser::to_vec_packed(&(channel_id, false, error))?)
        }
    }
}

// Create export message
pub fn export(channel_id: u32, hash: &str, target_path: &str, mode: u32) -> Result<Vec<u8>, Error> {
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
pub fn import(channel_id: u32, source_path: &str) -> Result<Vec<u8>, Error> {
    info!("-> {{ import, {} }}", source_path);
    Ok(ser::to_vec_packed(&(channel_id, "import", source_path))?)
}

// Create sync message
pub fn sync(hash: &str, num_chunks: u32) -> Result<Vec<u8>, Error> {
    info!("-> {{ {}, {} }}", hash, num_chunks);
    Ok(ser::to_vec_packed(&(hash, num_chunks))?)
}

// Generate ACK or NAK depending on state of
// received file
pub fn ack_or_nak(hash: &str, num_chunks: Option<u32>) -> Result<Vec<u8>, Error> {
    let (result, chunks) = storage::local_sync(hash, num_chunks).unwrap();

    info!("-> {{ {}, {:?}, {:?} }}", hash, result, chunks);
    let mut vec = ser::to_vec_packed(&(hash, result)).unwrap();
    for chunk in chunks.iter() {
        // Add the chunk number to the end of the CBOR array
        vec.append(&mut ser::to_vec_packed(&chunk).unwrap());
        // Update length of CBOR array
        vec[0] += 1;
    }
    Ok(vec)
}

// Send an acknowledge to the remote address
fn ack(hash: &str, num_chunks: u32) -> Result<Vec<u8>, Error> {
    info!("-> {{ {}, true, {} }}", hash, num_chunks);
    Ok(ser::to_vec_packed(&(hash, true, num_chunks))?)
}

// Send a NAK to the remote address
// TODO: should include missing chunks
fn nak(hash: &str) -> Result<Vec<u8>, Error> {
    info!("-> {{ {}, false}}", hash);
    Ok(ser::to_vec_packed(&(hash, false))?)
}

// Create chunk message
pub fn chunk(hash: &str, index: u32, chunk: &[u8]) -> Result<Vec<u8>, Error> {
    let chunk_bytes = Value::Bytes(chunk.to_vec());
    info!("-> {{ {}, {}, chunk_data", hash, index );
    Ok(ser::to_vec_packed(&(hash, index, chunk_bytes))?)
}
