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
use util::{activate_schedule, register_schedule, spawn_scheduler};

#[test]
fn register_new_schedule() {
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

    let expected = json!({
        "data" : {
            "register": {
                "errors": "",
                "success": true
            }
        }
    }
    );

    let response = register_schedule("operational", schedule_path, graphql_ip, graphql_port);

    assert_eq!(expected, response);
}

#[test]
fn register_new_schedule_nonexistant_file() {
    let schedules_dir = TempDir::new().unwrap();
    let graphql_ip = "127.0.0.1";
    let graphql_port = 8021;

    let _scheduler_service = spawn_scheduler(
        &graphql_ip,
        graphql_port,
        schedules_dir.path().to_str().unwrap(),
    );

    let expected = json!({
            "data" : {
                "register": {
                    "errors": "Schedule copy failed: No such file or directory (os error 2)",
                    "success": false
                }
            }
        }
    );
    let response = register_schedule("operational", "", graphql_ip, graphql_port);

    assert_eq!(expected, response);
}

#[test]
fn register_duplicate_schedule() {
    let schedules_dir = TempDir::new().unwrap();
    let graphql_ip = "127.0.0.1";
    let graphql_port = 8022;

    let _scheduler_service = spawn_scheduler(
        &graphql_ip,
        graphql_port,
        schedules_dir.path().to_str().unwrap(),
    );

    let schedule_one = NamedTempFile::new().unwrap();
    let schedule_one_path = schedule_one.path().to_str().unwrap();

    let schedule_two = NamedTempFile::new().unwrap();
    let schedule_two_path = schedule_two.path().to_str().unwrap();

    let expected = json!({
        "data" : {
            "register": {
                "errors": "",
                "success": true
            }
        }
    }
    );

    let response = register_schedule("operational", schedule_one_path, graphql_ip, graphql_port);
    assert_eq!(expected, response);

    let response = register_schedule("operational", schedule_two_path, graphql_ip, graphql_port);
    assert_eq!(expected, response);
}

#[test]
fn activate_existing_schedule() {
    let schedules_dir = TempDir::new().unwrap();
    let graphql_ip = "127.0.0.1";
    let graphql_port = 8023;

    let _scheduler_service = spawn_scheduler(
        graphql_ip,
        graphql_port,
        schedules_dir.path().to_str().unwrap(),
    );

    let schedule = NamedTempFile::new().unwrap();
    let schedule_path = schedule.path().to_str().unwrap();

    let expected = json!({
            "data" : {
                "register": {
                    "errors": "",
                    "success": true
                }
            }
        }
    );
    let response = register_schedule("imaging", schedule_path, graphql_ip, graphql_port);
    assert_eq!(expected, response);

    let expected = json!({
        "data" : {
            "activate": {
                "errors": "",
                "success": true
            }
        }
    }
    );
    let response = activate_schedule("imaging", graphql_ip, graphql_port);
    assert_eq!(expected, response);
}

#[test]
fn activate_non_existent_schedule() {
    let schedules_dir = TempDir::new().unwrap();
    let graphql_ip = "127.0.0.1";
    let graphql_port = 8024;

    let _scheduler_service = spawn_scheduler(
        graphql_ip,
        graphql_port,
        schedules_dir.path().to_str().unwrap(),
    );

    let expected = json!({
            "data" : {
                "activate": {
                    "errors": "Schedule operational.json not found",
                    "success": false
                }
            }
        }
    );
    let response = activate_schedule("operational", graphql_ip, graphql_port);

    assert_eq!(expected, response);
}

#[test]
fn active_two_schedules() {
    let schedules_dir = TempDir::new().unwrap();
    let graphql_ip = "127.0.0.1";
    let graphql_port = 8025;

    let _scheduler_service = spawn_scheduler(
        graphql_ip,
        graphql_port,
        schedules_dir.path().to_str().unwrap(),
    );

    let schedule_one = NamedTempFile::new().unwrap();
    let schedule_one_path = schedule_one.path().to_str().unwrap();

    let schedule_two = NamedTempFile::new().unwrap();
    let schedule_two_path = schedule_two.path().to_str().unwrap();

    let expected = json!({
            "data" : {
                "register": {
                    "errors": "",
                    "success": true
                }
            }
        }
    );

    let response = register_schedule("imaging", schedule_one_path, graphql_ip, graphql_port);
    assert_eq!(expected, response);

    let response = register_schedule("operational", schedule_two_path, graphql_ip, graphql_port);
    assert_eq!(expected, response);

    let expected = json!({
        "data" : {
            "activate": {
                "errors": "",
                "success": true
            }
        }
    }
    );
    let response = activate_schedule("imaging", graphql_ip, graphql_port);
    assert_eq!(expected, response);

    let response = activate_schedule("operational", graphql_ip, graphql_port);
    assert_eq!(expected, response);
}
