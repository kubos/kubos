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

use serde_json::json;
mod utils;
use crate::utils::*;
use tempfile::TempDir;

#[test]
fn test() {
    let db_dir = TempDir::new().unwrap();
    let db_path = db_dir.path().join("test.db");
    let db = db_path.to_str().unwrap();
    let (handle, sender) = setup(db, None, None, None);
    let res = do_query(None, "{ping}");
    teardown(handle, sender);
    assert_eq!(
        res,
        json!({
            "data": {
                "ping": "pong"
            }
        })
    );
}
