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
use std::fs;
use std::thread;
use std::time::Duration;
use tempfile::TempDir;

// NOTE: Each test's file contents must be unique. Otherwise the hash is the same, so
// the same storage directory is used across all of them, creating conflicts

// NOTE: The large_download test has been moved from this location to a new location:
//       test/integration/large_download

// Download single-chunk file from scratch
#[test]
fn download_single() {
    let test_dir = TempDir::new().expect("Failed to create test dir");
    let test_dir_str = test_dir.path().to_str().unwrap();
    let source = format!("{}/source", test_dir_str);
    let dest = format!("{}/dest", test_dir_str);
    let service_port = 8000;
    let downlink_port = 7000;

    let contents = "download_single".as_bytes();

    let hash = create_test_file(&source, &contents);

    let service_dir = format!("{}/service", test_dir_str);
    service_new!(service_port, downlink_port, 4096, service_dir);

    let result = download(
        "127.0.0.1",
        downlink_port,
        &format!("127.0.0.1:{}", service_port),
        &source,
        &dest,
        Some(format!("{}/client", test_dir_str)),
        4096,
    );
    result.unwrap();

    // Verify the final file's contents
    let dest_contents = fs::read(dest).unwrap();
    assert_eq!(&contents[..], dest_contents.as_slice());

    // Cleanup the temporary files so that the test can be repeatable
    // The client folder is cleaned up by the protocol as a result
    // of the hash mismatch
    let _ = fs::remove_dir_all(format!("service/storage/{}", hash));
}

// Download multi-chunk file from scratch
#[test]
fn download_multi_clean() {
    let test_dir = TempDir::new().expect("Failed to create test dir");
    let test_dir_str = test_dir.path().to_str().unwrap();
    let source = format!("{}/source", test_dir_str);
    let dest = format!("{}/dest", test_dir_str);
    let service_port = 8001;
    let downlink_port = 7001;

    let contents = [1; 6000];

    let hash = create_test_file(&source, &contents);

    let service_dir = format!("{}/service", test_dir_str);
    service_new!(service_port, downlink_port, 4096, service_dir);

    let result = download(
        "127.0.0.1",
        downlink_port,
        &format!("127.0.0.1:{}", service_port),
        &source,
        &dest,
        Some(format!("{}/client", test_dir_str)),
        4096,
    );
    result.unwrap();

    // Verify the final file's contents
    let dest_contents = fs::read(dest).unwrap();
    assert_eq!(&contents[..], dest_contents.as_slice());

    // Cleanup the temporary files so that the test can be repeatable
    // The client folder is cleaned up by the protocol as a result
    // of the hash mismatch
    let _ = fs::remove_dir_all(format!("service/storage/{}", hash));
}

// Download multi-chunk file which we already have 1 chunk for
#[test]
fn download_multi_resume() {
    let test_dir = TempDir::new().expect("Failed to create test dir");
    let test_dir_str = test_dir.path().to_str().unwrap();
    let source = format!("{}/source", test_dir_str);
    let dest = format!("{}/dest", test_dir_str);
    let service_port = 8002;
    let downlink_port = 7002;

    let contents = [2; 6000];

    let hash = create_test_file(&source, &contents);

    let service_dir = format!("{}/service", test_dir_str);
    service_new!(service_port, downlink_port, 4096, service_dir);

    // Download a partial file so that we can resume the download later
    let result = download_partial(
        "127.0.0.1",
        downlink_port,
        &format!("127.0.0.1:{}", service_port),
        &source,
        &dest,
        Some(format!("{}/client", test_dir_str)),
        4096,
    );
    // The download should *not* complete with success because we aren't
    // requesting all of the chunks
    assert!(result.is_err());

    // download the file again
    let result = download(
        "127.0.0.1",
        downlink_port,
        "127.0.0.1:8002",
        &source,
        &dest,
        Some(format!("{}/client", test_dir_str)),
        4096,
    );
    result.unwrap();

    // Verify the final file's contents
    let dest_contents = fs::read(dest).unwrap();
    assert_eq!(&contents[..], dest_contents.as_slice());

    // Cleanup the temporary files so that the test can be repeatable
    // The client folder is cleaned up by the protocol as a result
    // of the hash mismatch
    let _ = fs::remove_dir_all(format!("service/storage/{}", hash));
}

// Download multi-chunk file which we already have downloaded
// This should behave the same as two independent clean downloads
#[test]
fn download_multi_complete() {
    let test_dir = TempDir::new().expect("Failed to create test dir");
    let test_dir_str = test_dir.path().to_str().unwrap();
    let source = format!("{}/source", test_dir_str);
    let dest = format!("{}/dest", test_dir_str);
    let service_port = 8003;
    let downlink_port = 7003;

    let contents = "download_multi_complete".as_bytes();

    let hash = create_test_file(&source, &contents);

    let service_dir = format!("{}/service", test_dir_str);
    service_new!(service_port, downlink_port, 4096, service_dir);

    // download the file once (clean download)
    let result = download(
        "127.0.0.1",
        downlink_port,
        &format!("127.0.0.1:{}", service_port),
        &source,
        &dest,
        Some(format!("{}/client", test_dir_str)),
        4096,
    );
    result.unwrap();

    thread::sleep(Duration::from_millis(100));

    // download the file again
    let result = download(
        "127.0.0.1",
        downlink_port,
        &format!("127.0.0.1:{}", service_port),
        &source,
        &dest,
        Some(format!("{}/client", test_dir_str)),
        4096,
    );

    result.unwrap();

    // Verify the final file's contents
    let dest_contents = fs::read(dest).unwrap();
    assert_eq!(&contents[..], dest_contents.as_slice());

    // Cleanup the temporary files so that the test can be repeatable
    // The client folder is cleaned up by the protocol as a result
    // of the hash mismatch
    let _ = fs::remove_dir_all(format!("service/storage/{}", hash));
}

// Download. Create hash mismatch.
#[test]
fn download_bad_hash() {
    let test_dir = TempDir::new().expect("Failed to create test dir");
    let test_dir_str = test_dir.path().to_str().unwrap();
    let source = format!("{}/source", test_dir_str);
    let dest = format!("{}/dest", test_dir_str);
    let service_port = 8004;
    let downlink_port = 7004;

    let contents = "download_bad_hash".as_bytes();

    let hash = create_test_file(&source, &contents);

    let service_dir = format!("{}/service", test_dir_str);
    service_new!(service_port, downlink_port, 4096, service_dir);

    // Download the file so we can mess with the temporary storage
    let result = download(
        "127.0.0.1",
        downlink_port,
        &format!("127.0.0.1:{}", service_port),
        &source,
        &dest,
        Some(format!("{}/client", test_dir_str)),
        4096,
    );
    result.unwrap();

    // Recreate temp folder with bad chunk so that future hash calculation will fail
    fs::create_dir(format!("{}/client/storage/{}", test_dir_str, hash)).unwrap();
    fs::write(
        format!("{}/client/storage/{}/0", test_dir_str, hash),
        "bad data".as_bytes(),
    )
    .unwrap();

    let result = download(
        "127.0.0.1",
        downlink_port,
        &format!("127.0.0.1:{}", service_port),
        &source,
        &dest,
        Some(format!("{}/client", test_dir_str)),
        4096,
    );
    assert_eq!("File hash mismatch", format!("{}", result.unwrap_err()));
}
