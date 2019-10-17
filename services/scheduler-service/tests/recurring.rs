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
use std::thread;
use std::time::Duration;
use util::SchedulerFixture;
use utils::testing::ServiceListener;

#[test]
fn run_recurring_no_delay() {
    let listener = ServiceListener::spawn("127.0.0.1", 9021);
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8021);

    fixture.create_mode("init");

    // Create some schedule with a recurring task starting now
    let schedule = json!({
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
    });
    let schedule_path = fixture.create_task_list(Some(schedule.to_string()));
    fixture.import_task_list("imaging", &schedule_path, "init");
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
fn run_recurring_delay() {
    let listener = ServiceListener::spawn("127.0.0.1", 9022);
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8022);

    fixture.create_mode("init");

    // Create some schedule with a recurring task starting now
    let schedule = json!({
        "tasks": [
            {
                "description": "basic-task",
                "delay": "1s",
                "period": "1s",
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
    thread::sleep(Duration::from_millis(2100));

    let query = r#"{"query":"mutation { startApp(name: \"basic-app\") { success, errors } }"}"#;

    // Check if the task was run only twice
    assert_eq!(listener.get_request(), Some(query.to_owned()));
    assert_eq!(listener.get_request(), Some(query.to_owned()));
    assert_eq!(listener.get_request(), None)
}
