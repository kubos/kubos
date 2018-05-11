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

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

pub struct Database {
    pub connection: SqliteConnection,
}

impl Database {
    pub fn new(path: &str) -> Self {
        Database {
            connection: establish_connection(path),
        }
    }
}

fn establish_connection(path: &str) -> SqliteConnection {
    println!("Opening database {}", path);
    SqliteConnection::establish(&String::from(path)).expect(&format!(
        "Could not create SQLite database connection to: {}",
        path
    ))
}

table! {
    telemetry (timestamp) {
        timestamp -> Nullable<Integer>,
        subsystem -> Nullable<Text>,
        param -> Nullable<Text>,
        value -> Nullable<Double>,
    }
}
