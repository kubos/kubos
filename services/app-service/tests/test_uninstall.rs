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

use std::panic;
use std::time::Duration;

mod utils;
use utils::*;

const UNINSTALL_QUERY: &'static str = r#"mutation {
    uninstall(uuid: "a-b-c-d-e", version: "0.0.1")
}"#;
const APPS_QUERY: &'static str = "{ apps { active } }";

#[test]
fn uninstall_app() {
    let mut fixture = AppServiceFixture::setup();
    let mut app = MockAppBuilder::new("dummy", "a-b-c-d-e");
    app.active(true)
       .run_level("OnBoot")
       .version("0.0.1")
       .author("user");

    app.install(&fixture.registry_dir.path());
    fixture.start_service();

    let addr = fixture.addr.clone();
    let result = panic::catch_unwind(|| {
        let result = kubos_system::query(&addr, UNINSTALL_QUERY,
                                         Some(Duration::from_secs(5)));
        assert!(result.is_ok(), "{:?}", result.err());
        assert!(result.unwrap()["uninstall"].as_bool().unwrap());

        let result = kubos_system::query(&addr, APPS_QUERY,
                                         Some(Duration::from_secs(1)));
        assert!(result.is_ok(), "{:?}", result.err());
        assert_eq!(result.unwrap()["apps"].as_array().expect("Not an array").len(), 0);
    });

    fixture.teardown();
    assert!(result.is_ok());
}
