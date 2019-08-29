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

// Test ability to kill running app
// Note: Negative test cases are in `src/tests/kill_app`

use kubos_app::ServiceConfig;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::thread;
use std::time::Duration;

mod utils;
pub use crate::utils::*;

fn setup_app(registry_dir: &Path) {
    // Build our test project
    Command::new("cargo")
        .arg("build")
        .current_dir("tests/utils/rust-proj")
        .status()
        .unwrap();

    // Add our project to the app registry
    let app_dir = registry_dir.join("rust-proj/1.0");

    fs::create_dir_all(app_dir.clone()).unwrap();

    // Copy our app executable into our app registry
    fs::copy(
        "tests/utils/rust-proj/target/debug/rust-proj",
        app_dir.join("rust-proj"),
    )
    .unwrap();

    // Copy our test file to make sure we can access it later
    fs::copy("tests/utils/rust-proj/testfile", app_dir.join("testfile")).unwrap();

    // Copy our config file to make sure we can access it later
    fs::copy(
        "tests/utils/rust-proj/config.toml",
        app_dir.join("config.toml"),
    )
    .unwrap();

    // Create our manifest file
    let toml = format!(
        r#"
            active_version = true
            run_level = "onCommand"

            [app]
            executable = "{}/rust-proj/1.0/rust-proj"
            name = "rust-proj"
            version = "1.0"
            author = "user"
            config = "/home/system/etc/config.toml"
            "#,
        registry_dir.to_string_lossy(),
    );

    fs::write(app_dir.join("app.toml"), toml).unwrap();
}

#[test]
fn kill_by_name() {
    let mut fixture = AppServiceFixture::setup();
    let config = ServiceConfig::new_from_path(
        "app-service",
        format!(
            "{}",
            fixture
                .registry_dir
                .path()
                .join("config.toml")
                .to_string_lossy()
        ),
    )
    .unwrap();

    setup_app(&fixture.registry_dir.path());

    fixture.start_service(false);

    let result = send_query(
        config.clone(),
        r#"mutation {
            startApp(name: "rust-proj", runLevel: "OnCommand", args: "-l") {
                errors,
                success
            }
        }"#,
    );

    assert_eq!(result["startApp"]["success"].as_bool().unwrap(), true);

    // Query the app service to make sure the app is still running
    let result = send_query(
        config.clone(),
        r#"{
            appStatus(name: "rust-proj") {
                running
            }
        }"#,
    );

    assert_eq!(result["appStatus"][0]["running"].as_bool().unwrap(), true);

    // Kill the app
    let result = send_query(
        config.clone(),
        r#"mutation {
            killApp(name: "rust-proj", runLevel: "OnCommand") {
                errors,
                success
            }
        }"#,
    );

    assert_eq!(result["killApp"]["success"].as_bool().unwrap(), true);

    // Give the service just a sec to figure out the app has stopped
    thread::sleep(Duration::from_millis(10));

    // The monitoring entry should now show that the app has stopped with a good last RC
    let result = send_query(
        config,
        r#"{
            appStatus(name: "rust-proj") {
                name,
                running,
                lastRc,
                lastSignal
            }
        }"#,
    );

    fixture.teardown();

    assert_eq!(
        result["appStatus"][0]["name"].as_str().unwrap(),
        "rust-proj"
    );
    assert_eq!(result["appStatus"][0]["running"].as_bool().unwrap(), false);
    assert_eq!(result["appStatus"][0]["lastRc"], serde_json::Value::Null);
    // The default kill signal value is 9
    assert_eq!(result["appStatus"][0]["lastSignal"].as_i64().unwrap(), 9);
}

#[test]
fn kill_by_pid() {
    let mut fixture = AppServiceFixture::setup();
    let config = ServiceConfig::new_from_path(
        "app-service",
        format!(
            "{}",
            fixture
                .registry_dir
                .path()
                .join("config.toml")
                .to_string_lossy()
        ),
    )
    .unwrap();

    setup_app(&fixture.registry_dir.path());

    fixture.start_service(false);

    let result = send_query(
        config.clone(),
        r#"mutation {
            startApp(name: "rust-proj", runLevel: "OnCommand", args: "-l") {
                errors,
                success,
                pid
            }
        }"#,
    );

    // This should be non-null since the app is still running
    let pid = result["startApp"]["pid"].as_i64().unwrap();

    // Query the app service to make sure the app is still running
    let result = send_query(
        config.clone(),
        r#"{
            appStatus(name: "rust-proj") {
                running
            }
        }"#,
    );

    assert_eq!(result["appStatus"][0]["running"].as_bool().unwrap(), true);

    // Kill the app
    let result = send_query(
        config.clone(),
        &format!(
            r#"mutation {{
            killApp(pid: {}) {{
                errors,
                success
            }}
        }}"#,
            pid
        ),
    );

    assert_eq!(result["killApp"]["success"].as_bool().unwrap(), true);

    // Give the service just a sec to figure out the app has stopped
    thread::sleep(Duration::from_millis(10));

    // The monitoring entry should now show that the app has stopped with a good last RC
    let result = send_query(
        config,
        r#"{
            appStatus(name: "rust-proj") {
                name,
                running,
                lastRc,
                lastSignal
            }
        }"#,
    );

    fixture.teardown();

    assert_eq!(
        result["appStatus"][0]["name"].as_str().unwrap(),
        "rust-proj"
    );
    assert_eq!(result["appStatus"][0]["running"].as_bool().unwrap(), false);
    assert_eq!(result["appStatus"][0]["lastRc"], serde_json::Value::Null);
    // The default kill signal value is 9
    assert_eq!(result["appStatus"][0]["lastSignal"].as_i64().unwrap(), 9);
}

#[test]
fn kill_custom_signal() {
    let mut fixture = AppServiceFixture::setup();
    let config = ServiceConfig::new_from_path(
        "app-service",
        format!(
            "{}",
            fixture
                .registry_dir
                .path()
                .join("config.toml")
                .to_string_lossy()
        ),
    )
    .unwrap();

    setup_app(&fixture.registry_dir.path());

    fixture.start_service(false);

    let result = send_query(
        config.clone(),
        r#"mutation {
            startApp(name: "rust-proj", runLevel: "OnCommand", args: "-l") {
                errors,
                success,
                pid
            }
        }"#,
    );

    // This should be non-null since the app is still running
    let pid = result["startApp"]["pid"].as_i64().unwrap();

    // Query the app service to make sure the app is still running
    let result = send_query(
        config.clone(),
        r#"{
            appStatus(name: "rust-proj") {
                running
            }
        }"#,
    );

    assert_eq!(result["appStatus"][0]["running"].as_bool().unwrap(), true);

    // Kill the app
    let result = send_query(
        config.clone(),
        &format!(
            r#"mutation {{
            killApp(pid: {}, signal: 2) {{
                errors,
                success
            }}
        }}"#,
            pid
        ),
    );

    assert_eq!(result["killApp"]["success"].as_bool().unwrap(), true);

    // Give the service just a sec to figure out the app has stopped
    thread::sleep(Duration::from_millis(10));

    // The monitoring entry should now show that the app has stopped with a good last RC
    let result = send_query(
        config,
        r#"{
            appStatus(name: "rust-proj") {
                name,
                running,
                lastRc,
                lastSignal
            }
        }"#,
    );

    fixture.teardown();

    assert_eq!(
        result["appStatus"][0]["name"].as_str().unwrap(),
        "rust-proj"
    );
    assert_eq!(result["appStatus"][0]["running"].as_bool().unwrap(), false);
    assert_eq!(result["appStatus"][0]["lastRc"], serde_json::Value::Null);
    assert_eq!(result["appStatus"][0]["lastSignal"].as_i64().unwrap(), 2);
}
