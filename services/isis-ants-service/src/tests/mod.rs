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
#[allow(unused_parens)]

use double;
use isis_ants_api::*;
use kubos_service::{Config, Service};
use model::*;
use objects::*;
use schema::*;
use std::cell::{Cell, RefCell};

macro_rules! mock_new {
    () => {
        MockAntS::new(
            Err(AntsError::ConfigError.into()),
            Err(AntsError::ConfigError.into()),
            Err(AntsError::ConfigError.into()),
            Err(AntsError::ConfigError.into()),
            Err(AntsError::ConfigError.into()),
            Err(AntsError::ConfigError.into()),
            Err(AntsError::ConfigError.into()),
            Err(AntsError::ConfigError.into()),
            Err(AntsError::ConfigError.into()),
            Err(AntsError::ConfigError.into()),
            Err(AntsError::ConfigError.into()),
            Err(AntsError::ConfigError.into()),
            Err(AntsError::ConfigError.into()),
            Err(AntsError::ConfigError.into()),
            Err(AntsError::ConfigError.into()),
            Err(AntsError::ConfigError.into()),
        )
    };
}

mock_trait_no_default!(
		MockAntS,
		configure(KANTSController) -> AntSResult<()>,
		reset() -> AntSResult<()>,
		arm() -> AntSResult<()>,
		disarm() -> AntSResult<()>,
		deploy(KANTSAnt, bool, u8) -> AntSResult<()>,
		auto_deploy(u8) -> AntSResult<()>,
		cancel_deploy() -> AntSResult<()>,
		get_deploy() -> AntSResult<DeployStatus>,
		get_uptime() -> AntSResult<u32>,
		get_system_telemetry() -> AntSResult<AntsTelemetry>,
		get_activation_count(KANTSAnt) -> AntSResult<u8>,
		get_activation_time(KANTSAnt) -> AntSResult<u16>,
		watchdog_kick() -> AntSResult<()>,
		watchdog_start() -> AntSResult<()>,
		watchdog_stop() -> AntSResult<()>,
		passthrough(Vec<u8>, Vec<u8>) -> AntSResult<()>
	);

impl IAntS for MockAntS {
    fn new(
        _bus: &str,
        _primary: u8,
        _secondary: u8,
        _ant_count: u8,
        _timeout: u32,
    ) -> AntSResult<MockAntS> {
        Ok(mock_new!())
    }
    mock_method!(configure(&self, config: KANTSController) -> AntSResult<()>);
    mock_method!(reset(&self) -> AntSResult<()>);
    mock_method!(arm(&self) -> AntSResult<()>);
    mock_method!(disarm(&self) -> AntSResult<()>);
    mock_method!(deploy(&self, antenna: KANTSAnt, force: bool, timeout: u8) -> AntSResult<()>);
    mock_method!(auto_deploy(&self, timeout: u8) -> AntSResult<()>);
    mock_method!(cancel_deploy(&self) -> AntSResult<()>);
    mock_method!(get_deploy(&self) -> AntSResult<DeployStatus>);
    mock_method!(get_uptime(&self) -> AntSResult<u32>);
    mock_method!(get_system_telemetry(&self) -> AntSResult<AntsTelemetry>);
    mock_method!(get_activation_count(&self, antenna: KANTSAnt) -> AntSResult<u8>);
    mock_method!(get_activation_time(&self, antenna: KANTSAnt) -> AntSResult<u16>);
    mock_method!(watchdog_kick(&self) -> AntSResult<()>);
    mock_method!(watchdog_start(&self) -> AntSResult<()>);
    mock_method!(watchdog_stop(&self) -> AntSResult<()>);
    mock_method!(passthrough(&self, tx: &[u8], rx: &mut [u8]) -> AntSResult<()>, self, {
            for (i, elem) in rx.iter_mut().enumerate () {
                *elem = i as u8;
            }
            self.passthrough.call((tx.to_vec(), rx.to_vec()))
        });
}

macro_rules! wrap {
    ($result:ident) => {{
        json!({
                "msg": $result,
                "errs": ""
        }).to_string()
    }};
}

macro_rules! service_new {
    ($mock:ident) => {{
        Service::new(
            Config::new("isis-ants-service"),
            Subsystem {
                ants: Box::new($mock),
                count: 4,
                errors: RefCell::new(vec![]),
                last_cmd: Cell::new(AckCommand::None)
            },
            QueryRoot,
            MutationRoot,
        )
    }};
}

mod mutations;
mod queries;

#[test]
fn ping() {
    let mock = mock_new!();
    let service = service_new!(mock);

    let query = r#"
        {
            ping
        }"#;

    let expected = json!({
            "ping": "pong"
    });

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}
