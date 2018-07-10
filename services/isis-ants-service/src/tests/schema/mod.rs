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

//use double;
use isis_ants_api::*;
use kubos_service::{Config, Service};
use model::*;
use std::cell::RefCell;
//use objects::*;
use super::*;
use schema::*;

macro_rules! wrap {
    ($result:ident) => {{
        json!({
                "msg": $result,
                "errs": ""
        }).to_string()
    }};
}

macro_rules! service_new {
    ($mock:ident) => {{
        Service::new(
            Config::new("isis-ants-service"),
            Subsystem {
                ants: Box::new($mock),
                errors: RefCell::new(vec![]),
                count: 4,
            },
            QueryRoot,
            MutationRoot,
        )
    }};
}

mod mutations;
mod queries;

#[test]
fn ping() {
    let mock = mock_new!();
    let service = service_new!(mock);

    let query = r#"
        {
            ping
        }"#;

    let expected = json!({
            "ping": "pong"
    });

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}
