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

use crate::models::subsystem::{Mutations, Subsystem};
use crate::models::*;
use crate::schema::mutation::Root as MutationRoot;
use crate::schema::query::Root as QueryRoot;
use clyde_3g_eps_api::*;
use eps_api::*;
use kubos_service::{Config, Service};
use serde_json::json;
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
    fn get_motherboard_telemetry(&self, telem_type: MotherboardTelemetry::Type) -> EpsResult<f64> {
        Err(EpsError::GenericError)
    }
    fn get_daughterboard_telemetry(
        &self,
        telem_type: DaughterboardTelemetry::Type,
    ) -> EpsResult<f64> {
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
    fn get_motherboard_telemetry(&self, telem_type: MotherboardTelemetry::Type) -> EpsResult<f64> {
        Ok(105.13)
    }
    fn get_daughterboard_telemetry(
        &self,
        telem_type: DaughterboardTelemetry::Type,
    ) -> EpsResult<f64> {
        Ok(101.55)
    }
    fn get_reset_telemetry(
        &self,
        telem_type: ResetTelemetry::Type,
    ) -> EpsResult<ResetTelemetry::Data> {
        let (motherboard, daughterboard) = match telem_type {
            ResetTelemetry::Type::AutomaticSoftware => (1, 2),
            ResetTelemetry::Type::BrownOut => (3, 4),
            ResetTelemetry::Type::Manual => (5, 6),
            ResetTelemetry::Type::Watchdog => (7, 8),
        };

        Ok(ResetTelemetry::Data {
            motherboard,
            daughterboard: Some(daughterboard),
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

macro_rules! request {
    ($service:ident, $query:ident) => {{
        // Warp doesn't like control characters (ie. new line characters)
        // so we need to remove them before we send the request
        let query = $query.replace("\n", "");
        warp::test::request()
            .header("Content-Type", "application/json")
            .method("POST")
            .body(format!("{{\"query\": \"{}\"}}", query))
            .reply(&$service.filter)
    }};
}

macro_rules! wrap {
    ($result:ident) => {{
        &json!({ "data": $result }).to_string()
    }};
}

macro_rules! test {
    ($service:ident, $query:ident, $expected:ident) => {{
        let res = request!($service, $query);

        assert_eq!(res.body(), wrap!($expected));
    }};
}

mod mutation;
mod query;
