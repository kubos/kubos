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
use file_protocol::ProtocolError;
use file_service::recv_loop;
use kubos_system::Config as ServiceConfig;
use std::fs;
use std::thread;
use std::time::Duration;
use tempfile::TempDir;

// NOTE: Each test's file contents must be unique. Otherwise the hash is the same, so
// the same storage directory is used across all of them, creating conflicts

// NOTE: The large_upload test has been moved from this location to a new location:
//       test/integration/large_upload

// Upload single-chunk file from scratch
#[test]
fn upload_single() {
    let test_dir = TempDir::new().expect("Failed to create test dir");
    let test_dir_str = test_dir.path().to_str().unwrap();
    let source = format!("{}/source", test_dir_str);
    let dest = format!("{}/dest", test_dir_str);
    let service_port = 7000;
    let downlink_port = 6000;

    let contents = "upload_single".as_bytes();

    let hash = create_test_file(&source, contents);

    let storage_dir = format!("{}/service", test_dir_str);
    service_new!(service_port, downlink_port, 4096, storage_dir);

    let result = upload(
        "127.0.0.1",
        downlink_port,
        &format!("127.0.0.1:{}", service_port),
        &source,
        &dest,
        Some(format!("{}/client", test_dir_str)),
        4096,
    );

    if let Err(err) = &result {
        println!("Error: {}", err);
    }

    result.unwrap();

    // Verify the final file's contents
    let dest_contents = fs::read(dest).unwrap();

    assert_eq!(contents, dest_contents.as_slice());

    // Cleanup the temporary files so that the test can be repeatable
    // The client folder is cleaned up by the protocol as a result
    // of the hash mismatch
    let _ = fs::remove_dir_all(format!("service/storage/{}", hash));
}

// Upload multi-chunk file from scratch
#[test]
fn upload_multi_clean() {
    let test_dir = TempDir::new().expect("Failed to create test dir");
    let test_dir_str = test_dir.path().to_str().unwrap();
    let source = format!("{}/source", test_dir_str);
    let dest = format!("{}/dest", test_dir_str);
    let service_port = 7001;
    let downlink_port = 6001;

    let contents = [1; 5000];

    let hash = create_test_file(&source, &contents);

    let storage_dir = format!("{}/service", test_dir_str);
    service_new!(service_port, downlink_port, 4096, storage_dir);

    let result = upload(
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

// Upload multi-chunk file which we already have 1 chunk for
#[test]
fn upload_multi_resume() {
    let test_dir = TempDir::new().expect("Failed to create test dir");
    let test_dir_str = test_dir.path().to_str().unwrap();
    let source = format!("{}/source", test_dir_str);
    let dest = format!("{}/dest", test_dir_str);
    let service_port = 7002;
    let downlink_port = 6002;

    let contents = [2; 5000];

    let hash = create_test_file(&source, &contents);

    let storage_dir = format!("{}/service", test_dir_str);
    service_new!(service_port, downlink_port, 4096, storage_dir);

    // Upload a partial version of the file
    let result = upload_partial(
        "127.0.0.1",
        downlink_port,
        "127.0.0.1:7002",
        &source,
        &dest,
        Some(format!("{}/client", test_dir_str)),
        4096,
    );
    assert!(result.is_err());

    // Upload the whole file this time
    let result = upload(
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

// Upload multi-chunk file which we already have all chunks for
#[test]
fn upload_multi_complete() {
    let test_dir = TempDir::new().expect("Failed to create test dir");
    let test_dir_str = test_dir.path().to_str().unwrap();
    let source = format!("{}/source", test_dir_str);
    let dest = format!("{}/dest", test_dir_str);
    let service_port = 7005;
    let downlink_port = 6005;

    let contents = [3; 5000];

    let hash = create_test_file(&source, &contents);

    let storage_dir = format!("{}/service", test_dir_str);
    service_new!(service_port, downlink_port, 4096, storage_dir);

    // Upload the file once (clean upload)
    let result = upload(
        "127.0.0.1",
        downlink_port,
        &format!("127.0.0.1:{}", service_port),
        &source,
        &dest,
        Some(format!("{}/client", test_dir_str)),
        4096,
    );
    result.unwrap();

    // Upload the file again
    let result = upload(
        "127.0.0.1",
        downlink_port,
        "127.0.0.1:7005",
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

// Upload. Create hash mismatch.
#[test]
fn upload_bad_hash() {
    let test_dir = TempDir::new().expect("Failed to create test dir");
    let test_dir_str = test_dir.path().to_str().unwrap();
    let source = format!("{}/source", test_dir_str);
    let dest = format!("{}/dest", test_dir_str);
    let service_port = 7003;
    let downlink_port = 6003;

    let contents = "upload_bad_hash".as_bytes();

    let _ = create_test_file(&source, contents);

    let storage_dir = format!("{}/service", test_dir_str);
    service_new!(service_port, downlink_port, 4096, storage_dir);

    // Upload the file so we can mess with the temporary storage
    let result = upload(
        "127.0.0.1",
        downlink_port,
        &format!("127.0.0.1:{}", service_port),
        &source,
        &dest,
        Some(format!("{}/client", test_dir_str)),
        4096,
    );
    assert!(result.is_ok());
    let hash = result.unwrap();

    // Give the service a moment to go through its cleanup logic before we mess with things
    thread::sleep(Duration::from_millis(10));

    // Create temp folder with bad chunk so that future hash calculation will fail
    fs::create_dir(format!("{}/service/storage/{}", test_dir_str, hash)).unwrap();
    fs::write(
        format!("{}/service/storage/{}/0", test_dir_str, hash),
        "bad data".as_bytes(),
    )
    .unwrap();

    let result = upload(
        "127.0.0.1",
        downlink_port,
        "127.0.0.1:7003",
        &source,
        &dest,
        Some(format!("{}/client", test_dir_str)),
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
    let _ = fs::remove_dir_all(format!("client/storage/{}", hash));
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
    let downlink_port = 6007;

    let contents = "upload_single_after_bad_input".as_bytes();

    let hash = create_test_file(&source, contents);

    let storage_dir = format!("{}/service", test_dir_str);
    service_new!(service_port, downlink_port, 4096, storage_dir);

    {
        let send_socket = UdpSocket::bind("127.0.0.1:0").unwrap();
        let send_buf = "{ping}".as_bytes();
        send_socket.send_to(send_buf, "127.0.0.1:7007").unwrap();
    }

    let result = upload(
        "127.0.0.1",
        downlink_port,
        &format!("127.0.0.1:{}", service_port),
        &source,
        &dest,
        Some(format!("{}/client", test_dir_str)),
        4096,
    );

    if let Err(err) = &result {
        println!("Error: {}", err);
    }

    result.unwrap();

    // Verify the final file's contents
    let dest_contents = fs::read(dest).unwrap();
    assert_eq!(contents, dest_contents.as_slice());

    // Cleanup the temporary files so that the test can be repeatable
    // The client folder is cleaned up by the protocol as a result
    // of the hash mismatch
    let _ = fs::remove_dir_all(format!("service/storage/{}", hash));
}
