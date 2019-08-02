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
fn run_init_no_tasks() {
    let listener = ServiceListener::spawn("127.0.0.1", 9020);
    let _fixture = SchedulerFixture::spawn("127.0.0.1", 8020);

    thread::sleep(Duration::from_millis(100));

    // Check if the task actually ran
    assert_eq!(listener.get_request(), None);
}

#[test]
fn run_init_single_no_delay() {
    let listener = ServiceListener::spawn("127.0.0.1", 9021);
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8021);

    // Register some schedule with an init task
    let schedule = json!({
        "init": {
            "basic-task": {
                "delay": "00:00:00",
                "app": {
                    "name": "basic-app"
                }
            }
        }
    });
    let schedule_path = fixture.create(Some(schedule.to_string()));
    fixture.register("imaging", &schedule_path);
    fixture.activate("imaging");

    // Restart the service to run the task
    fixture.restart();
    thread::sleep(Duration::from_millis(100));

    let query = r#"{"query":"mutation { startApp(name: \"basic-app\") { success } }"}"#;

    // Check if the task actually ran
    assert_eq!(listener.get_request(), Some(query.to_owned()))
}

#[test]
fn run_init_single_with_delay() {
    let listener = ServiceListener::spawn("127.0.0.1", 9022);
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8022);

    // Register some schedule with an init task
    let schedule = json!({
        "init": {
            "basic-task": {
                "delay": "00:00:01",
                "app": {
                    "name": "basic-app"
                }
            }
        }
    });
    let schedule_path = fixture.create(Some(schedule.to_string()));
    fixture.register("imaging", &schedule_path);
    fixture.activate("imaging");

    // Restart the service to run the task
    fixture.restart();
    thread::sleep(Duration::from_secs(1));

    let query = r#"{"query":"mutation { startApp(name: \"basic-app\") { success } }"}"#;

    // Check if the task actually ran
    assert_eq!(listener.get_request(), Some(query.to_owned()))
}

#[test]
fn run_init_two_tasks() {
    let listener = ServiceListener::spawn("127.0.0.1", 9023);
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8023);

    // Register some schedule with an init task
    let schedule = json!({
        "init": {
            "second-task": {
                "delay": "00:00:01",
                "app": {
                    "name": "other-app"
                }
            },
            "basic-task": {
                "delay": "00:00:00",
                "app": {
                    "name": "basic-app"
                }
            }
        }
    });
    let schedule_path = fixture.create(Some(schedule.to_string()));
    fixture.register("two", &schedule_path);
    fixture.activate("two");

    // Restart the service to run the task
    fixture.restart();
    thread::sleep(Duration::from_secs(1));

    // Check if first task ran
    let query = r#"{"query":"mutation { startApp(name: \"basic-app\") { success } }"}"#;
    assert_eq!(listener.get_request(), Some(query.to_owned()));

    // Check if second app ran in order
    let query = r#"{"query":"mutation { startApp(name: \"other-app\") { success } }"}"#;
    assert_eq!(listener.get_request(), Some(query.to_owned()));
}
