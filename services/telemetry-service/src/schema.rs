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

use db::Database;
use diesel::prelude::*;
use juniper::FieldResult;
use kubos_service;
use models::Entry;

type Context = kubos_service::Context<Database>;

graphql_object!(Entry: () |&self| {
    description: "A telemetry entry"

    field timestamp() -> i32 as "Timestamp" {
        self.timestamp
    }

    field subsystem() -> &String as "Subsystem name" {
        &self.subsystem
    }

    field parameter() -> &String as "Telemetry parameter" {
        &self.parameter
    }

    field value() -> &String as "Telemetry value" {
        &self.value
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
        use ::db::telemetry::dsl;
        use ::db::telemetry;
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

        Ok(query.load::<Entry>(&executor.context().subsystem().connection)?)
    }
});

pub struct MutationRoot;

graphql_object!(MutationRoot: Context | &self | {});
