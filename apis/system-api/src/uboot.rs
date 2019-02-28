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

use failure::{format_err, Error};
use std::process::Command;
use std::str::FromStr;

pub const VAR_KUBOS_CURR_VERSION: &str = "kubos_curr_version";
pub const VAR_KUBOS_PREV_VERSION: &str = "kubos_prev_version";
pub const VAR_KUBOS_INITIAL_DEPLOY: &str = "kubos_initial_deploy";

const PRINTENV_PATH: &str = "/usr/sbin/fw_printenv";

/// A convenience wrapper for fetching UBoot variables used by KubOS
#[derive(Default)]
pub struct UBootVars {
    cmd_path: String,
}

impl UBootVars {
    /// Default constructor that fetches UBoot vars using `/usr/sbin/fw_printenv`
    pub fn new() -> Self {
        Self::new_from_path(PRINTENV_PATH)
    }

    /// Constructor that fetches UBoot vars with a custom path to `fw_printenv`
    pub fn new_from_path(path: &str) -> Self {
        Self {
            cmd_path: String::from(path),
        }
    }

    fn get(&self, name: &str) -> Result<String, Error> {
        let output = match Command::new(&self.cmd_path).args(&["-n", name]).output() {
            Ok(output) => output,
            Err(_) => return Err(format_err!("Failed to execute: {}", self.cmd_path)),
        };

        if !output.status.success() {
            Err(format_err!("Var not found: {}", name))
        } else {
            Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
        }
    }

    /// Returns the value of a UBoot variable encoded as a u32
    pub fn get_u32(&self, name: &str) -> Option<u32> {
        match self.get(name) {
            Ok(v) => match u32::from_str(&v) {
                Ok(val) => Some(val),
                Err(_) => None,
            },
            Err(_) => None,
        }
    }

    /// Returns the value of a UBoot variable encoded as a String
    pub fn get_str(&self, name: &str) -> Option<String> {
        match self.get(name) {
            Ok(v) => Some(v),
            Err(_) => None,
        }
    }

    /// Returns the value of a UBoot variable encoded as a bool
    pub fn get_bool(&self, name: &str) -> Option<bool> {
        match self.get(name) {
            Ok(v) => match v.to_lowercase().as_ref() {
                "t" | "true" | "1" | "y" | "yes" => Some(true),
                _ => Some(false),
            },
            Err(_) => None,
        }
    }
}
