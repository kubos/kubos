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
use juniper::FieldResult;
use kubos_service;
use kubos_telemetry::{self, Database};

type Context = kubos_service::Context<Database>;

pub struct Entry(kubos_telemetry::Entry);

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
        use kubos_telemetry::telemetry::dsl;
        use kubos_telemetry::telemetry;
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

        query = query.order(dsl::timestamp);

        let entries = query.load::<kubos_telemetry::Entry>(
            &executor.context().subsystem().connection)?;
        let mut g_entries: Vec<Entry> = Vec::new();
        for entry in entries {
            g_entries.push(Entry(entry));
        }

        Ok(g_entries)
    }
});

pub struct MutationRoot;

graphql_object!(MutationRoot: Context | &self | {});
