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

use failure::format_err;
use kubos_system::Config as ServiceConfig;
use serde_json;
use std::time::Duration;

/// The result type used by `query`
type AppResult<T> = Result<T, failure::Error>;

/// Execute a GraphQL query against a running KubOS Service using UDP.
///
/// Returns the parsed JSON result as a serde_json::Value on success
///
/// # Arguments
///
/// * `config` - The configuration information for the service which should be queried
/// * `query` - The raw GraphQL query as a string
/// * `timeout` - The timeout provided to the UDP socket. Note: This function will block when `None`
///               is provided here
///
/// # Examples
///
/// ```
/// # use failure;
/// use kubos_app::*;
/// use std::time::Duration;
///
/// # fn func() -> Result<(), failure::Error> {
/// let request = r#"{
/// 		ping
/// 	}"#;
///
/// let result = query(&ServiceConfig::new_from_path("radio-service", "/home/kubos/config.toml".to_owned())?, request, Some(Duration::from_secs(1)))?;
///
/// let data = result.get("ping").unwrap().as_str();
///
/// assert_eq!(data, Some("pong"));
/// # Ok(())
/// # }
/// ```
///
/// ```
/// # use failure;
/// use kubos_app::*;
/// use std::time::Duration;
///
/// # fn func() -> Result<(), failure::Error> {
/// let request = r#"{
/// 		power
/// 	}"#;
///
/// let result = query(&ServiceConfig::new("antenna-service")?, request, Some(Duration::from_secs(1)))?;
///
/// let data = result.get("power").unwrap().as_str();
///
/// assert_eq!(data, Some("ON"));
/// # Ok(())
/// # }
/// ```
///
pub fn query(
    config: &ServiceConfig,
    query: &str,
    timeout: Option<Duration>,
) -> AppResult<serde_json::Value> {
    let client = match timeout {
        Some(time) => reqwest::blocking::Client::builder().timeout(time).build()?,
        None => reqwest::blocking::Client::builder().build()?,
    };

    let uri = format!(
        "http://{}",
        config
            .hosturl()
            .ok_or_else(|| format_err!("Unable to fetch addr for service"))?
    );

    let mut map = ::std::collections::HashMap::new();
    map.insert("query", query);

    let response: serde_json::Value = client.post(&uri).json(&map).send()?.json()?;

    if let Some(errs) = response.get("errors") {
        if errs.is_string() {
            let errs_str = errs.as_str().unwrap();
            if !errs_str.is_empty() {
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

    match response.get(0) {
        Some(err) if err.get("message").is_some() => {
            return Err(format_err!(
                "{}",
                err["message"].as_str().unwrap().to_string(),
            ));
        }
        _ => {}
    }

    match response.get("data") {
        Some(result) => Ok(result.clone()),
        None => Err(format_err!(
            "No result returned in 'data' key: {}",
            serde_json::to_string(&response).unwrap()
        )),
    }
}
