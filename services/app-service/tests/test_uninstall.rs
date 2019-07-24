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
use std::fs;
use std::panic;

use tempfile::TempDir;

mod utils;
pub use crate::utils::*;

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
    app.active(true)
        .run_level("OnBoot")
        .version("0.0.1")
        .author("user");

    app.install(&fixture.registry_dir.path());
    fixture.start_service(false);

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
            "{ apps { active } }",
        );

        assert_eq!(result["apps"].as_array().expect("Not an array").len(), 0);
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
    app.active(true)
        .run_level("OnBoot")
        .version("0.0.1")
        .author("user");

    app.install(&fixture.registry_dir.path());

    let mut app = MockAppBuilder::new("dummy");
    app.active(true)
        .run_level("OnBoot")
        .version("0.0.2")
        .author("user");

    app.install(&fixture.registry_dir.path());
    fixture.start_service(false);

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
            "{ apps { active } }",
        );

        assert_eq!(result["apps"][0]["active"], true);
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
    app.active(true)
        .run_level("OnBoot")
        .version("0.0.1")
        .author("user");

    app.install(&fixture.registry_dir.path());

    let mut app = MockAppBuilder::new("dummy");
    app.active(true)
        .run_level("OnBoot")
        .version("0.0.2")
        .author("user");

    app.install(&fixture.registry_dir.path());
    fixture.start_service(false);

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
            "{ apps { active } }",
        );

        assert_eq!(result["apps"].as_array().expect("Not an array").len(), 0);
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
    app.active(true)
        .run_level("OnBoot")
        .version("0.0.2")
        .author("user");

    app.install(&fixture.registry_dir.path());

    let mut app = MockAppBuilder::new("dummy");
    app.active(true)
        .run_level("OnBoot")
        .version("0.0.3")
        .author("user");

    app.install(&fixture.registry_dir.path());

    let mut app = MockAppBuilder::new("dummy2");
    app.active(true)
        .run_level("OnBoot")
        .version("0.0.2")
        .author("user");

    app.install(&fixture.registry_dir.path());
    fixture.start_service(false);

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
