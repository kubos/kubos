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

use super::CHUNK_SIZE;

use blake2_rfc::blake2s::Blake2s;
use serde_cbor::{de, to_vec, Value};
use std::fs;
use std::fs::File;
use std::fs::Permissions;
use std::io::{Read, Write};
use std::os::unix::fs::MetadataExt;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::str;
use time;

const HASH_SIZE: usize = 16;

// Save new chunk in a temporary storage file
pub fn store_chunk(hash: &str, index: u32, data: &[u8]) -> Result<(), String> {
    // if data is type uint8_t[]
    // change data to ffi.string
    let file_name = format!("{}", index);
    let storage_path = Path::new("storage").join(hash).join(file_name);

    fs::create_dir_all(&storage_path.parent().unwrap()).unwrap();
    let mut file = File::create(&storage_path).unwrap();
    file.write_all(data).unwrap();

    Ok(())
}

pub fn store_meta(hash: &str, num_chunks: u32) -> Result<(), String> {
    let data = vec![("num_chunks", num_chunks)];

    // let mut e = Encoder::from_memory();
    // e.encode(&data).unwrap();
    let vec = to_vec(&data).unwrap();

    let file_dir = Path::new("storage").join(hash);
    // Make sure the directory exists
    fs::create_dir_all(file_dir.clone())
        .map_err(|err| format!("Failed to create temp storage directory: {:?}", err))?;

    let meta_path = file_dir.join("meta");
    let temp_path = file_dir.join(".meta.tmp");

    File::create(&temp_path).unwrap().write_all(&vec).unwrap();

    fs::rename(temp_path, meta_path).unwrap();

    Ok(())
}

// Load a chunk from its temporary storage file
pub fn load_chunk(hash: &str, index: u32) -> Result<Vec<u8>, String> {
    let mut data = vec![];
    let path = Path::new("storage").join(hash).join(format!("{}", index));

    File::open(path).unwrap().read_to_end(&mut data).unwrap();
    Ok(data)
}

// Load number of chunks in file from metadata
pub fn load_meta(hash: &str) -> Result<u32, String> {
    let mut data = vec![];
    let meta_path = Path::new("storage").join(hash).join("meta");
    File::open(meta_path)
        .unwrap()
        .read_to_end(&mut data)
        .unwrap();

    let metadata: Value = de::from_slice(&data).unwrap();

    // Returned data should be CBOR: '[["num_chunks", value]]'
    let num_chunks = metadata
        .as_array()
        .and_then(|data| data[0].as_array())
        .and_then(|data| {
            let mut entries = data.iter();

            entries
                .next()
                .and_then(|val| val.as_string())
                .and_then(|key| {
                    if key == "num_chunks" {
                        entries.next().and_then(|val| val.as_u64())
                    } else {
                        None
                    }
                })
        })
        .ok_or("Failed to parse temporary file's metadata".to_owned())?;

    Ok(num_chunks as u32)
}

// Check if all of a files chunks are present in the temporary directory
pub fn local_sync(hash: &str, num_chunks: Option<u32>) -> Result<(bool, Vec<u32>), String> {
    let num_chunks = if let Some(num) = num_chunks {
        store_meta(hash, num).unwrap();
        num
    } else {
        load_meta(hash)?
    };

    let mut missing_ranges: Vec<u32> = vec![];

    let hash_path = Path::new("storage").join(hash);

    let mut prev_entry: i32 = -1;

    let entries = fs::read_dir(hash_path.clone())
        .map_err(|err| format!("Failed to read {:?} directory: {}", hash_path, err))?;

    let mut converted_entries: Vec<i32> = entries
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| {
            match entry
                .file_name()
                .into_string()
                .map_err(|err| format!("Failed to parse file name: {:?}", err))
                .and_then(|val| {
                    val.parse::<i32>()
                        .map_err(|err| format!("Failed to parse chunk number: {:?}", err))
                }) {
                Ok(num) => Some(num),
                _ => None,
            }
        })
        .collect();

    converted_entries.sort();

    for &entry_num in converted_entries.iter() {
        //println!("checking {} vs {}", entry_num, prev_entry);
        // Check for non-sequential dir entries to detect missing chunk ranges
        if entry_num - prev_entry > 1 {
            println!("Found missing chunk");
            // Add start of range (inclusive)
            missing_ranges.push((prev_entry + 1) as u32);
            // Add end of range (non-inclusive)
            missing_ranges.push(entry_num as u32);
        }

        prev_entry = entry_num;
    }

    // Check for a trailing range
    // Ex. Last known chunk is 5, but there are 10 chunks.
    //     We will already have added '6', so we need to add '10'
    //     to close it out.
    if (num_chunks as i32) - prev_entry != 1 {
        println!("Detected missing range");
        // Add start of range
        missing_ranges.push((prev_entry + 1) as u32);
        // Add end of range
        missing_ranges.push(num_chunks as u32);
    }

    Ok((missing_ranges.is_empty(), missing_ranges))
}

/// Create temporary folder for chunks
/// Stream copy file from mutable space to immutable space
/// Move folder to hash of contents
pub fn local_import(source_path: &str) -> Result<(String, u32, u32), String> {
    let storage_path = String::from("storage");

    if let Err(e) = fs::metadata(source_path) {
        return Err(format!("failed to stat file {}: {:?}", source_path, e));
    }

    // Copy input file to storage area and calculate hash
    if let Err(e) = fs::create_dir_all(&storage_path) {
        return Err(format!("failed to create dir {}: {:?}", storage_path, e));
    }

    let temp_path = Path::new(&storage_path).join(format!(".{}", time::get_time().nsec));
    let mut hasher = Blake2s::new(HASH_SIZE);
    {
        let mut input = File::open(&source_path).unwrap();
        let mut output = File::create(&temp_path).unwrap();

        // Need to bring in blake2fs here to create hash
        loop {
            let mut chunk = vec![0u8; CHUNK_SIZE];
            match input.read(&mut chunk) {
                Ok(n) => {
                    if n == 0 {
                        output.sync_all().unwrap();
                        break;
                    }
                    hasher.update(&chunk[0..n]);
                    if let Err(e) = output.write(&chunk[0..n]) {
                        return Err(format!("failed to write chunk {:?}", e));
                    }
                }
                Err(e) => return Err(format!("failed to read chunk from source {:?}", e)),
            }
        }
    }
    let hash_result = hasher.finalize();
    let mut hash = String::from("");
    for c in hash_result.as_bytes().iter() {
        hash = format!("{}{:02x}", hash, c);
    }

    let mut output = match File::open(&temp_path) {
        Ok(f) => f,
        Err(e) => {
            return Err(format!(
                "failed to open temp file {:?} : {:?}",
                temp_path, e
            ))
        }
    };

    let mut index = 0;
    let mut offset = 0;

    loop {
        let mut chunk = vec![0u8; CHUNK_SIZE];
        match output.read(&mut chunk) {
            Ok(n) => {
                if n == 0 {
                    break;
                }
                store_chunk(&hash, index, &chunk[0..n]).unwrap();
                index = index + 1;
                offset = offset + n;
            }
            Err(e) => {
                return Err(format!(
                    "failed to read chunk from temp {:?}: {:?}",
                    temp_path, e
                ))
            }
        }
    }
    store_meta(&hash, index).unwrap();

    let meta = fs::metadata(source_path).unwrap();

    Ok((hash, index, meta.mode()))
}

// Copy temporary data chunks into permanent file?
pub fn local_export(hash: &str, target_path: &str, mode: Option<u32>) -> Result<(), String> {
    // Double check that all the chunks of the file are present and the hash matches up
    let (result, _) = local_sync(hash, None)?;

    if result != true {
        return Err("File missing chunks".to_owned());
    }

    // Get the total number of chunks we're saving
    let num_chunks = load_meta(hash)?;

    // Q: Do we want to create the parent directories if they don't exist?
    let mut file = File::create(target_path)
        .map_err(|err| format!("Failed to create/open file for writing: {}", err))?;

    if let Some(mode_val) = mode {
        file.set_permissions(Permissions::from_mode(mode_val))
            .map_err(|err| format!("Failed to set target file's mode: {}", err))?;
    }

    let mut calc_hash = Blake2s::new(HASH_SIZE);

    for chunk_num in 0..num_chunks {
        let chunk = load_chunk(hash, chunk_num)?;

        // Update our verification hash
        calc_hash.update(&chunk);
        // Write the chunk to the destination file
        file.write_all(&chunk)
            .map_err(|err| format!("Failed to write chunk {}: {}", chunk_num, err))?;
    }

    let calc_hash_str = calc_hash
        .finalize()
        .as_bytes()
        .iter()
        .map(|val| format!("{:02x}", val))
        .collect::<String>();

    if calc_hash_str == hash {
        // TODO: Do we want to clean up the temporary directory here?
        // Alternatively, the service can be resposible for that
        Ok(())
    } else {
        Err("File hash mismatch".to_owned())
    }
}
