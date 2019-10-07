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
    let schedule_path = fixture.create_config(Some(schedule.to_string()));
    assert_eq!(
        fixture.import_config("first", &schedule_path, "operational"),
        json!({
            "data" : {
                "importConfig": {
                    "errors": "",
                    "success": true
                }
            }
        })
    );

    assert_eq!(
        fixture.query(r#"{ availableModes { name, active, schedules { name } } }"#),
        json!({
            "data": {
                "availableModes": [
                    {
                        "name": "operational",
                        "active": false,
                        "schedules": [
                            {
                                "name": "first"
                            }
                        ]
                    },
                    {
                        "name": "safe",
                        "active": true,
                        "schedules": [ ]
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
        fixture.import_config("first", "", ""),
        json!({
            "data" : {
                "importConfig": {
                    "errors": "Failed to import 'first': No such file or directory (os error 2)",
                    "success": false
                }
            }
        })
    );

    assert_eq!(
        fixture.query(
            r#"{ availableModes(name: "operational") { name, active, schedules { name } } }"#
        ),
        json!({
            "data": {
                "availableModes": [
                    {
                        "name": "operational",
                        "active": false,
                        "schedules": [ ]
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
    let schedule_path = fixture.create_config(Some(schedule.to_string()));
    assert_eq!(
        fixture.import_config("first", &schedule_path, "operational"),
        json!({
            "data" : {
                "importConfig": {
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
    let schedule_one_path = fixture.create_config(Some(schedule.to_string()));
    let schedule_two_path = fixture.create_config(Some(schedule.to_string()));

    assert_eq!(
        fixture.import_config("imager", &schedule_one_path, "operational"),
        json!({
            "data" : {
                "importConfig": {
                    "errors": "",
                    "success": true
                }
            }
        })
    );

    assert_eq!(
        fixture.query(
            r#"{ availableModes(name: "operational") { name, active, schedules { name } } }"#
        ),
        json!({
            "data": {
                "availableModes": [
                    {
                        "name": "operational",
                        "active": false,
                        "schedules": [
                            {
                                "name": "imager"
                            }
                        ]
                    }
                ]
            }
        })
    );

    assert_eq!(
        fixture.import_config("imager", &schedule_two_path, "operational"),
        json!({
            "data" : {
                "importConfig": {
                    "errors": "",
                    "success": true
                }
            }
        })
    );

    assert_eq!(
        fixture.query(
            r#"{ availableModes(name: "operational") { name, active, schedules { name } } }"#
        ),
        json!({
            "data": {
                "availableModes": [
                    {
                        "name": "operational",
                        "active": false,
                        "schedules": [
                            {
                                "name": "imager"
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
    let schedule_one_path = fixture.create_config(Some(schedule.to_string()));
    let schedule_two_path = fixture.create_config(Some(schedule.to_string()));

    assert_eq!(
        fixture.import_config("solar", &schedule_one_path, "flight"),
        json!({
            "data" : {
                "importConfig": {
                    "errors": "",
                    "success": true
                }
            }
        })
    );

    assert_eq!(
        fixture.import_config("imaging", &schedule_two_path, "flight"),
        json!({
            "data" : {
                "importConfig": {
                    "errors": "",
                    "success": true
                }
            }
        })
    );

    assert_eq!(
        fixture.query(r#"{ availableModes(name: "flight") { name, active, schedules { name } } }"#),
        json!({
            "data": {
                "availableModes": [
                    {
                        "name": "flight",
                        "active": false,
                        "schedules": [
                            {
                                "name": "imaging"
                            },
                            {
                                "name": "solar"
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
    let schedule_one_path = fixture.create_config(Some(schedule.to_string()));
    let schedule_two_path = fixture.create_config(Some(schedule.to_string()));

    let sched_one_time = Utc::now();
    let sched_one_time = sched_one_time.format("%Y-%m-%d %H:%M:%S").to_string();
    fixture.import_config("solar", &schedule_one_path, "flight");

    assert_eq!(
        fixture.query(
            r#"{ availableModes(name: "flight") { name, active, lastRevised, schedules { name, timeImported } } }"#
        ),
        json!({
            "data": {
                "availableModes": [
                    {
                        "name": "flight",
                        "active": false,
                        "lastRevised": sched_one_time,
                        "schedules": [
                            {
                                "name": "solar",
                                "timeImported": sched_one_time
                            }
                        ]
                    }
                ]
            }
        })
    );

    thread::sleep(Duration::from_secs(1));

    fixture.import_config("imaging", &schedule_two_path, "flight");
    let sched_two_time = Utc::now();
    let sched_two_time = sched_two_time.format("%Y-%m-%d %H:%M:%S").to_string();

    assert_eq!(
        fixture.query(
            r#"{ availableModes(name: "flight") { name, active, lastRevised, schedules { name, timeImported } } }"#
        ),
        json!({
            "data": {
                "availableModes": [
                    {
                        "name": "flight",
                        "active": false,
                        "lastRevised": sched_two_time,
                        "schedules": [
                            {
                                "name": "imaging",
                                "timeImported": sched_two_time
                            },
                            {
                                "name": "solar",
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
    let schedule_path = fixture.create_config(Some(schedule.to_string()));
    assert_eq!(
        fixture.import_config("FIrst", &schedule_path, "OPerational"),
        json!({
            "data" : {
                "importConfig": {
                    "errors": "",
                    "success": true
                }
            }
        })
    );

    assert_eq!(
        fixture.query(r#"{ availableModes { name, active, schedules { name } } }"#),
        json!({
            "data": {
                "availableModes": [
                    {
                        "name": "operational",
                        "active": false,
                        "schedules": [
                            {
                                "name": "first"
                            }
                        ]
                    },
                    {
                        "name": "safe",
                        "active": true,
                        "schedules": [ ]
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

    let schedule_path = fixture.create_config(Some("not json".to_owned()));
    assert_eq!(
        fixture.import_config("firST", &schedule_path, "operational"),
        json!({
            "data" : {
                "importConfig": {
                    "errors": "Failed to import config file 'first': Failed to parse json",
                    "success": false
                }
            }
        })
    );

    assert_eq!(
        fixture.query(r#"{ availableModes { name, active, schedules { name } } }"#),
        json!({
            "data": {
                "availableModes": [
                    {
                        "name": "operational",
                        "active": false,
                        "schedules": [ ]
                    },
                    {
                        "name": "safe",
                        "active": true,
                        "schedules": [ ]
                    }
                ]
            }
        })
    );
}
