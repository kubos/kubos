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

extern crate blake2_rfc;
extern crate cbor_protocol;
extern crate file_protocol;
extern crate file_service;
extern crate kubos_system;
extern crate rand;
extern crate tempfile;

mod common;

use blake2_rfc::blake2s::Blake2s;
use common::*;
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

// Download single-chunk file from scratch
#[test]
fn download_single() {
    let test_dir = TempDir::new().expect("Failed to create test dir");
    let test_dir_str = test_dir.path().to_str().unwrap();
    let source = format!("{}/source", test_dir_str);
    let dest = format!("{}/dest", test_dir_str);
    let service_port = 8000;

    let contents = "download_single".as_bytes();

    let hash = create_test_file(&source, &contents);

    service_new!(service_port, 4096);

    let result = download(
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

// Download multi-chunk file from scratch
#[test]
fn download_multi_clean() {
    let test_dir = TempDir::new().expect("Failed to create test dir");
    let test_dir_str = test_dir.path().to_str().unwrap();
    let source = format!("{}/source", test_dir_str);
    let dest = format!("{}/dest", test_dir_str);
    let service_port = 8001;

    let contents = [1; 6000];

    let hash = create_test_file(&source, &contents);

    service_new!(service_port, 4096);

    let result = download(
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

// Download multi-chunk file which we already have 1 chunk for
#[test]
fn download_multi_resume() {
    let test_dir = TempDir::new().expect("Failed to create test dir");
    let test_dir_str = test_dir.path().to_str().unwrap();
    let source = format!("{}/source", test_dir_str);
    let dest = format!("{}/dest", test_dir_str);
    let service_port = 8002;

    let contents = [2; 6000];

    let hash = create_test_file(&source, &contents);

    service_new!(service_port, 4096);

    // Go ahead and download the whole file so we can manipulate the temporary directory
    let result = download(
        "127.0.0.1",
        &format!("127.0.0.1:{}", service_port),
        &source,
        &dest,
        Some("client".to_owned()),
        4096,
    );
    assert!(result.is_ok());

    // Remove a chunk so we can test the retry logic
    fs::remove_file(format!("service/storage/{}/0", hash)).unwrap();

    // download the file again
    let result = download(
        "127.0.0.1",
        "127.0.0.1:8002",
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

// Download multi-chunk file which we already have all chunks for
#[test]
fn download_multi_complete() {
    let test_dir = TempDir::new().expect("Failed to create test dir");
    let test_dir_str = test_dir.path().to_str().unwrap();
    let source = format!("{}/source", test_dir_str);
    let dest = format!("{}/dest", test_dir_str);
    let service_port = 8005;

    let contents = [3; 6000];

    let hash = create_test_file(&source, &contents);

    service_new!(service_port, 4096);

    // download the file once (clean download)
    let result = download(
        "127.0.0.1",
        &format!("127.0.0.1:{}", service_port),
        &source,
        &dest,
        Some("client".to_owned()),
        4096,
    );
    assert!(result.is_ok());

    // download the file again
    let result = download(
        "127.0.0.1",
        "127.0.0.1:8005",
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

// This test is broken! The storage folder is cleaned up before
// we have a chance to edit the hash file.
// // Download. Create hash mismatch.
// #[test]
// fn download_bad_hash() {
//     let test_dir = TempDir::new().expect("Failed to create test dir");
//     let test_dir_str = test_dir.path().to_str().unwrap();
//     let source = format!("{}/source", test_dir_str);
//     let dest = format!("{}/dest", test_dir_str);
//     let service_port = 8003;

//     let contents = "download_bad_hash".as_bytes();

//     let hash = create_test_file(&source, &contents);

//     service_new!(service_port, 4096);

//     // Download the file so we can mess with the temporary storage
//     let result = download(
//         "127.0.0.1",
//         &format!("127.0.0.1:{}", service_port),
//         &source,
//         &dest,
//         Some("client".to_owned()),
//         4096,
//     );
//     assert!(result.is_ok());

//     // Tweak the chunk contents so the future hash calculation will fail
//     fs::write(format!("client/storage/{}/0", hash), "bad data".as_bytes()).unwrap();

//     let result = download(
//         "127.0.0.1",
//         "127.0.0.1:8003",
//         &source,
//         &dest,
//         Some("client".to_owned()),
//         4096,
//     );
//     assert_eq!("File hash mismatch", format!("{}", result.unwrap_err()));

//     // Cleanup the temporary files so that the test can be repeatable
//     // The client folder is cleaned up by the protocol as a result
//     // of the hash mismatch
//     fs::remove_dir_all(format!("service/storage/{}", hash)).unwrap();
// }

// Download a single file in 5 simultaneous client instances
#[test]
fn download_multi_client() {
    let service_port = 8004;

    // Spawn our single service
    service_new!(service_port, 4096);

    let mut thread_handles = vec![];

    // Spawn 5 simultaneous clients
    for num in 0..5 {
        thread_handles.push(thread::spawn(move || {
            let test_dir = TempDir::new().expect("Failed to create test dir");
            let test_dir_str = test_dir.path().to_str().unwrap();
            let source = format!("{}/source", test_dir_str);
            let dest = format!("{}/dest", test_dir_str);
            let contents = [num; 6500];

            let hash = create_test_file(&source, &contents);

            let result = download(
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

// Massive (100MB) download
// Note 1: This test will take several minutes to run.
//         Ignore the Rust warning about the test taking to long
// Note 2: This is named differently so that the not-massive tests can
//         all be (quickly) run at the same time with `cargo test download`
#[test]
fn large_down() {
    let test_dir = TempDir::new().expect("Failed to create test dir");
    let test_dir_str = test_dir.path().to_str().unwrap();
    let source = format!("{}/source", test_dir_str);
    let dest = format!("{}/dest", test_dir_str);
    let service_port = 8006;

    // Create a 100MB file filled with random data
    let hash: String = {
        let mut hasher = Blake2s::new(16);
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
            hasher.update(&contents);
        }

        let hash_result = hasher.finalize();

        hash_result
            .as_bytes()
            .iter()
            .map(|byte| format!("{:02x}", byte))
            .collect()
    };

    service_new!(service_port, 4096);

    let result = download(
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
    for num in 0..24415 {
        let mut source_buf = [0u8; 4096];
        let mut dest_buf = [0u8; 4096];

        source_file.read(&mut source_buf).unwrap();
        dest_file.read(&mut dest_buf).unwrap();

        assert_eq!(&source_buf[..], &dest_buf[..], "Chunk mismatch: {}", num);
    }
}
