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

extern crate cbor_protocol;
extern crate kubos_telemetry_db;
#[macro_use]
extern crate serde_json;
extern crate tempfile;

mod utils;

use serde_json::ser;
use std::net::UdpSocket;
use std::time::Duration;
use tempfile::TempDir;
use utils::*;

#[test]
fn test_udp_timestamp() {
    let db_dir = TempDir::new().unwrap();
    let db_path = db_dir.path().join("test.db");

    let db = db_path.to_str().unwrap();
    let port = 8111;
    let udp = 8121;

    let (handle, sender) = setup(Some(db), Some(port), Some(udp), None);

    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    let service = format!("0.0.0.0:{}", udp);

    let entry1 = json!({
            "timestamp": 1000,
            "subsystem": "eps",
            "parameter": "voltage",
            "value": "3.3"
    });
    let entry2 = json!({
            "timestamp": 1001,
            "subsystem": "eps",
            "parameter": "voltage",
            "value": "3.4"
    });
    let entry3 = json!({
            "timestamp": 1002,
            "subsystem": "eps",
            "parameter": "voltage",
            "value": "3.2"
    });

    socket
        .send_to(&ser::to_vec(&entry1).unwrap(), &service)
        .unwrap();
    socket
        .send_to(&ser::to_vec(&entry2).unwrap(), &service)
        .unwrap();
    socket
        .send_to(&ser::to_vec(&entry3).unwrap(), &service)
        .unwrap();

    // Give the service time to process the messages, since we're not actually waiting
    // for a response
    ::std::thread::sleep(Duration::from_millis(100));

    let res = do_query(
        Some(port),
        "{telemetry{timestamp,subsystem,parameter,value}}",
    );
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

#[test]
fn test_udp_no_timestamp() {
    let db_dir = TempDir::new().unwrap();
    let db_path = db_dir.path().join("test.db");

    let db = db_path.to_str().unwrap();

    let port = 8112;
    let udp = 8122;

    let (handle, sender) = setup(Some(db), Some(port), Some(udp), None);

    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    let service = format!("0.0.0.0:{}", udp);

    let entry1 = json!({
            "subsystem": "test1",
            "parameter": "current",
            "value": "2.0"
    });
    let entry2 = json!({
            "subsystem": "test2",
            "parameter": "current",
            "value": "2.3"
    });
    let entry3 = json!({
            "subsystem": "test3",
            "parameter": "current",
            "value": "2.1"
    });
    let entry4 = json!({
            "subsystem": "test4",
            "parameter": "current",
            "value": "2.2"
    });

    socket
        .send_to(&ser::to_vec(&entry1).unwrap(), &service)
        .unwrap();
    socket
        .send_to(&ser::to_vec(&entry2).unwrap(), &service)
        .unwrap();
    socket
        .send_to(&ser::to_vec(&entry3).unwrap(), &service)
        .unwrap();
    socket
        .send_to(&ser::to_vec(&entry4).unwrap(), &service)
        .unwrap();

    // Give the service time to process the messages, since we're not actually waiting
    // for a response
    ::std::thread::sleep(Duration::from_millis(100));

    let res = do_query(Some(port), "{telemetry{subsystem,parameter,value}}");
    teardown(handle, sender);
    assert_eq!(
        res,
        json!({
            "errs": "",
            "msg": {
                "telemetry":[
                    {"subsystem":"test4","parameter":"current","value":"2.2"},
                    {"subsystem":"test3","parameter":"current","value":"2.1"},
                    {"subsystem":"test2","parameter":"current","value":"2.3"},
                    {"subsystem":"test1","parameter":"current","value":"2.0"},
                ]
            }
        })
    );
}
