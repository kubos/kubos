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
extern crate tempfile;

use std::time::Duration;
use std::{env, panic};

mod utils;
use utils::*;

#[test]
fn register_app() {
    let mut fixture = AppServiceFixture::setup();
    let mut app_bin = env::temp_dir();
    app_bin.push("dummy-app");

    MockAppBuilder::new("dummy", "a-b-c-d-e")
        .version("0.0.1")
        .author("user")
        .generate_bin(&app_bin);

    let register_query = format!(
        r#"mutation {{
        register(path: "{}") {{
            active, runLevel, app {{
                uuid, name, version, author, pid, path
            }}
        }}
    }}"#,
        app_bin.to_str().unwrap()
    );

    let apps_query = r#"{ apps { active, runLevel, app {
        uuid, name, version, author, pid, path
    } } }"#;

    fn assert_expected(v: &serde_json::Value) {
        assert_eq!(v["active"], json!(true));
        assert_eq!(v["runLevel"], json!("OnCommand"));
        assert!(v["app"].is_object());
        assert_eq!(v["app"]["name"], json!("dummy"));
        assert_eq!(v["app"]["version"], json!("0.0.1"));
        assert_eq!(v["app"]["author"], json!("user"));
    }

    fixture.start_service();

    let addr = fixture.addr.clone();
    let result = panic::catch_unwind(|| {
        println!("{}", register_query);
        let result = kubos_system::query(&addr, &register_query, Some(Duration::from_secs(2)));
        assert!(result.is_ok(), "{:?}", result.err());
        assert_expected(&result.unwrap()["register"]);

        let result = kubos_system::query(&addr, &apps_query, Some(Duration::from_secs(5)));
        assert!(result.is_ok());
        assert_expected(&result.unwrap()["apps"][0]);
    });
    fixture.teardown();
    assert!(result.is_ok());
}
