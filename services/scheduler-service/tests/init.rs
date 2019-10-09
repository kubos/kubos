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

    fixture.create_mode("init");

    // Create some schedule with an init task
    let schedule = json!({
        "tasks": [
            {
                "name": "basic-task",
                "delay": "0s",
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
    thread::sleep(Duration::from_millis(100));

    let query = r#"{"query":"mutation { startApp(name: \"basic-app\") { success, errors } }"}"#;

    // Check if the task actually ran
    assert_eq!(listener.get_request(), Some(query.to_owned()))
}

#[test]
fn run_init_single_with_delay() {
    let listener = ServiceListener::spawn("127.0.0.1", 9022);
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8022);
    fixture.create_mode("init");

    // Create some schedule with an init task
    let schedule = json!({
        "tasks": [
            {
                "name": "basic-task",
                "delay": "1s",
                "app": {
                    "name": "basic-app"
                }
            }
        ]
    });
    let schedule_path = fixture.create_task_list(Some(schedule.to_string()));
    fixture.import_task_list("imaging", &schedule_path, "init");
    fixture.activate_mode("init");

    // This task should not have run immediately
    thread::sleep(Duration::from_millis(100));
    assert_eq!(listener.get_request(), None);

    // Wait for service to run the task
    thread::sleep(Duration::from_millis(1000));

    let query = r#"{"query":"mutation { startApp(name: \"basic-app\") { success, errors } }"}"#;

    // Check if the task actually ran
    assert_eq!(listener.get_request(), Some(query.to_owned()))
}

#[test]
fn run_init_two_tasks() {
    let listener = ServiceListener::spawn("127.0.0.1", 9023);
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8023);
    fixture.create_mode("init");

    // Create some schedule with an init task
    let schedule = json!({
        "tasks": [
            {
                "name": "second-task",
                "delay": "1s",
                "app": {
                    "name": "other-app"
                }
            },
            {
                "name": "basic-task",
                "delay": "0s",
                "app": {
                    "name": "basic-app"
                }
            }
        ]
    });
    let schedule_path = fixture.create_task_list(Some(schedule.to_string()));
    fixture.import_task_list("two", &schedule_path, "init");
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
fn run_init_single_args() {
    let listener = ServiceListener::spawn("127.0.0.1", 9024);
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8024);
    fixture.create_mode("init");

    // Create some schedule with an init task
    let schedule = json!({
        "tasks": [
            {
                "name": "basic-task",
                "delay": "0s",
                "app": {
                    "name": "basic-app",
                    "args": ["-l", "-h"]
                }
            }
        ]
    });
    let schedule_path = fixture.create_task_list(Some(schedule.to_string()));
    fixture.import_task_list("imaging", &schedule_path, "init");
    fixture.activate_mode("init");

    // Wait for service to restart scheduler and run task
    thread::sleep(Duration::from_millis(100));

    let query = r#"{"query":"mutation { startApp(name: \"basic-app\", args: [\"-l\",\"-h\"]) { success, errors } }"}"#;

    // Check if the task actually ran
    assert_eq!(listener.get_request(), Some(query.to_owned()))
}

#[test]
fn run_init_single_custom_task_list() {
    let listener = ServiceListener::spawn("127.0.0.1", 9025);
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8025);
    fixture.create_mode("init");

    // Create some schedule with an init task
    let schedule = json!({
        "tasks": [
            {
                "name": "basic-task",
                "delay": "0s",
                "app": {
                    "name": "basic-app",
                    "config": "path/to/custom.toml"
                }
            }
        ]
    });
    let schedule_path = fixture.create_task_list(Some(schedule.to_string()));
    fixture.import_task_list("imaging", &schedule_path, "init");
    fixture.activate_mode("init");

    // Wait for service to restart scheduler and run task
    thread::sleep(Duration::from_millis(100));

    let query = r#"{"query":"mutation { startApp(name: \"basic-app\", config: \"path/to/custom.toml\") { success, errors } }"}"#;

    // Check if the task actually ran
    assert_eq!(listener.get_request(), Some(query.to_owned()))
}

#[test]
fn run_init_two_schedules_one_mode() {
    let listener = ServiceListener::spawn("127.0.0.1", 9027);
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8027);
    fixture.create_mode("init");

    // Create first schedule with an init task
    let schedule = json!({
        "tasks": [
            {
                "name": "basic-task",
                "delay": "0s",
                "app": {
                    "name": "first-app"
                }
            }
        ]
    });
    let schedule_path = fixture.create_task_list(Some(schedule.to_string()));
    fixture.import_task_list("first", &schedule_path, "init");

    // Create second schedule with an init task
    let schedule = json!({
        "tasks": [
            {
                "name": "basic-task",
                "delay": "1s",
                "app": {
                    "name": "second-app"
                }
            }
        ]
    });
    let schedule_path = fixture.create_task_list(Some(schedule.to_string()));
    fixture.import_task_list("second", &schedule_path, "init");

    // Activate first schedule, wait for task to run
    fixture.activate_mode("init");
    thread::sleep(Duration::from_millis(1100));

    // Check if the task ran
    let query = r#"{"query":"mutation { startApp(name: \"first-app\") { success, errors } }"}"#;
    assert_eq!(listener.get_request(), Some(query.to_owned()));

    // Check if the task ran
    let query = r#"{"query":"mutation { startApp(name: \"second-app\") { success, errors } }"}"#;
    assert_eq!(listener.get_request(), Some(query.to_owned()))
}

#[test]
fn run_init_two_modes() {
    let listener = ServiceListener::spawn("127.0.0.1", 9028);
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8028);
    fixture.create_mode("init");
    fixture.create_mode("secondary");

    // Create first schedule with an init task
    let schedule = json!({
        "tasks": [
            {
                "name": "basic-task",
                "delay": "0s",
                "app": {
                    "name": "first-app"
                }
            }
        ]
    });
    let schedule_path = fixture.create_task_list(Some(schedule.to_string()));
    fixture.import_task_list("first", &schedule_path, "init");

    // Create second schedule with an init task
    let schedule = json!({
        "tasks": [
            {
                "name": "basic-task",
                "delay": "0s",
                "app": {
                    "name": "second-app"
                }
            }
        ]
    });
    let schedule_path = fixture.create_task_list(Some(schedule.to_string()));
    fixture.import_task_list("second", &schedule_path, "secondary");

    // Activate first schedule, wait for task to run
    fixture.activate_mode("init");
    thread::sleep(Duration::from_millis(100));

    // Check if the task ran
    let query = r#"{"query":"mutation { startApp(name: \"first-app\") { success, errors } }"}"#;
    assert_eq!(listener.get_request(), Some(query.to_owned()));

    // Activate second schedule, wait for task to run
    fixture.activate_mode("secondary");
    thread::sleep(Duration::from_millis(100));

    // Check if the task ran
    let query = r#"{"query":"mutation { startApp(name: \"second-app\") { success, errors } }"}"#;
    assert_eq!(listener.get_request(), Some(query.to_owned()))
}

#[test]
fn run_init_two_modes_check_stop() {
    let listener = ServiceListener::spawn("127.0.0.1", 9029);
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8029);
    fixture.create_mode("init");
    fixture.create_mode("secondary");

    // Register first schedule with an init task
    let schedule = json!({
        "tasks": [
            {
                "name": "basic-task",
                "delay": "1s",
                "app": {
                    "name": "delay-app"
                }
            }
        ]
    });
    let schedule_path = fixture.create_task_list(Some(schedule.to_string()));
    fixture.import_task_list("first", &schedule_path, "init");

    // Register second schedule with an init task
    let schedule = json!({
        "tasks": [
            {
                "name": "basic-task",
                "delay": "0s",
                "app": {
                    "name": "second-app"
                }
            }
        ]
    });
    let schedule_path = fixture.create_task_list(Some(schedule.to_string()));
    fixture.import_task_list("second", &schedule_path, "secondary");

    // Activate first schedule, wait for task to run
    fixture.activate_mode("init");
    thread::sleep(Duration::from_millis(100));

    // Activate second schedule, wait for task to run
    fixture.activate_mode("secondary");
    thread::sleep(Duration::from_millis(100));

    // Check if the task ran
    let query = r#"{"query":"mutation { startApp(name: \"second-app\") { success, errors } }"}"#;
    assert_eq!(listener.get_request(), Some(query.to_owned()));

    // Give the scheduler time to run (or not) delayed task from first schedule
    thread::sleep(Duration::from_millis(1100));

    // Check if the task ran
    assert_eq!(listener.get_request(), None)
}

#[test]
fn run_init_after_import() {
    let listener = ServiceListener::spawn("127.0.0.1", 9030);
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8030);
    fixture.create_mode("init");

    // Register first schedule with an init task
    let schedule = json!({
        "tasks": [
            {
                "name": "basic-task",
                "delay": "0s",
                "app": {
                    "name": "first-app"
                }
            }
        ]
    });
    let schedule_path = fixture.create_task_list(Some(schedule.to_string()));

    // Activate mode
    fixture.activate_mode("init");
    thread::sleep(Duration::from_millis(100));

    // Mode is empty, so no task should have run
    assert_eq!(listener.get_request(), None);

    // Import task_list then confirm task is run afterwards
    fixture.import_task_list("first", &schedule_path, "init");
    thread::sleep(Duration::from_millis(100));

    // Check if the task ran
    let query = r#"{"query":"mutation { startApp(name: \"first-app\") { success, errors } }"}"#;
    assert_eq!(listener.get_request(), Some(query.to_owned()));
}

#[test]
fn run_init_check_remove() {
    let listener = ServiceListener::spawn("127.0.0.1", 9031);
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8031);
    fixture.create_mode("init");

    // Register task_list with a delayed task
    let schedule = json!({
        "tasks": [
            {
                "name": "basic-task",
                "delay": "1s",
                "app": {
                    "name": "delay-app"
                }
            }
        ]
    });
    let schedule_path = fixture.create_task_list(Some(schedule.to_string()));
    fixture.import_task_list("first", &schedule_path, "init");

    // Activate mode
    fixture.activate_mode("init");
    thread::sleep(Duration::from_millis(100));

    // Task is delayed so it shouldn't have run
    assert_eq!(listener.get_request(), None);

    // Remove the task_list
    fixture.remove_task_list("first", "init");
    thread::sleep(Duration::from_millis(1100));

    // Verify task did not run
    assert_eq!(listener.get_request(), None);
}

#[test]
fn run_init_import_twice() {
    let listener = ServiceListener::spawn("127.0.0.1", 9032);
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8032);

    fixture.create_mode("init");

    // Create some schedule with an init task
    let schedule = json!({
        "tasks": [
            {
                "name": "basic-task",
                "delay": "0s",
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
    thread::sleep(Duration::from_millis(100));

    let query = r#"{"query":"mutation { startApp(runLevel: \"onBoot\", name: \"basic-app\") { success, errors } }"}"#;

    // Check if the task actually ran
    assert_eq!(listener.get_request(), Some(query.to_owned()));

    // Import task list again and verify task ran again
    fixture.import_task_list("imaging", &schedule_path, "init");

    // Wait for the service to restart the scheduler
    thread::sleep(Duration::from_millis(100));

    // Check if the task actually ran
    assert_eq!(listener.get_request(), Some(query.to_owned()))
}
