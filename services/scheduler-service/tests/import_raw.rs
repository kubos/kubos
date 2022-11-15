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

use chrono::{DateTime, Utc};
use serde_json::json;
use std::thread;
use std::time::Duration;
use util::SchedulerFixture;
use utils::testing::ServiceListener;

#[test]
fn import_raw_tasks() {
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8020);

    fixture.create_mode("operational");

    // Create some schedule with an init task
    let schedule: String = json!({
        "tasks": [
            {
                "description": "basic-task",
                "delay": "0s",
                "app": {
                    "name": "basic-app"
                }
            }
        ]
    })
    .to_string()
    .escape_default()
    .collect();
    assert_eq!(
        fixture.import_raw_task_list("first", "operational", &schedule),
        json!({
            "data" : {
                "importRawTaskList": {
                    "errors": "",
                    "success": true
                }
            }
        })
    );

    assert_eq!(
        fixture.query(r#"{ availableModes { name, active, schedule { filename } } }"#),
        json!({
            "data": {
                "availableModes": [
                    {
                        "name": "operational",
                        "active": false,
                        "schedule": [
                            {
                                "filename": "first"
                            }
                        ]
                    },
                    {
                        "name": "safe",
                        "active": true,
                        "schedule": [ ]
                    }
                ]
            }
        })
    );
}

#[test]
fn import_raw_run_delay() {
    let listener = ServiceListener::spawn("127.0.0.1", 9021);
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8021);

    fixture.create_mode("init");

    // Create some schedule with an init task
    let schedule: String = json!({
        "tasks": [
            {
                "description": "basic-task",
                "delay": "0s",
                "app": {
                    "name": "basic-app"
                }
            }
        ]
    })
    .to_string()
    .escape_default()
    .collect();
    fixture.import_raw_task_list("imaging", "init", &schedule);
    fixture.activate_mode("init");

    // Wait for the service to restart the scheduler
    thread::sleep(Duration::from_millis(100));

    let query = r#"{"query":"mutation { startApp(name: \"basic-app\") { success, errors } }"}"#;

    // Check if the task actually ran
    assert_eq!(listener.get_request(), Some(query.to_owned()))
}

#[test]
fn import_raw_run_two_tasks() {
    let listener = ServiceListener::spawn("127.0.0.1", 9022);
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8022);
    fixture.create_mode("init");

    // Create some schedule with an init task
    let schedule: String = json!({
        "tasks": [
            {
                "description": "second-task",
                "delay": "1s",
                "app": {
                    "name": "other-app"
                }
            },
            {
                "description": "basic-task",
                "delay": "0s",
                "app": {
                    "name": "basic-app"
                }
            }
        ]
    })
    .to_string()
    .escape_default()
    .collect();
    fixture.import_raw_task_list("two", "init", &schedule);
    fixture.activate_mode("init");

    // Wait for service to restart scheduler and run tasks
    thread::sleep(Duration::from_millis(1100));

    // Check if first task ran
    let query = r#"{"query":"mutation { startApp(name: \"basic-app\") { success, errors } }"}"#;
    assert_eq!(listener.get_request(), Some(query.to_owned()));

    // Check if second app ran in order
    let query = r#"{"query":"mutation { startApp(name: \"other-app\") { success, errors } }"}"#;
    assert_eq!(listener.get_request(), Some(query.to_owned()));
}

#[test]
fn import_raw_run_onetime_future() {
    let listener = ServiceListener::spawn("127.0.0.1", 9023);
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8023);

    fixture.create_mode("init");
    fixture.activate_mode("init");

    let now_time: DateTime<Utc> = Utc::now()
        .checked_add_signed(chrono::Duration::seconds(1))
        .unwrap();
    let now_time = now_time.format("%Y-%m-%d %H:%M:%S").to_string();

    // Create some schedule with a task starting now
    let schedule: String = json!({
        "tasks": [
            {
                "description": "basic-task",
                "time": now_time,
                "app": {
                    "name": "basic-app"
                }
            }
        ]
    })
    .to_string()
    .escape_default()
    .collect();
    fixture.import_raw_task_list("imaging", "init", &schedule);

    // Check if the task actually ran
    assert_eq!(listener.get_request(), None);

    // Wait for the service to restart the scheduler
    thread::sleep(Duration::from_millis(2000));

    let query = r#"{"query":"mutation { startApp(name: \"basic-app\") { success, errors } }"}"#;

    // Check if the task actually ran
    assert_eq!(listener.get_request(), Some(query.to_owned()))
}

#[test]
fn import_raw_run_recurring_no_delay() {
    let listener = ServiceListener::spawn("127.0.0.1", 9024);
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8024);

    fixture.create_mode("init");

    // Create some schedule with a recurring task starting now
    let schedule: String = json!({
        "tasks": [
            {
                "description": "basic-task",
                "delay": "0s",
                "period": "1s",
                "app": {
                    "name": "basic-app"
                }
            }
        ]
    })
    .to_string()
    .escape_default()
    .collect();
    fixture.import_raw_task_list("imaging", "init", &schedule);
    fixture.activate_mode("init");

    // Wait for the service to restart the scheduler
    thread::sleep(Duration::from_millis(1100));

    let query = r#"{"query":"mutation { startApp(name: \"basic-app\") { success, errors } }"}"#;

    // Check if the task was run only twice
    assert_eq!(listener.get_request(), Some(query.to_owned()));
    assert_eq!(listener.get_request(), Some(query.to_owned()));
    assert_eq!(listener.get_request(), None)
}

#[test]
fn import_raw_bad_json() {
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8025);

    fixture.create_mode("operational");

    // Create some schedule with an init task
    let schedule = "this is not json";
    assert_eq!(
        fixture.import_raw_task_list("first", "operational", schedule),
        json!({
            "data" : {
                "importRawTaskList": {
                    "errors": "Failed to parse task list 'first': Failed to parse json: expected ident at line 1 column 2",
                    "success": false
                }
            }
        })
    );

    assert_eq!(
        fixture.query(r#"{ availableModes { name, active, schedule { filename } } }"#),
        json!({
            "data": {
                "availableModes": [
                    {
                        "name": "operational",
                        "active": false,
                        "schedule": [ ]
                    },
                    {
                        "name": "safe",
                        "active": true,
                        "schedule": [ ]
                    }
                ]
            }
        })
    );
}

#[test]
fn import_raw_run_delay_duplicate() {
    let listener = ServiceListener::spawn("127.0.0.1", 9026);
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8026);

    fixture.create_mode("init");

    // Create some schedule with an init task
    let schedule: String = json!({
        "tasks": [
            {
                "description": "basic-task",
                "delay": "0s",
                "app": {
                    "name": "basic-app"
                }
            }
        ]
    })
    .to_string()
    .escape_default()
    .collect();
    fixture.import_raw_task_list("imaging", "init", &schedule);
    fixture.activate_mode("init");

    // Wait for the service to restart the scheduler
    thread::sleep(Duration::from_millis(100));

    let query = r#"{"query":"mutation { startApp(name: \"basic-app\") { success, errors } }"}"#;

    // Check if the task actually ran
    assert_eq!(listener.get_request(), Some(query.to_owned()));

    fixture.import_raw_task_list("imaging", "init", &schedule);

    // Wait for the service to restart the scheduler
    thread::sleep(Duration::from_millis(100));

    // Check if the task actually ran
    assert_eq!(listener.get_request(), Some(query.to_owned()))
}
