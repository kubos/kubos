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

// Request a cleanup of a specific hash's temp storage
#[test]
fn cleanup_hash_dir() {
    let test_dir = TempDir::new().expect("Failed to create test dir");
    let test_dir_str = test_dir.path().to_str().unwrap();
    let source = format!("{}/source", test_dir_str);
    let dest = format!("{}/dest", test_dir_str);
    let service_port = 8002;

    let contents = [2; 6000];

    let hash = create_test_file(&source, &contents);

    service_new!(service_port, 4096);

    // Download a partial file so that we can resume the download later
    let _result = download_partial(
        "127.0.0.1",
        &format!("127.0.0.1:{}", service_port),
        &source,
        &dest,
        Some("client".to_owned()),
        4096,
    );

    // Storage directory should still exist after successful transfer
    assert!(fs::read_dir(format!("service/storage/{}", hash)).is_ok());

    cleanup(
        "127.0.0.1",
        &format!("127.0.0.1:{}", service_port),
        Some(hash.to_owned()),
        Some("client".to_owned()),
        4069,
    )
    .unwrap();

    // Hash's storage directory should be gone after request for cleanup
    assert!(fs::read_dir(format!("service/storage/{}", hash)).is_err());

    // General storage directory should still exist after request for hash cleanup
    assert!(fs::read_dir(format!("service/storage")).is_ok());
}
