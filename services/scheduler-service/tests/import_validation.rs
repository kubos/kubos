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
use util::SchedulerFixture;

#[test]
fn validate_bad_json() {
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8020);

    fixture.create_mode("operational");

    let schedule = json!("not json");
    let schedule_path = fixture.create_task_list(Some(schedule.to_string()));
    assert_eq!(
        fixture.import_task_list("first", &schedule_path, "operational"),
        json!({
            "data" : {
                "importTaskList": {
                    "errors": "Failed to parse task list \'first\': Failed to parse json: invalid type: string \"not json\", expected struct ListContents at line 1 column 10",
                    "success": false
                }
            }
        })
    );
}

#[test]
fn validate_no_task_app() {
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8022);

    fixture.create_mode("operational");

    let schedule = json!({
        "tasks": [
            {
                "description": "first-task",
                "delay": "10s",
            },
        ]
    });
    let schedule_path = fixture.create_task_list(Some(schedule.to_string()));
    assert_eq!(
        fixture.import_task_list("first", &schedule_path, "operational"),
        json!({
            "data" : {
                "importTaskList": {
                    "errors": "Failed to parse task list \'first\': Failed to parse json: missing field `app` at line 1 column 52",
                    "success": false
                }
            }
        })
    );
}

#[test]
fn validate_no_app_name() {
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8023);

    fixture.create_mode("operational");

    let schedule = json!({
        "tasks": [
            {
                "description": "first-task",
                "delay": "10s",
                "app": { },
            },
        ]
    });
    let schedule_path = fixture.create_task_list(Some(schedule.to_string()));
    assert_eq!(
        fixture.import_task_list("first", &schedule_path, "operational"),
        json!({
            "data" : {
                "importTaskList": {
                    "errors": "Failed to parse task list \'first\': Failed to parse json: missing field `name` at line 1 column 19",
                    "success": false
                }
            }
        })
    );
}

#[test]
fn validate_no_time_specification() {
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8021);

    fixture.create_mode("operational");

    let schedule = json!({
        "tasks": [
            {
                "description": "first-task",
                "app": {
                    "name": "other-app"
                }
            },
        ]
    });
    let schedule_path = fixture.create_task_list(Some(schedule.to_string()));
    assert_eq!(
        fixture.import_task_list("first", &schedule_path, "operational"),
        json!({
            "data" : {
                "importTaskList": {
                    "errors": "Failed to parse task \'first-task\': No delay or time defined",
                    "success": false
                }
            }
        })
    );
}

#[test]
fn validate_bad_delay() {
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8024);

    fixture.create_mode("operational");

    let schedule = json!({
        "tasks": [
            {
                "description": "first-task",
                "delay": "invalid",
                "app": {
                    "name": "app-name"
                },
            },
        ]
    });
    let schedule_path = fixture.create_task_list(Some(schedule.to_string()));
    assert_eq!(
        fixture.import_task_list("first", &schedule_path, "operational"),
        json!({
            "data" : {
                "importTaskList": {
                    "errors": "Failed to parse hms field \'invalid\': Failed to parse number",
                    "success": false
                }
            }
        })
    );
}

#[test]
fn validate_bad_time() {
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8025);

    fixture.create_mode("operational");

    let schedule = json!({
        "tasks": [
            {
                "description": "first-task",
                "time": "invalid",
                "app": {
                    "name": "app-name"
                },
            },
        ]
    });
    let schedule_path = fixture.create_task_list(Some(schedule.to_string()));
    assert_eq!(
        fixture.import_task_list("first", &schedule_path, "operational"),
        json!({
            "data" : {
                "importTaskList": {
                    "errors": "Failed to parse task \'first-task\': Failed to parse time field \'invalid\': input contains invalid characters",
                    "success": false
                }
            }
        })
    );
}

#[test]
fn validate_bad_period() {
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8026);

    fixture.create_mode("operational");

    let schedule = json!({
        "tasks": [
            {
                "description": "first-task",
                "delay": "0s",
                "period": "invalid",
                "app": {
                    "name": "app-name"
                },
            },
        ]
    });
    let schedule_path = fixture.create_task_list(Some(schedule.to_string()));
    assert_eq!(
        fixture.import_task_list("first", &schedule_path, "operational"),
        json!({
            "data" : {
                "importTaskList": {
                    "errors": "Failed to parse hms field \'invalid\': Failed to parse number",
                    "success": false
                }
            }
        })
    );
}

#[test]
fn validate_past_time() {
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8027);

    fixture.create_mode("operational");

    let schedule = json!({
        "tasks": [
            {
                "description": "first-task",
                "time": "1970-1-1 00:00:00",
                "app": {
                    "name": "app-name"
                },
            },
        ]
    });
    let schedule_path = fixture.create_task_list(Some(schedule.to_string()));
    assert_eq!(
        fixture.import_task_list("first", &schedule_path, "operational"),
        json!({
            "data" : {
                "importTaskList": {
                    "errors": "Out of bounds time found in task \'first-task\': Task scheduled for past time: 1970-1-1 00:00:00",
                    "success": false
                }
            }
        })
    );
}

#[test]
fn validate_far_future_time() {
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8028);

    fixture.create_mode("operational");

    let schedule = json!({
        "tasks": [
            {
                "description": "first-task",
                "time": "2070-1-1 00:00:00",
                "app": {
                    "name": "app-name"
                },
            },
        ]
    });
    let schedule_path = fixture.create_task_list(Some(schedule.to_string()));
    assert_eq!(
        fixture.import_task_list("first", &schedule_path, "operational"),
        json!({
            "data" : {
                "importTaskList": {
                    "errors": "Out of bounds time found in task \'first-task\': Task scheduled beyond 90 days in the future: 2070-1-1 00:00:00",
                    "success": false
                }
            }
        })
    );
}

#[test]
fn validate_time_and_delay() {
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8029);

    fixture.create_mode("operational");

    let schedule = json!({
        "tasks": [
            {
                "description": "first-task",
                "time": "1970-1-1 00:00:00",
                "delay": "0s",
                "app": {
                    "name": "app-name"
                },
            },
        ]
    });
    let schedule_path = fixture.create_task_list(Some(schedule.to_string()));
    assert_eq!(
        fixture.import_task_list("first", &schedule_path, "operational"),
        json!({
            "data" : {
                "importTaskList": {
                    "errors": "Failed to parse task \'first-task\': Both delay and time defined",
                    "success": false
                }
            }
        })
    );
}
