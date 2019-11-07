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
    // OS X's temporary directory is a link from /var/folders/ to /private/var/folders/, so we need
    // to normalize off the /private to compare.
    assert_eq!(registry.apps_dir.trim_start_matches("/private"), String::from(registry_path));
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
    assert_eq!(registry.entries.lock().unwrap().len(), 0);
}

#[test]
fn serialize_entry() {
    let dummy = AppRegistryEntry {
        app: App {
            name: String::from("dummy"),
            version: String::from("0.0.1"),
            author: String::from("noone"),
            executable: String::from("/fake/path"),
            config: String::from("/etc/kubos-config.toml"),
        },
        active_version: true,
    };

    let str = toml::to_string(&dummy).unwrap();
    let parsed: AppRegistryEntry = toml::from_str(&str).unwrap();

    assert_eq!(parsed.active_version, dummy.active_version);
    assert_eq!(parsed.app.executable, dummy.app.executable);
    assert_eq!(parsed.app.name, dummy.app.name);
    assert_eq!(parsed.app.version, dummy.app.version);
    assert_eq!(parsed.app.author, dummy.app.author);
}
