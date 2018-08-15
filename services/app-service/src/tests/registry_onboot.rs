/*
 * Copyright (C) 2018 Kubos Corporation
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use std::io::Write;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::thread;
use std::time::Duration;

use tempfile::TempDir;

use registry::*;

#[test]
fn registry_onboot_good() {
    let registry_dir = TempDir::new().unwrap();
    let registry = AppRegistry::new_from_dir(&registry_dir.path().to_string_lossy());

    let app_dir = TempDir::new().unwrap();
    let app_bin = app_dir.path().join("tiny-app");

    fs::create_dir(app_bin.clone()).unwrap();

    let src = r#"
            #!/bin/bash
            exit 0
            "#;

    let mut bin = fs::File::create(app_bin.join("tiny-app")).unwrap();
    bin.write_all(src.as_bytes()).unwrap();
    let mut perms = bin.metadata().unwrap().permissions();
    perms.set_mode(0o755);
    bin.set_permissions(perms).unwrap();

    let manifest = r#"
            name = "tiny-app"
            version = "0.0.1"
            author = "user"
            "#;
    fs::write(app_bin.join("manifest.toml"), manifest).unwrap();

    registry.register(&app_bin.to_string_lossy()).unwrap();

    let result = registry.run_onboot();

    // Small sleep to prevent tiny-app from being destroyed before
    // the system finishes calling it
    thread::sleep(Duration::from_millis(10));

    assert!(result.is_ok());
}

#[test]
fn registry_onboot_fail() {
    let registry_dir = TempDir::new().unwrap();
    let registry = AppRegistry::new_from_dir(&registry_dir.path().to_string_lossy());

    let app_dir = TempDir::new().unwrap();
    let app_bin = app_dir.path().join("dummy");

    fs::create_dir(app_bin.clone()).unwrap();

    fs::File::create(app_bin.join("dummy")).unwrap();

    let manifest = r#"
            name = "dummy"
            version = "0.0.1"
            author = "user"
            "#;
    fs::write(app_bin.join("manifest.toml"), manifest).unwrap();

    registry.register(&app_bin.to_string_lossy()).unwrap();

    assert_eq!(
        registry.run_onboot(),
        Err("Failed to start 1 app/s".to_owned())
    );
}
