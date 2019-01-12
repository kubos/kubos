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
extern crate kubos_app;
extern crate kubos_system;
extern crate serde_json;
extern crate tempfile;

use kubos_app::ServiceConfig;
use std::panic;
use std::time::Duration;

mod utils;
pub use crate::utils::*;

#[test]
fn uninstall_app() {
    let mut fixture = AppServiceFixture::setup();
    let config = format!(
        "{}",
        fixture
            .registry_dir
            .path()
            .join("config.toml")
            .to_string_lossy()
    );
    let mut app = MockAppBuilder::new("dummy", "a-b-c-d-e");
    app.active(true)
        .run_level("OnBoot")
        .version("0.0.1")
        .author("user");

    app.install(&fixture.registry_dir.path());
    fixture.start_service(false);

    let result = panic::catch_unwind(|| {
        let result = kubos_app::query(
            &ServiceConfig::new_from_path("app-service", config.to_owned()),
            r#"mutation {
            uninstall(uuid: "a-b-c-d-e", version: "0.0.1") {
                errors,
                success
            }
        }"#,
            Some(Duration::from_secs(5)),
        );

        assert!(result.is_ok(), "{:?}", result.err());
        assert!(result.unwrap()["uninstall"]["success"].as_bool().unwrap());

        let result = kubos_app::query(
            &ServiceConfig::new_from_path("app-service", config.to_owned()),
            "{ apps { active } }",
            Some(Duration::from_secs(1)),
        );
        assert!(result.is_ok(), "{:?}", result.err());
        assert_eq!(
            result.unwrap()["apps"]
                .as_array()
                .expect("Not an array")
                .len(),
            0
        );
    });

    fixture.teardown();
    assert!(result.is_ok());
}
