/*
 * Copyright (C) 2019 Kubos Corporation
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

use crate::registry::*;
use crate::schema;
use kubos_service::{Config, Service};
use serde_json::json;
use std::fs;
use tempfile::TempDir;

// Add two versions of the app to the requested app service
fn test_setup(service: &Service) {
    let app_dir = TempDir::new().unwrap();
    let app_bin = app_dir.path().join("dummy-app");

    fs::create_dir(app_bin.clone()).unwrap();

    fs::File::create(app_bin.join("dummy")).unwrap();

    let manifest = r#"
            name = "dummy"
            version = "0.0.1"
            author = "user"
            "#;
    fs::write(app_bin.join("manifest.toml"), manifest).unwrap();

    let query = format!(
        r#"mutation {{
        register(path: \"{}\") {{
            success,
        }}
    }}"#,
        app_bin.to_str().unwrap()
    );

    let expected = json!({
           "register": {
               "success": true,
           }
    });

    // Register the initial app
    test!(service, query, expected);

    // Update the manifest for the new version of the app
    let manifest = r#"
            name = "dummy"
            version = "0.0.2"
            author = "user"
            "#;
    fs::write(app_bin.join("manifest.toml"), manifest).unwrap();

    let expected = json!({
           "register": {
               "success": true,
           }
    });

    // Register the new version
    test!(service, query, expected);
}

#[test]
fn set_version_good() {
    let registry_dir = TempDir::new().unwrap();
    let service = mock_service!(registry_dir);

    test_setup(&service);

    let query = r#"mutation {
        setVersion(name: \"dummy\", version: \"0.0.1\") {
            errors,
            success
        }
    }"#;

    let expected = json!({
       "setVersion": {
           "errors": "",
           "success": true,
       }
    });

    test!(service, query, expected);

    let app_query = r#"{ 
        registeredApps(name: \"dummy\") {
            active,
            app {
                name,
                version,
            }
        }
    }"#;

    let expected = json!({
           "registeredApps": [
             {
                  "active": true,
                   "app": {
                       "name": "dummy",
                       "version": "0.0.1",
                   }
               },
               {
                  "active": false,
                   "app": {
                       "name": "dummy",
                       "version": "0.0.2",
                   }
               }
           ]
    });

    test!(service, app_query, expected);
}

#[test]
fn set_version_same() {
    let registry_dir = TempDir::new().unwrap();
    let service = mock_service!(registry_dir);

    test_setup(&service);

    let query = r#"mutation {
        setVersion(name: \"dummy\", version: \"0.0.2\") {
            errors,
            success
        }
    }"#;

    let expected = json!({
       "setVersion": {
           "errors": "",
           "success": true,
       }
    });

    test!(service, query, expected);

    let app_query = r#"{ 
        registeredApps(name: \"dummy\") {
            active,
            app {
                name,
                version,
            }
        }
    }"#;

    let expected = json!({
           "registeredApps": [
             {
                  "active": false,
                   "app": {
                       "name": "dummy",
                       "version": "0.0.1",
                   }
               },
               {
                  "active": true,
                   "app": {
                       "name": "dummy",
                       "version": "0.0.2",
                   }
               }
           ]
    });

    test!(service, app_query, expected);
}

#[test]
fn set_version_bad() {
    let registry_dir = TempDir::new().unwrap();
    let service = mock_service!(registry_dir);

    test_setup(&service);

    let query = r#"mutation {
        setVersion(name: \"dummy\", version: \"0.0.3\") {
            errors,
            success
        }
    }"#;

    let expected = json!({
       "setVersion": {
           "errors": "Registry Error: App dummy version 0.0.3 not found in registry",
           "success": false,
       }
    });

    test!(service, query, expected);
}
