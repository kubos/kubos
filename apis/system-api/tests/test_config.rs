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
extern crate kubos_system;
extern crate tempfile;
extern crate toml;

use std::io::Write;
use tempfile::NamedTempFile;
use toml::Value;

#[test]
fn default_address() {
    let address = kubos_system::Address::default();
    assert_eq!(address.ip(), kubos_system::DEFAULT_IP);
    assert_eq!(address.port(), kubos_system::DEFAULT_PORT);
}

#[test]
fn default_config() {
    let config = kubos_system::Config::default();
    assert_eq!(
        config.hosturl(),
        format!(
            "{}:{}",
            kubos_system::DEFAULT_IP,
            kubos_system::DEFAULT_PORT
        )
    );
    assert_eq!(config.get("addr"), None);
}

#[test]
fn new_from_str() {
    let config = kubos_system::Config::new_from_str(
        "category-1",
        r#"
    [category-1]
    a = 1
    b = 2
    "#,
    );

    assert_eq!(config.get("a"), Some(Value::Integer(1)));
    assert_eq!(config.get("b"), Some(Value::Integer(2)));
    assert_eq!(
        config.hosturl(),
        format!(
            "{}:{}",
            kubos_system::DEFAULT_IP,
            kubos_system::DEFAULT_PORT
        )
    );
}

#[test]
fn new_from_file() {
    let result = NamedTempFile::new();
    assert!(result.is_ok());

    let mut file = result.unwrap();

    let result = writeln!(
        file,
        r#"
    root-a = "blah"
    [category-1]
    a = 1
    b = 2
    [category-1.addr]
    ip = "1.2.3.4"
    port = 1234
    "#
    );
    assert!(result.is_ok());

    let config = kubos_system::Config::new_from_path(
        "category-1",
        file.path().to_string_lossy().to_string(),
    );

    assert_eq!(config.get("a"), Some(Value::Integer(1)));
    assert_eq!(config.get("b"), Some(Value::Integer(2)));
    assert_eq!(config.hosturl(), "1.2.3.4:1234");
    assert_eq!(config.get("root-a"), None);
}

#[test]
fn override_ip() {
    let config = kubos_system::Config::new_from_str(
        "category-1",
        r#"
    [category-1.addr]
    ip = "10.0.1.1"
    "#,
    );
    assert_eq!(
        config.hosturl(),
        format!("10.0.1.1:{}", kubos_system::DEFAULT_PORT)
    );
}

#[test]
fn override_port() {
    let config = kubos_system::Config::new_from_str(
        "category-1",
        r#"
    [category-1.addr]
    port = 9876
    "#,
    );
    assert_eq!(
        config.hosturl(),
        format!("{}:9876", kubos_system::DEFAULT_IP)
    );
}

#[test]
fn override_ip_port() {
    let config = kubos_system::Config::new_from_str(
        "category-1",
        r#"
    [category-1.addr]
    ip = "10.0.1.1"
    port = 9876
    "#,
    );
    assert_eq!(config.hosturl(), "10.0.1.1:9876");
}

#[test]
fn only_category_config() {
    let config = kubos_system::Config::new_from_str(
        "category-1",
        r#"
    root-a = 1
    root-b = 2

    [category-1]
    a = 3
    b = 4

    [category-2]
    c = 5
    d = 6
    "#,
    );

    assert_eq!(config.get("root-a"), None);
    assert_eq!(config.get("root-b"), None);

    assert_eq!(config.get("a"), Some(Value::Integer(3)));
    assert_eq!(config.get("b"), Some(Value::Integer(4)));

    assert_eq!(config.get("c"), None);
    assert_eq!(config.get("d"), None);
}
