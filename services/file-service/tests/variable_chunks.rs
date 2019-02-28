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

mod common;

use crate::common::*;
use file_service::recv_loop;
use kubos_system::Config as ServiceConfig;
use rand::{thread_rng, Rng};
use std::fs;
use std::thread;
use std::time::Duration;
use tempfile::TempDir;

// NOTE: Each test's file contents must be unique. Otherwise the hash is the same, so
// the same storage directory is used across all of them, creating conflicts

// Download a single file in 5 simultaneous client instances
#[test]
fn download_multi_client_small_chunk() {
    let service_port = 8001;

    // Spawn our single service
    service_new!(service_port, 256);

    let mut thread_handles = vec![];

    // Spawn 5 simultaneous clients
    for num in 0..5 {
        thread_handles.push(thread::spawn(move || {
            let test_dir = TempDir::new().expect("Failed to create test dir");
            let test_dir_str = test_dir.path().to_str().unwrap();
            let source = format!("{}/source", test_dir_str);
            let dest = format!("{}/dest", test_dir_str);
            let contents = [num; 100_000];

            let _hash = create_test_file(&source, &contents);

            let result = download(
                "127.0.0.1",
                &format!("127.0.0.1:{}", service_port),
                &source,
                &dest,
                Some("client".to_owned()),
                256,
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

// Download a single file in 5 simultaneous client instances
#[test]
fn download_multi_client_large_chunk() {
    let service_port = 8002;

    // Spawn our single service
    service_new!(service_port, 8192);

    let mut thread_handles = vec![];

    // Spawn 5 simultaneous clients
    for num in 1..6 {
        thread_handles.push(thread::spawn(move || {
            let test_dir = TempDir::new().expect("Failed to create test dir");
            let test_dir_str = test_dir.path().to_str().unwrap();
            let source = format!("{}/source", test_dir_str);
            let dest = format!("{}/dest", test_dir_str);
            let contents = [num * 10; 100_000];

            let _hash = create_test_file(&source, &contents);

            let result = download(
                "127.0.0.1",
                &format!("127.0.0.1:{}", service_port),
                &source,
                &dest,
                Some("client".to_owned()),
                8192,
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

// Upload a single file in 5 simultaneous client instances
#[test]
fn upload_multi_client_small_chunk() {
    let service_port = 8003;

    // Spawn our single service
    service_new!(service_port, 256);

    let mut thread_handles = vec![];

    // Spawn 4 simultaneous clients
    for _num in 0..4 {
        thread_handles.push(thread::spawn(move || {
            let test_dir = TempDir::new().expect("Failed to create test dir");
            let test_dir_str = test_dir.path().to_str().unwrap();
            let source = format!("{}/source", test_dir_str);
            let dest = format!("{}/dest", test_dir_str);

            let mut contents = [100u8; 100_000];
            thread_rng().fill(&mut contents[..]);

            create_test_file(&source, &contents);

            let result = upload(
                "127.0.0.1",
                &format!("127.0.0.1:{}", service_port),
                &source,
                &dest,
                Some("client".to_owned()),
                256,
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

// Upload a single file in 5 simultaneous client instances
#[test]
fn upload_multi_client_large_chunk() {
    let service_port = 8004;

    // Spawn our single service
    service_new!(service_port, 8192);

    let mut thread_handles = vec![];

    // Spawn 4 simultaneous clients
    for _num in 0..4 {
        thread_handles.push(thread::spawn(move || {
            let test_dir = TempDir::new().expect("Failed to create test dir");
            let test_dir_str = test_dir.path().to_str().unwrap();
            let source = format!("{}/source", test_dir_str);
            let dest = format!("{}/dest", test_dir_str);

            let mut contents = [200u8; 100_000];
            thread_rng().fill(&mut contents[..]);

            create_test_file(&source, &contents);

            let result = upload(
                "127.0.0.1",
                &format!("127.0.0.1:{}", service_port),
                &source,
                &dest,
                Some("client".to_owned()),
                8192,
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
