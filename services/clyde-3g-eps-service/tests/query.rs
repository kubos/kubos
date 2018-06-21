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

use clyde_3g_eps_api::Eps;
use clyde_3g_eps_service::models::subsystem::{Mutations, Subsystem};
use clyde_3g_eps_service::models::*;
use clyde_3g_eps_service::schema::mutation::Root as MutationRoot;
use clyde_3g_eps_service::schema::query::Root as QueryRoot;
use eps_api::{EpsError, EpsResult};
use failure::Error;
use kubos_service::{Config, MutationResponse, Service};
use rust_i2c::Connection;
use std::cell::{Cell, RefCell};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

mock_trait_no_default!(
    MockSubsystem,
    get_motherboard_telemetry(motherboard_telemetry::Type) -> EpsResult<f32>,
    get_daughterboard_telemetry(daughterboard_telemetry::Type) -> EpsResult<f32>,
    get_reset_telemetry(reset_telemetry::Type) -> EpsResult<reset_telemetry::Data>,
    get_comms_watchdog_period() -> EpsResult<u8>,
    get_version() -> EpsResult<version::Data>,
    manual_reset() -> EpsResult<MutationResponse>,
    reset_watchdog() -> EpsResult<MutationResponse>,
    set_watchdog_period(u8) -> EpsResult<MutationResponse>,
    raw_command(u8, Vec<u8>) -> EpsResult<MutationResponse>,
    get_last_mutation() -> Mutations,
    set_last_mutation(Mutations) -> ()
);

impl Subsystem for MockSubsystem {
    mock_method!(get_motherboard_telemetry(&self, t: motherboard_telemetry::Type) -> EpsResult<f32>);
    mock_method!(get_daughterboard_telemetry(&self, t: daughterboard_telemetry::Type) -> EpsResult<f32>);
    mock_method!(get_reset_telemetry(&self, t: reset_telemetry::Type) -> EpsResult<reset_telemetry::Data>);
    mock_method!(get_comms_watchdog_period(&self) -> EpsResult<u8>);
    mock_method!(get_version(&self) -> EpsResult<version::Data>);
    mock_method!(manual_reset(&self) -> EpsResult<MutationResponse>);
    mock_method!(reset_watchdog(&self) -> EpsResult<MutationResponse>);
    mock_method!(set_watchdog_period(&self, period: u8) -> EpsResult<MutationResponse>);
    mock_method!(raw_command(&self, cmd: u8, data: Vec<u8>) -> EpsResult<MutationResponse>);
    mock_method!(get_last_mutation(&self) -> Mutations);
    mock_method!(set_last_mutation(&self, mutation: Mutations) -> ());
}

#[test]
fn test_ping() {
    let config: Config = Default::default();
    let subsystem: Box<Subsystem + 'static> = Box::new(MockSubsystem::new(
        Err(EpsError::GenericError),
        Err(EpsError::GenericError),
        Err(EpsError::GenericError),
        Err(EpsError::GenericError),
        Err(EpsError::GenericError),
        Err(EpsError::GenericError),
        Err(EpsError::GenericError),
        Err(EpsError::GenericError),
        Err(EpsError::GenericError),
        Mutations::None,
        (),
    ));
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

    let mock = MockSubsystem::new(
        Err(EpsError::GenericError),
        Err(EpsError::GenericError),
        Err(EpsError::GenericError),
        Err(EpsError::GenericError),
        Err(EpsError::GenericError),
        Err(EpsError::GenericError),
        Err(EpsError::GenericError),
        Err(EpsError::GenericError),
        Err(EpsError::GenericError),
        Mutations::None,
        (),
    );

    mock.get_version.return_value(Ok(version::Data {
        motherboard: version::VersionData {
            firmware_number: 100,
            revision: 10,
        },
        daughterboard: None,
    }));

    let subsystem: Box<Subsystem + 'static> = Box::new(mock);

    let service = Service::new(config, subsystem, QueryRoot, MutationRoot);

    let query = r#"{ version { motherboard { revision }}}"#;

    assert_eq!(
        service.process(&query),
        json!({
            "msg":{
                "version":{
                    "motherboard":{
                        "revision":10
                    }
                }
            },
            "errs":""
        }).to_string()
    );
}
