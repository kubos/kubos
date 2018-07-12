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
insert into telemetry values(1531412196211, 'eps', 'voltage', '3.3');
insert into telemetry values(1531412196212, 'eps', 'voltage', '3.4');
insert into telemetry values(1531412196213, 'eps', 'voltage', '3.2');
insert into telemetry values(1531412196214, 'eps', 'voltage', '3.1');
insert into telemetry values(1531412196215, 'eps', 'voltage', '3.0');
insert into telemetry values(1531412196216, 'eps', 'voltage', '2.9');
insert into telemetry values(1531412196217, 'eps', 'voltage', '2.8');
insert into telemetry values(1531412196218, 'eps', 'voltage', '2.7');
insert into telemetry values(1531412196219, 'eps', 'voltage', '2.6');
insert into telemetry values(1531412196220, 'eps', 'voltage', '2.5');
insert into telemetry values(1531412196221, 'eps', 'voltage', '2.4');
";

#[test]
fn test() {
    let (handle, sender) = setup(Some(SQL));
    let res =
        do_query("{telemetry(limit: 2, timestampGe: 1531412196215){timestamp,subsystem,parameter,value}}");
    teardown(handle, sender);
    assert_eq!(
        res,
        json!({
            "errs": "",
            "msg": {
                "telemetry":[
                    {"timestamp":1531412196216.0,"subsystem":"eps","parameter":"voltage","value":"2.9"},
                    {"timestamp":1531412196217.0,"subsystem":"eps","parameter":"voltage","value":"2.8"},
                ]
            }
        })
    );
}
