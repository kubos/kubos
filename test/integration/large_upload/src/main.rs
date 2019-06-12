//
// Copyright (C) 2019 Kubos Corporation
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

use file_protocol::{FileProtocol, FileProtocolConfig, ProtocolError, State};
use file_service::recv_loop;
use kubos_system::Config as ServiceConfig;
use rand::{thread_rng, Rng};
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::thread;
use std::time::Duration;
use tempfile::TempDir;

macro_rules! service_new {
    ($port:expr, $down_port:expr, $chunk_size:expr) => {{
        thread::spawn(move || {
            recv_loop(&ServiceConfig::new_from_str(
                "file-transfer-service",
                &format!(
                    r#"
                [file-transfer-service]
                storage_dir = "service"
                chunk_size = {}
                hold_count = 5
                downlink_ip = "127.0.0.1"
                downlink_port = {}
                [file-transfer-service.addr]
                ip = "127.0.0.1"
                port = {}
                "#,
                    $chunk_size, $down_port, $port
                ),
            ))
            .unwrap();
        });

        thread::sleep(Duration::new(1, 0));
    }};
}

// Massive (100MB) upload
// Note  : This test will take several minutes to run.
//         Ignore the Rust warning about the test taking to long
// Note 2: This has been moved to test/integration so it can be run in
//         parallel with the rest of `cargo test`
fn main() {
    let test_dir = TempDir::new().expect("Failed to create test dir");
    let test_dir_str = test_dir.path().to_str().unwrap();
    let source = format!("{}/source", test_dir_str);
    let dest = format!("{}/dest", test_dir_str);
    let service_port = 7006;
    let down_port = 6006;

    // Create a 100MB file filled with random data
    {
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(source.clone())
            .unwrap();
        for _ in 0..100 {
            let mut contents = [0u8; 1_000_000];
            thread_rng().fill(&mut contents[..]);

            file.write_all(&contents).unwrap();
        }
    }

    service_new!(service_port, down_port, 4096);

    let result = upload(
        "127.0.0.1",
        down_port,
        &format!("127.0.0.1:{}", service_port),
        &source,
        &dest,
        Some("client".to_owned()),
        4096,
    );

    assert!(result.is_ok());

    // Verify the final file's contents
    let mut source_file = File::open(source).unwrap();
    let mut dest_file = File::open(dest).unwrap();
    // 24415 = 100M / 4096
    // 2442 = 10M / 4096
    for num in 0..24415 {
        let mut source_buf = [0u8; 4096];
        let mut dest_buf = [0u8; 4096];

        let _ = source_file.read(&mut source_buf).unwrap();
        let _ = dest_file.read(&mut dest_buf).unwrap();

        assert_eq!(&source_buf[..], &dest_buf[..], "Chunk mismatch: {}", num);
    }
}

pub fn upload(
    host_ip: &str,
    host_port: u16,
    remote_addr: &str,
    source_path: &str,
    target_path: &str,
    prefix: Option<String>,
    chunk_size: u32,
) -> Result<String, ProtocolError> {
    let hold_count = 5;
    let f_config = FileProtocolConfig::new(prefix, chunk_size as usize, hold_count);
    let f_protocol = FileProtocol::new(host_ip, host_port, remote_addr, f_config);

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
        &State::Transmitting,
    )?;

    // note: the original upload client function does not return the hash.
    // we're only doing it here so that we can manipulate the temporary storage
    Ok(hash.to_owned())
}
