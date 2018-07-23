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
#![deny(missing_docs)]

//! KubOS System level APIs

#[macro_use]
extern crate failure;

extern crate getopts;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate toml;

use std::net::UdpSocket;
use std::time::Duration;

mod config;
mod uboot;

pub use config::*;
pub use uboot::UBootVars;

/// The name of the KubOS app service that can be used to derive service configuration
pub const SERVICE_APP: &'static str = "app-service";
/// The name of the KubOS telemetry db service that can be used to dervice service configuration
pub const SERVICE_TELEMETRY: &'static str = "telemetry-service";

/// The result type used by `query`
type Result<T> = std::result::Result<T, failure::Error>;

/// Information about the version(s) of KubOS installed in the system
pub struct KubosVersions {
    /// The current or "active" version of KubOS
    pub curr: Option<String>,
    /// The previous or "inactive" version of KubOS. If there is no previous version, this will be
    /// None
    pub prev: Option<String>,
}

/// Fetch information about the version(s) of KubOS installed in the system
///
/// Returns the current and previous version(s) of KubOS.
pub fn kubos_versions() -> KubosVersions {
    let vars = UBootVars::new();
    KubosVersions {
        curr: vars.get_str(uboot::VAR_KUBOS_CURR_VERSION),
        prev: vars.get_str(uboot::VAR_KUBOS_PREV_VERSION),
    }
}

/// Whether or not the system has been marked as deployed
pub fn initial_deploy() -> Option<bool> {
    let vars = UBootVars::new();
    vars.get_bool(uboot::VAR_KUBOS_INITIAL_DEPLOY)
}

/// Execute a GraphQL query against a running KubOS Service using UDP.
///
/// Returns the parsed JSON result as a serde_json::Value on success
///
/// # Arguments
///
/// * `host_addr` - An address in `IP:PORT` format where the Service is running
/// * `query` - the raw GraphQL query as a string
/// * `timeout` - The timeout provided to the UDP socket. Note: this function will block when `None`
///               is provided here
///
pub fn query(host_addr: &str, query: &str, timeout: Option<Duration>) -> Result<serde_json::Value> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.connect(host_addr)?;
    socket.send(query.as_bytes())?;

    // Allow the caller to set a read timeout on the socket
    socket.set_read_timeout(timeout).unwrap();

    let mut buf = [0; 4096];
    let (amt, _) = socket.recv_from(&mut buf)?;

    let v: serde_json::Value = serde_json::from_slice(&buf[0..(amt)])?;

    if let Some(errs) = v.get("errs") {
        if errs.is_string() {
            let errs_str = errs.as_str().unwrap();
            if errs_str.len() > 0 {
                return Err(format_err!("{}", errs_str.to_string()));
            }
        } else {
            match errs.get("message") {
                Some(message) => {
                    return Err(format_err!("{}", message.as_str().unwrap().to_string()));
                }
                None => {
                    return Err(format_err!("{}", serde_json::to_string(errs).unwrap()));
                }
            }
        }
    }

    match v.get(0) {
        Some(err) if err.get("message").is_some() => {
            return Err(format_err!(
                "{}",
                err["message"].as_str().unwrap().to_string(),
            ));
        }
        _ => {}
    }

    match v.get("msg") {
        Some(result) => Ok(result.clone()),
        None => Err(format_err!(
            "No result returned in 'msg' key: {}",
            serde_json::to_string(&v).unwrap()
        )),
    }
}
