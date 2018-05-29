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

use super::*;
use self::test_data::*;

use kubos_service::{Config, Service};
use model::*;
use schema::*;
use serde_json;
use std::sync::mpsc::sync_channel;

macro_rules! wrap {
    ($result:ident) => {{
        json!({
                    "msg": serde_json::to_string(&$result).unwrap(),
                    "errs": ""
            }).to_string()
    }};
}

mod mutations;
mod queries;
mod test_data;

#[test]
fn ping() {
    let mut mock = MockStream::default();

    let service = service_new!(mock);

    let query = r#"{
            ping
        }"#;

    let expected = json!({
            "ping": "pong"
    });

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}
