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
use diesel::*;
use diesel::dsl::sql;
use diesel::sql_types::Bool;
use diesel::sql_query;

pub struct Database {
    pub connection: SqliteConnection,
}

impl Database {
    pub fn new(path: &str) -> Self {
        if !::std::path::Path::new(path).exists() {
            println!("Creating database {}", path);
        }
        Database {
            connection: SqliteConnection::establish(&String::from(path)).expect(&format!(
                "Could not create SQLite database connection to: {}",
                path
            )),
        }
    }

    pub fn setup(&self) {
        match select(sql::<Bool>(
            "EXISTS \
             (SELECT 1 \
             FROM sqlite_master \
             WHERE type = 'table' \
             AND name = 'telemetry')",
        )).get_result::<bool>(&self.connection)
        {
            Err(_) => panic!("Error querying table"),
            Ok(true) => println!("Table exists"),
            Ok(false) => {
                println!("Telemetry table not found. Creating table.");
                match sql_query(
                    "CREATE TABLE telemetry (
                    timestamp INTEGER PRIMARY KEY NULL,
                    subsystem VARCHAR(255) NOT NULL,
                    param VARCHAR(255) NOT NULL,
                    value DOUBLE NOT NULL)",
                ).execute(&self.connection)
                {
                    Ok(_) => println!("Telemetry table created"),
                    _ => panic!("Error creating table"),
                }
            }
        };
    }
}

table! {
    telemetry (timestamp) {
        timestamp -> Nullable<Integer>,
        subsystem -> Nullable<Text>,
        param -> Nullable<Text>,
        value -> Nullable<Double>,
    }
}