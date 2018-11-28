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
#![macro_use]

extern crate blake2_rfc;
extern crate file_protocol;

use common::blake2_rfc::blake2s::Blake2s;
use file_protocol::{FileProtocol, FileProtocolConfig, ProtocolError, State};
use std::fs::File;
use std::io::prelude::*;
use std::time::Duration;

#[macro_export]
macro_rules! service_new {
    ($port:expr, $chunk_size:expr) => {{
        thread::spawn(move || {
            recv_loop(ServiceConfig::new_from_str(
                "file-transfer-service",
                &format!(
                    r#"
                [file-transfer-service]
                storage_dir = "service"
                chunk_size = {}
                hold_count = 5
                [file-transfer-service.addr]
                ip = "127.0.0.1"
                port = {}
                "#,
                    $chunk_size, $port
                ),
            ))
            .unwrap();
        });

        thread::sleep(Duration::new(1, 0));
    }};
}

pub fn download(
    host_ip: &str,
    remote_addr: &str,
    source_path: &str,
    target_path: &str,
    prefix: Option<String>,
    chunk_size: u32,
) -> Result<(), ProtocolError> {
    let hold_count = 5;
    let f_config = FileProtocolConfig::new(prefix, chunk_size as usize, hold_count);
    let f_protocol = FileProtocol::new(host_ip, remote_addr, f_config);

    let channel = f_protocol.generate_channel()?;

    // Send our file request to the remote addr and verify that it's
    // going to be able to send it
    f_protocol.send_import(channel, source_path)?;

    // Wait for the request reply.
    // Note/TODO: We don't use a timeout here because we don't know how long it will
    // take the server to prepare the file we've requested.
    // Larger files (> 100MB) can take over a minute to process.
    let reply = match f_protocol.recv(None) {
        Ok(message) => message,
        Err(error) => return Err(error),
    };

    let state = f_protocol.process_message(
        reply,
        State::StartReceive {
            path: target_path.to_string(),
        },
    )?;

    Ok(f_protocol.message_engine(|d| f_protocol.recv(Some(d)), Duration::from_secs(2), state)?)
}

pub fn upload(
    host_ip: &str,
    remote_addr: &str,
    source_path: &str,
    target_path: &str,
    prefix: Option<String>,
    chunk_size: u32,
) -> Result<String, ProtocolError> {
    let hold_count = 5;
    let f_config = FileProtocolConfig::new(prefix, chunk_size as usize, hold_count);
    let f_protocol = FileProtocol::new(host_ip, remote_addr, f_config);

    // copy file to upload to temp storage. calculate the hash and chunk info
    let (hash, num_chunks, mode) = f_protocol.initialize_file(&source_path)?;

    let channel = f_protocol.generate_channel()?;

    // tell our destination the hash and number of chunks to expect
    f_protocol.send_metadata(channel, &hash, num_chunks)?;

    // send export command for file
    f_protocol.send_export(channel, &hash, &target_path, mode)?;

    // start the engine to send the file data chunks
    f_protocol.message_engine(
        |d| f_protocol.recv(Some(d)),
        Duration::from_secs(2),
        State::Transmitting,
    )?;

    // note: the original upload client function does not return the hash.
    // we're only doing it here so that we can manipulate the temporary storage
    Ok(hash.to_owned())
}

pub fn create_test_file(name: &str, contents: &[u8]) -> String {
    let mut file = File::create(name).unwrap();
    file.write_all(contents).unwrap();

    let mut hasher = Blake2s::new(16);
    hasher.update(contents);
    let hash = hasher.finalize();

    let hash_str = hash
        .as_bytes()
        .iter()
        .map(|byte| format!("{:02x}", byte))
        .collect();

    hash_str
}
