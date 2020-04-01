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

mod utils;

use crate::utils::*;
use serde_json::json;
use tempfile::TempDir;

static SQL: &'static str = r"
insert into telemetry values(1000, 'eps', 'voltage', '3.3');
insert into telemetry values(1001, 'eps', 'voltage', '3.4');
insert into telemetry values(1002, 'eps', 'voltage', '3.5');
insert into telemetry values(1003, 'eps', 'voltage', '3.6');
insert into telemetry values(1004, 'eps', 'voltage', '3.7');
insert into telemetry values(1005, 'eps', 'voltage', '3.8');
";

#[test]
fn test_ge() {
    let db_dir = TempDir::new().unwrap();
    let db_path = db_dir.path().join("test.db");

    let db = db_path.to_str().unwrap();
    let port = 8112;
    let udp = 8122;

    let _fixture = TelemetryServiceFixture::setup(db, Some(port), Some(udp), Some(SQL));

    let ge_res = do_query(Some(port), "{telemetry(timestampGe: 1004){value}}");

    assert_eq!(
        ge_res,
        json!({
            "data": {
                "telemetry": [
                    {"value":"3.8"},
                    {"value":"3.7"}
                ]
            }
        })
    );
}

#[test]
fn test_le() {
    let db_dir = TempDir::new().unwrap();
    let db_path = db_dir.path().join("test.db");

    let db = db_path.to_str().unwrap();
    let port = 8113;
    let udp = 8123;

    let _fixture = TelemetryServiceFixture::setup(db, Some(port), Some(udp), Some(SQL));

    let le_res = do_query(Some(port), "{telemetry(timestampLe: 1002){value}}");

    assert_eq!(
        le_res,
        json!({
            "data": {
                "telemetry": [
                    {"value":"3.5"},
                    {"value":"3.4"},
                    {"value":"3.3"}
                ]
            }
        })
    );
}

#[test]
fn test_range() {
    let db_dir = TempDir::new().unwrap();
    let db_path = db_dir.path().join("test.db");

    let db = db_path.to_str().unwrap();
    let port = 8114;
    let udp = 8124;

    let _fixture = TelemetryServiceFixture::setup(db, Some(port), Some(udp), Some(SQL));

    let range_res = do_query(
        Some(port),
        "{telemetry(timestampGe: 1001, timestampLe:1003){value}}",
    );

    assert_eq!(
        range_res,
        json!({
            "data": {
                "telemetry": [
                    {"value":"3.6"},
                    {"value":"3.5"},
                    {"value":"3.4"}
                ]
            }
        })
    );
}

#[test]
fn test_single() {
    let db_dir = TempDir::new().unwrap();
    let db_path = db_dir.path().join("test.db");

    let db = db_path.to_str().unwrap();
    let port = 8115;
    let udp = 8125;

    let _fixture = TelemetryServiceFixture::setup(db, Some(port), Some(udp), Some(SQL));

    let single_res = do_query(
        Some(port),
        "{telemetry(timestampGe: 1003, timestampLe:1003){value}}",
    );

    assert_eq!(
        single_res,
        json!({
            "data": {
                "telemetry": [
                    {"value":"3.6"},
                ]
            }
        })
    );
}
