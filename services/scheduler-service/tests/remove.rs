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
fn remove_existing_mode() {
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8020);

    fixture.create_mode("orbit");

    assert_eq!(
        fixture.query(r#"{ availableModes { name, active } }"#),
        json!({
            "data": {
                "availableModes": [
                    {
                        "name": "orbit",
                        "active": false
                    },
                    {
                        "name": "safe",
                        "active": true,
                    }
                ]
            }
        })
    );

    assert_eq!(
        fixture.remove_mode("orbit"),
        json!({
            "data" : {
                "removeMode": {
                    "errors": "",
                    "success": true
                }
            }
        })
    );

    assert_eq!(
        fixture.query(r#"{ availableModes { name, active } }"#),
        json!({
            "data": {
                "availableModes": [
                    {
                        "name": "safe",
                        "active": true,
                    }
                 ]
            }
        })
    );
}

#[test]
fn remove_active_mode() {
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8021);

    fixture.create_mode("orbit");
    fixture.activate_mode("orbit");

    assert_eq!(
        fixture.query(r#"{ availableModes { name, active } }"#),
        json!({
            "data": {
                "availableModes": [
                    {
                        "name": "orbit",
                        "active": true
                    },
                    {
                        "name": "safe",
                        "active": false,
                    }
                ]
            }
        })
    );

    assert_eq!(
        fixture.remove_mode("orbit"),
        json!({
            "data" : {
                "removeMode": {
                    "errors": "Failed to remove 'orbit': Cannot remove active mode",
                    "success": false
                }
            }
        })
    );

    assert_eq!(
        fixture.query(r#"{ availableModes { name, active } }"#),
        json!({
            "data": {
                "availableModes": [
                    {
                        "name": "orbit",
                        "active": true
                    },
                    {
                        "name": "safe",
                        "active": false,
                    }
                ]
            }
        })
    );
}

#[test]
fn remove_nonexistent_mode() {
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8022);

    assert_eq!(
        fixture.remove_mode("orbit"),
        json!({
            "data" : {
                "removeMode": {
                    "errors": "Failed to remove 'orbit': No such file or directory (os error 2)",
                    "success": false
                }
            }
        })
    );
}

#[test]
fn remove_existing_schedule() {
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8023);

    fixture.create_mode("operational");

    let schedule = json!({ "tasks": [ ] });
    let schedule_path = fixture.create_task_list(Some(schedule.to_string()));
    fixture.import_task_list("first", &schedule_path, "operational");

    assert_eq!(
        fixture.query(
            r#"{ availableModes(name: "operational") { name, active, schedule { filename } } }"#
        ),
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
                    }
                ]
            }
        })
    );

    assert_eq!(
        fixture.remove_task_list("first", "operational"),
        json!({
            "data" : {
                "removeTaskList": {
                    "errors": "",
                    "success": true
                }
            }
        })
    );

    assert_eq!(
        fixture.query(
            r#"{ availableModes(name: "operational") { name, active, schedule { filename } } }"#
        ),
        json!({
            "data": {
                "availableModes": [
                    {
                        "name": "operational",
                        "active": false,
                        "schedule": [ ]
                    }
                ]
            }
        })
    );
}

#[test]
fn remove_nonexistant_task_list() {
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8024);

    fixture.create_mode("operational");

    assert_eq!(
        fixture.remove_task_list("first", "operational"),
        json!({
            "data" : {
                "removeTaskList": {
                    "errors": "Failed to remove 'first': File not found",
                    "success": false
                }
            }
        })
    );
}

#[test]
fn remove_task_list_nonexistant_mode() {
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8025);

    assert_eq!(
        fixture.remove_task_list("first", "operational"),
        json!({
            "data" : {
                "removeTaskList": {
                    "errors": "Failed to remove 'first': Mode not found",
                    "success": false
                }
            }
        })
    );
}

#[test]
fn remove_safe_mode() {
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8026);

    assert_eq!(
        fixture.remove_mode("safe"),
        json!({
            "data" : {
                "removeMode": {
                    "errors": "Failed to remove 'safe': The safe mode cannot be removed",
                    "success": false
                }
            }
        })
    );
}
