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

use tempfile::TempDir;

use crate::app_entry::*;
use crate::error::*;
use crate::registry::*;

#[test]
fn custom_apps_dir() {
    let registry_dir = TempDir::new().unwrap();
    let registry_path = registry_dir.path().to_string_lossy();

    let registry = AppRegistry::new_from_dir(&registry_path).unwrap();
    assert_eq!(registry.apps_dir, String::from(registry_path));
}

#[test]
fn invalid_apps_dir_empty_reg() {
    let result = AppRegistry::new_from_dir("/sys/fake");
    match result {
        Ok(val) => panic!("Bad test didn't throw error: {:?}", val),
        // Expected result. Specific error varies depending on whether the test is run by a
        // normal user or root
        Err(AppError::IoError { .. }) => {}
        Err(other) => panic!("Unexpected error: {:?}", other),
    }
}

#[test]
fn empty_apps_dir_empty_reg() {
    let registry_dir = TempDir::new().unwrap();

    let registry = AppRegistry::new_from_dir(&registry_dir.path().to_string_lossy()).unwrap();
    assert_eq!(registry.entries.borrow().len(), 0);
}

#[test]
fn serialize_entry() {
    let dummy = AppRegistryEntry {
        app: App {
            uuid: String::from("a-b-c-d"),
            metadata: AppMetadata {
                name: String::from("dummy"),
                version: String::from("0.0.1"),
                author: String::from("noone"),
            },
            pid: 101,
            path: String::from("/fake/path"),
        },
        active_version: true,
    };

    let str = toml::to_string(&dummy).unwrap();
    let parsed: AppRegistryEntry = toml::from_str(&str).unwrap();

    assert_eq!(parsed.active_version, dummy.active_version);
    assert_eq!(parsed.app.uuid, dummy.app.uuid);
    assert_eq!(parsed.app.pid, dummy.app.pid);
    assert_eq!(parsed.app.path, dummy.app.path);
    assert_eq!(parsed.app.metadata.name, dummy.app.metadata.name);
    assert_eq!(parsed.app.metadata.version, dummy.app.metadata.version);
    assert_eq!(parsed.app.metadata.author, dummy.app.metadata.author);
}
