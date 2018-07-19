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
//#![deny(missing_docs)]

/// KubOS System level APIs
extern crate getopts;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate toml;

use std::env;
use std::error;
use std::fmt;
use std::mem;
use std::net::UdpSocket;
use std::process::Command;
use std::str::FromStr;
use std::time::Duration;

mod config;

pub use config::*;

pub const VAR_BOOT_COUNT: &'static str = "bootcount";
pub const VAR_BOOT_LIMIT: &'static str = "bootlimit";
pub const VAR_KUBOS_CURR_VERSION: &'static str = "kubos_curr_version";
pub const VAR_KUBOS_PREV_VERSION: &'static str = "kubos_prev_version";
pub const VAR_KUBOS_CURR_TRIED: &'static str = "kubos_curr_tried";
pub const VAR_KUBOS_INITIAL_DEPLOY: &'static str = "kubos_initial_deploy";

pub const SVC_APP: &'static str = "app-service";
pub const SVC_GPS: &'static str = "gps-service";
pub const SVC_TELEMETRY: &'static str = "telemetry-service";

type Result<T> = std::result::Result<T, SystemError>;

lazy_static! {
    static ref FW_PRINTENV_PATH: &'static str = {
        match env::var("KUBOS_PRINTENV") {
            Ok(path) => unsafe {
                let ret = mem::transmute(&path as &str);
                mem::forget(path);
                ret
            },
            _ => "/usr/sbin/fw_printenv"
        }
    };
}

#[derive(Debug)]
pub enum SystemError {
    Message(String),
    JsonError(serde_json::Error),
    IoError(std::io::Error),
}

impl fmt::Display for SystemError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SystemError::Message(ref s) => write!(f, "{}", s),
            SystemError::JsonError(ref e) => e.fmt(f),
            SystemError::IoError(ref e) => e.fmt(f),
        }
    }
}

impl error::Error for SystemError {
    fn description(&self) -> &str {
        match *self {
            SystemError::Message(ref s) => s.as_str(),
            SystemError::JsonError(ref e) => e.description(),
            SystemError::IoError(ref e) => e.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            SystemError::Message(_) => None,
            SystemError::JsonError(ref e) => Some(e),
            SystemError::IoError(ref e) => Some(e),
        }
    }
}

impl From<String> for SystemError {
    fn from(err: String) -> SystemError {
        SystemError::Message(err)
    }
}

impl From<serde_json::Error> for SystemError {
    fn from(err: serde_json::Error) -> SystemError {
        SystemError::JsonError(err)
    }
}

impl From<std::io::Error> for SystemError {
    fn from(err: std::io::Error) -> SystemError {
        SystemError::IoError(err)
    }
}

fn get_boot_var(name: &str) -> Result<String> {
    let output = Command::new(*FW_PRINTENV_PATH)
                         .args(&["-n", name])
                         .output()
                         .expect(&format!("Failed to execute {}", *FW_PRINTENV_PATH));

    if !output.status.success() {
        Err(SystemError::Message(format!("Var not found: {}, printenv: {}, stdout: {}", name,
                                        *FW_PRINTENV_PATH,
                                        &String::from_utf8_lossy(&output.stdout))))
    } else {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }
}

macro_rules! u32_boot_var_getter {
    ($fn:ident, $prop:expr) => {
        pub fn $fn() -> Option<u32> {
            match get_boot_var($prop) {
                Ok(v) => match u32::from_str(&v) {
                    Ok(val) => Some(val),
                    Err(_) => None,
                },
                Err(err) => {
                    eprintln!("{:?}", err);
                    None
                }
            }
        }
    }
}

macro_rules! str_boot_var_getter {
    ($fn:ident, $prop:expr) => {
        pub fn $fn() -> Option<String> {
            match get_boot_var($prop) {
                Ok(v) => Some(v),
                Err(err) => {
                    eprintln!("{:?}", err);
                    None
                }
            }
        }
    }
}

macro_rules! bool_boot_var_getter {
    ($fn:ident, $prop:expr) => {
        pub fn $fn() -> Option<bool> {
            match get_boot_var($prop) {
                Ok(v) => match v.to_lowercase().as_ref() {
                    "t" | "true" | "on" | "1" | "y" | "yes" => Some(true),
                    _ => Some(false)
                },
                Err(err) => {
                    eprintln!("{:?}", err);
                    None
                }
            }
        }
    }
}

u32_boot_var_getter!(boot_count, VAR_BOOT_COUNT);
u32_boot_var_getter!(boot_limit, VAR_BOOT_LIMIT);
str_boot_var_getter!(kubos_curr_version, VAR_KUBOS_CURR_VERSION);
str_boot_var_getter!(kubos_prev_version, VAR_KUBOS_PREV_VERSION);
u32_boot_var_getter!(kubos_curr_tried, VAR_KUBOS_CURR_TRIED);
bool_boot_var_getter!(kubos_initial_deploy, VAR_KUBOS_INITIAL_DEPLOY);

pub fn query(host_url: &str, query: &str, timeout: Option<Duration>)
    -> Result<serde_json::Value>
{
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.connect(host_url)?;
    socket.send(query.as_bytes())?;

    // Allow the caller to set a read timeout on the socket
    socket.set_read_timeout(timeout).unwrap();

    let mut buf = [0; 4096];
    let (amt, _) = socket.recv_from(&mut buf)?;

    let v: serde_json::Value = serde_json::from_slice(&buf[0..(amt)])?;
    println!("data: {:?}", serde_json::to_string(&v));

    if let Some(errs) = v.get("errs") {
        if errs.is_string() {
            let errs_str = errs.as_str().unwrap();
            if errs_str.len() > 0 {
                return Err(SystemError::Message(errs_str.to_string()));
            }
        } else {
            match errs.get("message") {
                Some(message) => {
                    return Err(SystemError::Message(message.as_str().unwrap().to_string()));
                },
                None => {
                    return Err(SystemError::Message(serde_json::to_string(errs).unwrap()));
                }
            }
        }
    }

    match v.get(0) {
        Some(err) if err.get("message").is_some() => {
            return Err(SystemError::Message(err["message"].as_str().unwrap().to_string()));
        },
        _ => {}
    }

    match v.get("msg") {
        Some(result) => Ok(result.clone()),
        None => Err(SystemError::Message(format!("No result returned in 'msg' key: {}",
                                         serde_json::to_string(&v).unwrap())))
    }
}
