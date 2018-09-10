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

use failure;
use kubos_system::Config as ServiceConfig;
use serde_json;
use std::net::UdpSocket;
use std::time::Duration;

/// The result type used by `query`
type AppResult<T> = Result<T, failure::Error>;

/// Execute a GraphQL query against a running KubOS Service using UDP.
///
/// Returns the parsed JSON result as a serde_json::Value on success
///
/// # Arguments
///
/// * `service` - The name of the service to send the query to
/// * `config_path` - The system location of the `config.toml` file which has the IP and port information
///                   of the service to query. If `None` is specified, the default config location will be
///                   used
/// * `query` - The raw GraphQL query as a string
/// * `timeout` - The timeout provided to the UDP socket. Note: This function will block when `None`
///               is provided here
///
/// # Examples
///
/// ```
/// # extern crate failure;
/// # extern crate kubos_app;
/// # use failure;
/// use kubos_app::*;
/// use std::time::Duration;
///
/// # fn func() -> Result<(), failure::Error> {
/// let request = r#"{
/// 		ping
/// 	}"#;
///
/// let result = query(ServiceConfig::new_from_path("radio-service", "/home/kubos/config.toml".to_owned()), request, Some(Duration::from_secs(1)))?;
///
/// let data = result.get("ping").unwrap().as_str();
///
/// assert_eq!(data, Some("pong"));
/// # Ok(())
/// # }
/// ```
///
/// ```
/// # extern crate failure;
/// # extern crate kubos_app;
/// # use failure;
/// use kubos_app::*;
/// use std::time::Duration;
///
/// # fn func() -> Result<(), failure::Error> {
/// let request = r#"{
/// 		power
/// 	}"#;
///
/// let result = query(ServiceConfig::new("antenna-service"), request, Some(Duration::from_secs(1)))?;
///
/// let data = result.get("power").unwrap().as_str();
///
/// assert_eq!(data, Some("ON"));
/// # Ok(())
/// # }
/// ```
///
pub fn query(
    config: ServiceConfig,
    query: &str,
    timeout: Option<Duration>,
) -> AppResult<serde_json::Value> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.connect(config.hosturl())?;
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
        } else if !errs.is_null() {
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
