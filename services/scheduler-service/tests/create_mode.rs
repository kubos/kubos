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
fn create_new_mode() {
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
        fixture.query(r#"{ availableModes { name, active } }"#),
        json!({
            "data": {
                "availableModes": [
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
fn create_duplicate_mode() {
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8021);

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
        fixture.create_mode("operational"),
        json!({
            "data" : {
                "createMode": {
                    "errors": "Failed to create mode directory: File exists (os error 17)",
                    "success": false
                }
            }
        })
    );
}

#[test]
fn create_two_modes() {
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8022);

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
        fixture.create_mode("low_power"),
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
        fixture.query(r#"{ availableModes { name, active } }"#),
        json!({
            "data": {
                "availableModes": [
                    {
                        "name": "low_power",
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
fn create_modes_name_filter() {
    let fixture = SchedulerFixture::spawn("127.0.0.1", 8023);

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
        fixture.create_mode("low_power"),
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
        fixture.query(r#"{ availableModes(name: "operational") { name, active } }"#),
        json!({
            "data": {
                "availableModes": [
                    {
                        "name": "operational",
                        "active": false
                    }
                ]
            }
        })
    );

    assert_eq!(
        fixture.query(r#"{ availableModes(name: "low_power") { name, active } }"#),
        json!({
            "data": {
                "availableModes": [
                    {
                        "name": "low_power",
                        "active": false
                    }
                ]
            }
        })
    );
}
