//
// Copyright (C) 2019 Kubos Corporation
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

mod util;

use serde_json::json;
use tempfile::{NamedTempFile, TempDir};
use util::{activate_schedule, register_schedule, remove_schedule, spawn_scheduler};

#[test]
fn remove_existing_schedule() {
    let schedules_dir = TempDir::new().unwrap();
    let graphql_ip = "127.0.0.1";
    let graphql_port = 8020;

    let _scheduler_service = spawn_scheduler(
        &graphql_ip,
        graphql_port,
        schedules_dir.path().to_str().unwrap(),
    );

    let schedule = NamedTempFile::new().unwrap();
    let schedule_path = schedule.path().to_str().unwrap();

    register_schedule("operational", schedule_path, graphql_ip, graphql_port);

    let expected = json!({
        "data": {
            "remove": {
                "errors": "",
                "success": true
            }
        }
    });
    let response = remove_schedule("operational", graphql_ip, graphql_port);
    assert_eq!(expected, response);
}

#[test]
fn remove_active_schedule() {
    let schedules_dir = TempDir::new().unwrap();
    let graphql_ip = "127.0.0.1";
    let graphql_port = 8022;

    let _scheduler_service = spawn_scheduler(
        &graphql_ip,
        graphql_port,
        schedules_dir.path().to_str().unwrap(),
    );

    let schedule = NamedTempFile::new().unwrap();
    let schedule_path = schedule.path().to_str().unwrap();

    register_schedule("operational", schedule_path, graphql_ip, graphql_port);
    activate_schedule("operational", graphql_ip, graphql_port);

    let expected = json!({
        "data": {
            "remove": {
                "errors": "",
                "success": true
            }
        }
    });
    let response = remove_schedule("operational", graphql_ip, graphql_port);
    assert_eq!(expected, response);
}

#[test]
fn remove_nonexistant_schedule() {
    let schedules_dir = TempDir::new().unwrap();
    let graphql_ip = "127.0.0.1";
    let graphql_port = 8021;

    let _scheduler_service = spawn_scheduler(
        &graphql_ip,
        graphql_port,
        schedules_dir.path().to_str().unwrap(),
    );

    let expected = json!({
        "data": {
            "remove": {
                "errors": "Failed to remove schedule operational.json: No such file or directory (os error 2)",
                "success": false
            }
        }
    });
    let response = remove_schedule("operational", graphql_ip, graphql_port);
    assert_eq!(expected, response);
}
