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
fn remove_existing_schedule() {
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8020);

    let schedule_path = fixture.create(None);
    fixture.register("operational", &schedule_path);

    assert_eq!(
        fixture.remove("operational"),
        json!({
            "data": {
                "remove": {
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
                "availableSchedules": [

                ]
            }
        })
    );
}

#[test]
fn remove_active() {
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8021);

    let schedule_path = fixture.create(None);
    fixture.register("operational", &schedule_path);
    fixture.activate("operational");

    assert_eq!(
        fixture.remove("operational"),
        json!({
            "data": {
                "remove": {
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
                "activeSchedule": serde_json::Value::Null
            }
        })
    );

    assert_eq!(
        fixture.query(r#"{ availableSchedules { name, active } }"#),
        json!({
            "data": {
                "availableSchedules": [

                ]
            }
        })
    );
}

#[test]
fn remove_nonexistant() {
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8022);

    assert_eq!(
        fixture.remove("operational"),
        json!({
            "data": {
                "remove": {
                    "errors": "Failed to remove schedule operational.json: No such file or directory (os error 2)",
                    "success": false
                }
            }
        })
    );
}
