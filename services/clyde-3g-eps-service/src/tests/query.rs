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

use clyde_3g_eps_api::*;
use crate::models::subsystem::{Mutations, Subsystem};
use crate::models::*;
use crate::schema::mutation::Root as MutationRoot;
use crate::schema::query::Root as QueryRoot;
use eps_api::*;
use kubos_service::{Config, Service};
use serde_json::json;
use std::cell::{Cell, RefCell};
use std::sync::{Arc, Mutex};
use super::*;

#[test]
fn test_ping() {
    let config: Config = Default::default();
    let subsystem: Box<Subsystem> = Box::new(Subsystem::new(gen_mock_bad_eps()).unwrap());
    let service = Service::new(config, subsystem, QueryRoot, MutationRoot);

    let query = r#"{ ping }"#;
    
    let expected = json!({"ping":"pong"});
    
    test!(service, query, expected);
}

#[test]
fn test_version() {
    let config: Config = Default::default();
    let subsystem: Box<Subsystem> = Box::new(Subsystem::new(gen_mock_good_eps()).unwrap());
    let service = Service::new(config, subsystem, QueryRoot, MutationRoot);

    let query = r#"{ 
        telemetry {
            version {
                daughterboard {
                    revision
                },
                motherboard {
                    revision
                }
            }
        }
    }"#;

    let expected = json!({
                "telemetry":{
                    "version":{
                        "daughterboard": {
                            "revision": 12
                        },
                        "motherboard": {
                            "revision": 10
                        }
                    }
                }
        });
    
    test!(service, query, expected);
}

#[test]
fn test_daughterboard_telemetry() {
    let config: Config = Default::default();
    let subsystem: Box<Subsystem> = Box::new(Subsystem::new(gen_mock_good_eps()).unwrap());
    let service = Service::new(config, subsystem, QueryRoot, MutationRoot);

    let query = r#"{ telemetry { daughterboard { BoardTemperature }}}"#;

    let expected = json!({
            "telemetry":{
                "daughterboard":{
                    "BoardTemperature": 101.55,
                }
            }
        });
    
    test!(service, query, expected);
}

#[test]
fn test_telemetry_status_last_error() {
    let config: Config = Default::default();
    let subsystem: Box<Subsystem> = Box::new(Subsystem::new(gen_mock_good_eps()).unwrap());
    let service = Service::new(config, subsystem, QueryRoot, MutationRoot);

    let query = r#"
    {
        telemetry {
            boardStatus {
                daughterboard, 
                motherboard, 
            },
            daughterboard {
                BoardTemperature
            },
            lastEpsError { 
                daughterboard,
                motherboard,    
            },
            motherboard {
                BoardTemperature
            },
        }
    }
    "#;

    let expected = json!({
            "telemetry":{
                "boardStatus":{
                    "daughterboard":"WATCHDOG_ERROR",
                    "motherboard":"LAST_COMMAND_FAILED",
                },
                "daughterboard":{
                    "BoardTemperature": 101.55,
                },
                "lastEpsError":{
                    "daughterboard":null,
                    "motherboard":"BAD_CRC",
                },
                "motherboard":{
                    "BoardTemperature": 105.13
                }
            }
        });
    
    test!(service, query, expected);
}
