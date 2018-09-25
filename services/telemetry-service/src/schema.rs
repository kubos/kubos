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

use diesel;
use diesel::prelude::*;
use juniper::FieldResult;
use kubos_service;
use kubos_telemetry_db::{self, Database};

type Context = kubos_service::Context<Database>;

pub struct Entry(kubos_telemetry_db::Entry);

graphql_object!(Entry: () |&self| {
    description: "A telemetry entry"

    field timestamp() -> i32 as "Timestamp" {
        self.0.timestamp
    }

    field subsystem() -> &String as "Subsystem name" {
        &self.0.subsystem
    }

    field parameter() -> &String as "Telemetry parameter" {
        &self.0.parameter
    }

    field value() -> &String as "Telemetry value" {
        &self.0.value
    }
});

pub struct QueryRoot;

graphql_object!(QueryRoot: Context |&self| {
    field telemetry(
        &executor,
        timestamp_ge: Option<i32>,
        timestamp_le: Option<i32>,
        subsystem: Option<String>,
        parameter: Option<String>,
        limit: Option<i32>,
    ) -> FieldResult<Vec<Entry>>
        as "Telemetry entries in database"
    {
        use kubos_telemetry_db::telemetry::dsl;
        use kubos_telemetry_db::telemetry;
        use diesel::sqlite::SqliteConnection;

        let mut query = telemetry::table.into_boxed::<<SqliteConnection as Connection>::Backend>();

        if let Some(sub) = subsystem {
            query = query.filter(dsl::subsystem.eq(sub));
        }

        if let Some(param) = parameter {
            query = query.filter(dsl::parameter.eq(param));
        }

        if let Some(time_ge) = timestamp_ge {
            query = query.filter(dsl::timestamp.ge(time_ge));
        }

        if let Some(time_le) = timestamp_le {
            query = query.filter(dsl::timestamp.le(time_le));
        }

        if let Some(l) = limit {
            query = query.limit(l.into());
        }

        query = query.order(dsl::timestamp.desc());

        let entries = query.load::<kubos_telemetry_db::Entry>(
            &executor.context().subsystem().connection)?;
        let mut g_entries: Vec<Entry> = Vec::new();
        for entry in entries {
            g_entries.push(Entry(entry));
        }

        Ok(g_entries)
    }
});

pub struct MutationRoot;

#[derive(GraphQLObject)]
struct InsertResponse {
    success: bool,
    errors: String,
}

#[derive(GraphQLObject)]
struct DeleteResponse {
    success: bool,
    errors: String,
    entries_deleted: Option<i32>,
}

graphql_object!(MutationRoot: Context | &self | {
    field insert(&executor, timestamp: Option<i32>, subsystem: String, parameter: String, value: String) -> FieldResult<InsertResponse> {
        let result = match timestamp {
            Some(time) => executor.context().subsystem().insert(time, &subsystem, &parameter, &value),
            None => executor.context().subsystem().insert_systime(&subsystem, &parameter, &value),
        };

        Ok(InsertResponse {
            success: result.is_ok(),
            errors: match result {
                Ok(_) => "".to_owned(),
                Err(err) => format!("{}", err),
            },
        })
    }
    
    field delete(
        &executor,
        timestamp_ge: Option<i32>,
        timestamp_le: Option<i32>,
        subsystem: Option<String>,
        parameter: Option<String>,
    ) -> FieldResult<DeleteResponse>
    {
        use kubos_telemetry_db::telemetry::dsl;
        use kubos_telemetry_db::telemetry;
        use diesel::sqlite::SqliteConnection;

        let mut selection = diesel::delete(telemetry::table).into_boxed::<<SqliteConnection as Connection>::Backend>();

        if let Some(sub) = subsystem {
            selection = selection.filter(dsl::subsystem.eq(sub));
        }

        if let Some(param) = parameter {
            selection = selection.filter(dsl::parameter.eq(param));
        }

        if let Some(time_ge) = timestamp_ge {
            selection = selection.filter(dsl::timestamp.ge(time_ge));
        }

        if let Some(time_le) = timestamp_le {
            selection = selection.filter(dsl::timestamp.le(time_le));
        }

        let result = selection.execute(&executor.context().subsystem().connection);

        match result {
            Ok(num) => Ok(DeleteResponse {
                    success: true,
                    errors: "".to_owned(),
                    entries_deleted: Some(num as i32),
                }),
            Err(err) => Ok(DeleteResponse {
                    success: false,
                    errors: format!("{}", err),
                    entries_deleted: None
                }),
        }
    }
});
