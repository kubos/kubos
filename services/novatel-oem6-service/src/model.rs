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
use novatel_oem6_api::Log::*;
use novatel_oem6_api::*;
use std::cell::{Cell, RefCell};
use std::io::Error;
use std::sync::mpsc::{sync_channel, Receiver, SyncSender, TryRecvError, TrySendError};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use objects::*;

pub const RECV_TIMEOUT: Duration = Duration::from_millis(300);

pub struct LockData {
    pub status: Mutex<LockStatus>,
    pub info: Mutex<LockInfo>,
}

impl LockData {
    pub fn new() -> Self {
        LockData {
            status: Mutex::new(LockStatus::default()),
            info: Mutex::new(LockInfo::default()),
        }
    }

    pub fn update_status(&self, status: LockStatus) {
        let mut local = self.status.lock().unwrap();
        *local = status;
    }

    pub fn update_info(&self, info: LockInfo) {
        let mut local = self.info.lock().unwrap();
        *local = info;
    }
}

// Listen for log messages from the OEM6 and route data to the appropriate
// listener or structure.
//
// The OEM6 will send us one of three log messages:
// - Lock information. The OEM6 will likely be set up to output this data
//   once per second.
// - Version information. This data will be output immediately upon request by
//   the `noop` and `get_test_results` functions
// - Error information. If enabled, this will be output by the OEM6 when an
//   error or event occurs.
pub fn log_thread(
    oem: OEM6,
    data: Arc<LockData>,
    error_send: SyncSender<RxStatusEventLog>,
    version_send: SyncSender<VersionLog>,
) {
    loop {
        match oem.get_log()
            .expect("Underlying read thread no longer communicating")
        {
            BestXYZ(log) => {
                // TODO: Do we want to only update our stored values if
                // both position and velocity are valid? The alternative
                // would be to just update which ever one is valid and
                // ignore the other
                if log.pos_status == 0 && log.vel_status == 0 {
                    data.update_info(LockInfo {
                        time: OEMTime {
                            week: log.week as i32,
                            ms: log.ms,
                        },
                        position: log.position,
                        velocity: log.velocity,
                    });
                }
                data.update_status(LockStatus {
                    time_status: log.time_status,
                    time: OEMTime {
                        week: log.week as i32,
                        ms: log.ms,
                    },
                    position_status: log.pos_status,
                    position_type: log.pos_type,
                    velocity_status: log.vel_status,
                    velocity_type: log.vel_type,
                });
            }
            RxStatusEvent(log) => error_send
                .try_send(log)
                .or_else::<TrySendError<RxStatusEventLog>, _>(|err| match err {
                    // Our buffer is full, but the receiver should still be alive, so let's keep going
                    TrySendError::Full(_) => Ok(()),
                    TrySendError::Disconnected(_) => {
                        panic!("Error receiver disconnected. Assuming system has become corrupted")
                    }
                })
                .unwrap(),
            Version(log) => version_send
                .try_send(log)
                .or_else::<TrySendError<VersionLog>, _>(|err| match err {
                    // Our buffer is full, but the receiver should still be alive, so let's keep going
                    TrySendError::Full(_) => Ok(()),
                    TrySendError::Disconnected(_) => panic!(
                        "Version receiver disconnected. Assuming system has become corrupted"
                    ),
                })
                .unwrap(),
        }
    }
}

pub struct Subsystem {
    pub oem: OEM6,
    pub last_cmd: Cell<AckCommand>,
    pub errors: RefCell<Vec<String>>,
    pub lock_data: Arc<LockData>,
    pub error_recv: Receiver<RxStatusEventLog>,
    pub version_recv: Receiver<VersionLog>,
}

impl Subsystem {
    pub fn new(bus: &'static str, data: Arc<LockData>) -> OEMResult<Subsystem> {
        let (log_send, log_recv) = sync_channel(5);
        let (response_send, response_recv) = sync_channel(5);

        let oem = OEM6::new(bus, BaudRate::Baud9600, log_recv, response_recv)?;

        let rx_conn = oem.conn.clone();
        thread::spawn(move || read_thread(rx_conn, log_send, response_send));

        let (error_send, error_recv) = sync_channel(10);
        let (version_send, version_recv) = sync_channel(1);

        let data_ref = data.clone();
        let oem_ref = oem.clone();
        thread::spawn(move || log_thread(oem_ref, data_ref, error_send, version_send));

        println!("Kubos OEM6 service started");

        Ok(Subsystem {
            oem,
            last_cmd: Cell::new(AckCommand::None),
            errors: RefCell::new(vec![]),
            lock_data: data.clone(),
            error_recv,
            version_recv,
        })
    }

    fn get_version_log(&self) -> Result<VersionLog, String> {
        match self.oem.request_version() {
            Ok(_) => match self.version_recv.recv_timeout(RECV_TIMEOUT) {
                Ok(log) => Ok(log),
                Err(err) => Err(format!("Failed to receive version info - {}", err).to_owned()),
            },
            Err(err) => Err(process_errors!(err)),
        }
    }

    // Queries

    pub fn get_errors(&self) {
        match self.error_recv.try_recv() {
            Ok(msg) => {
                push_err!(
                    self.errors,
                    format!(
                        "RxStatusEvent({}, {}, {}): {}",
                        msg.word, msg.bit, msg.event, msg.description
                    )
                );

                while let Ok(err) = self.error_recv.try_recv() {
                    push_err!(
                        self.errors,
                        format!(
                            "RxStatusEvent({}, {}, {}): {}",
                            err.word, err.bit, err.event, err.description
                        )
                    );
                }
            }
            Err(err) => match err {
                // Do nothing. This is the good case
                TryRecvError::Empty => {}
                // We've lost connection with the errors channel
                TryRecvError::Disconnected => {
                    push_err!(self.errors, "Errors sender disconnected".to_owned());
                }
            },
        }
    }

    pub fn get_power(&self) -> Result<GetPowerResponse, Error> {
        let (state, uptime) = match self.get_version_log().is_ok() {
            true => (PowerState::On, 1),
            false => (PowerState::Off, 0),
        };

        Ok(GetPowerResponse { state, uptime })
    }

    pub fn get_system_status(&self) -> Result<SystemStatus, Error> {
        self.get_errors();

        let mut errors = match self.errors.try_borrow() {
            Ok(master_vec) => master_vec.clone(),
            _ => vec!["Error: Failed to borrow master errors vector".to_owned()],
        };

        let status = match self.get_version_log() {
            Ok(log) => log.recv_status,
            Err(err) => {
                errors.push(format!("System Status: {}", err).to_owned());
                /* My first thought was to use ReceiverStatusFlags::empty(),
                 * but I'm worried that that is easily mistaken for this function
                 * actually having succeeded
                 */
                ReceiverStatusFlags::all()
            }
        };

        Ok(SystemStatus {
            status: ReceiverStatus(status),
            errors,
        })
    }

    pub fn get_lock_status(&self) -> Result<LockStatus, Error> {
        Ok(self.lock_data.status.lock().unwrap().clone())
    }

    pub fn get_lock_info(&self) -> Result<LockInfo, Error> {
        Ok(self.lock_data.info.lock().unwrap().clone())
    }

    pub fn get_test_results(&self) -> Result<IntegrationTestResults, Error> {
        let telem = self.get_telemetry()?;

        Ok(IntegrationTestResults {
            success: !telem.debug.is_none(),
            errors: telem.nominal.system_status.errors.clone().join("; "),
            telemetry_debug: telem.debug.clone(),
            telemetry_nominal: telem.nominal.clone(),
        })
    }

    pub fn get_telemetry(&self) -> Result<Telemetry, Error> {
        self.get_errors();

        let mut errors = match self.errors.try_borrow() {
            Ok(master_vec) => master_vec.clone(),
            _ => vec!["Error: Failed to borrow master errors vector".to_owned()],
        };

        let (status, version_info) = match self.get_version_log() {
            Ok(log) => (
                log.recv_status,
                Some(VersionInfo {
                    num_components: log.num_components as i32,
                    components: log.components
                        .iter()
                        .map(|comp| VersionComponent(comp.clone()))
                        .collect(),
                }),
            ),
            Err(err) => {
                let temp = format!("Get Telemetry: {}", err);
                errors.push(temp.clone());
                push_err!(self.errors, temp);
                (ReceiverStatusFlags::all(), None)
            }
        };

        let lock_status = self.get_lock_status().ok();
        let lock_info = self.get_lock_info().ok();

        Ok(Telemetry {
            nominal: TelemetryNominal {
                system_status: SystemStatus {
                    status: ReceiverStatus(status),
                    errors,
                },
                lock_status,
                lock_info,
            },
            debug: version_info,
        })
    }

    // Mutations

    pub fn noop(&self) -> Result<GenericResponse, Error> {
        let result = self.get_version_log();

        let success = result.is_ok();

        let errors = match result {
            Ok(_) => "".to_owned(),
            Err(err) => {
                let temp = format!("Noop: {}", err);
                push_err!(self.errors, temp);
                err
            }
        };

        Ok(GenericResponse { success, errors })
    }

    pub fn configure_hardware(
        &self,
        input: Vec<ConfigStruct>,
    ) -> Result<ConfigureHardwareResponse, Error> {
        let mut success = true;
        let mut errors = "".to_owned();
        let mut config = "".to_owned();

        for entry in input.iter() {
            let result = run!(
                match entry.option {
                    ConfigOption::LogErrorData => self.oem.request_errors(entry.hold),
                    ConfigOption::LogPositionData => {
                        self.oem
                            .request_position(entry.interval, entry.offset, entry.hold)
                    }
                    ConfigOption::UnlogAll => self.oem.request_unlog_all(entry.hold),
                    ConfigOption::UnlogErrorData => {
                        self.oem.request_unlog(MessageID::RxStatusEvent)
                    }
                    ConfigOption::UnlogPositionData => self.oem.request_unlog(MessageID::BestXYZ),
                },
                self.errors
            );

            success &= result.is_ok();
            if let Err(err) = result {
                if !errors.is_empty() {
                    errors.push_str(". ");
                }
                errors.push_str(&format!("{:?}: {}", entry.option, err));
            }

            if !config.is_empty() {
                config.push_str(", ");
            }
            config.push_str(&format!("{:?}(Hold: {})", entry.option, entry.hold));
            if entry.interval != 0.0 {
                config.push_str(&format!(": {}+{}sec", entry.interval, entry.offset));
            }
        }

        Ok(ConfigureHardwareResponse {
            success,
            errors,
            config,
        })
    }

    pub fn passthrough(&self, command: String) -> Result<GenericResponse, Error> {
        // Convert the hex values in the string into actual hex values
        // Ex. "c3c2" -> [0xc3, 0xc2]
        let tx: Vec<u8> = command
            .as_bytes()
            .chunks(2)
            .into_iter()
            .map(|chunk| u8::from_str_radix(::std::str::from_utf8(chunk).unwrap(), 16).unwrap())
            .collect();

        let result = run!(self.oem.passthrough(tx.as_slice()), self.errors);

        Ok(GenericResponse {
            success: result.is_ok(),
            errors: match result {
                Ok(_) => "".to_owned(),
                Err(err) => err,
            },
        })
    }
}
