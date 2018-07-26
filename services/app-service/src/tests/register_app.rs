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

use kubos_service::{Config, Service};
use std::fs;

use tempfile::TempDir;

use registry::*;
use schema;

macro_rules! mock_service {
    ($registry_dir:ident) => {{

    let registry = AppRegistry::new_from_dir(&$registry_dir.path().to_string_lossy());

    let config =
            format!(
                r#"
            [app-service]
            registry-dir = "{}"
            [app-service.addr]
            ip = "127.0.0.1"
            port = 9999"#,
                $registry_dir.path().to_str().unwrap(),
            );

    Service::new(Config::new_from_str("app-service", &config), registry, schema::QueryRoot, schema::MutationRoot)

        }};
}

#[test]
fn register_good() {
    let registry_dir = TempDir::new().unwrap();
    let service = mock_service!(registry_dir);

    let app_dir = TempDir::new().unwrap();
    let app_bin = app_dir.path().join("dummy-app");

    fs::create_dir(app_bin.clone()).unwrap();

    // Create dummy app file
    fs::File::create(app_bin.join("dummy")).unwrap();

    // Create manifest file
    let manifest = r#"
            name = "dummy"
            version = "0.0.1"
            author = "user"
            "#;
    fs::write(app_bin.join("manifest.toml"), manifest).unwrap();

    let register_query = format!(
        r#"mutation {{
        register(path: "{}") {{
            active, app {{
                name, version, author
            }}
        }}
    }}"#,
        app_bin.to_str().unwrap()
    );

    let expected = json!({
            "errs": "",
            "msg": {
               "register": {
                   "active": true,
                   "app": {
                       "name": "dummy",
                       "version": "0.0.1",
                       "author": "user"
                   }
               }
            }
        }).to_string();

    assert_eq!(service.process(register_query.to_owned()), expected);
}

#[test]
fn register_no_manifest() {
    let registry_dir = TempDir::new().unwrap();
    let service = mock_service!(registry_dir);

    let app_dir = TempDir::new().unwrap();
    let app_bin = app_dir.path().join("dummy-app");

    fs::create_dir(app_bin.clone()).unwrap();

    // Create dummy app file
    fs::File::create(app_bin.join("dummy")).unwrap();

    let register_query = format!(
        r#"mutation {{
        register(path: "{}") {{
            active, app {{
                name, version, author
            }}
        }}
    }}"#,
        app_bin.to_str().unwrap()
    );

    let expected = "{\"errs\":\"{\\\"message\\\":\\\"Exactly two files should be present in the app directory\\\",\\\"locations\\\":[{\\\"line\\\":2,\\\"column\\\":9}],\\\"path\\\":[\\\"register\\\"]}\",\"msg\":null}";

    assert_eq!(service.process(register_query.to_owned()), expected);
}

#[test]
fn register_extra_file() {
    let registry_dir = TempDir::new().unwrap();
    let service = mock_service!(registry_dir);

    let app_dir = TempDir::new().unwrap();
    let app_bin = app_dir.path().join("dummy-app");

    fs::create_dir(app_bin.clone()).unwrap();

    // Create dummy app file
    fs::File::create(app_bin.join("dummy")).unwrap();

    // Create extra app file
    fs::File::create(app_bin.join("extra")).unwrap();

    // Create manifest file
    let manifest = r#"
            name = "dummy"
            version = "0.0.1"
            author = "user"
            "#;
    fs::write(app_bin.join("manifest.toml"), manifest).unwrap();

    let register_query = format!(
        r#"mutation {{
        register(path: "{}") {{
            active, app {{
                name, version, author
            }}
        }}
    }}"#,
        app_bin.to_str().unwrap()
    );

    let expected = "{\"errs\":\"{\\\"message\\\":\\\"Exactly two files should be present in the app directory\\\",\\\"locations\\\":[{\\\"line\\\":2,\\\"column\\\":9}],\\\"path\\\":[\\\"register\\\"]}\",\"msg\":null}";

    assert_eq!(service.process(register_query.to_owned()), expected);
}

#[test]
fn register_no_name() {
    let registry_dir = TempDir::new().unwrap();
    let service = mock_service!(registry_dir);

    let app_dir = TempDir::new().unwrap();
    let app_bin = app_dir.path().join("dummy-app");

    fs::create_dir(app_bin.clone()).unwrap();

    // Create dummy app file
    fs::File::create(app_bin.join("dummy")).unwrap();

    // Create manifest file
    let manifest = r#"
            version = "0.0.1"
            author = "user"
            "#;
    fs::write(app_bin.join("manifest.toml"), manifest).unwrap();

    let register_query = format!(
        r#"mutation {{
        register(path: "{}") {{
            active, app {{
                name, version, author
            }}
        }}
    }}"#,
        app_bin.to_str().unwrap()
    );

    let expected = "{\"errs\":\"{\\\"message\\\":\\\"Failed to parse manifest: missing field `name`\\\",\\\"locations\\\":[{\\\"line\\\":2,\\\"column\\\":9}],\\\"path\\\":[\\\"register\\\"]}\",\"msg\":null}";

    assert_eq!(service.process(register_query.to_owned()), expected);
}

#[test]
fn register_no_version() {
    let registry_dir = TempDir::new().unwrap();
    let service = mock_service!(registry_dir);

    let app_dir = TempDir::new().unwrap();
    let app_bin = app_dir.path().join("dummy-app");

    fs::create_dir(app_bin.clone()).unwrap();

    // Create dummy app file
    fs::File::create(app_bin.join("dummy")).unwrap();

    // Create manifest file
    let manifest = r#"
            name = "dummy"
            author = "user"
            "#;
    fs::write(app_bin.join("manifest.toml"), manifest).unwrap();

    let register_query = format!(
        r#"mutation {{
        register(path: "{}") {{
            active, app {{
                name, version, author
            }}
        }}
    }}"#,
        app_bin.to_str().unwrap()
    );

    let expected = "{\"errs\":\"{\\\"message\\\":\\\"Failed to parse manifest: missing field `version`\\\",\\\"locations\\\":[{\\\"line\\\":2,\\\"column\\\":9}],\\\"path\\\":[\\\"register\\\"]}\",\"msg\":null}";

    assert_eq!(service.process(register_query.to_owned()), expected);
}

#[test]
fn register_no_author() {
    let registry_dir = TempDir::new().unwrap();
    let service = mock_service!(registry_dir);

    let app_dir = TempDir::new().unwrap();
    let app_bin = app_dir.path().join("dummy-app");

    fs::create_dir(app_bin.clone()).unwrap();

    // Create dummy app file
    fs::File::create(app_bin.join("dummy")).unwrap();

    // Create manifest file
    let manifest = r#"
            name = "dummy"
            version = "0.0.1"
            "#;
    fs::write(app_bin.join("manifest.toml"), manifest).unwrap();

    let register_query = format!(
        r#"mutation {{
        register(path: "{}") {{
            active, app {{
                name, version, author
            }}
        }}
    }}"#,
        app_bin.to_str().unwrap()
    );

    let expected = "{\"errs\":\"{\\\"message\\\":\\\"Failed to parse manifest: missing field `author`\\\",\\\"locations\\\":[{\\\"line\\\":2,\\\"column\\\":9}],\\\"path\\\":[\\\"register\\\"]}\",\"msg\":null}";

    assert_eq!(service.process(register_query.to_owned()), expected);
}

#[test]
fn register_bad_path() {
    let registry_dir = TempDir::new().unwrap();
    let service = mock_service!(registry_dir);

    let register_query = r#"mutation {
        register(path: "fake/files") {
            active, app {
                name, version, author
            }
        }
    }"#;

    let expected = "{\"errs\":\"{\\\"message\\\":\\\"fake/files does not exist\\\",\\\"locations\\\":[{\\\"line\\\":2,\\\"column\\\":9}],\\\"path\\\":[\\\"register\\\"]}\",\"msg\":null}";

    assert_eq!(service.process(register_query.to_owned()), expected);
}
