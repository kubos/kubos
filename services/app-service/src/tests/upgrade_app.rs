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

// Perform an "upgrade" of a brand new application.
// It's basically allowing a user to register a new application with a custom UUID
#[test]
fn upgrade_new() {
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

    let upgrade_query = format!(
        r#"mutation {{
        register(path: "{}", uuid: "a-b-c-d-e") {{
            success,
            errors,
            entry {{
                active, 
                app {{
                    name,
                    version,
                    uuid
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
                           "uuid": "a-b-c-d-e"
                       }
                   },
                   "errors": "",
                   "success": true,
               }
            }
        }).to_string();

    assert_eq!(service.process(upgrade_query.to_owned()), expected);
}

#[test]
fn upgrade_good() {
    let registry_dir = TempDir::new().unwrap();
    let service = mock_service!(registry_dir);

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

    let upgrade_query = format!(
        r#"mutation {{
        register(path: "{}", uuid: "a-b-c-d-e") {{
            success,
            errors,
            entry {{
                active, 
                app {{
                    name,
                    version,
                    uuid
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
                           "uuid": "a-b-c-d-e"
                       }
                   },
                   "errors": "",
                   "success": true,
               }
            }
        }).to_string();

    // Register the initial app so we have something to upgrade
    assert_eq!(service.process(upgrade_query.to_owned()), expected);

    // Update the manifest for the new version of the app
    let manifest = r#"
            name = "dummy"
            version = "0.0.2"
            author = "user"
            "#;
    fs::write(app_bin.join("manifest.toml"), manifest).unwrap();

    let expected = json!({
            "errors": "",
            "data": {
               "register": {
                   "entry": {
                      "active": true,
                       "app": {
                           "name": "dummy",
                           "version": "0.0.2",
                           "uuid": "a-b-c-d-e"
                       }
                   },
                   "errors": "",
                   "success": true,
               }
            }
        }).to_string();

    // Register the new version
    assert_eq!(service.process(upgrade_query.to_owned()), expected);

    let app_query = r#"
        { apps(uuid: "a-b-c-d-e") { active, app { name, version, uuid } } }
    "#;

    let expected = json!({
            "errors": "",
            "data": {
               "apps": [
                 {
                      "active": false,
                       "app": {
                           "name": "dummy",
                           "version": "0.0.1",
                           "uuid": "a-b-c-d-e"
                       }
                   },
                   {
                      "active": true,
                       "app": {
                           "name": "dummy",
                           "version": "0.0.2",
                           "uuid": "a-b-c-d-e"
                       }
                   }
               ]
            }
    }).to_string();

    // Verify:
    //   - There are two registered versions of the app
    //   - The 0.0.2 version is the active version
    assert_eq!(service.process(app_query.to_owned()), expected);
}

#[test]
fn upgrade_new_name() {
    let registry_dir = TempDir::new().unwrap();
    let service = mock_service!(registry_dir);

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

    let upgrade_query = format!(
        r#"mutation {{
        register(path: "{}", uuid: "a-b-c-d-e") {{
            success,
            errors,
            entry {{
                active, 
                app {{
                    name,
                    version,
                    uuid
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
                           "uuid": "a-b-c-d-e"
                       }
                   },
                   "errors": "",
                   "success": true,
               }
            }
        }).to_string();

    // Register the initial app so we have something to upgrade
    assert_eq!(service.process(upgrade_query.to_owned()), expected);

    // Delete the old app file
    fs::remove_file(app_bin.join("dummy")).unwrap();
    // Create the new app file
    fs::File::create(app_bin.join("dummy2")).unwrap();

    // Update the manifest for the new version of the app,
    // with the new app file name
    let manifest = r#"
            name = "dummy2"
            version = "0.0.2"
            author = "user"
            "#;
    fs::write(app_bin.join("manifest.toml"), manifest).unwrap();

    let expected = json!({
            "errors": "",
            "data": {
               "register": {
                   "entry": {
                      "active": true,
                       "app": {
                           "name": "dummy2",
                           "version": "0.0.2",
                           "uuid": "a-b-c-d-e"
                       }
                   },
                   "errors": "",
                   "success": true,
               }
            }
        }).to_string();

    // Register the new version
    assert_eq!(service.process(upgrade_query.to_owned()), expected);

    let app_query = r#"
        { apps(uuid: "a-b-c-d-e") { active, app { name, version, uuid } } }
    "#;

    let expected = json!({
            "errors": "",
            "data": {
               "apps": [
                 {
                      "active": false,
                       "app": {
                           "name": "dummy",
                           "version": "0.0.1",
                           "uuid": "a-b-c-d-e"
                       }
                   },
                   {
                      "active": true,
                       "app": {
                           "name": "dummy2",
                           "version": "0.0.2",
                           "uuid": "a-b-c-d-e"
                       }
                   }
               ]
            }
    }).to_string();

    // Verify:
    //   - There are two registered versions of the app
    //   - The 0.0.2 version is the active version
    //   - The app names are different
    assert_eq!(service.process(app_query.to_owned()), expected);
}
