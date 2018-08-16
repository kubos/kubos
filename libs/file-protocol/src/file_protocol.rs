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

use blake2_rfc::blake2s::Blake2s;
use cbor_codec::Protocol as CborProtocol;
use serde::Serializer;
use serde_cbor::{ser, to_vec};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;
use time;

const CHUNK_SIZE: usize = 4096;

pub struct Protocol {
    pub cbor_proto: CborProtocol,
    pub host: String,
    pub dest_port: u16,
}

impl Protocol {
    pub fn store_chunk(&self, hash: &str, index: u32, data: &[u8]) -> Result<(), String> {
        // if data is type uint8_t[]
        // change data to ffi.string
        let file_name = format!("{:x}", index);
        let storage_path = Path::new("storage").join(hash).join(file_name);

        // may need to check directory existence
        fs::create_dir_all(&storage_path.parent().unwrap()).unwrap();
        let mut file = File::create(&storage_path).unwrap();
        file.write_all(data).unwrap();

        Ok(())
    }

    pub fn store_meta(&self, hash: &str, num_chunks: u32) -> Result<(), String> {
        let data = vec![("num_chunks", num_chunks)];

        // let mut e = Encoder::from_memory();
        // e.encode(&data).unwrap();
        let vec = to_vec(&data).unwrap();

        let meta_path = Path::new("storage").join(hash).join("meta");
        let temp_path = Path::new("storage").join(hash).join(".meta.tmp");

        // may need to check directory existence
        File::create(&temp_path).unwrap().write_all(&vec).unwrap();

        fs::rename(temp_path, meta_path).unwrap();

        Ok(())
    }

    pub fn load_chunk(&self, hash: &str, index: u32) -> Result<Vec<u8>, String> {
        let mut data = vec![];
        let path = Path::new("storage").join(hash).join(format!("{:x}", index));

        File::open(path).unwrap().read_to_end(&mut data).unwrap();
        Ok(data)
    }

    pub fn load_meta(&self, hash: &str) -> Result<Option<u32>, String> {
        let mut data = vec![];
        let meta_path = Path::new("storage").join(hash).join("meta");
        File::open(meta_path)
            .unwrap()
            .read_to_end(&mut data)
            .unwrap();

        // cbor decode meta
        // let _d = Decoder::from_bytes(data);

        Ok(Some(10))
    }

    pub fn local_sync(&self, hash: &str, num_chunks: Option<u32>) -> Result<(bool, u32), String> {
        if let Some(num) = num_chunks {
            self.store_meta(hash, num).unwrap();
        } else {
            let _num_chunks = match self.load_meta(hash) {
                Ok(d) => match d {
                    Some(d) => d,
                    None => return Ok((false, 0)),
                },
                Err(e) => return Err(format!("failed loading meta {:?}", e)),
            };
        }

        let mut _bits: Vec<u8> = vec![];

        let _hash_path = Path::new("storage").join(hash);

        Ok((true, 0))
    }

    /// Create temporary folder for chunks
    /// Stream copy file from mutable space to immutable space
    /// Move folder to hash of contents
    pub fn local_import(&self, source_path: &str) -> Result<(String, u32, u16), String> {
        let storage_path = String::from("storage");

        if let Err(e) = fs::metadata(source_path) {
            return Err(format!("failed to stat file {}: {:?}", source_path, e));
        }

        // Copy input file to storage area and calculate hash
        if let Err(e) = fs::create_dir_all(&storage_path) {
            return Err(format!("failed to create dir {}: {:?}", storage_path, e));
        }

        let temp_path = Path::new(&storage_path).join(format!(".{}", time::get_time().nsec));
        let mut hasher = Blake2s::new(16);
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
                    self.store_chunk(&hash, index, &chunk[0..n]).unwrap();
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
        self.store_meta(&hash, index).unwrap();

        Ok((hash, index, 0))
    }

    pub fn send_sync(&self, hash: &str, num_chunks: u32) -> Result<(), String> {
        let vec = ser::to_vec_packed(&(hash, num_chunks)).unwrap();
        self.cbor_proto
            .send_message(&vec, &self.host, self.dest_port)
            .unwrap();
        Ok(())
    }

    pub fn send_export(&self, hash: &str, target_path: &str, mode: u16) -> Result<(), String> {
        let vec = ser::to_vec_packed(&("export", hash, target_path, mode)).unwrap();
        self.cbor_proto
            .send_message(&vec, &self.host, self.dest_port)
            .unwrap();
        Ok(())
    }

    pub fn send_import(&self, source_path: &str) -> Result<(String, u32, u16), String> {
        unimplemented!();
    }

    pub fn sync_and_send(&self, hash: &str, num_chunks: u32) -> Result<(), String> {
        unimplemented!();
    }

    pub fn local_export(&self, hash: &str, target_path: &str, mode: u16) -> Result<(), String> {
        unimplemented!();
    }
}
