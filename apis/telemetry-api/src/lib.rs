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
#[macro_use]
extern crate diesel;
extern crate time;

use std::time::SystemTime;

pub mod models;
pub use models::*;

use diesel::dsl::sql;
use diesel::insert_into;
use diesel::prelude::*;
use diesel::sql_query;
use diesel::sql_types::Bool;
use diesel::sqlite::SqliteConnection;
use diesel::*;

pub struct Database {
    pub connection: SqliteConnection,
}

impl Database {
    /// Construct new database structure and database file if needed
    ///
    /// # Arguments
    /// `path` - Path to database file
    ///
    /// # Panics
    ///
    /// Attempts to connect to sqlite database and will `panic!` if connection fails.
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

    /// Check if database has correct table and creates table if needed
    ///
    /// # Panics
    ///
    /// Will `panic!` if fails to locate and/or create telemetry table
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
                    timestamp INTEGER NOT NULL,
                    subsystem VARCHAR(255) NOT NULL,
                    parameter VARCHAR(255) NOT NULL,
                    value VARCHAR(255) NOT NULL,
                    PRIMARY KEY (timestamp, subsystem, parameter))",
                ).execute(&self.connection)
                {
                    Ok(_) => println!("Telemetry table created"),
                    Err(err) => panic!("Error creating table: {:?}", err),
                }
            }
        };
    }

    pub fn insert<'a>(&self, timestamp: i32,
                             subsystem: &'a str,
                             parameter: &'a str,
                             value: &'a str) -> QueryResult<usize>
    {
        use self::telemetry;

        let new_entry = NewEntry {
            timestamp: timestamp,
            subsystem: subsystem,
            parameter: parameter,
            value: value,
        };

        insert_into(telemetry::table)
            .values(&new_entry)
            .execute(&self.connection)
    }

    pub fn insert_systime<'a>(&self, subsystem: &'a str,
                                     parameter: &'a str,
                                     value: &'a str) -> QueryResult<usize>
    {
        let timestamp = time::now_utc().to_timespec().sec;
        self.insert(timestamp as i32, subsystem, parameter, value)
    }
}

table! {
    telemetry (timestamp) {
        timestamp -> Integer,
        subsystem -> Text,
        parameter -> Text,
        value -> Text,
    }
}
