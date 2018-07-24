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

use std::net::UdpSocket;
use std::time::Duration;
use failure;
use serde_json;

/// The result type used by `query`
type AppResult<T> = Result<T, failure::Error>;

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
pub fn query(
    host_addr: &str,
    query: &str,
    timeout: Option<Duration>,
) -> AppResult<serde_json::Value> {
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
