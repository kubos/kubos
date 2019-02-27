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
use std::panic;

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

    // Make sure our app directory exists
    assert_eq!(fixture.registry_dir.path().join("dummy").exists(), true);

    let result = panic::catch_unwind(|| {
        let result = send_query(
            ServiceConfig::new_from_path("app-service", config.to_owned()),
            r#"mutation {
            uninstall(name: "dummy", version: "0.0.1") {
                errors,
                success
            }
        }"#,
        );

        assert!(result["uninstall"]["success"].as_bool().unwrap());

        let result = send_query(
            ServiceConfig::new_from_path("app-service", config.to_owned()),
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
            ServiceConfig::new_from_path("app-service", config.to_owned()),
            r#"mutation {
            uninstall(name: "dummy", version: "0.0.1") {
                errors,
                success
            }
        }"#,
        );

        assert!(result["uninstall"]["success"].as_bool().unwrap());

        let result = send_query(
            ServiceConfig::new_from_path("app-service", config.to_owned()),
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
            ServiceConfig::new_from_path("app-service", config.to_owned()),
            r#"mutation {
            uninstall(name: "dummy") {
                errors,
                success
            }
        }"#,
        );

        assert!(result["uninstall"]["success"].as_bool().unwrap());

        let result = send_query(
            ServiceConfig::new_from_path("app-service", config.to_owned()),
            "{ apps { active } }",
        );

        assert_eq!(result["apps"].as_array().expect("Not an array").len(), 0);
    });

    // Our app directory should now no longer exist
    assert_eq!(fixture.registry_dir.path().join("dummy").exists(), false);

    fixture.teardown();
    assert!(result.is_ok());
}
