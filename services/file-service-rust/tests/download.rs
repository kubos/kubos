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
extern crate file_service_rust;
extern crate kubos_system;
extern crate rand;
extern crate tempfile;

use kubos_system::Config as ServiceConfig;
use file_service_rust::recv_loop;
use std::thread;
use cbor_protocol::Protocol as CborProtocol;
use file_protocol::FileProtocol;
use rand::{thread_rng, Rng};
use std::env;
use std::path::Path;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
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

fn create_test_file(name: &str, contents: &[u8]) {
    let mut file = File::create(name).unwrap();
    file.write_all(contents).unwrap();
}

// upload single-chunk file from scratch
#[test]
fn download_single() {
    let test_dir = TempDir::new().expect("Failed to create test dir");
    let test_dir_str = test_dir.path().to_str().unwrap();
    let source = format!("{}/source", test_dir_str);
    let dest = format!("{}/dest", test_dir_str);
    let service_port = 8000;

    let contents = "download_single".as_bytes();

    create_test_file(&source, &contents);

    service_new!(service_port);

    let result = file_protocol::download(service_port, &source, &dest, Some("client".to_owned()));
    assert!(result.is_ok());

    let hash = result.unwrap();

    // Cleanup the temporary files so that the test can be repeatable
    fs::remove_dir_all(format!("client/storage/{}", hash)).unwrap();
    fs::remove_dir_all(format!("service/storage/{}", hash)).unwrap();

    // Verify the final file's contents
    let dest_contents = fs::read(dest).unwrap();
    assert_eq!(&contents[..], dest_contents.as_slice());
}

// download multi-chunk file from scratch
#[test]
fn download_multi_clean() {
    let test_dir = TempDir::new().expect("Failed to create test dir");
    let test_dir_str = test_dir.path().to_str().unwrap();
    let source = format!("{}/source", test_dir_str);
    let dest = format!("{}/dest", test_dir_str);
    let service_port = 8001;

    let contents = [1; 6000];

    create_test_file(&source, &contents);

    service_new!(service_port);

    let result = file_protocol::download(service_port, &source, &dest, Some("client".to_owned()));

    assert!(result.is_ok());

    let hash = result.unwrap();

    // Cleanup the temporary files so that the test can be repeatable
    fs::remove_dir_all(format!("client/storage/{}", hash)).unwrap();
    fs::remove_dir_all(format!("service/storage/{}", hash)).unwrap();

    // Verify the final file's contents
    let dest_contents = fs::read(dest).unwrap();
    assert_eq!(&contents[..], dest_contents.as_slice());
}

// download multi-chunk file which we already have 1 chunk for
#[test]
fn download_multi_resume() {
    let test_dir = TempDir::new().expect("Failed to create test dir");
    let test_dir_str = test_dir.path().to_str().unwrap();
    let source = format!("{}/source", test_dir_str);
    let dest = format!("{}/dest", test_dir_str);
    let service_port = 8002;

    let contents = [2; 6000];

    create_test_file(&source, &contents);

    service_new!(service_port);

    // Go ahead and download the whole file so we can manipulate the temporary directory
    let result = file_protocol::download(service_port, &source, &dest, Some("client".to_owned()));
    assert!(result.is_ok());
    let hash = result.unwrap();

    // Remove a chunk so we can test the retry logic
    fs::remove_file(format!("service/storage/{}/0", hash)).unwrap();

    // download the file again
    let result = file_protocol::download(service_port, &source, &dest, Some("client".to_owned()));
    assert!(result.is_ok());
    let hash = result.unwrap();

    // Cleanup the temporary files so that the test can be repeatable
    fs::remove_dir_all(format!("client/storage/{}", hash)).unwrap();
    fs::remove_dir_all(format!("service/storage/{}", hash)).unwrap();

    // Verify the final file's contents
    let dest_contents = fs::read(dest).unwrap();
    assert_eq!(&contents[..], dest_contents.as_slice());
}

// download multi-chunk file which we already have all chunks for
#[test]
fn download_multi_complete() {
    let test_dir = TempDir::new().expect("Failed to create test dir");
    let test_dir_str = test_dir.path().to_str().unwrap();
    let source = format!("{}/source", test_dir_str);
    let dest = format!("{}/dest", test_dir_str);
    let service_port = 8005;

    let contents = [3; 6000];

    create_test_file(&source, &contents);

    service_new!(service_port);

    // download the file once (clean download)
    let result = file_protocol::download(service_port, &source, &dest, Some("client".to_owned()));
    assert!(result.is_ok());

    // download the file again
    let result = file_protocol::download(service_port, &source, &dest, Some("client".to_owned()));

    assert!(result.is_ok());
    let hash = result.unwrap();

    // Cleanup the temporary files so that the test can be repeatable
    fs::remove_dir_all(format!("client/storage/{}", hash)).unwrap();
    fs::remove_dir_all(format!("service/storage/{}", hash)).unwrap();

    // Verify the final file's contents
    let dest_contents = fs::read(dest).unwrap();
    assert_eq!(&contents[..], dest_contents.as_slice());
}

// download. Create hash mismatch.
#[test]
fn download_bad_hash() {
    let test_dir = TempDir::new().expect("Failed to create test dir");
    let test_dir_str = test_dir.path().to_str().unwrap();
    let source = format!("{}/source", test_dir_str);
    let dest = format!("{}/dest", test_dir_str);
    let service_port = 8003;

    let contents = "download_bad_hash".as_bytes();

    create_test_file(&source, &contents);

    service_new!(service_port);

    // download the file so we can mess with the temporary storage
    let result = file_protocol::download(service_port, &source, &dest, Some("client".to_owned()));
    assert!(result.is_ok());
    let hash = result.unwrap();

    // Tweak the chunk contents so the future hash calculation will fail
    fs::write(format!("client/storage/{}/0", hash), "bad data".as_bytes()).unwrap();

    let result = file_protocol::download(service_port, &source, &dest, Some("client".to_owned()));
    assert_eq!(result.unwrap_err(), "File hash mismatch");

    // Cleanup the temporary files so that the test can be repeatable
    fs::remove_dir_all(format!("client/storage/{}", hash)).unwrap();
    fs::remove_dir_all(format!("service/storage/{}", hash)).unwrap();
}

/*
#[test]
fn download_multi_client() {
    let service_port = 8004;

    // Spawn our single service
    service_new!(service_port);

    let mut thread_handles = vec![];

    // Spawn 5 simultaneous clients
    for num in 0..5 {
        thread_handles.push(thread::spawn(move || {
            let test_dir = TempDir::new().expect("Failed to create test dir");
            let test_dir_str = test_dir.path().to_str().unwrap();
            let source = format!("{}/source", test_dir_str);
            let dest = format!("{}/dest", test_dir_str);
            let contents = [num; 6000];

            create_test_file(&source, &contents);

            let result =
                file_protocol::download(service_port, &source, &dest, Some("client".to_owned()));
            assert!(result.is_ok());

            let hash = result.unwrap();

            // TODO: Remove this sleep. We need it to let the service
            // finish its work. The upload logic needs to wait on
            // the final ACK message before returning
            thread::sleep(Duration::new(2, 0));

            // Cleanup the temporary files so that the test can be repeatable
            fs::remove_dir_all(format!("storage/{}", hash)).unwrap();

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
*/
// Massive download
#[test]
fn download_large() {
    let test_dir = TempDir::new().expect("Failed to create test dir");
    let test_dir_str = test_dir.path().to_str().unwrap();
    let source = format!("{}/source", test_dir_str);
    let dest = format!("{}/dest", test_dir_str);
    let service_port = 8006;

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

    let result = file_protocol::download(service_port, &source, &dest, Some("client".to_owned()));
    println!("Result: {:?}", result);
    assert!(result.is_ok());

    let hash = result.unwrap();

    // Cleanup the temporary files so that the test can be repeatable
    fs::remove_dir_all(format!("client/storage/{}", hash)).unwrap();
    fs::remove_dir_all(format!("service/storage/{}", hash)).unwrap();

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