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

// Test starting a Python application with and without additional arguments
//
// Note: The app service cannot verify the return code of Python apps, which means that CI cannot
// properly run these tests. You'll need to manually run these tests and verify the output by
// checking for any extraneous error messages.

use fs_extra;
use kubos_app::ServiceConfig;
use std::fs;
use std::path::Path;
use std::thread;
use std::time::Duration;

mod utils;
pub use crate::utils::*;

fn setup_app(registry_dir: &Path) {
    // Add our project to the app registry
    let app_dir = registry_dir.join("python-proj/1.0");

    fs::create_dir_all(app_dir.clone()).unwrap();

    // Copy our app files into our app registry
    fs::copy("tests/utils/python-proj/main.py", app_dir.join("main.py")).unwrap();
    fs::copy(
        "tests/utils/python-proj/config.toml",
        app_dir.join("config.toml"),
    )
    .unwrap();
    fs_extra::dir::copy(
        "tests/utils/python-proj/sub",
        app_dir.clone(),
        &fs_extra::dir::CopyOptions::new(),
    )
    .unwrap();

    // Create our manifest file
    let toml = format!(
        r#"
            active_version = true
            run_level = "onCommand"

            [app]
            executable = "{}/python-proj/1.0/main.py"
            name = "python-proj"
            version = "1.0"
            author = "user"
            config = "/home/system/etc/config.toml"
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

    setup_app(&fixture.registry_dir.path());

    fixture.start_service(false);

    let result = send_query(
        config,
        r#"mutation {
            startApp(name: "python-proj", runLevel: "OnBoot", config: "config.toml") {
                errors,
                success
            }
        }"#,
    );

    // Give the app a moment to run successfully before we tear everything down
    thread::sleep(Duration::from_millis(400));

    fixture.teardown();

    assert!(result["startApp"]["success"].as_bool().unwrap());
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

    setup_app(&fixture.registry_dir.path());

    fixture.start_service(false);

    let result = send_query(
        config,
        r#"mutation {
            startApp(name: "python-proj", runLevel: "OnCommand", config: "config.toml", args: ["pos"]) {
                errors,
                success
            }
        }"#,
    );

    thread::sleep(Duration::from_millis(400));

    fixture.teardown();

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

    setup_app(&fixture.registry_dir.path());

    fixture.start_service(false);

    let result = send_query(
        config,
        r#"mutation {
            startApp(name: "python-proj", runLevel: "OnCommand", config: "config.toml", args: ["-f"]) {
                errors,
                success
            }
        }"#,
    );

    thread::sleep(Duration::from_millis(400));

    fixture.teardown();

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

    setup_app(&fixture.registry_dir.path());

    fixture.start_service(false);

    let result = send_query(
        config,
        r#"mutation {
            startApp(name: "python-proj", runLevel: "OnCommand", config: "config.toml", args: ["-t", "test"]) {
                errors,
                success
            }
        }"#,
    );

    thread::sleep(Duration::from_millis(400));

    fixture.teardown();

    assert!(result["startApp"]["success"].as_bool().unwrap());
}

#[test]
fn app_failure() {
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

    // -e forces an error.
    let result = send_query(
        config,
        r#"mutation {
            startApp(name: "python-proj", runLevel: "OnCommand", config: "config.toml", args: ["-e"]) {
                errors,
                success
            }
        }"#,
    );

    thread::sleep(Duration::from_millis(400));

    fixture.teardown();

    assert!(!result["startApp"]["success"].as_bool().unwrap());
}
