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

use serde_json::json;
mod utils;
use crate::utils::*;
use tempfile::TempDir;

static SQL: &str = r"
insert into telemetry values(1000, 'eps', 'voltage', '3.3');
insert into telemetry values(1001, 'eps', 'voltage', '3.4');
insert into telemetry values(1002, 'eps', 'voltage', '3.2');
";

#[test]
fn test() {
    let db_dir = TempDir::new().unwrap();
    let db_path = db_dir.path().join("test.db");
    let db = db_path.to_str().unwrap();
    let _fixture = TelemetryServiceFixture::setup(db, None, None, Some(SQL));
    let res = do_query(None, "{telemetry{timestamp,subsystem,parameter,value}}");

    assert_eq!(
        res,
        json!({
            "data": {
                "telemetry":[
                    {"timestamp":1002.0,"subsystem":"eps","parameter":"voltage","value":"3.2"},
                    {"timestamp":1001.0,"subsystem":"eps","parameter":"voltage","value":"3.4"},
                    {"timestamp":1000.0,"subsystem":"eps","parameter":"voltage","value":"3.3"},
                ]
            }
        })
    );
}
