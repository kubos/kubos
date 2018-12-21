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
            success,
            errors,
            entry {{
                active, 
                app {{
                    name,
                    version,
                    author
                }}
            }}
        }}
    }}"#,
        app_bin.to_str().unwrap()
    );

    let expected = json!({
            "errors": "",
            "data": {
               "register": {
                   "entry": {
                      "active": true,
                       "app": {
                           "name": "dummy",
                           "version": "0.0.1",
                           "author": "user"
                       }
                   },
                   "errors": "",
                   "success": true,
               }
            }
        }).to_string();

    assert_eq!(service.process(&register_query.to_owned()), expected);
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
            success,
            errors,
            entry {{
                active, 
                app {{
                    name,
                    version,
                    author
                }}
            }}
        }}
    }}"#,
        app_bin.to_str().unwrap()
    );

    let expected = json!({
            "errors": "",
            "data": {
               "register": {
                   "entry": null,
                   "errors": "IO Error: No such file or directory (os error 2)",
                   "success": false,
               }
            }
        }).to_string();

    assert_eq!(service.process(&register_query.to_owned()), expected);
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
            success,
            errors,
            entry {{
                active, 
                app {{
                    name,
                    version,
                    author
                }}
            }}
        }}
    }}"#,
        app_bin.to_str().unwrap()
    );

    let expected = json!({
            "errors": "",
            "data": {
               "register": {
                   "entry": null,
                   "errors": "Failed to parse manifest.toml: missing field `name`",
                   "success": false,
               }
            }
        }).to_string();

    assert_eq!(service.process(&register_query.to_owned()), expected);
}

#[test]
fn register_bad_name() {
    let registry_dir = TempDir::new().unwrap();
    let service = mock_service!(registry_dir);

    let app_dir = TempDir::new().unwrap();
    let app_bin = app_dir.path().join("dummy-app");

    fs::create_dir(app_bin.clone()).unwrap();

    // Create dummy app file
    fs::File::create(app_bin.join("dummy")).unwrap();

    // Create manifest file
    let manifest = r#"
            name = "fake"
            version = "0.0.1"
            author = "user"
            "#;
    fs::write(app_bin.join("manifest.toml"), manifest).unwrap();

    let register_query = format!(
        r#"mutation {{
        register(path: "{}") {{
            success,
            errors,
            entry {{
                active, 
                app {{
                    name,
                    version,
                    author
                }}
            }}
        }}
    }}"#,
        app_bin.to_str().unwrap()
    );

    let expected = json!({
            "errors": "",
            "data": {
               "register": {
                   "entry": null,
                   "errors": "Failed to register app: Application file fake not found in given path",
                   "success": false,
               }
            }
        }).to_string();

    assert_eq!(service.process(&register_query.to_owned()), expected);
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
            success,
            errors,
            entry {{
                active, 
                app {{
                    name,
                    version,
                    author
                }}
            }}
        }}
    }}"#,
        app_bin.to_str().unwrap()
    );

    let expected = json!({
            "errors": "",
            "data": {
               "register": {
                   "entry": null,
                   "errors": "Failed to parse manifest.toml: missing field `version`",
                   "success": false,
               }
            }
        }).to_string();

    assert_eq!(service.process(&register_query.to_owned()), expected);
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
            success,
            errors,
            entry {{
                active, 
                app {{
                    name,
                    version,
                    author
                }}
            }}
        }}
    }}"#,
        app_bin.to_str().unwrap()
    );

    let expected = json!({
            "errors": "",
            "data": {
               "register": {
                   "entry": null,
                   "errors": "Failed to parse manifest.toml: missing field `author`",
                   "success": false,
               }
            }
        }).to_string();

    assert_eq!(service.process(&register_query.to_owned()), expected);
}

#[test]
fn register_bad_path() {
    let registry_dir = TempDir::new().unwrap();
    let service = mock_service!(registry_dir);

    let register_query = r#"mutation {
        register(path: "fake/files") {
            success,
            errors,
            entry {
                active, 
                app {
                    name,
                    version,
                    author
                }
            }
        }
    }"#;

    let expected = json!({
            "errors": "",
            "data": {
               "register": {
                   "entry": null,
                   "errors": "Failed to register app: fake/files does not exist",
                   "success": false,
               }
            }
        }).to_string();

    assert_eq!(service.process(&register_query.to_owned()), expected);
}
