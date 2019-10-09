/*
 * Copyright (C) 2019 Kubos Corporation
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

//!
//! Definitions and functions for dealing with scheduled app execution
//!

use crate::error::SchedulerError;
use crate::schema::GenericResponse;
use juniper::GraphQLObject;
use log::{error, info};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug, Deserialize)]
pub struct StartAppResponse {
    #[serde(rename = "startApp")]
    pub start_app: GenericResponse,
}

#[derive(Debug, Deserialize)]
pub struct StartAppGraphQL {
    pub data: StartAppResponse,
}

// Helper function for sending query to app service
pub fn service_query(query: &str, hosturl: &str) -> Result<StartAppGraphQL, SchedulerError> {
    let client = Client::builder()
        .timeout(Duration::from_millis(100))
        .build()
        .map_err(|e| SchedulerError::QueryError { err: e.to_string() })?;
    let mut map = HashMap::new();
    map.insert("query", query);
    let url = format!("http://{}", hosturl);

    let mut res = client
        .post(&url)
        .json(&map)
        .send()
        .map_err(|e| SchedulerError::QueryError { err: e.to_string() })?;

    Ok(from_str(
        &res.text()
            .map_err(|e| SchedulerError::QueryError { err: e.to_string() })?,
    )
    .map_err(|e| SchedulerError::QueryError { err: e.to_string() })?)
}

// Configuration used for execution of an app
#[derive(Clone, Debug, GraphQLObject, Serialize, Deserialize)]
pub struct App {
    pub name: String,
    pub args: Option<Vec<String>>,
    pub config: Option<String>,
    pub run_level: Option<String>,
}

impl App {
    pub fn execute(&self, service_url: &str) {
        info!("Start app {}", self.name);
        let run_level = match &self.run_level {
            Some(run) => run,
            None => "onBoot",
        };
        let mut query_args = format!("runLevel: \"{}\", name: \"{}\"", run_level, self.name);
        if let Some(config) = &self.config {
            query_args.push_str(&format!(", config: \"{}\"", config));
        }
        if let Some(args) = &self.args {
            let app_args: Vec<String> = args.iter().map(|x| format!("\"{}\"", x)).collect();

            let app_args = app_args.join(",");
            query_args.push_str(&format!(", args: [{}]", app_args));
        }
        let query = format!(
            r#"mutation {{ startApp({}) {{ success, errors }} }}"#,
            query_args
        );
        match service_query(&query, &service_url) {
            Err(e) => {
                error!("Failed to send start app query: {}", e);
            }
            Ok(resp) => {
                if !resp.data.start_app.success {
                    error!(
                        "Failed to start scheduled app: {}",
                        resp.data.start_app.errors
                    );
                }
            }
        }
    }
}
