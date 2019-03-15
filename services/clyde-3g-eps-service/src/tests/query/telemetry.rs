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

use super::*;

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
fn test_motherboard_telemetry() {
    let config: Config = Default::default();
    let subsystem: Box<Subsystem> = Box::new(Subsystem::new(gen_mock_good_eps()).unwrap());
    let service = Service::new(config, subsystem, QueryRoot, MutationRoot);

    let query = r#"{ telemetry { motherboard { BoardTemperature }}}"#;

    let expected = json!({
        "telemetry":{
            "motherboard":{
                "BoardTemperature": 105.13,
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
fn test_last_error() {
    let config: Config = Default::default();
    let subsystem: Box<Subsystem> = Box::new(Subsystem::new(gen_mock_good_eps()).unwrap());
    let service = Service::new(config, subsystem, QueryRoot, MutationRoot);

    let query = r#"
    {
        telemetry {
            lastEpsError { 
                daughterboard,
                motherboard,    
            },
        }
    }
    "#;

    let expected = json!({
        "telemetry":{
            "lastEpsError":{
                "daughterboard":null,
                "motherboard":"BAD_CRC",
            },
        }
    });

    test!(service, query, expected);
}

#[test]
fn test_board_status() {
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
        }
    }
    "#;

    let expected = json!({
        "telemetry":{
            "boardStatus":{
                "daughterboard":["WATCHDOG_ERROR", "BAD_COMMAND_DATA"],
                "motherboard":["LAST_COMMAND_FAILED", "POWER_ON_RESET"],
            },
        }
    });

    test!(service, query, expected);
}

#[test]
fn test_reset_telemetry() {
    let config: Config = Default::default();
    let subsystem: Box<Subsystem> = Box::new(Subsystem::new(gen_mock_good_eps()).unwrap());
    let service = Service::new(config, subsystem, QueryRoot, MutationRoot);

    let query = r#"
    {
        telemetry
        {
            reset {
                automaticSoftware {
                    daughterboard,
                    motherboard
                },
                brownOut {
                    daughterboard,
                    motherboard
                },
                manual {
                    daughterboard,
                    motherboard
                },
                watchdog {
                    daughterboard,
                    motherboard
                }
            }
        }
    }
    "#;

    let expected = json!({
        "telemetry":{
            "reset":{
                "automaticSoftware":{
                    "daughterboard": 2,
                    "motherboard": 1,
                },
                "brownOut":{
                    "daughterboard": 4,
                    "motherboard": 3,
                },
                "manual":{
                    "daughterboard": 6,
                    "motherboard": 5,
                },
                "watchdog":{
                    "daughterboard": 8,
                    "motherboard": 7,
                },
            }
        }
    });

    test!(service, query, expected);
}

#[test]
fn test_watchdog_period() {
    let config: Config = Default::default();
    let subsystem: Box<Subsystem> = Box::new(Subsystem::new(gen_mock_good_eps()).unwrap());
    let service = Service::new(config, subsystem, QueryRoot, MutationRoot);

    let query = r#"{ telemetry { watchdogPeriod }}"#;

    let expected = json!({
        "telemetry":{
            "watchdogPeriod": 10
        }
    });

    test!(service, query, expected);
}
