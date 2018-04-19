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
#![allow(unused_variables)]

use failure::Fail;
use mai400_api::*;
use std::cell::RefCell;
use std::io::Error;
//use std::str;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread::spawn;

use objects::*;

pub struct ReadData {
    pub config: Mutex<ConfigInfo>,
    pub std_telem: Mutex<StandardTelemetry>,
    pub irehs_telem: Mutex<IREHSTelemetry>,
    pub imu: Mutex<RawIMU>,
    pub rotating: Mutex<RotatingTelemetry>,
}

impl ReadData {
    pub fn new() -> ReadData {
        ReadData {
            config: Mutex::new(ConfigInfo::default()),
            std_telem: Mutex::new(StandardTelemetry::default()),
            irehs_telem: Mutex::new(IREHSTelemetry::default()),
            imu: Mutex::new(RawIMU::default()),
            rotating: Mutex::new(RotatingTelemetry::default()),
        }
    }

    pub fn update_std(&self, telem: StandardTelemetry) {
        {
            //TODO: change to try_lock
            let mut local = self.std_telem.lock().unwrap();
            *local = telem.clone();
        }

        let mut local = self.rotating.lock().unwrap();
        local.update(&telem);
    }

    pub fn update_irehs(&self, irehs: IREHSTelemetry) {
        let mut local = self.irehs_telem.lock().unwrap();
        *local = irehs;
    }

    pub fn update_imu(&self, imu: RawIMU) {
        let mut local = self.imu.lock().unwrap();
        *local = imu;
    }

    pub fn update_config(&self, config: ConfigInfo) {
        let mut local = self.config.lock().unwrap();
        *local = config;
    }
}

pub fn read_thread(bus: String, data: Arc<ReadData>, sender: Sender<ConfigInfo>) -> MAIResult<()> {
    let connection = Connection::new(bus);
    let mai = MAI400::new(connection);

    loop {
        // TODO: Error handling and reporting
        let msg = mai.get_message().unwrap();
        match msg {
            Response::StdTelem(telem) => {
                data.update_std(telem);
                println!("Got StdTelem");
            }
            Response::Config(config) => {
                // Update our persistent config struct
                data.update_config(config.clone());
                // Tell whoever requested the config that we got the response
                // TODO: Maybe check the result and print an error message?
                // It's not a critical error, so we don't want to exit the loop
                // or anything
                let _ = sender.send(config);
                println!("Got config");
            }
            Response::IMU(telem) => {
                data.update_imu(telem);
                println!("Got RawIMU");
            }
            Response::IREHS(telem) => {
                data.update_irehs(telem);
                println!("Got IREHS");
            }
        }
    }
}

pub struct Subsystem {
    pub mai: MAI400,
    pub errors: RefCell<Vec<String>>,
    pub persistent: Arc<ReadData>,
    pub receiver: Receiver<ConfigInfo>,
}

impl Subsystem {
    pub fn new(bus: String, data: Arc<ReadData>) -> Subsystem {
        println!("New Subsystem");

        let connection = Connection::new(bus.clone());
        let mai = MAI400::new(connection);

        let (sender, receiver) = channel();
        let data_ref = data.clone();

        spawn(move || read_thread(bus, data_ref, sender));

        Subsystem {
            mai,
            errors: RefCell::new(vec![]),
            persistent: data.clone(),
            receiver,
        }
    }

    // Queries

    pub fn get_config(&self) -> Result<Config, Error> {
        let res = run!(self.mai.get_info(), self.errors).and_then(|_| {
            self.receiver.recv().map_err(|err| format!("{}", err))
        });

        let config = match res {
            Ok(cfg) => cfg,
            _ => self.persistent.config.lock().unwrap().clone(),
        };

        Ok(Config(config))
    }

    pub fn get_power(&self) -> Result<GetPowerResponse, Error> {
        unimplemented!();
    }

    pub fn get_telemetry(&self) -> Result<Telemetry, Error> {
        Ok(Telemetry {
            nominal: TelemetryNominal {
                std: StdTelem(self.persistent.std_telem.lock().unwrap().clone()),
                rotating: Rotating(self.persistent.rotating.lock().unwrap().clone()),
            },
            debug: TelemetryDebug {
                irehs: IREHSTelem(self.persistent.irehs_telem.lock().unwrap().clone()),
                raw_imu: RawIMUTelem(self.persistent.imu.lock().unwrap().clone()),
                config: Config(self.persistent.config.lock().unwrap().clone()),
            },
        })
    }

    pub fn get_test_results(&self) -> Result<IntegrationTestResults, Error> {
        unimplemented!();
    }

    pub fn get_mode(&self) -> Result<Mode, Error> {
        let raw = self.persistent.std_telem.lock()?.acs_mode;

        Ok(match raw {
            0 => Mode::TestMode,
            1 => Mode::RateNulling,
            2 => Mode::Reserved1,
            3 => Mode::NadirPointing,
            4 => Mode::LatLongPointing,
            5 => Mode::QbxMode,
            6 => Mode::Reserved2,
            7 => Mode::NormalSun,
            8 => Mode::LatLongSun,
            9 => Mode::Qintertial,
            10 => Mode::Reserved3,
            11 => Mode::Qtable,
            12 => Mode::SunRam,
        })
    }

    pub fn get_orientation(&self) -> Result<Orientation, Error> {
        unimplemented!();
    }

    pub fn get_spin(&self) -> Result<Spin, Error> {
        let rotating = self.persistent.rotating.lock()?;
        Ok(Spin {
            x: rotating.k_bdot[0],
            y: rotating.k_bdot[1],
            z: rotating.k_bdot[2],
        })
    }

    // Mutations

    pub fn noop(&self) -> Result<NoopResponse, Error> {
        // Get config? Would test sending and receiving
        unimplemented!();
    }

    pub fn control_power(&self) -> Result<ControlPowerResponse, Error> {
        // Reset command
        // (Copy AntS impl)
        unimplemented!();
    }

    pub fn configure_hardware(&self) -> Result<ConfigureHardwareResponse, Error> {
        // ????
        unimplemented!();
    }

    pub fn test_hardware(&self) -> Result<HardwareTestResults, Error> {
        // Get config? Would test sending and receiving
        unimplemented!();
    }

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

    pub fn set_mode(&self) -> Result<GenericResponse, Error> {
        // Set mode commnd
        unimplemented!();
    }

    pub fn update(&self) -> Result<GenericResponse, Error> {
        // Set_RV, Set_GPS_time
        unimplemented!();
    }
}
