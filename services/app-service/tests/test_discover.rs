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
#[macro_use]
extern crate serde_json;

use std::panic;
use std::time::Duration;

mod utils;
use utils::*;

#[test]
fn discover_app() {
    let mut fixture = AppServiceFixture::setup();
    fixture.install_dummy_app();
    fixture.start_service();

    let result = panic::catch_unwind(|| {
        let apps_query = "{ apps { active, runLevel, app { uuid, name, version, author } } }";
        let result = kubos_system::query("localhost:9999", &apps_query,
                                        Some(Duration::from_secs(5)));
        assert!(result.is_ok());

        let apps = &result.unwrap()["apps"];
        assert!(apps.is_array());
        assert_eq!(apps.as_array().unwrap().len(), 1);

        let v = &apps.as_array().unwrap()[0];
        assert_eq!(v["active"], json!(true));
        assert_eq!(v["runLevel"], json!("OnBoot"));
        assert!(v["app"].is_object());
        assert_eq!(v["app"]["uuid"], json!("a-b-c-d-e"));
        assert_eq!(v["app"]["name"], json!("dummy"));
        assert_eq!(v["app"]["version"], json!("0.0.1"));
        assert_eq!(v["app"]["author"], json!("user"));
    });

    fixture.teardown();
    assert!(result.is_ok());
}
