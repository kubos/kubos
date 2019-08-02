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
fn register_new_schedule() {
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8020);

    let schedule_path = fixture.create();
    assert_eq!(
        fixture.register("operational", &schedule_path),
        json!({
            "data" : {
                "register": {
                    "errors": "",
                    "success": true
                }
            }
        })
    );

    assert_eq!(
        fixture.query(r#"{ registeredSchedules { name, active } }"#),
        json!({
            "data": {
                "registeredSchedules": [
                    {
                        "name": "operational",
                        "active": false
                    }
                ]
            }
        })
    );
}

#[test]
fn register_new_schedule_nonexistant_file() {
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8021);

    assert_eq!(
        fixture.register("operational", ""),
        json!({
            "data" : {
                "register": {
                    "errors": "Schedule copy failed: No such file or directory (os error 2)",
                    "success": false
                }
            }
        }
        )
    );

    assert_eq!(
        fixture.query(r#"{ registeredSchedules { name, active } }"#),
        json!({
            "data": {
                "registeredSchedules": [

                ]
            }
        })
    );
}

#[test]
fn register_duplicate_schedule() {
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8022);

    let schedule_one_path = fixture.create();
    let schedule_two_path = fixture.create();

    assert_eq!(
        fixture.register("operational", &schedule_one_path),
        json!({
            "data" : {
                "register": {
                    "errors": "",
                    "success": true
                }
            }
        })
    );

    assert_eq!(
        fixture.query(r#"{ registeredSchedules { name, active } }"#),
        json!({
            "data": {
                "registeredSchedules": [
                    {
                        "name": "operational",
                        "active": false
                    }
                ]
            }
        })
    );

    assert_eq!(
        fixture.register("operational", &schedule_two_path),
        json!({
            "data" : {
                "register": {
                    "errors": "",
                    "success": true
                }
            }
        })
    );

    assert_eq!(
        fixture.query(r#"{ registeredSchedules { name, active } }"#),
        json!({
            "data": {
                "registeredSchedules": [
                    {
                        "name": "operational",
                        "active": false
                    }
                ]
            }
        })
    );
}

#[test]
fn register_two_schedules() {
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8023);

    let schedule_one_path = fixture.create();
    let schedule_two_path = fixture.create();

    assert_eq!(
        fixture.register("operational", &schedule_one_path),
        json!({
            "data" : {
                "register": {
                    "errors": "",
                    "success": true
                }
            }
        })
    );

    assert_eq!(
        fixture.query(r#"{ registeredSchedules { name, active } }"#),
        json!({
            "data": {
                "registeredSchedules": [{
                    "name": "operational",
                    "active": false
                }]
            }
        })
    );

    assert_eq!(
        fixture.register("imaging", &schedule_two_path),
        json!({
            "data" : {
                "register": {
                    "errors": "",
                    "success": true
                }
            }
        })
    );

    assert_eq!(
        fixture.query(r#"{ registeredSchedules { name, active } }"#),
        json!({
            "data": {
                "registeredSchedules": [
                    {
                        "name": "imaging",
                        "active": false
                    },
                    {
                        "name": "operational",
                        "active": false
                    }
                ]
            }
        })
    );
}

#[test]
fn register_two_schedules_filter_query() {
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8024);

    let schedule_one_path = fixture.create();
    let schedule_two_path = fixture.create();

    fixture.register("operational", &schedule_one_path);
    fixture.register("imaging", &schedule_two_path);

    assert_eq!(
        fixture.query(r#"{ registeredSchedules(name: "imaging") { name, active } }"#),
        json!({
            "data": {
                "registeredSchedules": [
                    {
                        "name": "imaging",
                        "active": false
                    }
                ]
            }
        })
    );

    assert_eq!(
        fixture.query(r#"{ registeredSchedules(name: "operational") { name, active } }"#),
        json!({
            "data": {
                "registeredSchedules": [
                    {
                        "name": "operational",
                        "active": false
                    }
                ]
            }
        })
    );
}
