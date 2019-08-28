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

// Test application monitoring

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
fn monitor_good() {
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

    fixture.start_service(true);

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
            runningApps(name: "rust-proj") {
                name,
                version,
                pid,
                config,
                args,
                runLevel,
                startTime
            }
        }"#,
    );

    // Make sure our app info matches what we'd expect
    let args: Vec<&str> = result["runningApps"][0]["args"]
        .as_array()
        .unwrap()
        .iter()
        .map(|val| val.as_str().unwrap())
        .collect();
    assert_eq!(
        result["runningApps"][0]["name"].as_str().unwrap(),
        "rust-proj"
    );
    assert_eq!(result["runningApps"][0]["version"].as_str().unwrap(), "1.0");
    assert_eq!(result["runningApps"][0]["pid"].as_i64().unwrap(), pid);
    assert_eq!(
        result["runningApps"][0]["config"].as_str().unwrap(),
        "/home/system/etc/config.toml"
    );
    assert_eq!(args, ["--", "-l"]);
    assert_eq!(
        result["runningApps"][0]["runLevel"].as_str().unwrap(),
        "OnCommand"
    );
    assert!(result["runningApps"][0]["startTime"].is_string());

    // The app has its own 2 second sleep time, so we need to wait that long for it to finish
    thread::sleep(Duration::from_secs(2));

    // The `runningApps` query should now return an empty array
    let result = send_query(
        config,
        r#"{
            runningApps(name: "rust-proj") {
                name
            }
        }"#,
    );

    fixture.teardown();

    let array = result["runningApps"].as_array().unwrap();

    assert!(array.is_empty());
}

#[test]
fn monitor_existing_bad() {
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

    fixture.start_service(true);

    let start_app = r#"mutation {
            startApp(name: "rust-proj", runLevel: "OnCommand", args: "-l") {
                errors
            }
        }"#;

    send_query(config.clone(), start_app);

    // If we try to start the app a second time, it should fail
    let result = send_query(config.clone(), start_app);

    fixture.teardown();

    assert_eq!(
        result["startApp"]["errors"].as_str().unwrap(),
        "Failed to start app: Instance of rust-proj already running OnCommand"
    );
}

#[test]
fn monitor_existing_good() {
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

    fixture.start_service(true);

    send_query(
        config.clone(),
        r#"mutation {
            startApp(name: "rust-proj", runLevel: "OnCommand", args: "-l") {
                errors
            }
        }"#,
    );

    // If we try to start the app a second time, it should pass, because we're starting with the
    // other run level
    let result = send_query(
        config.clone(),
        r#"mutation {
            startApp(name: "rust-proj", runLevel: "OnBoot") {
                success
            }
        }"#,
    );

    fixture.teardown();

    assert!(result["startApp"]["success"].as_bool().unwrap());
}
