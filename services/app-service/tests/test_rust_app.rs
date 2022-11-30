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

// Test starting a Rust application with and without additional arguments

use kubos_app::ServiceConfig;
use std::fs;
use std::path::Path;
use std::process::Command;

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

            [app]
            executable = "{}/rust-proj/1.0/rust-proj"
            name = "rust-proj"
            version = "1.0"
            author = "user"
            config = "/etc/kubos-config.toml"
            "#,
        registry_dir.to_string_lossy(),
    );

    fs::write(app_dir.join("app.toml"), toml).unwrap();
}

#[test]
fn app_no_args() {
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

    setup_app(fixture.registry_dir.path());

    fixture.start_service();

    let result = send_query(
        config,
        r#"mutation {
            startApp(name: "rust-proj") {
                errors,
                success
            }
        }"#,
    );

    // The test app is setup to verify arguments, so for this case we want to make sure it failed
    // as expected
    assert_eq!(
        result["startApp"]["errors"].as_str().unwrap(),
        "Failed to start app: App returned exit status: 1"
    );
}

#[test]
fn app_single_pos_arg() {
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

    setup_app(fixture.registry_dir.path());

    fixture.start_service();

    let result = send_query(
        config,
        r#"mutation {
            startApp(name: "rust-proj", args: ["pos"]) {
                errors,
                success
            }
        }"#,
    );

    assert!(result["startApp"]["success"].as_bool().unwrap());
}

#[test]
fn app_single_flag() {
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

    setup_app(fixture.registry_dir.path());

    fixture.start_service();

    let result = send_query(
        config,
        r#"mutation {
            startApp(name: "rust-proj", args: ["-f"]) {
                errors,
                success
            }
        }"#,
    );

    assert!(result["startApp"]["success"].as_bool().unwrap());
}

#[test]
fn app_flag_arg() {
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

    setup_app(fixture.registry_dir.path());

    fixture.start_service();

    let result = send_query(
        config,
        r#"mutation {
            startApp(name: "rust-proj", args: ["-t", "test"]) {
                errors,
                success
            }
        }"#,
    );

    assert!(result["startApp"]["success"].as_bool().unwrap());
}

#[test]
fn app_custom_config() {
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

    setup_app(fixture.registry_dir.path());

    fixture.start_service();

    let result = send_query(
        config,
        r#"mutation {
            startApp(name: "rust-proj", config: "config.toml", args: ["-f"]) {
                errors,
                success
            }
        }"#,
    );

    assert!(result["startApp"]["success"].as_bool().unwrap());
}
