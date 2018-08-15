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
insert into telemetry values(1000, 'eps', 'voltage', '3.3');
insert into telemetry values(1001, 'eps', 'voltage', '3.4');
insert into telemetry values(1002, 'eps', 'voltage', '3.2');
";

#[test]
fn test() {
    let (handle, sender) = setup(Some(SQL));
    let res = do_query("{telemetry{timestamp,subsystem,parameter,value}}");
    teardown(handle, sender);
    assert_eq!(
        res,
        json!({
            "errs": "",
            "msg": {
                "telemetry":[
                    {"timestamp":1002,"subsystem":"eps","parameter":"voltage","value":"3.2"},
                    {"timestamp":1001,"subsystem":"eps","parameter":"voltage","value":"3.4"},
                    {"timestamp":1000,"subsystem":"eps","parameter":"voltage","value":"3.3"},
                ]
            }
        })
    );
}
