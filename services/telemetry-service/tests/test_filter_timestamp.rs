//
// Copyright (C) 2018 Kubos Corporation
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

#[macro_use]
extern crate serde_json;

mod utils;
use utils::*;

static SQL: &'static str = r"
insert into telemetry values(1531347346000, 'eps', 'voltage', '3.3');
insert into telemetry values(1531347346001, 'eps', 'voltage', '3.4');
insert into telemetry values(15313473460002, 'eps', 'voltage', '3.5');
insert into telemetry values(15313473460003, 'eps', 'voltage', '3.6');
insert into telemetry values(15313473460004, 'eps', 'voltage', '3.7');
insert into telemetry values(15313473460005, 'eps', 'voltage', '3.8');
";

/// These four test cases are all in one test function because
/// rust runs tests functions concurrently and this was the fastest
/// way around that.
#[test]
fn tests() {
    let (handle, sender) = setup(Some(SQL));

    let ge_res = do_query("{telemetry(timestampGe: 15313473460004){value}}");
    let le_res = do_query("{telemetry(timestampLe: 15313473460002){value}}");
    let range_res = do_query("{telemetry(timestampGe: 15313473460001, timestampLe:15313473460003){value}}");
    let single_res = do_query("{telemetry(timestampGe: 15313473460003, timestampLe:15313473460003){value}}");

    teardown(handle, sender);

    assert_eq!(
        ge_res,
        json!({
            "errs": "",
            "msg": {
                "telemetry": [
                    {"value":"3.7"},
                    {"value":"3.8"}
                ]
            }
        })
    );

    assert_eq!(
        le_res,
        json!({
            "errs": "",
            "msg": {
                "telemetry": [
                    {"value":"3.3"},
                    {"value":"3.4"},
                    {"value":"3.5"}
                ]
            }
        })
    );

    assert_eq!(
        range_res,
        json!({
            "errs": "",
            "msg": {
                "telemetry": [
                    {"value":"3.4"},
                    {"value":"3.5"},
                    {"value":"3.6"}
                ]
            }
        })
    );

    assert_eq!(
        single_res,
        json!({
            "errs": "",
            "msg": {
                "telemetry": [
                    {"value":"3.6"},
                ]
            }
        })
    );
}
