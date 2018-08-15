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
extern crate toml;

use std::env;
use std::fs;
use std::path::PathBuf;

use registry::*;

fn setup_registry() -> PathBuf {
    let mut registry_dir = env::temp_dir();
    registry_dir.push("apps");
    if registry_dir.exists() {
        assert!(fs::remove_dir_all(registry_dir.clone()).is_ok());
    }

    assert!(fs::create_dir_all(registry_dir.clone()).is_ok());

    registry_dir
}

#[test]
fn default_apps_dir() {
    let registry = AppRegistry::new();
    assert_eq!(registry.apps_dir, K_APPS_DIR);
}

#[test]
fn custom_apps_dir() {
    let registry = AppRegistry::new_from_dir("/custom/dir");
    assert_eq!(registry.apps_dir, String::from("/custom/dir"));
}

#[test]
fn invalid_apps_dir_empty_reg() {
    let registry = AppRegistry::new_from_dir("/i/dont/exist");
    assert_eq!(registry.entries.borrow().len(), 0);
}

#[test]
fn empty_apps_dir_empty_reg() {
    let registry_dir = setup_registry();

    let registry = AppRegistry::new_from_dir(registry_dir.to_str().unwrap());
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
