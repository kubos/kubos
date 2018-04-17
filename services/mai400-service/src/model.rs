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

use failure::Fail;
use mai400_api::*;
use std::cell::RefCell;
use std::io::Error;
//use std::str;
use std::sync::Mutex;
use std::thread::spawn;

use objects::*;

pub struct Subsystem {
    pub mai: MAI400,
    pub errors: RefCell<Vec<String>>,
    pub config: Mutex<ConfigInfo>,
    pub std_telem: Mutex<StandardTelemetry>,
    pub irehs_telem: Mutex<IREHSTelemetry>,
    pub imu: Mutex<RawIMU>,
    pub rotating: Mutex<RotatingTelemetry>,
}

impl Subsystem {
    pub fn new(bus: String) -> Subsystem {
        println!("New Subsystem");

        let connection = Connection::new(bus);
        let mai = MAI400::new(connection);

        let subsystem = Subsystem {
            mai,
            errors: RefCell::new(vec![]),
            config: Mutex::new(ConfigInfo::default()),
            std_telem: Mutex::new(StandardTelemetry::default()),
            irehs_telem: Mutex::new(IREHSTelemetry::default()),
            imu: Mutex::new(RawIMU::default()),
            rotating: Mutex::new(RotatingTelemetry::default()),
        };

        let handle = spawn(|| println!("Receive thread"));

        subsystem
    }

    pub fn update_std(&self, telem: StandardTelemetry) {
        {
            //TODO: change to try_lock
            let mut std = self.std_telem.lock().unwrap();
            *std = telem.clone();
        }

        let mut rotating = self.rotating.lock().unwrap();
        rotating.update(&telem);
    }

    pub fn update_irehs(&self, telem: IREHSTelemetry) {
        let mut irehs = self.irehs_telem.lock().unwrap();
        *irehs = telem;
    }

    pub fn update_imu(&self, telem: RawIMU) {
        let mut imu = self.imu.lock().unwrap();
        *imu = telem;
    }

    // Queries

    pub fn get_config(&self) -> Result<Config, Error> {
        unimplemented!();
    }

    pub fn get_power(&self) -> Result<GetPowerResponse, Error> {
        unimplemented!();
    }

    pub fn get_telemetry(&self) -> Result<Telemetry, Error> {
        Ok(Telemetry {
            nominal: TelemetryNominal {
                std: StdTelem(self.std_telem.lock().unwrap().clone()),
                rotating: Rotating(self.rotating.lock().unwrap().clone()),
            },
            debug: TelemetryDebug {
                irehs: IREHSTelem(self.irehs_telem.lock().unwrap().clone()),
                raw_imu: RawIMUTelem(self.imu.lock().unwrap().clone()),
                config: Config(self.config.lock().unwrap().clone()),
            },
        })
    }

    pub fn get_test_results(&self) -> Result<IntegrationTestResults, Error> {
        unimplemented!();
    }

    pub fn get_mode(&self) -> Result<Mode, Error> {
        unimplemented!();
    }

    pub fn get_orientation(&self) -> Result<Orientation, Error> {
        unimplemented!();
    }

    pub fn get_spin(&self) -> Result<Spin, Error> {
        unimplemented!();
    }

    // Mutations

    pub fn passthrough(&self, command: String) -> Result<GenericResponse, Error> {
        let result = run!(self.mai.passthrough(command.as_bytes()), self.errors);

        Ok(GenericResponse {
            success: result.is_ok(),
            errors: match result {
                Ok(_) => "".to_owned(),
                Err(err) => err,
            },
        })
    }

    pub fn noop(&self) -> Result<NoopResponse, Error> {
        unimplemented!();
    }
}
