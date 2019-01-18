//
// Copyright (C) 2018 Kubos Corporation
//
// Licensed under the Apache License, Version 2.0 (the "License")
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

use serde_derive::{Deserialize, Serialize};
use super::telemetry;

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct Entry {
    pub timestamp: f64,
    pub subsystem: String,
    pub parameter: String,
    pub value: String,
}

#[derive(Insertable)]
#[table_name = "telemetry"]
pub struct NewEntry<'a> {
    pub timestamp: f64,
    pub subsystem: &'a str,
    pub parameter: &'a str,
    pub value: &'a str,
}
