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
                "availableModes": [ ]
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
                    "errors": "Cannot remove active mode",
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
                    "errors": "Failed to remove mode directory: No such file or directory (os error 2)",
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
    let schedule_path = fixture.create_config(Some(schedule.to_string()));
    fixture.import_config("first", &schedule_path, "operational");

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
                    }
                ]
            }
        })
    );

    assert_eq!(
        fixture.remove_config("first", "operational"),
        json!({
            "data" : {
                "removeConfig": {
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
                        "schedules": [ ]
                    }
                ]
            }
        })
    );
}

#[test]
fn remove_nonexistant_schedule() {
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8024);

    fixture.create_mode("operational");

    assert_eq!(
        fixture.remove_config("first", "operational"),
        json!({
            "data" : {
                "removeConfig": {
                    "errors": "Config file first.json not found",
                    "success": false
                }
            }
        })
    );
}

#[test]
fn remove_config_nonexistant_mode() {
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8025);

    assert_eq!(
        fixture.remove_config("first", "operational"),
        json!({
            "data" : {
                "removeConfig": {
                    "errors": "Mode operational not found",
                    "success": false
                }
            }
        })
    );
}
