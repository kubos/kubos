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

use chrono::Utc;
use serde_json::json;
use std::thread;
use std::time::Duration;
use util::SchedulerFixture;

#[test]
fn import_new_schedule() {
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8020);

    fixture.create_mode("operational");

    let schedule = json!({ "tasks": [ ] });
    let schedule_path = fixture.create_task_list(Some(schedule.to_string()));
    assert_eq!(
        fixture.import_task_list("first", &schedule_path, "operational"),
        json!({
            "data" : {
                "importTaskList": {
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
fn import_new_schedule_nonexistant_file() {
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8021);
    fixture.create_mode("operational");

    assert_eq!(
        fixture.import_task_list("first", "", ""),
        json!({
            "data" : {
                "importTaskList": {
                    "errors": "Failed to import 'first': No such file or directory (os error 2)",
                    "success": false
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
fn import_new_schedule_nonexistant_mode() {
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8022);

    let schedule = json!({ "init": [ ] });
    let schedule_path = fixture.create_task_list(Some(schedule.to_string()));
    assert_eq!(
        fixture.import_task_list("first", &schedule_path, "operational"),
        json!({
            "data" : {
                "importTaskList": {
                    "errors": "Failed to import 'first': Mode not found",
                    "success": false
                }
            }
        })
    );
}

#[test]
fn import_duplicate_schedule() {
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8023);
    fixture.create_mode("operational");

    let schedule = json!({ "tasks": [ ] });
    let schedule_one_path = fixture.create_task_list(Some(schedule.to_string()));
    let schedule_two_path = fixture.create_task_list(Some(schedule.to_string()));

    assert_eq!(
        fixture.import_task_list("imager", &schedule_one_path, "operational"),
        json!({
            "data" : {
                "importTaskList": {
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
                        "schedule": [
                            {
                                "filename": "imager"
                            }
                        ]
                    }
                ]
            }
        })
    );

    assert_eq!(
        fixture.import_task_list("imager", &schedule_two_path, "operational"),
        json!({
            "data" : {
                "importTaskList": {
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
                        "schedule": [
                            {
                                "filename": "imager"
                            }
                        ]
                    }
                ]
            }
        })
    );
}

#[test]
fn import_two_schedules() {
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8024);
    fixture.create_mode("flight");

    let schedule = json!({ "tasks": [ ] });
    let schedule_one_path = fixture.create_task_list(Some(schedule.to_string()));
    let schedule_two_path = fixture.create_task_list(Some(schedule.to_string()));

    assert_eq!(
        fixture.import_task_list("solar", &schedule_one_path, "flight"),
        json!({
            "data" : {
                "importTaskList": {
                    "errors": "",
                    "success": true
                }
            }
        })
    );

    assert_eq!(
        fixture.import_task_list("imaging", &schedule_two_path, "flight"),
        json!({
            "data" : {
                "importTaskList": {
                    "errors": "",
                    "success": true
                }
            }
        })
    );

    assert_eq!(
        fixture
            .query(r#"{ availableModes(name: "flight") { name, active, schedule { filename } } }"#),
        json!({
            "data": {
                "availableModes": [
                    {
                        "name": "flight",
                        "active": false,
                        "schedule": [
                            {
                                "filename": "imaging"
                            },
                            {
                                "filename": "solar"
                            }
                        ]
                    }
                ]
            }
        })
    );
}

#[test]
fn import_two_schedules_check_revised() {
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8025);
    fixture.create_mode("flight");

    let schedule = json!({ "tasks": [ ] });
    let schedule_one_path = fixture.create_task_list(Some(schedule.to_string()));
    let schedule_two_path = fixture.create_task_list(Some(schedule.to_string()));

    let sched_one_time = Utc::now();
    let sched_one_time = sched_one_time.format("%Y-%m-%d %H:%M:%S").to_string();
    fixture.import_task_list("solar", &schedule_one_path, "flight");

    assert_eq!(
        fixture.query(
            r#"{ availableModes(name: "flight") { name, active, lastRevised, schedule { filename, timeImported } } }"#
        ),
        json!({
            "data": {
                "availableModes": [
                    {
                        "name": "flight",
                        "active": false,
                        "lastRevised": sched_one_time,
                        "schedule": [
                            {
                                "filename": "solar",
                                "timeImported": sched_one_time
                            }
                        ]
                    }
                ]
            }
        })
    );

    thread::sleep(Duration::from_secs(1));

    fixture.import_task_list("imaging", &schedule_two_path, "flight");
    let sched_two_time = Utc::now();
    let sched_two_time = sched_two_time.format("%Y-%m-%d %H:%M:%S").to_string();

    assert_eq!(
        fixture.query(
            r#"{ availableModes(name: "flight") { name, active, lastRevised, schedule { filename, timeImported } } }"#
        ),
        json!({
            "data": {
                "availableModes": [
                    {
                        "name": "flight",
                        "active": false,
                        "lastRevised": sched_two_time,
                        "schedule": [
                            {
                                "filename": "imaging",
                                "timeImported": sched_two_time
                            },
                            {
                                "filename": "solar",
                                "timeImported": sched_one_time
                            }
                        ]
                    }
                ]
            }
        })
    );
}

#[test]
fn import_new_schedule_mixed_case_code() {
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8026);

    fixture.create_mode("operational");

    let schedule = json!({ "tasks": [ ] });
    let schedule_path = fixture.create_task_list(Some(schedule.to_string()));
    assert_eq!(
        fixture.import_task_list("FIrst", &schedule_path, "OPerational"),
        json!({
            "data" : {
                "importTaskList": {
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
fn import_invalid_schedule() {
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8027);

    fixture.create_mode("operational");

    let schedule_path = fixture.create_task_list(Some("not json".to_owned()));
    assert_eq!(
        fixture.import_task_list("firST", &schedule_path, "operational"),
        json!({
            "data" : {
                "importTaskList": {
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
fn import_safe_schedule_upper_case() {
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8028);

    assert_eq!(
        fixture.query(r#"{ availableModes { name, active, schedule { filename } } }"#),
        json!({
            "data": {
                "availableModes": [
                    {
                        "name": "safe",
                        "active": true,
                        "schedule": []
                    }
                ]
            }
        })
    );

    let schedule = json!({ "tasks": [ ] });
    let schedule_path = fixture.create_task_list(Some(schedule.to_string()));
    assert_eq!(
        fixture.import_task_list("first", &schedule_path, "SAFE"),
        json!({
            "data" : {
                "importTaskList": {
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
                        "name": "safe",
                        "active": true,
                        "schedule": [{ "filename": "first" } ]
                    }
                ]
            }
        })
    );
}
