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

use std::fs;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::thread;
use std::time::Duration;

use tempfile::TempDir;

use crate::error::*;
use crate::registry::*;

#[test]
fn registry_onboot_good() {
    let registry_dir = TempDir::new().unwrap();
    let registry = AppRegistry::new_from_dir(&registry_dir.path().to_string_lossy()).unwrap();

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

    registry.register(&app_bin.to_string_lossy(), None).unwrap();

    let result = registry.run_onboot();

    // Small sleep to prevent tiny-app from being destroyed before
    // the system finishes calling it
    thread::sleep(Duration::from_millis(10));

    assert!(result.is_ok());
}

#[test]
fn registry_onboot_bad() {
    let registry_dir = TempDir::new().unwrap();
    let registry = AppRegistry::new_from_dir(&registry_dir.path().to_string_lossy()).unwrap();

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

    registry.register(&app_bin.to_string_lossy(), None).unwrap();

    assert_eq!(
        registry.run_onboot().unwrap_err(),
        AppError::SystemError {
            err: "Failed to start 1 app/s".to_owned()
        }
    );
}

#[test]
fn registry_onboot_fail() {
    let registry_dir = TempDir::new().unwrap();
    let registry = AppRegistry::new_from_dir(&registry_dir.path().to_string_lossy()).unwrap();

    let app_dir = TempDir::new().unwrap();
    let app_bin = app_dir.path().join("tiny-app");

    fs::create_dir(app_bin.clone()).unwrap();

    let src = r#"
            #!/bin/bash
            exit 1
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

    registry.register(&app_bin.to_string_lossy(), None).unwrap();

    let result = registry.run_onboot();

    assert_eq!(
        result.unwrap_err(),
        AppError::SystemError {
            err: "Failed to start 1 app/s".to_owned()
        }
    );
}

#[test]
fn registry_onboot_preexisting() {
    let registry_dir = TempDir::new().unwrap();

    // Create a pre-existing app for our registry to discover
    //
    // Since we're creating the app files directly in the app registry, we need to manually
    // control the lifetime of the app binary so that all the data gets written and the file gets
    // closed before we attempt to execute it
    {
        let app_dir = registry_dir.path().join("a-b-c-d-e/1.0");

        fs::create_dir_all(app_dir.clone()).unwrap();

        let src = r#"
            #!/bin/bash
            exit 0
            "#;

        let mut bin = fs::File::create(app_dir.join("tiny-app")).unwrap();
        bin.write_all(src.as_bytes()).unwrap();
        let mut perms = bin.metadata().unwrap().permissions();
        perms.set_mode(0o755);
        bin.set_permissions(perms).unwrap();

        let toml = format!(
            r#"
                active_version = true
                run_level = "onCommand"
    
                [app]
                uuid = "a-b-c-d-e"
                pid = 0
                path = "{}/a-b-c-d-e/1.0/tiny-app"
    
                [app.metadata]
                name = "tiny-app"
                version = "1.0"
                author = "user"
                "#,
            registry_dir.path().to_string_lossy(),
        );

        fs::write(app_dir.join("app.toml"), toml).unwrap();
    }

    // Create the registry
    let registry = AppRegistry::new_from_dir(&registry_dir.path().to_string_lossy()).unwrap();

    let result = registry.run_onboot();

    // Small sleep to prevent tiny-app from being destroyed before
    // the system finishes calling it
    thread::sleep(Duration::from_millis(10));

    assert!(result.is_ok());
}
