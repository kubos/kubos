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
use crate::model::*;
use crate::objects::*;
use crate::schema::*;
#[allow(unused_parens)]
use isis_ants_api::*;
use kubos_service::{Config, Service};
use serde_json::json;
use std::sync::{Arc, RwLock};

/// Structure for interacting with an ISIS Antenna System
#[derive(Clone)]
pub struct MockAntS {
    pub state: bool,
    pub deploy_status: DeployStatus
}

impl IAntS for MockAntS {

    fn new(_bus: &str, _primary: u8, _secondary: u8, _ant_count: u8, _timeout: u32) -> AntSResult<MockAntS> {
        Ok(MockAntS{ 
            state: true,
            deploy_status: DeployStatus::default()
        })
    }

    fn configure(&self, _config: KANTSController) -> AntSResult<()> {
        if self.state == true {
            Ok(())
        } else {
            Err(AntsError::ConfigError)
        }
    }

    fn reset(&self) -> AntSResult<()> {
        if self.state == true {
            Ok(())
        } else {
            Err(AntsError::ConfigError)
        }
    }

    fn arm(&self) -> AntSResult<()> {
        if self.state == true {
            Ok(())
        } else {
            Err(AntsError::ConfigError)
        }
    }

    fn disarm(&self) -> AntSResult<()> {
        if self.state == true {
            Ok(())
        } else {
            Err(AntsError::ConfigError)
        }
    }

    fn deploy(&self, _antenna: KANTSAnt, _force: bool, _timeout: u8) -> AntSResult<()> {
        if self.state == true {
            Ok(())
        } else {
            Err(AntsError::ConfigError)
        }
    }

    fn auto_deploy(&self, _timeout: u8) -> AntSResult<()> {
        if self.state == true {
            Ok(())
        } else {
            Err(AntsError::ConfigError)
        }
    }

    fn cancel_deploy(&self) -> AntSResult<()> {
        if self.state == true {
            Ok(())
        } else {
            Err(AntsError::ConfigError)
        }
    }

    fn get_deploy(&self) -> AntSResult<DeployStatus> {
        if self.state == true {
            Ok(self.deploy_status.clone())
        } else {
           Err(AntsError::GenericError)
        }
    }

    fn get_uptime(&self) -> AntSResult<u32> {
        if self.state == true {
            Ok(10)
        } else {
            Ok(0)
        }
    }

    fn get_system_telemetry(&self) -> AntSResult<AntsTelemetry> {
        if self.state == true {
            Ok(AntsTelemetry {
                raw_temp: 15,
                uptime: 35,
                deploy_status: DeployStatus {
                    sys_armed: true,
                    ant_1_active: true,
                    ant_4_not_deployed: false,
                    ..Default::default()
                },
            })
        } else {
            Err(AntsError::GenericError)
        }
    }
	
    fn get_activation_count(&self, antenna: KANTSAnt) -> AntSResult<u8> {
        if self.state == true {
            match antenna {
                KANTSAnt::Ant1 => Ok(1),
                KANTSAnt::Ant2 => Ok(2),
                KANTSAnt::Ant3 => Ok(3),
                KANTSAnt::Ant4 => Ok(4)
            }
        } else {
            Err(AntsError::GenericError)
        }
    }

    fn get_activation_time(&self, antenna: KANTSAnt) -> AntSResult<u16> {
        if self.state == true {
            match antenna {
                KANTSAnt::Ant1 => Ok(11),
                KANTSAnt::Ant2 => Ok(22),
                KANTSAnt::Ant3 => Ok(33),
                KANTSAnt::Ant4 => Ok(44)
            }
        } else {
            Err(AntsError::GenericError)
        }
    }

    fn watchdog_kick(&self) -> AntSResult<()> {
        if self.state == true {
            Ok(())
        } else {
            Err(AntsError::ConfigError)
        }
    }

    fn watchdog_start(&self) -> AntSResult<()> {
        if self.state == true {
            Ok(())
        } else {
            Err(AntsError::ConfigError)
        }
    }

    fn watchdog_stop(&self) -> AntSResult<()> {
        if self.state == true {
            Ok(())
        } else {
            Err(AntsError::ConfigError)
        }
    }

    fn passthrough(&self, _tx: &[u8], rx_in: &mut [u8]) -> AntSResult<()> {
        if self.state == true {
            for (i, elem) in rx_in.iter_mut().enumerate () {
                *elem = i as u8;
            }
            Ok(())
        } else {
            Err(AntsError::ConfigError)
        }
    }
}

macro_rules! mock_new {
    () => {{
        MockAntS::new("/dev/i2c-1", 0x30, 0x31, 4, 10).unwrap()        
    }};
}

macro_rules! request {
    ($service:ident, $query:ident) => {{
        // Warp doesn't like control characters (ie. new line characters)
        // so we need to remove them before we send the request
        let query = $query.replace("\n","");
        warp::test::request()
            .header("Content-Type", "application/json")
            .method("POST")
            .body(format!("{{\"query\": \"{}\"}}", query))
            .reply(&$service.filter)
    }};
}

macro_rules! wrap {
    ($result:ident) => {{
        &json!({
                "data": $result
        }).to_string()
    }};
}

macro_rules! test {
    ($service:ident, $query:ident, $expected:ident) => {{
        let res = request!($service, $query);

        assert_eq!(res.body(), wrap!($expected));
        
    }};
}

macro_rules! service_new {
    ($mock:ident) => {{
        Service::new(
            Config::new("isis-ants-service"),
            Subsystem {
                ants: Box::new($mock),
                count: 4,
                controller: Arc::new(RwLock::new(ConfigureController::Primary)),
                errors: Arc::new(RwLock::new(vec![])),
                last_cmd: Arc::new(RwLock::new(AckCommand::None)),
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

    test!(service, query, expected);
}
