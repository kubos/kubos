/*
 * Copyright (C) 2019 Kubos Corporation
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

use kubos_service::{Config, Service};
use serde_json::json;
use std::fs;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;

use tempfile::TempDir;

use crate::registry::*;
use crate::schema;

#[test]
fn kill_app_no_input() {
    let registry_dir = TempDir::new().unwrap();
    let service = mock_service!(registry_dir);

    let kill_request = r#"mutation {
        killApp {
            errors,
            success
        }
    }"#;

    let result = request!(service, kill_request);

    let expected =
        "{\"data\":null,\
         \"errors\":\
         [{\"message\":\"Bad input arguments\",\
         \"locations\":[{\"line\":1,\"column\":19}],\
         \"path\":[\"killApp\"],\
         \"extensions\":{\"Bad request\":\"`name`/`runLevel` or `pid` must be specified\"}}]}";
    assert_eq!(result.body(), expected);
}

#[test]
fn kill_app_no_runlevel() {
    let registry_dir = TempDir::new().unwrap();
    let service = mock_service!(registry_dir);

    let kill_request = r#"mutation {
        killApp(name: \"dummy\") {
            errors,
            success
        }
    }"#;

    let result = request!(service, kill_request);

    let expected =
        "{\"data\":null,\
         \"errors\":\
         [{\"message\":\"Bad input arguments\",\
         \"locations\":[{\"line\":1,\"column\":19}],\
         \"path\":[\"killApp\"],\
         \"extensions\":{\"Bad request\":\"`name` and `runLevel` must both be specified\"}}]}";
    assert_eq!(result.body(), expected);
}

#[test]
fn kill_app_name_pid() {
    let registry_dir = TempDir::new().unwrap();
    let service = mock_service!(registry_dir);

    let kill_request = r#"mutation {
        killApp(name: \"dummy\", pid: 5) {
            errors,
            success
        }
    }"#;

    let result = request!(service, kill_request);

    let expected = "{\"data\":null,\
        \"errors\":\
            [{\"message\":\"Bad input arguments\",\
            \"locations\":[{\"line\":1,\"column\":19}],\
            \"path\":[\"killApp\"],\
            \"extensions\":{\"Bad request\":\"`pid` is mutually exclusive with `name` and `runLevel`\"}}]}";
    assert_eq!(result.body(), expected);
}

#[test]
fn kill_app_name_runlevel() {
    let registry_dir = TempDir::new().unwrap();
    let service = mock_service!(registry_dir);

    let kill_request = r#"mutation {
        killApp(runLevel: \"OnBoot\", pid: 5) {
            errors,
            success
        }
    }"#;

    let result = request!(service, kill_request);

    let expected = "{\"data\":null,\
        \"errors\":\
            [{\"message\":\"Bad input arguments\",\
            \"locations\":[{\"line\":1,\"column\":19}],\
            \"path\":[\"killApp\"],\
            \"extensions\":{\"Bad request\":\"`pid` is mutually exclusive with `name` and `runLevel`\"}}]}";
    assert_eq!(result.body(), expected);
}

#[test]
fn kill_app_bad_name() {
    let registry_dir = TempDir::new().unwrap();
    let service = mock_service!(registry_dir);

    let kill_request = r#"mutation {
        killApp(name: \"dummy\", runLevel: \"OnBoot\") {
            errors,
            success
        }
    }"#;

    let expected = json!({
       "killApp": {
           "errors": "Failed to kill app: No matching monitoring entry found",
           "success": false
        }
    });

    test!(service, kill_request, expected);
}

#[test]
fn kill_app_bad_pid() {
    let registry_dir = TempDir::new().unwrap();
    let service = mock_service!(registry_dir);

    let kill_request = r#"mutation {
        killApp(pid: 5) {
            errors,
            success
        }
    }"#;

    let expected = json!({
        "killApp": {
           "errors": "Failed to kill app: No matching monitoring entry found",
           "success": false
        }
    });

    test!(service, kill_request, expected);
}

#[test]
fn kill_app_not_running() {
    let registry_dir = TempDir::new().unwrap();
    let service = mock_service!(registry_dir);

    let app_dir = TempDir::new().unwrap();
    let app_bin = app_dir.path().join("dummy-app");

    fs::create_dir(app_bin.clone()).unwrap();

    let src = r#"
            #!/bin/bash
            exit 0
            "#;

    let mut bin = fs::File::create(app_bin.join("dummy")).unwrap();
    bin.write_all(src.as_bytes()).unwrap();
    let mut perms = bin.metadata().unwrap().permissions();
    perms.set_mode(0o755);
    bin.set_permissions(perms).unwrap();

    let manifest = r#"
            name = "dummy"
            version = "0.0.1"
            author = "user"
            "#;
    fs::write(app_bin.join("manifest.toml"), manifest).unwrap();

    let register = format!(
        r#"mutation {{
        register(path: \"{}\") {{
            success,
        }}
    }}"#,
        app_bin.to_str().unwrap()
    );

    let _ = request!(service, register);

    let start = r#"mutation {
        startApp(name: \"dummy\", runLevel: \"OnCommand\") {
            errors,
            success
        }
    }"#;

    let _ = request!(service, start);

    let kill = r#"mutation {
        killApp(name: \"dummy\", runLevel: \"OnCommand\") {
            errors,
            success
        }
    }"#;

    let expected = json!({
        "killApp": {
           "errors": "Failed to kill app: No matching monitoring entry found",
           "success": false
        }
    });

    test!(service, kill, expected);
}
