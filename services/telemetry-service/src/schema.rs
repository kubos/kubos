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
use juniper::{Context as JuniperContext, FieldResult};
use kubos_service;
use models::Telem;
use db::Database;

type Context = kubos_service::Context<Database>;

graphql_object!(Telem: () |&self| {
    description: "A telemetry point"

        field timestamp() -> Option<i32> as "The timestamp" {
            self.timestamp
        }

    field subsystem() -> &Option<String> as "The subsystem name" {
        &self.subsystem
    }

    field param() -> &Option<String> as "Telemetry params" {
        &self.param
    }

    field value() -> Option<i32> as "Telem values" {
        self.value
    }
});

pub struct QueryRoot;

graphql_object!(QueryRoot: Context |&self| {
    field telemItems(&executor) -> FieldResult<Vec<Telem>>
        as "Get all telemetry items in the system ordered by date"
    {
        use ::db::Telemetry::dsl;

        Ok(dsl::Telemetry.order(dsl::timestamp)
            .load::<Telem>(&executor.context().subsystem().connection)?)
    }

    field telem(&executor) -> FieldResult<Vec<Telem>>
    {
        use ::db::Telemetry::dsl;

        Ok(dsl::Telemetry.order(dsl::subsystem)
            .load::<Telem>(&executor.context().subsystem().connection)?)
    }

    // field telemetry(&executor, subsystem: Option<String>) -> FieldResult<Vec<Telem>>
    // {
    //     use ::db::Telemetry::dsl;

    //     let mut tq = dsl::Telemetry.all();

    //     match subsystem {
    //         Some(ref s) => {
    //             tq = tq.filter(dsl::subsystem.eq(s));
    //         },
    //         None => {

    //         }
    //     }
    //     return tq.load::<Telem>(&executor.context().connection).to_field_err();
    // }
});

pub struct MutationRoot;

graphql_object!(
    MutationRoot: Context | &self | {
        /*field add_todo(&executor, title: String, completed: bool) -> FieldResult<Todo>
        as "Create a new todo item and return it"
    {
        use ::db::todos::dsl;

        executor.context().connection.transaction(|| {
            let new_post = NewTodo {
                title: &title,
                completed: completed,
            };

            diesel::insert(&new_post).into(::db::todos::table)
                .execute(&executor.context().connection)?;

            dsl::todos.order(dsl::id.desc())
                .first::<Todo>(&executor.context().connection)
        }).to_field_err()
    }

    field update_todo(&executor, id: i32, completed: Option<bool>, title: Option<String>) -> FieldResult<Option<Todo>>
        as "Update an existing todo item.

        Will only update the provided fields - if either `completed` or `title`
        are omitted or null, they will be ignored.

        The mutation will return null if no todo item with the specified ID could be found."
    {
        use ::db::todos::dsl;

        let updated = jtry!(diesel::update(dsl::todos.find(id))
            .set((
                completed.map(|completed| dsl::completed.eq(completed)),
                title.map(|title| dsl::title.eq(title)),
            ))
            .execute(&executor.context().connection));

        if updated == 0 {
            Ok(None)
        }
        else {
            Ok(Some(jtry!(dsl::todos.find(id)
                .get_result::<Todo>(&executor.context().connection))))
        }
    }*/
    }
);
