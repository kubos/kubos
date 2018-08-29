extern crate cbor_protocol;
extern crate file_protocol;
extern crate file_service_rust;
extern crate kubos_system;
extern crate rand;
extern crate tempfile;

use cbor_protocol::Protocol as CborProtocol;
use file_protocol::{storage, FileProtocol};
use file_service_rust::recv_loop;
use kubos_system::Config as ServiceConfig;
use rand::{thread_rng, Rng};
use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
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
                [file-transfer-service.addr]
                ip = "127.0.0.1"
                port = {}
                "#,
                    $port
                ),
            )).unwrap();
        });
    }};
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

    let contents = "test1".as_bytes();

    create_test_file(&source, &contents);

    service_new!(service_port);

    let result = file_protocol::upload(service_port, &source, &dest);

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

    service_new!(service_port);

    let result = file_protocol::upload(service_port, &source, &dest);

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
    let result = file_protocol::upload(service_port, &source, &dest);
    assert!(result.is_ok());
    let hash = result.unwrap();

    // TODO: Remove this sleep. We need it to let the service
    // finish its work. The upload logic needs to wait on
    // the final ACK message before returning
    thread::sleep(Duration::new(2, 0));

    // Remove a chunk so we can test the retry logic
    fs::remove_file(format!("storage/{}/0", hash)).unwrap();

    // Upload the file again
    let result = file_protocol::upload(service_port, &source, &dest);
    assert!(result.is_ok());

    // TODO: Remove this sleep. We need it to let the service
    // finish its work. The upload logic needs to wait on
    // the final ACK message before returning
    thread::sleep(Duration::new(2, 0));

    // Cleanup the temporary files so that the test can be repeatable
    fs::remove_dir_all(format!("storage/{}", hash)).unwrap();

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
    let result = file_protocol::upload(service_port, &source, &dest);
    assert!(result.is_ok());
    let hash = result.unwrap();

    // TODO: Remove this sleep. We need it to let the service
    // finish its work. The upload logic needs to wait on
    // the final ACK message before returning
    thread::sleep(Duration::new(2, 0));

    // Upload the file again
    let result = file_protocol::upload(service_port, &source, &dest);
    assert!(result.is_ok());

    // TODO: Remove this sleep. We need it to let the service
    // finish its work. The upload logic needs to wait on
    // the final ACK message before returning
    thread::sleep(Duration::new(2, 0));

    // Cleanup the temporary files so that the test can be repeatable
    fs::remove_dir_all(format!("storage/{}", hash)).unwrap();

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

    let contents = "test1".as_bytes();

    create_test_file(&source, &contents);

    service_new!(service_port);

    // Upload the file so we can mess with the temporary storage
    let result = file_protocol::upload(service_port, &source, &dest);
    assert!(result.is_ok());
    let hash = result.unwrap();

    // TODO: Remove this sleep. We need it to let the service
    // finish its work. The upload logic needs to wait on
    // the final ACK message before returning
    thread::sleep(Duration::new(2, 0));

    // Tweak the chunk contents so the future hash calculation will fail
    fs::write(format!("storage/{}/0", hash), "bad data".as_bytes()).unwrap();

    // TODO: THIS SHOULD FAIL
    let result = file_protocol::upload(service_port, &source, &dest);
    // TODO: Verify exact error message
    assert!(result.is_ok());

    // TODO: Remove this sleep. We need it to let the service
    // finish its work. The upload logic needs to wait on
    // the final ACK message before returning
    thread::sleep(Duration::new(2, 0));

    // Cleanup the temporary files so that the test can be repeatable
    fs::remove_dir_all(format!("storage/{}", hash)).unwrap();
}

/*
#[test]
fn upload_multi_client() {
    let service_port = 7004;

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
            let contents = [num; 5000];

            create_test_file(&source, &contents);

            let result = file_protocol::upload(service_port, &source, &dest);
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

// // Massive upload
// // TODO: Enable once chunk numbers > 9 are supported properly

// #[test]
// fn upload_large() {
//     let test_dir = TempDir::new().expect("Failed to create test dir");
//     let test_dir_str = test_dir.path().to_str().unwrap();
//     let source = format!("{}/source", test_dir_str);
//     let dest = format!("{}/dest", test_dir_str);
//     let service_port = 7006;

//     let mut contents = [0u8; 1_000_000];
//     thread_rng().fill(&mut contents[..]);

//     create_test_file(&source, &contents);

//     service_new!(service_port);

//     let result = upload(service_port, &source, &dest);

//     assert!(result.is_ok());

//     let hash = result.unwrap();

//     // TODO: Remove this sleep. We need it to let the service
//     // finish its work. The upload logic needs to wait on
//     // the final ACK message before returning
//     thread::sleep(Duration::new(1, 0));

//     // Cleanup the temporary files so that the test can be repeatable
//     fs::remove_dir_all(format!("storage/{}", hash)).unwrap();

//     // Verify the final file's contents
//     let dest_contents = fs::read(dest).unwrap();
//     assert_eq!(&contents[..], dest_contents.as_slice());
// }
