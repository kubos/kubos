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
fn activate_existing_mode() {
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8020);

    assert_eq!(
        fixture.create_mode("operational"),
        json!({
            "data" : {
                "createMode": {
                    "errors": "",
                    "success": true
                }
            }
        })
    );

    assert_eq!(
        fixture.activate_mode("operational"),
        json!({
            "data" : {
                "activateMode": {
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
                        "name": "operational",
                        "active": true
                    }
                ]
            }
        })
    );
}

#[test]
fn activate_non_existent_schedule() {
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8021);

    assert_eq!(
        fixture.activate_mode("operational"),
        json!({
            "data" : {
                "activateMode": {
                    "errors": "Mode operational not found",
                    "success": false
                }
            }
        })
    );

    assert_eq!(
        fixture.query(r#"{ activeMode { name } }"#),
        json!({
            "data": serde_json::Value::Null,
            "errors": [{
                "locations": [{
                    "column": 3,
                    "line": 1
                }],
                "message": "Failed to read active mode link: No such file or directory (os error 2)",
                "path": ["activeMode"]
            }]
        })
    );
}

#[test]
fn activate_two_modes() {
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8022);

    fixture.create_mode("first");
    fixture.create_mode("second");

    assert_eq!(
        fixture.activate_mode("first"),
        json!({
            "data" : {
                "activateMode": {
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
                "availableModes": [{
                    "name": "first",
                    "active": true
                }, {
                    "name": "second",
                    "active": false
                }]
            }
        })
    );

    assert_eq!(
        fixture.activate_mode("second"),
        json!({
            "data" : {
                "activateMode": {
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
                "availableModes": [{
                    "name": "first",
                    "active": false
                }, {
                    "name": "second",
                    "active": true
                }]
            }
        })
    );
}
