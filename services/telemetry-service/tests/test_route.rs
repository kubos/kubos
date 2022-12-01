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
use flate2::read::GzDecoder;
use serde_json::{json, Value};
use std::fs::{self, File};
use std::io::Read;
use tempfile::TempDir;

static SQL: &str = r"
insert into telemetry values(1000, 'eps', 'voltage', '3.3');
insert into telemetry values(1001, 'eps', 'current', '3.4');
insert into telemetry values(1002, 'eps', 'voltage', '3.2');
insert into telemetry values(1003, 'eps', 'current', '3.5');
insert into telemetry values(1004, 'eps', 'voltage', '3.6');
insert into telemetry values(1000, 'mcu', 'voltage', '4.3');
insert into telemetry values(1001, 'mcu', 'current', '4.4');
insert into telemetry values(1002, 'mcu', 'voltage', '4.2');
insert into telemetry values(1003, 'mcu', 'current', '4.5');
insert into telemetry values(1004, 'mcu', 'voltage', '4.6');
";

#[test]
fn test_route_file() {
    let db_dir = TempDir::new().unwrap();
    let db_path = db_dir.path().join("test.db");

    let db = db_path.to_str().unwrap();
    let port = 8111;
    let udp = 8121;

    let _fixture = TelemetryServiceFixture::setup(db, Some(port), Some(udp), Some(SQL));

    let output_dir = TempDir::new().unwrap();
    let output_path = output_dir.path().join("output");

    let query = format!(
        r#"{{
        routedTelemetry(output: "{}", compress: false)
    }}"#,
        output_path.to_str().unwrap()
    );

    do_query(Some(port), &query);

    let mut output_file = File::open(output_path).unwrap();
    let mut contents = String::new();
    output_file.read_to_string(&mut contents).unwrap();

    let entries: serde_json::Value = serde_json::from_str(&contents).unwrap();

    assert_eq!(
        entries,
        json!([
            {"timestamp":1004.0,"subsystem":"mcu","parameter":"voltage","value":"4.6"},
            {"timestamp":1004.0,"subsystem":"eps","parameter":"voltage","value":"3.6"},
            {"timestamp":1003.0,"subsystem":"mcu","parameter":"current","value":"4.5"},
            {"timestamp":1003.0,"subsystem":"eps","parameter":"current","value":"3.5"},
            {"timestamp":1002.0,"subsystem":"mcu","parameter":"voltage","value":"4.2"},
            {"timestamp":1002.0,"subsystem":"eps","parameter":"voltage","value":"3.2"},
            {"timestamp":1001.0,"subsystem":"mcu","parameter":"current","value":"4.4"},
            {"timestamp":1001.0,"subsystem":"eps","parameter":"current","value":"3.4"},
            {"timestamp":1000.0,"subsystem":"mcu","parameter":"voltage","value":"4.3"},
            {"timestamp":1000.0,"subsystem":"eps","parameter":"voltage","value":"3.3"},
        ])
    );
}

#[test]
fn test_route_response() {
    let db_dir = TempDir::new().unwrap();
    let db_path = db_dir.path().join("test.db");

    let db = db_path.to_str().unwrap();
    let port = 8112;
    let udp = 8122;

    let _fixture = TelemetryServiceFixture::setup(db, Some(port), Some(udp), Some(SQL));

    // Use a file that won't have a randomly generated path
    let output_path = "output";

    let query = format!(
        r#"{{
        routedTelemetry(output: "{}", compress: false)
    }}"#,
        output_path
    );

    let res = do_query(Some(port), &query);

    // Since it's not a temporary file, we'll need to delete it ourselves
    fs::remove_file(output_path).unwrap();

    let expected = json!({
        "data": {
            "routedTelemetry": "output"
        }
    });

    assert_eq!(res, expected);
}

#[test]
fn test_route_filter() {
    let db_dir = TempDir::new().unwrap();
    let db_path = db_dir.path().join("test.db");

    let db = db_path.to_str().unwrap();
    let port = 8113;
    let udp = 8123;

    let _fixture = TelemetryServiceFixture::setup(db, Some(port), Some(udp), Some(SQL));

    let output_dir = TempDir::new().unwrap();
    let output_path = output_dir.path().join("output");

    let query = format!(
        r#"{{
        routedTelemetry(
            timestampGe: 1001, 
            timestampLe: 1003, 
            subsystem: "eps", 
            parameter: "current", 
            output: "{}",
            compress: false
            )
    }}"#,
        output_path.to_str().unwrap()
    );

    do_query(Some(port), &query);

    let mut output_file = File::open(output_path).unwrap();
    let mut contents = String::new();
    output_file.read_to_string(&mut contents).unwrap();

    let entries: serde_json::Value = serde_json::from_str(&contents).unwrap();

    assert_eq!(
        entries,
        json!([
            {"timestamp":1003.0,"subsystem":"eps","parameter":"current","value":"3.5"},
            {"timestamp":1001.0,"subsystem":"eps","parameter":"current","value":"3.4"},
        ])
    );
}

#[test]
fn test_route_compress_file() {
    let db_dir = TempDir::new().unwrap();
    let db_path = db_dir.path().join("test.db");

    let db = db_path.to_str().unwrap();
    let port = 8114;
    let udp = 8124;

    let _fixture = TelemetryServiceFixture::setup(db, Some(port), Some(udp), Some(SQL));

    let output_dir = TempDir::new().unwrap();
    let output_name = "output";
    let output_path = output_dir.path().join(output_name);

    let query = format!(
        r#"{{
        routedTelemetry(output: "{}", compress: true)
    }}"#,
        output_path.to_str().unwrap()
    );

    do_query(Some(port), &query);

    let tar_path = output_dir.path().join(format!("{}.tar.gz", output_name));
    let result_dir = output_dir.path().join("final");
    let result_path = result_dir.join(output_name);

    let tar_file = File::open(tar_path).unwrap();
    let tar = GzDecoder::new(tar_file);
    let mut archive = tar::Archive::new(tar);
    archive.unpack(result_dir).unwrap();

    let mut output_file = File::open(result_path).unwrap();
    let mut contents = String::new();
    output_file.read_to_string(&mut contents).unwrap();

    let entries: serde_json::Value = serde_json::from_str(&contents).unwrap();

    assert_eq!(
        entries,
        json!([
            {"timestamp":1004.0,"subsystem":"mcu","parameter":"voltage","value":"4.6"},
            {"timestamp":1004.0,"subsystem":"eps","parameter":"voltage","value":"3.6"},
            {"timestamp":1003.0,"subsystem":"mcu","parameter":"current","value":"4.5"},
            {"timestamp":1003.0,"subsystem":"eps","parameter":"current","value":"3.5"},
            {"timestamp":1002.0,"subsystem":"mcu","parameter":"voltage","value":"4.2"},
            {"timestamp":1002.0,"subsystem":"eps","parameter":"voltage","value":"3.2"},
            {"timestamp":1001.0,"subsystem":"mcu","parameter":"current","value":"4.4"},
            {"timestamp":1001.0,"subsystem":"eps","parameter":"current","value":"3.4"},
            {"timestamp":1000.0,"subsystem":"mcu","parameter":"voltage","value":"4.3"},
            {"timestamp":1000.0,"subsystem":"eps","parameter":"voltage","value":"3.3"},
        ])
    );
}

#[test]
fn test_route_compress_response() {
    let db_dir = TempDir::new().unwrap();
    let db_path = db_dir.path().join("test.db");

    let db = db_path.to_str().unwrap();
    let port = 8115;
    let udp = 8125;

    let _fixture = TelemetryServiceFixture::setup(db, Some(port), Some(udp), Some(SQL));

    // Use a file that won't have a randomly generated path
    let output_path = "compressed-output";

    let query = format!(
        r#"{{
        routedTelemetry(output: "{}", compress: true)
    }}"#,
        output_path
    );

    let res = do_query(Some(port), &query);

    // Since it's not a temporary file, we'll need to delete it ourselves
    fs::remove_file(&format!("{}.tar.gz", output_path)).unwrap();

    let expected = json!({
        "data": {
            "routedTelemetry": "compressed-output.tar.gz"
        }
    });

    assert_eq!(res, expected);
}

#[test]
fn test_route_parameters() {
    let db_dir = TempDir::new().unwrap();
    let db_path = db_dir.path().join("test.db");

    let db = db_path.to_str().unwrap();
    let port = 8116;
    let udp = 8126;

    let _fixture = TelemetryServiceFixture::setup(db, Some(port), Some(udp), Some(SQL));

    let output_dir = TempDir::new().unwrap();
    let output_path = output_dir.path().join("output");

    let query = format!(
        r#"{{
            routedTelemetry(
                subsystem: "eps",
                parameters: ["voltage", "current"],
                output: "{}",
                compress: false
                )
        }}"#,
        output_path.to_str().unwrap()
    );

    do_query(Some(port), &query);

    let mut output_file = File::open(output_path).unwrap();
    let mut contents = String::new();
    output_file.read_to_string(&mut contents).unwrap();

    let entries: serde_json::Value = serde_json::from_str(&contents).unwrap();

    assert_eq!(
        entries,
        json!([
            {"timestamp":1004.0,"subsystem":"eps","parameter":"voltage","value":"3.6"},
            {"timestamp":1003.0,"subsystem":"eps","parameter":"current","value":"3.5"},
            {"timestamp":1002.0,"subsystem":"eps","parameter":"voltage","value":"3.2"},
            {"timestamp":1001.0,"subsystem":"eps","parameter":"current","value":"3.4"},
            {"timestamp":1000.0,"subsystem":"eps","parameter":"voltage","value":"3.3"}
        ])
    );
}

#[test]
fn test_route_conflict() {
    let db_dir = TempDir::new().unwrap();
    let db_path = db_dir.path().join("test.db");

    let db = db_path.to_str().unwrap();
    let port = 8117;
    let udp = 8127;

    let _fixture = TelemetryServiceFixture::setup(db, Some(port), Some(udp), Some(SQL));

    let res = do_query(
        Some(port),
        r#"{
        telemetry(parameter: "voltage", parameters: ["current"]) {
            parameter,
            value
        }
    }"#,
    );

    assert_eq!(
        res,
        json!({
                "data": Value::Null,
                "errors": [{
                    "locations": [{"column": 9, "line": 2}],
                    "message": "The `parameter` and `parameters` input fields are mutually exclusive",
                    "path": ["telemetry"]
                }]
        })
    );
}
