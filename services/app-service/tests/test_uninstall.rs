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
#![deny(warnings)]

use kubos_app::ServiceConfig;
use nix::sys::signal;
use nix::unistd::Pid;
use std::fs;
use std::panic;
use std::path::Path;
use std::process::Command;
use std::thread;
use std::time::Duration;

use tempfile::TempDir;

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
            config = "/home/system/etc/config.toml"
            "#,
        registry_dir.to_string_lossy(),
    );

    fs::write(app_dir.join("app.toml"), toml).unwrap();
}

#[test]
fn uninstall_last_app() {
    let mut fixture = AppServiceFixture::setup();
    let config = format!(
        "{}",
        fixture
            .registry_dir
            .path()
            .join("config.toml")
            .to_string_lossy()
    );
    let mut app = MockAppBuilder::new("dummy");
    app.active(true).version("0.0.1").author("user");

    app.install(&fixture.registry_dir.path());
    fixture.start_service();

    // Make sure our app directory and active symlink exist
    assert_eq!(fixture.registry_dir.path().join("dummy").exists(), true);
    assert!(
        fs::symlink_metadata(fixture.registry_dir.path().join("active/dummy")).is_ok(),
        true
    );

    let result = panic::catch_unwind(|| {
        let result = send_query(
            ServiceConfig::new_from_path("app-service", config.to_owned()).unwrap(),
            r#"mutation {
            uninstall(name: "dummy", version: "0.0.1") {
                errors,
                success
            }
        }"#,
        );

        assert!(result["uninstall"]["success"].as_bool().unwrap());

        let result = send_query(
            ServiceConfig::new_from_path("app-service", config.to_owned()).unwrap(),
            "{ registeredApps { active } }",
        );

        assert_eq!(
            result["registeredApps"]
                .as_array()
                .expect("Not an array")
                .len(),
            0
        );
    });

    // Our app directory should now no longer exist
    assert_eq!(fixture.registry_dir.path().join("dummy").exists(), false);

    // The app's active version symlink should also no longer exist
    assert_eq!(
        format!(
            "{}",
            fs::symlink_metadata(fixture.registry_dir.path().join("active/dummy"))
                .err()
                .unwrap()
        ),
        "No such file or directory (os error 2)"
    );

    fixture.teardown();
    assert!(result.is_ok());
}

#[test]
fn uninstall_notlast_app() {
    let mut fixture = AppServiceFixture::setup();
    let config = format!(
        "{}",
        fixture
            .registry_dir
            .path()
            .join("config.toml")
            .to_string_lossy()
    );
    let mut app = MockAppBuilder::new("dummy");
    app.active(true).version("0.0.1").author("user");

    app.install(&fixture.registry_dir.path());

    let mut app = MockAppBuilder::new("dummy");
    app.active(true).version("0.0.2").author("user");

    app.install(&fixture.registry_dir.path());
    fixture.start_service();

    // Make sure our app directory exists
    assert_eq!(fixture.registry_dir.path().join("dummy").exists(), true);

    let result = panic::catch_unwind(|| {
        let result = send_query(
            ServiceConfig::new_from_path("app-service", config.to_owned()).unwrap(),
            r#"mutation {
            uninstall(name: "dummy", version: "0.0.1") {
                errors,
                success
            }
        }"#,
        );

        assert!(result["uninstall"]["success"].as_bool().unwrap());

        let result = send_query(
            ServiceConfig::new_from_path("app-service", config.to_owned()).unwrap(),
            "{ registeredApps { active } }",
        );

        assert_eq!(result["registeredApps"][0]["active"], true);
    });

    // Our app directory should still exist since there's a version left in the registry
    assert_eq!(fixture.registry_dir.path().join("dummy").exists(), true);

    fixture.teardown();
    assert!(result.is_ok());
}

#[test]
fn uninstall_all_app() {
    let mut fixture = AppServiceFixture::setup();
    let config = format!(
        "{}",
        fixture
            .registry_dir
            .path()
            .join("config.toml")
            .to_string_lossy()
    );
    let mut app = MockAppBuilder::new("dummy");
    app.active(true).version("0.0.1").author("user");

    app.install(&fixture.registry_dir.path());

    let mut app = MockAppBuilder::new("dummy");
    app.active(true).version("0.0.2").author("user");

    app.install(&fixture.registry_dir.path());
    fixture.start_service();

    // Make sure our app directory exists
    assert_eq!(fixture.registry_dir.path().join("dummy").exists(), true);

    let result = panic::catch_unwind(|| {
        let result = send_query(
            ServiceConfig::new_from_path("app-service", config.to_owned()).unwrap(),
            r#"mutation {
            uninstall(name: "dummy") {
                errors,
                success
            }
        }"#,
        );

        assert!(result["uninstall"]["success"].as_bool().unwrap());

        let result = send_query(
            ServiceConfig::new_from_path("app-service", config.to_owned()).unwrap(),
            "{ registeredApps { active } }",
        );

        assert_eq!(
            result["registeredApps"]
                .as_array()
                .expect("Not an array")
                .len(),
            0
        );
    });

    // Our app directory should now no longer exist
    assert_eq!(fixture.registry_dir.path().join("dummy").exists(), false);

    fixture.teardown();
    assert!(result.is_ok());
}

#[test]
fn uninstall_new_app() {
    let mut fixture = AppServiceFixture::setup();
    let config = format!(
        "{}",
        fixture
            .registry_dir
            .path()
            .join("config.toml")
            .to_string_lossy()
    );

    // Pre-load the app registry
    let mut app = MockAppBuilder::new("dummy");
    app.active(true).version("0.0.2").author("user");

    app.install(&fixture.registry_dir.path());

    let mut app = MockAppBuilder::new("dummy");
    app.active(true).version("0.0.3").author("user");

    app.install(&fixture.registry_dir.path());

    let mut app = MockAppBuilder::new("dummy2");
    app.active(true).version("0.0.2").author("user");

    app.install(&fixture.registry_dir.path());
    fixture.start_service();

    // Create a new version of our app to be installed
    let app_dir = TempDir::new().unwrap();
    let app_bin = app_dir.path().join("dummy-app");

    fs::create_dir(app_bin.clone()).unwrap();

    fs::File::create(app_bin.join("dummy")).unwrap();

    // We're intentionally making the version number "smaller" than prior version because it's
    // more likely to cause problems that way
    let manifest = r#"
            name = "dummy"
            version = "0.0.1"
            author = "user"
            "#;
    fs::write(app_bin.join("manifest.toml"), manifest).unwrap();

    let result = panic::catch_unwind(|| {
        // Register the new version of the app
        let result = send_query(
            ServiceConfig::new_from_path("app-service", config.to_owned()).unwrap(),
            &format!(
                r#"mutation {{
                register(path: "{}") {{
                    entry {{
                        active, 
                        app {{
                            author,
                            name,
                            version,
                        }}
                    }},
                    errors,
                    success,
                }}
            }}"#,
                app_bin.to_str().unwrap()
            ),
        );

        assert!(result["register"]["success"].as_bool().unwrap());

        // Uninstall the version we just registered
        let result = send_query(
            ServiceConfig::new_from_path("app-service", config.to_owned()).unwrap(),
            r#"mutation {
            uninstall(name: "dummy", version: "0.0.1") {
                errors,
                success
            }
        }"#,
        );

        assert!(result["uninstall"]["success"].as_bool().unwrap());
    });

    // Our app directory should still exist since there's a version left in the registry
    assert_eq!(fixture.registry_dir.path().join("dummy").exists(), true);

    fixture.teardown();
    assert!(result.is_ok());
}

#[test]
fn uninstall_unmonitor() {
    let mut fixture = AppServiceFixture::setup();
    let config = format!(
        "{}",
        fixture
            .registry_dir
            .path()
            .join("config.toml")
            .to_string_lossy()
    );
    let mut app = MockAppBuilder::new("dummy");
    app.active(true).version("0.0.1").author("user");

    app.install(&fixture.registry_dir.path());

    fixture.start_service();

    // Make sure our app directory exists
    assert_eq!(fixture.registry_dir.path().join("dummy").exists(), true);

    // Run the app so that a monitoring entry will be created
    let result = send_query(
        ServiceConfig::new_from_path("app-service", config.to_owned()).unwrap(),
        r#"mutation {
            startApp(name: "dummy") {
                success,
                errors
            }
        }"#,
    );

    assert_eq!(result["startApp"]["success"].as_bool().unwrap(), true);

    let result = panic::catch_unwind(|| {
        let result = send_query(
            ServiceConfig::new_from_path("app-service", config.to_owned()).unwrap(),
            r#"mutation {
            uninstall(name: "dummy", version: "0.0.1") {
                errors,
                success
            }
        }"#,
        );

        assert!(result["uninstall"]["success"].as_bool().unwrap());

        let result = send_query(
            ServiceConfig::new_from_path("app-service", config.to_owned()).unwrap(),
            r#"{
                appStatus(name: "dummy") {
                    name
                }
            }"#,
        );

        assert!(result["appStatus"].as_array().unwrap().is_empty());
    });

    fixture.teardown();
    assert!(result.is_ok());
}

#[test]
fn uninstall_all_unmonitor() {
    let mut fixture = AppServiceFixture::setup();
    let config = format!(
        "{}",
        fixture
            .registry_dir
            .path()
            .join("config.toml")
            .to_string_lossy()
    );
    let mut app = MockAppBuilder::new("dummy");
    app.active(true).version("0.0.1").author("user");

    app.install(&fixture.registry_dir.path());

    let mut app = MockAppBuilder::new("dummy");
    app.active(true).version("0.0.2").author("user");

    app.install(&fixture.registry_dir.path());
    fixture.start_service();

    // Make sure our app directory exists
    assert_eq!(fixture.registry_dir.path().join("dummy").exists(), true);

    // Run the app so that a monitoring entry will be created
    let result = send_query(
        ServiceConfig::new_from_path("app-service", config.to_owned()).unwrap(),
        r#"mutation {
            startApp(name: "dummy") {
                success,
                errors
            }
        }"#,
    );

    assert_eq!(result["startApp"]["success"].as_bool().unwrap(), true);

    let result = panic::catch_unwind(|| {
        let result = send_query(
            ServiceConfig::new_from_path("app-service", config.to_owned()).unwrap(),
            r#"mutation {
            uninstall(name: "dummy") {
                errors,
                success
            }
        }"#,
        );

        assert!(result["uninstall"]["success"].as_bool().unwrap());

        let result = send_query(
            ServiceConfig::new_from_path("app-service", config.to_owned()).unwrap(),
            "{ registeredApps { active } }",
        );

        assert_eq!(
            result["registeredApps"]
                .as_array()
                .expect("Not an array")
                .len(),
            0
        );

        let result = send_query(
            ServiceConfig::new_from_path("app-service", config.to_owned()).unwrap(),
            r#"{
                appStatus(name: "dummy") {
                    name
                }
            }"#,
        );

        assert!(result["appStatus"].as_array().unwrap().is_empty());
    });

    // Our app directory should now no longer exist
    assert_eq!(fixture.registry_dir.path().join("dummy").exists(), false);

    fixture.teardown();
    assert!(result.is_ok());
}

#[test]
fn uninstall_running() {
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

    fixture.start_service();

    // Run the app so that a monitoring entry will be created
    let result = send_query(
        config.clone(),
        r#"mutation {
            startApp(name: "rust-proj", args: "-l") {
                pid
            }
        }"#,
    );

    let pid = result["startApp"]["pid"].as_i64().unwrap();

    let result = panic::catch_unwind(|| {
        let result = send_query(
            config.clone(),
            r#"mutation {
            uninstall(name: "rust-proj", version: "1.0") {
                errors,
                success
            }
        }"#,
        );

        assert!(
            result["uninstall"]["success"].as_bool().unwrap(),
            "Result: {:?}",
            result
        );
    });

    fixture.teardown();

    assert!(result.is_ok());

    // Give everything a moment to finish
    thread::sleep(Duration::from_millis(100));

    // App should no longer be running (ESRCH = PID not found)
    let pid = Pid::from_raw(pid as i32);
    let result = signal::kill(pid, signal::Signal::SIGINT);
    assert_eq!(result, Err(nix::Error::Sys(nix::errno::Errno::ESRCH)));
}

#[test]
fn uninstall_all_running() {
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

    fixture.start_service();

    // Run the app so that a monitoring entry will be created
    let result = send_query(
        config.clone(),
        r#"mutation {
            startApp(name: "rust-proj", args: "-l") {
                pid
            }
        }"#,
    );

    let pid = result["startApp"]["pid"].as_i64().unwrap();

    let result = panic::catch_unwind(|| {
        let result = send_query(
            config.clone(),
            r#"mutation {
            uninstall(name: "rust-proj") {
                errors,
                success
            }
        }"#,
        );

        assert!(
            result["uninstall"]["success"].as_bool().unwrap(),
            "Result: {:?}",
            result
        );
    });

    fixture.teardown();

    assert!(result.is_ok());

    // Give everything a moment to finish
    thread::sleep(Duration::from_millis(100));

    // App should no longer be running (ESRCH = PID not found)
    let pid = Pid::from_raw(pid as i32);
    let result = signal::kill(pid, signal::Signal::SIGINT);
    assert_eq!(result, Err(nix::Error::Sys(nix::errno::Errno::ESRCH)));
}

#[test]
fn uninstall_kill() {
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

    fixture.start_service();

    // Run the app so that a monitoring entry will be created
    let result = send_query(
        config.clone(),
        r#"mutation {
            startApp(name: "rust-proj", args: "-s") {
                pid
            }
        }"#,
    );

    let pid = result["startApp"]["pid"].as_i64().unwrap();

    let result = panic::catch_unwind(|| {
        let result = send_query(
            config.clone(),
            r#"mutation {
            uninstall(name: "rust-proj", version: "1.0") {
                errors,
                success
            }
        }"#,
        );

        assert!(
            result["uninstall"]["success"].as_bool().unwrap(),
            "Result: {:?}",
            result
        );
    });

    assert!(result.is_ok());

    let pid = Pid::from_raw(pid as i32);
    // App should capture initial nice signal (SIGTERM) and still be running
    assert!(signal::kill(pid, None).is_ok());
    thread::sleep(Duration::from_secs(3));

    fixture.teardown();
    // Now it should be dead
    assert!(!signal::kill(pid, None).is_ok());
}
