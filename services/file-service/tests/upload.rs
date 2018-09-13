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

extern crate cbor_protocol;
extern crate file_protocol;
extern crate file_service;
extern crate kubos_system;
extern crate rand;
extern crate tempfile;
#[macro_use]
extern crate log;
extern crate simplelog;

use file_protocol::{FileProtocol, State};
use file_service::recv_loop;
use kubos_system::Config as ServiceConfig;
use rand::{thread_rng, Rng};
use simplelog::*;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::thread;
use std::time::Duration;
use tempfile::TempDir;

// NOTE: Each test's file contents must be unique. Otherwise the hash is the same, so
// the same storage directory is used across all of them, creating conflicts

macro_rules! service_new {
    ($port:expr) => {{
        thread::spawn(move || {
            recv_loop(ServiceConfig::new_from_str(
                "file-transfer-service",
                &format!(
                    r#"
                [file-transfer-service]
                storage_dir = "service"
                [file-transfer-service.addr]
                ip = "127.0.0.1"
                port = {}
                "#,
                    $port
                ),
            )).unwrap();
        });

        thread::sleep(Duration::new(1, 0));
    }};
}

fn upload(
    host_ip: &str,
    remote_addr: &str,
    source_path: &str,
    target_path: &str,
    prefix: Option<String>,
) -> Result<String, String> {
    let f_protocol = FileProtocol::new(host_ip, remote_addr, prefix);

    // Copy file to upload to temp storage. Calculate the hash and chunk info
    let (hash, num_chunks, mode) = f_protocol.initialize_file(&source_path)?;

    let channel = f_protocol.generate_channel()?;

    // Tell our destination the hash and number of chunks to expect
    f_protocol.send_metadata(channel, &hash, num_chunks)?;

    // Send export command for file
    f_protocol.send_export(channel, &hash, &target_path, mode)?;

    // Start the engine to send the file data chunks
    f_protocol.message_engine(
        |d| f_protocol.recv(Some(d)),
        Duration::from_secs(2),
        State::Transmitting,
    )?;

    // Note: The original upload client function does not return the hash.
    // We're only doing it here so that we can manipulate the temporary storage
    Ok(hash.to_owned())
}

fn create_test_file(name: &str, contents: &[u8]) {
    let mut file = File::create(name).unwrap();
    file.write_all(contents).unwrap();
}

// Upload single-chunk file from scratch
#[test]
fn upload_single() {
    let test_dir = TempDir::new().expect("Failed to create test dir");
    let test_dir_str = test_dir.path().to_str().unwrap();
    let source = format!("{}/source", test_dir_str);
    let dest = format!("{}/dest", test_dir_str);
    let service_port = 7000;

    let contents = "upload_single".as_bytes();

    create_test_file(&source, &contents);

    service_new!(service_port);

    let result = upload(
        "127.0.0.1",
        &format!("127.0.0.1:{}", service_port),
        &source,
        &dest,
        Some("client".to_owned()),
    );

    if let Err(err) = result.clone() {
        println!("Error: {}", err);
    }

    assert!(result.is_ok());

    let hash = result.unwrap();

    thread::sleep(Duration::from_secs(10));

    // Cleanup the temporary files so that the test can be repeatable
    fs::remove_dir_all(format!("client/storage/{}", hash)).unwrap();
    fs::remove_dir_all(format!("service/storage/{}", hash)).unwrap();

    // Verify the final file's contents
    let dest_contents = fs::read(dest).unwrap();
    assert_eq!(&contents[..], dest_contents.as_slice());
}

// Upload multi-chunk file from scratch
#[test]
fn upload_multi_clean() {
    let test_dir = TempDir::new().expect("Failed to create test dir");
    let test_dir_str = test_dir.path().to_str().unwrap();
    let source = format!("{}/source", test_dir_str);
    let dest = format!("{}/dest", test_dir_str);
    let service_port = 7001;

    let contents = [1; 10];

    create_test_file(&source, &contents);

    service_new!(service_port);

    let result = upload(
        "127.0.0.1",
        &format!("127.0.0.1:{}", service_port),
        &source,
        &dest,
        Some("client".to_owned()),
    );

    assert!(result.is_ok());

    let hash = result.unwrap();

    // Cleanup the temporary files so that the test can be repeatable
    fs::remove_dir_all(format!("client/storage/{}", hash)).unwrap();
    fs::remove_dir_all(format!("service/storage/{}", hash)).unwrap();

    // Verify the final file's contents
    let dest_contents = fs::read(dest).unwrap();
    assert_eq!(&contents[..], dest_contents.as_slice());
}

// Upload multi-chunk file which we already have 1 chunk for
#[test]
fn upload_multi_resume() {
    let test_dir = TempDir::new().expect("Failed to create test dir");
    let test_dir_str = test_dir.path().to_str().unwrap();
    let source = format!("{}/source", test_dir_str);
    let dest = format!("{}/dest", test_dir_str);
    let service_port = 7002;

    let contents = [2; 5000];

    create_test_file(&source, &contents);

    service_new!(service_port);

    // Go ahead and upload the whole file so we can manipulate the temporary directory
    let result = upload(
        "127.0.0.1",
        "127.0.0.1:7002",
        &source,
        &dest,
        Some("client".to_owned()),
    );
    assert!(result.is_ok());
    let hash = result.unwrap();

    // Remove a chunk so we can test the retry logic
    fs::remove_file(format!("service/storage/{}/0", hash)).unwrap();

    // Upload the file again
    let result = upload(
        "127.0.0.1",
        &format!("127.0.0.1:{}", service_port),
        &source,
        &dest,
        Some("client".to_owned()),
    );
    assert!(result.is_ok());
    let hash = result.unwrap();

    // Cleanup the temporary files so that the test can be repeatable
    fs::remove_dir_all(format!("client/storage/{}", hash)).unwrap();
    fs::remove_dir_all(format!("service/storage/{}", hash)).unwrap();

    // Verify the final file's contents
    let dest_contents = fs::read(dest).unwrap();
    assert_eq!(&contents[..], dest_contents.as_slice());
}

// Upload multi-chunk file which we already have all chunks for
#[test]
fn upload_multi_complete() {
    let test_dir = TempDir::new().expect("Failed to create test dir");
    let test_dir_str = test_dir.path().to_str().unwrap();
    let source = format!("{}/source", test_dir_str);
    let dest = format!("{}/dest", test_dir_str);
    let service_port = 7005;

    let contents = [3; 5000];

    create_test_file(&source, &contents);

    service_new!(service_port);

    // Upload the file once (clean upload)
    let result = upload(
        "127.0.0.1",
        &format!("127.0.0.1:{}", service_port),
        &source,
        &dest,
        Some("client".to_owned()),
    );
    assert!(result.is_ok());

    // Upload the file again
    let result = upload(
        "127.0.0.1",
        "127.0.0.1:7005",
        &source,
        &dest,
        Some("client".to_owned()),
    );
    assert!(result.is_ok());
    let hash = result.unwrap();

    // Cleanup the temporary files so that the test can be repeatable
    fs::remove_dir_all(format!("client/storage/{}", hash)).unwrap();
    fs::remove_dir_all(format!("service/storage/{}", hash)).unwrap();

    // Verify the final file's contents
    let dest_contents = fs::read(dest).unwrap();
    assert_eq!(&contents[..], dest_contents.as_slice());
}

// Upload. Create hash mismatch.
#[test]
fn upload_bad_hash() {
    let test_dir = TempDir::new().expect("Failed to create test dir");
    let test_dir_str = test_dir.path().to_str().unwrap();
    let source = format!("{}/source", test_dir_str);
    let dest = format!("{}/dest", test_dir_str);
    let service_port = 7003;

    let contents = "upload_bad_hash".as_bytes();

    create_test_file(&source, &contents);

    service_new!(service_port);

    // Upload the file so we can mess with the temporary storage
    let result = upload(
        "127.0.0.1",
        &format!("127.0.0.1:{}", service_port),
        &source,
        &dest,
        Some("client".to_owned()),
    );
    assert!(result.is_ok());
    let hash = result.unwrap();

    // Tweak the chunk contents so the future hash calculation will fail
    fs::write(format!("service/storage/{}/0", hash), "bad data".as_bytes()).unwrap();

    let result = upload(
        "127.0.0.1",
        "127.0.0.1:7003",
        &source,
        &dest,
        Some("client".to_owned()),
    );
    assert!(result.unwrap_err().contains("File hash mismatch"));

    // Cleanup the temporary files so that the test can be repeatable
    fs::remove_dir_all(format!("client/storage/{}", hash)).unwrap();
    fs::remove_dir_all(format!("service/storage/{}", hash)).unwrap();
}

// Upload a single file in 5 simultaneous client instances
#[test]
fn upload_multi_client() {
    let service_port = 7004;

    // Spawn our single service
    service_new!(service_port);

    let mut thread_handles = vec![];

    // Spawn 4 simultaneous clients
    for _num in 0..4 {
        thread_handles.push(thread::spawn(move || {
            let test_dir = TempDir::new().expect("Failed to create test dir");
            let test_dir_str = test_dir.path().to_str().unwrap();
            let source = format!("{}/source", test_dir_str);
            let dest = format!("{}/dest", test_dir_str);

            let mut contents = [0u8; 10_000];
            thread_rng().fill(&mut contents[..]);

            create_test_file(&source, &contents);

            let result = upload(
                "127.0.0.1",
                &format!("127.0.0.1:{}", service_port),
                &source,
                &dest,
                Some("client".to_owned()),
            );
            assert!(result.is_ok());

            let hash = result.unwrap();

            // Cleanup the temporary files so that the test can be repeatable
            fs::remove_dir_all(format!("client/storage/{}", hash)).unwrap();
            fs::remove_dir_all(format!("service/storage/{}", hash)).unwrap();

            // Verify the final file's contents
            let dest_contents = fs::read(dest).unwrap();
            assert_eq!(&contents[..], dest_contents.as_slice());
        }));
    }

    for entry in thread_handles {
        // Check for any thread failures
        assert!(entry.join().is_ok());
    }
}

// Massive (100MB) upload
// Note 1: This test will take several minutes to run.
//         Ignore the Rust warning about the test taking to long
// Note 2: This is named differently so that the not-massive tests can
//         all be (quickly) run at the same time with `cargo test upload`
#[test]
fn large_up() {
    let test_dir = TempDir::new().expect("Failed to create test dir");
    let test_dir_str = test_dir.path().to_str().unwrap();
    let source = format!("{}/source", test_dir_str);
    let dest = format!("{}/dest", test_dir_str);
    let service_port = 7006;

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

            file.write(&contents).unwrap();
        }
    }

    service_new!(service_port);

    let result = upload(
        "127.0.0.1",
        &format!("127.0.0.1:{}", service_port),
        &source,
        &dest,
        Some("client".to_owned()),
    );

    assert!(result.is_ok());

    let hash = result.unwrap();

    // Cleanup the temporary files so that the test can be repeatable
    fs::remove_dir_all(format!("client/storage/{}", hash)).unwrap();
    fs::remove_dir_all(format!("service/storage/{}", hash)).unwrap();

    // Verify the final file's contents
    let mut source_file = File::open(source).unwrap();
    let mut dest_file = File::open(dest).unwrap();
    // 24415 = 100M / 4096
    // 2442 = 10M / 4096
    for num in 0..24415 {
        let mut source_buf = [0u8; 4096];
        let mut dest_buf = [0u8; 4096];

        source_file.read(&mut source_buf).unwrap();
        dest_file.read(&mut dest_buf).unwrap();

        assert_eq!(&source_buf[..], &dest_buf[..], "Chunk mismatch: {}", num);
    }
}
