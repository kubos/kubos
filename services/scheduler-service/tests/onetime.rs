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

use chrono::prelude::*;
use chrono::Utc;
use serde_json::json;
use std::thread;
use std::time::Duration;
use util::SchedulerFixture;
use utils::testing::ServiceListener;

#[test]
fn run_onetime_future() {
    let listener = ServiceListener::spawn("127.0.0.1", 9021);
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8021);

    fixture.create_mode("init");

    let now_time: DateTime<Utc> = Utc::now()
        .checked_add_signed(chrono::Duration::seconds(1))
        .unwrap();
    let now_time = now_time.format("%Y-%m-%d %H:%M:%S").to_string();

    // Create some schedule with a task starting now
    let schedule = json!({
        "tasks": [
            {
                "name": "basic-task",
                "time": now_time,
                "app": {
                    "name": "basic-app"
                }
            }
        ]
    });
    let schedule_path = fixture.create_task_list(Some(schedule.to_string()));
    fixture.import_task_list("imaging", &schedule_path, "init");
    fixture.activate_mode("init");

    // Check if the task actually ran
    assert_eq!(listener.get_request(), None);

    // Wait for the service to restart the scheduler
    thread::sleep(Duration::from_millis(1100));

    let query = r#"{"query":"mutation { startApp(runLevel: \"onBoot\", name: \"basic-app\") { success, errors } }"}"#;

    // Check if the task actually ran
    assert_eq!(listener.get_request(), Some(query.to_owned()))
}

#[test]
fn run_onetime_past() {
    let listener = ServiceListener::spawn("127.0.0.1", 9022);
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8022);

    fixture.create_mode("init");

    let now_time: DateTime<Utc> = Utc::now()
        .checked_sub_signed(chrono::Duration::seconds(100))
        .unwrap();
    let now_time = now_time.format("%Y-%m-%d %H:%M:%S").to_string();

    // Create some schedule with a task starting now
    let schedule = json!({
        "tasks": [
            {
                "name": "basic-task",
                "time": now_time,
                "app": {
                    "name": "basic-app"
                }
            }
        ]
    });
    let schedule_path = fixture.create_task_list(Some(schedule.to_string()));
    fixture.import_task_list("imaging", &schedule_path, "init");
    fixture.activate_mode("init");

    // Wait for the service to restart the scheduler
    thread::sleep(Duration::from_millis(1100));

    // Verify the task did not run
    assert_eq!(listener.get_request(), None);
}

#[test]
fn run_onetime_far_future() {
    let listener = ServiceListener::spawn("127.0.0.1", 9023);
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8023);

    fixture.create_mode("init");

    let now_time: DateTime<Utc> = Utc::now()
        .checked_add_signed(chrono::Duration::weeks(52 * 100))
        .unwrap();
    let now_time = now_time.format("%Y-%m-%d %H:%M:%S").to_string();
    dbg!(&now_time);

    // Create some schedule with a task starting now
    let schedule = json!({
        "tasks": [
            {
                "name": "basic-task",
                "time": now_time,
                "app": {
                    "name": "basic-app"
                }
            }
        ]
    });
    let schedule_path = fixture.create_task_list(Some(schedule.to_string()));
    fixture.import_task_list("imaging", &schedule_path, "init");
    fixture.activate_mode("init");

    // Wait for the service to restart the scheduler
    thread::sleep(Duration::from_millis(1100));

    // Verify task did not run
    assert_eq!(listener.get_request(), None);

    // Check if scheduler is still alive
    assert_eq!(
        fixture.query(r#"{ availableModes { name, active, schedule { name } } }"#),
        json!({
            "data": {
                "availableModes": [
                    {
                        "name": "init",
                        "active": true,
                        "schedule": [ { "name" : "imaging" } ]
                    },
                    {
                        "name": "safe",
                        "active": false,
                        "schedule": [ ]
                    }
                ]
            }
        })
    );

    // Now actually restart the scheduler
    fixture.restart();
    thread::sleep(Duration::from_millis(100));

    // Check if scheduler is still alive
    assert_eq!(
        fixture.query(r#"{ availableModes { name, active, schedule { name } } }"#),
        json!({
            "data": {
                "availableModes": [
                    {
                        "name": "init",
                        "active": true,
                        "schedule": [ { "name" : "imaging" } ]
                    },
                    {
                        "name": "safe",
                        "active": false,
                        "schedule": [ ]
                    }
                ]
            }
        })
    );
}
