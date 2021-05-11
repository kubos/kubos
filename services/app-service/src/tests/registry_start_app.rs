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
fn start_app_good() {
    let registry_dir = TempDir::new().unwrap();

    // Since we're creating the app files directly in the app registry, we need to manually
    // control the lifetime of the app binary so that all the data gets written and the file gets
    // closed before we attempt to execute it
    {
        let app_dir = registry_dir.path().join("tiny-app/1.0");

        fs::create_dir_all(app_dir.clone()).unwrap();

        let src = r#"#!/bin/bash
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
    
                [app]
                executable = "{}/tiny-app/1.0/tiny-app"
                name = "tiny-app"
                version = "1.0"
                author = "user"
                config = "/custom/config.toml"
                "#,
            registry_dir.path().to_string_lossy(),
        );

        fs::write(app_dir.join("app.toml"), toml).unwrap();
    }

    // Create the registry
    let registry = AppRegistry::new_from_dir(&registry_dir.path().to_string_lossy()).unwrap();

    let result = registry.start_app("tiny-app", None, None);

    // Small sleep to prevent tiny-app from being destroyed before
    // the system finishes calling it
    thread::sleep(Duration::from_millis(10));

    eprintln!("Result: {:?}", result);
    assert!(result.is_ok());
}

#[test]
fn start_app_fail() {
    let registry_dir = TempDir::new().unwrap();

    // Since we're creating the app files directly in the app registry, we need to manually
    // control the lifetime of the app binary so that all the data gets written and the file gets
    // closed before we attempt to execute it
    {
        let app_dir = registry_dir.path().join("tiny-app/1.0");

        fs::create_dir_all(app_dir.clone()).unwrap();

        // Create an empty, non-executable app file
        fs::File::create(app_dir.join("dummy")).unwrap();

        let toml = format!(
            r#"
                active_version = true
    
                [app]
                executable = "{}/tiny-app/1.0/dummy"
                name = "tiny-app"
                version = "1.0"
                author = "user"
                config = "/custom/config.toml"
                "#,
            registry_dir.path().to_string_lossy(),
        );

        fs::write(app_dir.join("app.toml"), toml).unwrap();
    }

    // Create the registry
    let registry = AppRegistry::new_from_dir(&registry_dir.path().to_string_lossy()).unwrap();

    let result = registry.start_app("tiny-app", None, None);

    assert_eq!(
        result.unwrap_err(),
        AppError::StartError {
            err: "Failed to spawn app: Os { code: 13, kind: PermissionDenied, message: \"Permission denied\" }".to_owned()
        }
    );
}

#[test]
fn start_app_bad() {
    let registry_dir = TempDir::new().unwrap();

    // Create a pre-existing app for our registry to discover,
    // but omit the actual executable file
    let app_dir = registry_dir.path().join("tiny-app/1.0");

    fs::create_dir_all(app_dir.clone()).unwrap();

    let toml = format!(
        r#"
            active_version = true

            [app]
            executable = "{}/tiny-app/1.0/tiny-app"
            name = "tiny-app"
            version = "1.0"
            author = "user"
            config = "/custom/config.toml"
            "#,
        registry_dir.path().to_string_lossy(),
    );

    fs::write(app_dir.join("app.toml"), toml).unwrap();

    // Create the registry
    let registry = AppRegistry::new_from_dir(&registry_dir.path().to_string_lossy()).unwrap();

    let result = registry.start_app("tiny-app", None, None);

    match result.unwrap_err() {
        AppError::StartError { err } => {
            assert!(err.contains("tiny-app/1.0/tiny-app does not exist. tiny-app version 1.0 automatically uninstalled"));
        }
        other => panic!("Unexpected error received: {}", other),
    }

    // Make sure our bad app entry was removed from the registry directory
    assert!(!app_dir.exists());
}

#[test]
fn start_app_nonzero_rc() {
    let registry_dir = TempDir::new().unwrap();

    // Since we're creating the app files directly in the app registry, we need to manually
    // control the lifetime of the app binary so that all the data gets written and the file gets
    // closed before we attempt to execute it
    {
        let app_dir = registry_dir.path().join("tiny-app/1.0");

        fs::create_dir_all(app_dir.clone()).unwrap();

        let src = r#"#!/bin/bash
            exit 1
            "#;

        let mut bin = fs::File::create(app_dir.join("tiny-app")).unwrap();
        bin.write_all(src.as_bytes()).unwrap();
        let mut perms = bin.metadata().unwrap().permissions();
        perms.set_mode(0o755);
        bin.set_permissions(perms).unwrap();

        let toml = format!(
            r#"
                active_version = true
    
                [app]
                executable = "{}/tiny-app/1.0/tiny-app"
                name = "tiny-app"
                version = "1.0"
                author = "user"
                config = "/custom/config.toml"
                "#,
            registry_dir.path().to_string_lossy(),
        );

        fs::write(app_dir.join("app.toml"), toml).unwrap();
    }

    // Create the registry
    let registry = AppRegistry::new_from_dir(&registry_dir.path().to_string_lossy()).unwrap();

    let result = registry.start_app("tiny-app", None, None);

    // Small sleep to prevent tiny-app from being destroyed before
    // the system finishes calling it
    thread::sleep(Duration::from_millis(10));

    assert_eq!(
        result.unwrap_err(),
        AppError::StartError {
            err: "App returned exit code: 1".to_owned()
        }
    );
}
