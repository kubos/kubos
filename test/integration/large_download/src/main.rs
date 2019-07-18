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
    ($port:expr, $down_port:expr, $chunk_size:expr, $storage_dir:expr) => {{
        thread::spawn(move || {
            recv_loop(&ServiceConfig::new_from_str(
                "file-transfer-service",
                &format!(
                    r#"
                [file-transfer-service]
                storage_dir = "{}"
                chunk_size = {}
                hold_count = 5
                downlink_ip = "127.0.0.1"
                downlink_port = {}
                [file-transfer-service.addr]
                ip = "127.0.0.1"
                port = {}
                "#,
                    $storage_dir, $chunk_size, $down_port, $port
                ),
            ))
            .unwrap();
        });

        thread::sleep(Duration::new(1, 0));
    }};
}

// Massive (100MB) download
// Note 1: This test will take several minutes to run.
//         Ignore the Rust warning about the test taking to long
// Note 2: This is named differently so that the not-massive tests can
//         all be (quickly) run at the same time with `cargo test download`
fn main() {
    let test_dir = TempDir::new().expect("Failed to create test dir");
    let test_dir_str = test_dir.path().to_str().unwrap();
    let source = format!("{}/source", test_dir_str);
    let dest = format!("{}/dest", test_dir_str);
    let service_port = 8006;
    let down_port = 7006;

    // Create a 100MB file filled with random data
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

    let storage_dir = format!("{}/service", test_dir_str);
    service_new!(service_port, down_port, 4096, storage_dir);

    let result = download(
        "127.0.0.1",
        down_port,
        &format!("127.0.0.1:{}", service_port),
        &source,
        &dest,
        Some(format!("{}/client", test_dir_str)),
        4096,
    );

    assert!(result.is_ok());

    // Verify the final file's contents
    let mut source_file = File::open(source).unwrap();
    let mut dest_file = File::open(dest).unwrap();
    // 24415 = 100M / 4096
    for num in 0..24415 {
        let mut source_buf = [0u8; 4096];
        let mut dest_buf = [0u8; 4096];

        let _ = source_file.read(&mut source_buf).unwrap();
        let _ = dest_file.read(&mut dest_buf).unwrap();

        assert_eq!(&source_buf[..], &dest_buf[..], "Chunk mismatch: {}", num);
    }
}

pub fn download(
    host_ip: &str,
    host_port: u16,
    remote_addr: &str,
    source_path: &str,
    target_path: &str,
    prefix: Option<String>,
    chunk_size: u32,
) -> Result<(), ProtocolError> {
    let hold_count = 5;
    let f_config = FileProtocolConfig::new(prefix, chunk_size as usize, hold_count);
    let f_protocol =
        FileProtocol::new(&format!("{}:{}", host_ip, host_port), remote_addr, f_config);

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
        &State::StartReceive {
            path: target_path.to_string(),
        },
    )?;

    f_protocol.message_engine(|d| f_protocol.recv(Some(d)), Duration::from_secs(2), &state)?;
    Ok(())
}
