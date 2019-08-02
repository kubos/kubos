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

use reqwest::Client;
use serde_json::{from_str, Value};
use std::collections::HashMap;

// Helper function for sending query to app service
pub fn service_query(query: &str, hosturl: &str) -> Value {
    let client = Client::builder().build().unwrap();
    let mut map = HashMap::new();
    map.insert("query", query);
    let url = format!("http://{}", hosturl);

    let mut res = client.post(&url).json(&map).send().unwrap();

    from_str(&res.text().unwrap()).unwrap()
}
