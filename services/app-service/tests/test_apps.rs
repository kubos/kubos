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

use kubos_app::ServiceConfig;
use std::panic;
use std::path::Path;
use std::time::Duration;

mod utils;
pub use utils::*;

fn setup_apps(registry_dir: &Path) {
    MockAppBuilder::new("app1", "a-b-c-d-e")
        .version("0.0.1")
        .active(false)
        .run_level("OnCommand")
        .author("mham")
        .install(&registry_dir);
    MockAppBuilder::new("app1", "a-b-c-d-e")
        .version("0.0.2")
        .active(false)
        .install(&registry_dir);
    MockAppBuilder::new("app2", "f-g-h-i-j")
        .version("1.0.0")
        .active(false)
        .install(&registry_dir);
    MockAppBuilder::new("app2", "f-g-h-i-j")
        .version("1.0.1")
        .active(true)
        .install(&registry_dir);
    MockAppBuilder::new("app3", "a-b-c-d-e")
        .version("0.0.3")
        .active(true)
        .install(&registry_dir);
    MockAppBuilder::new("app4", "1-2-3-4-5")
        .version("1.0.0")
        .active(true)
        .run_level("OnBoot")
        .author("user")
        .install(&registry_dir);
}

fn apps_query(config: ServiceConfig, query: &str) -> Vec<serde_json::Value> {
    let result = kubos_app::query(config, query, Some(Duration::from_secs(5)));
    assert!(result.is_ok());

    let apps = result.unwrap()["apps"].clone();
    assert!(apps.is_array());

    let mut apps_sorted = apps.as_array().unwrap().clone();

    // Sort by uuid/name/version to make testing deterministic
    apps_sorted.sort_unstable_by_key(|a| {
        format!(
            "{}|{}|{}",
            a["app"]["uuid"].to_string(),
            a["app"]["name"].to_string(),
            a["app"]["version"].to_string()
        )
    });
    apps_sorted
}

macro_rules! test_query {
    ($name:ident, $query:expr, $test_closure:expr) => {
        #[test]
        fn $name() {
            let mut fixture = AppServiceFixture::setup();
            let config = format!(
                "{}",
                fixture
                    .registry_dir
                    .path()
                    .join("config.toml")
                    .to_string_lossy()
            );
            setup_apps(&fixture.registry_dir.path());
            fixture.start_service(false);

            let result = panic::catch_unwind(|| {
                let test: &Fn(Vec<serde_json::Value>) = &$test_closure;
                test(apps_query(ServiceConfig::new_from_path("app-service", config.to_owned()), $query));
            });

            fixture.teardown();
            assert!(result.is_ok());
        }
    };
}

test_query!(
    all_apps,
    "{ apps { active, app { uuid, name, version, author } } }",
    |apps| {
        assert_eq!(apps.len(), 6);
        assert_eq!(
            apps[0],
            json!({"active": true, "app": {"uuid": "1-2-3-4-5", "name": "app4", "version": "1.0.0", "author": "user"}})
        );
        assert_eq!(
            apps[1],
            json!({"active": false, "app": {"uuid": "a-b-c-d-e", "name": "app1", "version": "0.0.1", "author": "mham"}})
        );
        assert_eq!(
            apps[2],
            json!({"active": false, "app": {"uuid": "a-b-c-d-e", "name": "app1", "version": "0.0.2", "author": "unknown"}})
        );
        assert_eq!(
            apps[3],
            json!({"active": true, "app": {"uuid": "a-b-c-d-e", "name": "app3", "version": "0.0.3", "author": "unknown"}})
        );
        assert_eq!(
            apps[4],
            json!({"active": false, "app": {"uuid": "f-g-h-i-j", "name": "app2", "version": "1.0.0", "author": "unknown"}})
        );
        assert_eq!(
            apps[5],
            json!({"active": true, "app": {"uuid": "f-g-h-i-j", "name": "app2", "version": "1.0.1", "author": "unknown"}})
        );
    }
);

test_query!(
    by_uuid_abcde,
    "{ apps(uuid: \"a-b-c-d-e\") { app { uuid, name, version } } }",
    |apps| {
        assert_eq!(apps.len(), 3);
        assert_eq!(
            apps[0],
            json!({"app": {"uuid": "a-b-c-d-e", "name": "app1", "version": "0.0.1"}})
        );
        assert_eq!(
            apps[1],
            json!({"app": {"uuid": "a-b-c-d-e", "name": "app1", "version": "0.0.2"}})
        );
        assert_eq!(
            apps[2],
            json!({"app": {"uuid": "a-b-c-d-e", "name": "app3", "version": "0.0.3"}})
        );
    }
);

test_query!(
    by_uuid_fghij,
    "{ apps(uuid: \"f-g-h-i-j\") { app { uuid, name, version } } }",
    |apps| {
        assert_eq!(apps.len(), 2);
        assert_eq!(
            apps[0],
            json!({"app": {"uuid": "f-g-h-i-j", "name": "app2", "version": "1.0.0"}})
        );
        assert_eq!(
            apps[1],
            json!({"app": {"uuid": "f-g-h-i-j", "name": "app2", "version": "1.0.1"}})
        );
    }
);

test_query!(
    by_uuid_abcde_name_app1,
    r#"{ apps(uuid: "a-b-c-d-e", name: "app1") { app { uuid, name, version } } }"#,
    |apps| {
        assert_eq!(apps.len(), 2);
        assert_eq!(
            apps[0],
            json!({"app": {"uuid": "a-b-c-d-e", "name": "app1", "version": "0.0.1"}})
        );
        assert_eq!(
            apps[1],
            json!({"app": {"uuid": "a-b-c-d-e", "name": "app1", "version": "0.0.2"}})
        );
    }
);

test_query!(
    by_name_app1,
    "{ apps(name: \"app1\") { app { uuid, name, version } } }",
    |apps| {
        assert_eq!(apps.len(), 2);
        assert_eq!(
            apps[0],
            json!({"app": {"uuid": "a-b-c-d-e", "name": "app1", "version": "0.0.1"}})
        );
        assert_eq!(
            apps[1],
            json!({"app": {"uuid": "a-b-c-d-e", "name": "app1", "version": "0.0.2"}})
        );
    }
);

test_query!(
    by_name_app2,
    "{ apps(name: \"app2\") { app { uuid, name, version } } }",
    |apps| {
        assert_eq!(apps.len(), 2);
        assert_eq!(
            apps[0],
            json!({"app": {"uuid": "f-g-h-i-j", "name": "app2", "version": "1.0.0"}})
        );
        assert_eq!(
            apps[1],
            json!({"app": {"uuid": "f-g-h-i-j", "name": "app2", "version": "1.0.1"}})
        );
    }
);

test_query!(
    by_version_100,
    "{ apps(version: \"1.0.0\") { app { uuid, name, version } } }",
    |apps| {
        assert_eq!(apps.len(), 2);
        assert_eq!(
            apps[0],
            json!({"app": {"uuid": "1-2-3-4-5", "name": "app4", "version": "1.0.0"}})
        );
        assert_eq!(
            apps[1],
            json!({"app": {"uuid": "f-g-h-i-j", "name": "app2", "version": "1.0.0"}})
        );
    }
);

test_query!(
    by_version_002,
    "{ apps(version: \"0.0.2\") { app { uuid, name, version } } }",
    |apps| {
        assert_eq!(apps.len(), 1);
        assert_eq!(
            apps[0],
            json!({"app": {"uuid": "a-b-c-d-e", "name": "app1", "version": "0.0.2"}})
        );
    }
);

test_query!(
    by_name_app2_version_101,
    r#"{ apps(version: "1.0.1", name: "app2") { app { uuid, name, version } } }"#,
    |apps| {
        assert_eq!(apps.len(), 1);
        assert_eq!(
            apps[0],
            json!({"app": {"uuid": "f-g-h-i-j", "name": "app2", "version": "1.0.1"}})
        );
    }
);

test_query!(
    by_active_true,
    "{ apps(active: true) { app { uuid, name, version } } }",
    |apps| {
        assert_eq!(apps.len(), 3);
        assert_eq!(
            apps[0],
            json!({"app": {"uuid": "1-2-3-4-5", "name": "app4", "version": "1.0.0"}})
        );
        assert_eq!(
            apps[1],
            json!({"app": {"uuid": "a-b-c-d-e", "name": "app3", "version": "0.0.3"}})
        );
        assert_eq!(
            apps[2],
            json!({"app": {"uuid": "f-g-h-i-j", "name": "app2", "version": "1.0.1"}})
        );
    }
);

test_query!(
    by_active_false,
    "{ apps(active: false) { app { uuid, name, version } } }",
    |apps| {
        assert_eq!(apps.len(), 3);
        assert_eq!(
            apps[0],
            json!({"app": {"uuid": "a-b-c-d-e", "name": "app1", "version": "0.0.1"}})
        );
        assert_eq!(
            apps[1],
            json!({"app": {"uuid": "a-b-c-d-e", "name": "app1", "version": "0.0.2"}})
        );
        assert_eq!(
            apps[2],
            json!({"app": {"uuid": "f-g-h-i-j", "name": "app2", "version": "1.0.0"}})
        );
    }
);

test_query!(
    by_active_true_uuid_abcde,
    "{ apps(active: true, uuid: \"a-b-c-d-e\") { app { uuid, name, version } } }",
    |apps| {
        assert_eq!(apps.len(), 1);
        assert_eq!(
            apps[0],
            json!({"app": {"uuid": "a-b-c-d-e", "name": "app3", "version": "0.0.3"}})
        );
    }
);
