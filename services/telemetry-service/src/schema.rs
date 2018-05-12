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
use models::Entry;
use db::Database;

type Context = kubos_service::Context<Database>;

graphql_object!(Entry: () |&self| {
    description: "A telemetry entry"

    field timestamp() -> Option<i32> as "Timestamp" {
        self.timestamp
    }

    field subsystem() -> &Option<String> as "Subsystem name" {
        &self.subsystem
    }

    field parameter() -> &Option<String> as "Telemetry parameter" {
        &self.parameter
    }

    field value() -> Option<f64> as "Telemetry value" {
        self.value
    }
});

pub struct QueryRoot;

graphql_object!(QueryRoot: Context |&self| {
    field telemetry(
        &executor,
        timestamp_ge: Option<i32>,
        timestamp_le: Option<i32>,
        subsystem: Option<String>,
        parameter: Option<String>
    ) -> FieldResult<Vec<Entry>>
        as "Telemetry entries in database"
    {
        use ::db::telemetry::dsl;
        use ::db::telemetry;
        use diesel::sqlite::SqliteConnection;

        let mut query = telemetry::table.into_boxed::<<SqliteConnection as Connection>::Backend>();

        if let Some(subsystem) = subsystem {
            query = query.filter(dsl::subsystem.eq(subsystem));
        }

        if let Some(parameter) = parameter {
            query = query.filter(dsl::parameter.eq(parameter));
        }

        if let Some(timestamp_ge) = timestamp_ge {
            query = query.filter(dsl::timestamp.ge(timestamp_ge));
        }

        if let Some(timestamp_le) = timestamp_le {
            query = query.filter(dsl::timestamp.le(timestamp_le));
        }

        query = query.order(dsl::timestamp);

        Ok(query.load::<Entry>(&executor.context().subsystem().connection)?)
    }
});

pub struct MutationRoot;

graphql_object!(MutationRoot: Context | &self | {});
