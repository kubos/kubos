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

use kubos_system::UBootVars;
use std::env;
use std::fs;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use tempfile::TempDir;

const DUMMY_PRINTENV: &'static str = r#"#!/bin/bash
VAR="$2"
[[ -n "${!VAR+set}" ]] || exit 1
echo ${!VAR}
"#;

fn setup_dummy_vars(bin_dest: Path) -> UBootVars {
    bin_dest.join("dummy-printenv");

    let mut file = fs::File::create(bin_dest.clone()).unwrap();
    file.write_all(DUMMY_PRINTENV.as_bytes())
        .expect("Failed to write dummy printenv");;

    let mut perms = file.metadata().unwrap().permissions();
    perms.set_mode(0o755);
    file.set_permissions(perms)
        .expect("Failed to change file permissions");

    let bin_str = bin_dest.to_str().unwrap();
    UBootVars::new_from_path(bin_str)
}

#[test]
fn u32_vars() {
    let env_dir = TempDir::new().unwrap();
    let vars = setup_dummy_vars(env_dir.path());

    env::set_var("count", "123");
    assert_eq!(vars.get_u32("count"), Some(123));

    env::set_var("count", "");
    assert_eq!(vars.get_u32("count"), None);

    // should be undefined so far..
    assert_eq!(vars.get_u32("limit"), None);

    env::set_var("limit", "abc");
    assert_eq!(vars.get_u32("limit"), None);
}

#[test]
fn bool_vars() {
    let env_dir = TempDir::new().unwrap();
    let vars = setup_dummy_vars(env_dir.path());
    assert_eq!(vars.get_bool("abcdefg"), None);

    env::set_var("abcdefg", "0");
    assert_eq!(vars.get_bool("abcdefg"), Some(false));

    env::set_var("abcdefg", "1");
    assert_eq!(vars.get_bool("abcdefg"), Some(true));
}

#[test]
fn str_vars() {
    let env_dir = TempDir::new().unwrap();
    let vars = setup_dummy_vars(env_dir.path());
    assert_eq!(vars.get_str("currv"), None);

    env::set_var("currv", "1.23");
    assert_eq!(vars.get_str("currv"), Some(String::from("1.23")));

    env::set_var("currv", "");
    assert_eq!(vars.get_str("currv"), Some(String::from("")));
}
