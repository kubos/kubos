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

mod common;

use crate::common::*;
use file_protocol::ProtocolError;
use file_service::recv_loop;
use kubos_system::Config as ServiceConfig;
use rand::{thread_rng, Rng};
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::thread;
use std::time::Duration;
use tempfile::TempDir;

// NOTE: Each test's file contents must be unique. Otherwise the hash is the same, so
// the same storage directory is used across all of them, creating conflicts

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

    service_new!(service_port, 4096);

    let result = upload(
        "127.0.0.1",
        &format!("127.0.0.1:{}", service_port),
        &source,
        &dest,
        Some("client".to_owned()),
        4096,
    );

    if let Err(err) = &result {
        println!("Error: {}", err);
    }

    assert!(result.is_ok());

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

    let contents = [1; 5000];

    create_test_file(&source, &contents);

    service_new!(service_port, 4096);

    let result = upload(
        "127.0.0.1",
        &format!("127.0.0.1:{}", service_port),
        &source,
        &dest,
        Some("client".to_owned()),
        4096,
    );

    assert!(result.is_ok());

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

    service_new!(service_port, 4096);

    // Upload a partial version of the file
    let result = upload_partial(
        "127.0.0.1",
        "127.0.0.1:7002",
        &source,
        &dest,
        Some("client".to_owned()),
        4096,
    );
    assert!(result.is_err());

    // Upload the whole file this time
    let result = upload(
        "127.0.0.1",
        &format!("127.0.0.1:{}", service_port),
        &source,
        &dest,
        Some("client".to_owned()),
        4096,
    );
    assert!(result.is_ok());

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

    service_new!(service_port, 4096);

    // Upload the file once (clean upload)
    let result = upload(
        "127.0.0.1",
        &format!("127.0.0.1:{}", service_port),
        &source,
        &dest,
        Some("client".to_owned()),
        4096,
    );
    assert!(result.is_ok());

    // Upload the file again
    let result = upload(
        "127.0.0.1",
        "127.0.0.1:7005",
        &source,
        &dest,
        Some("client".to_owned()),
        4096,
    );
    assert!(result.is_ok());

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

    service_new!(service_port, 4096);

    // Upload the file so we can mess with the temporary storage
    let result = upload(
        "127.0.0.1",
        &format!("127.0.0.1:{}", service_port),
        &source,
        &dest,
        Some("client".to_owned()),
        4096,
    );
    assert!(result.is_ok());
    let hash = result.unwrap();

    // Give the service a moment to go through its cleanup logic before we mess with things
    thread::sleep(Duration::from_millis(10));

    // Create temp folder with bad chunk so that future hash calculation will fail
    fs::create_dir(format!("service/storage/{}", hash)).unwrap();
    fs::write(format!("service/storage/{}/0", hash), "bad data".as_bytes()).unwrap();

    let result = upload(
        "127.0.0.1",
        "127.0.0.1:7003",
        &source,
        &dest,
        Some("client".to_owned()),
        4096,
    );

    assert_eq!(
        "File hash mismatch",
        match result.unwrap_err() {
            ProtocolError::TransmissionError {
                channel_id: _,
                error_message,
            } => error_message,
            _ => "".to_owned(),
        }
    );

    // Cleanup the temporary files so that the test can be repeatable
    // The service storage folder is deleted by the protocol as a
    // result of the hash mismatch
    fs::remove_dir_all(format!("client/storage/{}", hash)).unwrap();
}

// Upload a single file in 5 simultaneous client instances
#[test]
fn upload_multi_client() {
    let service_port = 7004;

    // Spawn our single service
    service_new!(service_port, 4096);

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
                4096,
            );
            assert!(result.is_ok());

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

    service_new!(service_port, 4096);

    let result = upload(
        "127.0.0.1",
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

        source_file.read(&mut source_buf).unwrap();
        dest_file.read(&mut dest_buf).unwrap();

        assert_eq!(&source_buf[..], &dest_buf[..], "Chunk mismatch: {}", num);
    }
}

// Verify an upload still works after the server has
// received invalid input
#[test]
fn upload_single_after_bad_input() {
    use std::net::UdpSocket;

    let test_dir = TempDir::new().expect("Failed to create test dir");
    let test_dir_str = test_dir.path().to_str().unwrap();
    let source = format!("{}/source", test_dir_str);
    let dest = format!("{}/dest", test_dir_str);
    let service_port = 7007;

    let contents = "upload_single_after_bad_input".as_bytes();

    create_test_file(&source, &contents);

    service_new!(service_port, 4096);

    {
        let send_socket = UdpSocket::bind("127.0.0.1:0").unwrap();
        let send_buf = "{ping}".as_bytes();
        send_socket.send_to(&send_buf, "127.0.0.1:7007").unwrap();
    }

    let result = upload(
        "127.0.0.1",
        &format!("127.0.0.1:{}", service_port),
        &source,
        &dest,
        Some("client".to_owned()),
        4096,
    );

    if let Err(err) = &result {
        println!("Error: {}", err);
    }

    assert!(result.is_ok());

    // Verify the final file's contents
    let dest_contents = fs::read(dest).unwrap();
    assert_eq!(&contents[..], dest_contents.as_slice());
}
