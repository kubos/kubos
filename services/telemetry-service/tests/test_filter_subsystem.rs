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
insert into telemetry values(1010, 'gps', 'x_position', '-1.0');
insert into telemetry values(1011, 'eps', 'voltage', '10');
";

#[test]
fn test() {
    let (handle, sender) = setup(Some(SQL));
    let res = do_query("{telemetry(subsystem: \"gps\"){timestamp,subsystem,parameter,value}}");
    teardown(handle, sender);
    assert_eq!(
        res,
        json!({
            "errs": "",
            "msg": {
                "telemetry":[
                    {"timestamp":1010.0,"subsystem":"gps","parameter":"x_position","value":"-1.0"}
                ]
            }
        })
    );
}
