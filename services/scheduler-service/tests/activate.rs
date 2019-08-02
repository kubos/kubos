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
fn activate_existing_schedule() {
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8020);

    let schedule_path = fixture.create(Some("{}".to_owned()));
    assert_eq!(
        fixture.register("imaging", &schedule_path),
        json!({
            "data" : {
                "import": {
                    "errors": "",
                    "success": true
                }
            }
        })
    );

    assert_eq!(
        fixture.activate("imaging"),
        json!({
            "data" : {
                "activate": {
                    "errors": "",
                    "success": true
                }
            }
        })
    );

    assert_eq!(
        fixture.query(r#"{ activeSchedule { name } }"#),
        json!({
            "data": {
                "activeSchedule": {
                    "name": "imaging"
                }
            }
        })
    );

    assert_eq!(
        fixture.query(r#"{ availableSchedules { name, active } }"#),
        json!({
            "data": {
                "availableSchedules": [{
                    "name": "imaging",
                    "active": true
                }]
            }
        })
    );
}

#[test]
fn activate_non_existent_schedule() {
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8021);

    assert_eq!(
        fixture.activate("operational"),
        json!({
            "data" : {
                "activate": {
                    "errors": "Schedule operational.json not found",
                    "success": false
                }
            }
        }
        )
    );

    assert_eq!(
        fixture.query(r#"{ activeSchedule { name } }"#),
        json!({
            "data": {
                "activeSchedule": serde_json::Value::Null
            }
        })
    );
}

#[test]
fn activate_two_schedules() {
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8022);

    let schedule_one_path = fixture.create(Some("{}".to_owned()));
    let schedule_two_path = fixture.create(Some("{}".to_owned()));

    fixture.register("imaging", &schedule_one_path);
    fixture.register("operational", &schedule_two_path);

    assert_eq!(
        fixture.activate("imaging"),
        json!({
            "data" : {
                "activate": {
                    "errors": "",
                    "success": true
                }
            }
        })
    );

    assert_eq!(
        fixture.query(r#"{ availableSchedules { name, active } }"#),
        json!({
            "data": {
                "availableSchedules": [{
                    "name": "imaging",
                    "active": true
                }, {
                    "name": "operational",
                    "active": false
                }]
            }
        })
    );

    assert_eq!(
        fixture.activate("operational"),
        json!({
            "data" : {
                "activate": {
                    "errors": "",
                    "success": true
                }
            }
        })
    );

    assert_eq!(
        fixture.query(r#"{ activeSchedule { name } }"#),
        json!({
            "data": {
                "activeSchedule": {
                    "name": "operational"
                }
            }
        })
    );

    assert_eq!(
        fixture.query(r#"{ availableSchedules { name, active } }"#),
        json!({
            "data": {
                "availableSchedules": [{
                    "name": "imaging",
                    "active": false
                }, {
                    "name": "operational",
                    "active": true
                }]
            }
        })
    );
}
