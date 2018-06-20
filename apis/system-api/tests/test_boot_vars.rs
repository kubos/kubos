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

use std::env;
use std::fs;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;

const DUMMY_PRINTENV: &'static str = r#"#!/bin/bash
VAR="$2"
[[ -n "${!VAR+set}" ]] || exit 1
echo ${!VAR}
"#;

fn setup_dummy_printenv() -> String {
    let mut bin_dest = env::temp_dir();
    bin_dest.push("dummy-printenv");

    let mut file = fs::File::create(bin_dest.clone()).unwrap();
    file.write_all(DUMMY_PRINTENV.as_bytes()).expect("Failed to write dummy printenv");;

    let mut perms = file.metadata().unwrap().permissions();
    perms.set_mode(0o755);
    file.set_permissions(perms).expect("Failed to change file permissions");

    let bin_str = bin_dest.to_str().unwrap();
    env::set_var("KUBOS_PRINTENV", bin_str);

    String::from(bin_str)
}

#[test]
fn u32_vars() {
    setup_dummy_printenv();

    env::set_var(kubos_system::VAR_BOOT_COUNT, "123");
    assert_eq!(kubos_system::boot_count(), Some(123));

    env::set_var(kubos_system::VAR_BOOT_COUNT, "");
    assert_eq!(kubos_system::boot_count(), None);

    // should be undefined so far..
    assert_eq!(kubos_system::boot_limit(), None);

    env::set_var(kubos_system::VAR_BOOT_LIMIT, "abc");
    assert_eq!(kubos_system::boot_limit(), None);
}

#[test]
fn bool_vars() {
    setup_dummy_printenv();

    assert_eq!(kubos_system::kubos_initial_deploy(), None);

    env::set_var(kubos_system::VAR_KUBOS_INITIAL_DEPLOY, "0");

    assert_eq!(kubos_system::kubos_initial_deploy(), Some(false));

    env::set_var(kubos_system::VAR_KUBOS_INITIAL_DEPLOY, "1");
    assert_eq!(kubos_system::kubos_initial_deploy(), Some(true));
}

#[test]
fn str_vars() {
    setup_dummy_printenv();

    assert_eq!(kubos_system::kubos_curr_version(), None);

    env::set_var(kubos_system::VAR_KUBOS_CURR_VERSION, "1.23");
    assert_eq!(kubos_system::kubos_curr_version(), Some(String::from("1.23")));

    env::set_var(kubos_system::VAR_KUBOS_CURR_VERSION, "");
    assert_eq!(kubos_system::kubos_curr_version(), Some(String::from("")));
}
