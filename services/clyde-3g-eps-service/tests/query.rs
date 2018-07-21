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
extern crate double;
extern crate clyde_3g_eps_api;
extern crate clyde_3g_eps_service;
extern crate kubos_service;
#[macro_use]
extern crate serde_json;
extern crate eps_api;
extern crate failure;
extern crate rust_i2c;

use clyde_3g_eps_api::*;
use clyde_3g_eps_service::models::subsystem::{Mutations, Subsystem};
use clyde_3g_eps_service::models::*;
use clyde_3g_eps_service::schema::mutation::Root as MutationRoot;
use clyde_3g_eps_service::schema::query::Root as QueryRoot;
use eps_api::*;
use kubos_service::{Config, MutationResponse, Service};
use std::cell::{Cell, RefCell};
use std::sync::{Arc, Mutex};

struct MockBadEps {}

impl Clyde3gEps for MockBadEps {
    fn get_board_status(&self) -> EpsResult<BoardStatus> {
        Err(EpsError::GenericError)
    }
    fn get_checksum(&self) -> EpsResult<Checksum> {
        Err(EpsError::GenericError)
    }
    fn get_version_info(&self) -> EpsResult<VersionInfo> {
        Err(EpsError::GenericError)
    }
    fn get_last_error(&self) -> EpsResult<LastError> {
        Err(EpsError::GenericError)
    }
    fn manual_reset(&self) -> EpsResult<()> {
        Err(EpsError::GenericError)
    }
    fn reset_comms_watchdog(&self) -> EpsResult<()> {
        Err(EpsError::GenericError)
    }
    fn get_motherboard_telemetry(&self, telem_type: MotherboardTelemetry::Type) -> EpsResult<f32> {
        Err(EpsError::GenericError)
    }
    fn get_daughterboard_telemetry(
        &self,
        telem_type: DaughterboardTelemetry::Type,
    ) -> EpsResult<f32> {
        Err(EpsError::GenericError)
    }
    fn get_reset_telemetry(
        &self,
        telem_type: ResetTelemetry::Type,
    ) -> EpsResult<ResetTelemetry::Data> {
        Err(EpsError::GenericError)
    }
    fn set_comms_watchdog_period(&self, period: u8) -> EpsResult<()> {
        Err(EpsError::GenericError)
    }
    fn get_comms_watchdog_period(&self) -> EpsResult<u8> {
        Err(EpsError::GenericError)
    }
    fn raw_command(&self, cmd: u8, data: Vec<u8>) -> EpsResult<()> {
        Err(EpsError::GenericError)
    }
}

fn gen_mock_bad_eps() -> Box<Clyde3gEps + Send> {
    Box::new(MockBadEps {})
}

struct MockGoodEps {}

impl Clyde3gEps for MockGoodEps {
    fn get_board_status(&self) -> EpsResult<BoardStatus> {
        Ok(BoardStatus {
            motherboard: StatusCode::LAST_COMMAND_FAILED,
            daughterboard: Some(StatusCode::WATCHDOG_ERROR),
        })
    }
    fn get_checksum(&self) -> EpsResult<Checksum> {
        Ok(Checksum {
            motherboard: 155,
            daughterboard: Some(164),
        })
    }
    fn get_version_info(&self) -> EpsResult<VersionInfo> {
        Ok(VersionInfo {
            motherboard: Version {
                revision: 10,
                firmware_number: 100,
            },
            daughterboard: Some(Version {
                revision: 12,
                firmware_number: 105,
            }),
        })
    }
    fn get_last_error(&self) -> EpsResult<LastError> {
        Ok(LastError {
            motherboard: ErrorCode::BAD_CRC,
            daughterboard: None,
        })
    }
    fn manual_reset(&self) -> EpsResult<()> {
        Ok(())
    }
    fn reset_comms_watchdog(&self) -> EpsResult<()> {
        Ok(())
    }
    fn get_motherboard_telemetry(&self, telem_type: MotherboardTelemetry::Type) -> EpsResult<f32> {
        Ok(105.13)
    }
    fn get_daughterboard_telemetry(
        &self,
        telem_type: DaughterboardTelemetry::Type,
    ) -> EpsResult<f32> {
        Ok(101.55)
    }
    fn get_reset_telemetry(
        &self,
        telem_type: ResetTelemetry::Type,
    ) -> EpsResult<ResetTelemetry::Data> {
        Ok(ResetTelemetry::Data {
            motherboard: 89,
            daughterboard: Some(99),
        })
    }
    fn set_comms_watchdog_period(&self, period: u8) -> EpsResult<()> {
        Ok(())
    }
    fn get_comms_watchdog_period(&self) -> EpsResult<u8> {
        Ok(10)
    }
    fn raw_command(&self, cmd: u8, data: Vec<u8>) -> EpsResult<()> {
        Ok(())
    }
}

fn gen_mock_good_eps() -> Box<Clyde3gEps + Send> {
    Box::new(MockGoodEps {})
}

#[test]
fn test_ping() {
    let config: Config = Default::default();
    let subsystem: Box<Subsystem> = Box::new(Subsystem::new(gen_mock_bad_eps()).unwrap());
    let service = Service::new(config, subsystem, QueryRoot, MutationRoot);

    let query = r#"{ ping }"#;

    assert_eq!(
        service.process(&query),
        json!({"msg":{"ping":"pong"}, "errs":""}).to_string()
    );
}

#[test]
fn test_version() {
    let config: Config = Default::default();
    let subsystem: Box<Subsystem> = Box::new(Subsystem::new(gen_mock_good_eps()).unwrap());
    let service = Service::new(config, subsystem, QueryRoot, MutationRoot);

    let query = r#"{ telemetry { version { motherboard { revision } daughterboard { revision }}}}"#;

    assert_eq!(
        service.process(&query),
        json!({
            "msg":{
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
            },
            "errs":""
        }).to_string()
    );
}

#[test]
fn test_daughterboard_telemetry() {
    let config: Config = Default::default();
    let subsystem: Box<Subsystem> = Box::new(Subsystem::new(gen_mock_good_eps()).unwrap());
    let service = Service::new(config, subsystem, QueryRoot, MutationRoot);

    let query = r#"{ telemetry { daughterboard { BoardTemperature }}}"#;

    assert_eq!(
        service.process(&query),
        json!({
            "msg":{
                "telemetry":{
                    "daughterboard":{
                        "BoardTemperature": 101.55000305175781,
                    }
                }
            },
            "errs":""
        }).to_string()
    );
}

#[test]
fn test_telemetry_status_last_error() {
    let config: Config = Default::default();
    let subsystem: Box<Subsystem> = Box::new(Subsystem::new(gen_mock_good_eps()).unwrap());
    let service = Service::new(config, subsystem, QueryRoot, MutationRoot);

    let query = r#"
    {
        telemetry {
            daughterboard {
                BoardTemperature
            }
            motherboard {
                BoardTemperature
            }
            boardStatus { motherboard daughterboard }
            lastEpsError { motherboard daughterboard }
        }
    }
    "#;

    assert_eq!(
        service.process(&query),
        json!({
            "msg":{
                "telemetry":{
                    "boardStatus":{
                        "motherboard":"LAST_COMMAND_FAILED",
                        "daughterboard":"WATCHDOG_ERROR",
                    },
                    "daughterboard":{
                        "BoardTemperature": 101.55000305175781,
                    },
                    "lastEpsError":{
                        "daughterboard":null,
                        "motherboard":"BAD_CRC",
                    },
                    "motherboard":{
                        "BoardTemperature": 105.12999725341797
                    }
                }
            },
            "errs":""
        }).to_string()
    );
}
