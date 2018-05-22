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
use novatel_oem6_api::*;
use novatel_oem6_api::Log::*;
use std::cell::{Cell, RefCell};
use std::io::Error;
use std::sync::mpsc::{sync_channel, Receiver, SyncSender, TrySendError};
use std::thread;
use std::time::Duration;

use objects::*;

pub const RECV_TIMEOUT: Duration = Duration::from_millis(300);

pub fn log_thread(
    oem: OEM6,
    error_send: SyncSender<RxStatusEventLog>,
    version_send: SyncSender<VersionLog>,
) {
    loop {
        match oem.get_log()
            .expect("Underlying read thread no longer communicating")
        {
            BestXYZ(_log) => { /* TODO: Update our position information */ }
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
    pub error_recv: Receiver<RxStatusEventLog>,
    pub version_recv: Receiver<VersionLog>,
}

impl Subsystem {
    pub fn new(bus: &'static str) -> OEMResult<Subsystem> {
        let (log_send, log_recv) = sync_channel(5);
        let (response_send, response_recv) = sync_channel(5);

        let oem = OEM6::new(bus, BaudRate::Baud9600, log_recv, response_recv)?;

        let rx_conn = oem.conn.clone();
        thread::spawn(move || read_thread(rx_conn, log_send, response_send));

        let (error_send, error_recv) = sync_channel(10);
        let (version_send, version_recv) = sync_channel(1);

        let oem_ref = oem.clone();
        thread::spawn(move || log_thread(oem_ref, error_send, version_send));

        println!("Kubos OEM6 service started");

        Ok(Subsystem {
            oem,
            last_cmd: Cell::new(AckCommand::None),
            errors: RefCell::new(vec![]),
            error_recv,
            version_recv,
        })
    }

    pub fn get_test_results(&self) -> Result<IntegrationTestResults, Error> {
        // Send the request for version information
        let result = run!(self.oem.request_version(), self.errors);

        let mut success = result.is_ok();
        let mut errors = match result {
            Ok(_) => "".to_owned(),
            Err(err) => err,
        };

        let version_info = match success {
            true => {
                // Read the response
                let result = self.version_recv.recv_timeout(RECV_TIMEOUT);

                success &= result.is_ok();
                match result {
                    Ok(log) => Some(VersionInfo {
                        num_components: log.num_components as i32,
                        components: log.components
                            .iter()
                            .map(|comp| VersionComponent(comp.clone()))
                            .collect(),
                    }),
                    Err(err) => {
                        errors.push_str(&format!("{}", err));
                        push_err!(
                            self.errors,
                            format!("Self-Test: Failed to receive version info - {}", err)
                                .to_owned()
                        );
                        None
                    }
                }
            }
            false => None,
        };

        Ok(IntegrationTestResults {
            success,
            errors,
            telemetry_debug: version_info,
        })
    }

    pub fn noop(&self) -> Result<GenericResponse, Error> {
        // Send the request for version information
        let result = run!(self.oem.request_version(), self.errors);

        let mut success = result.is_ok();
        let mut errors = match result {
            Ok(_) => "".to_owned(),
            Err(err) => err,
        };

        if success == true {
            // Read the response
            let result = self.version_recv.recv_timeout(RECV_TIMEOUT);

            success &= result.is_ok();
            if let Err(err) = result {
                errors.push_str(&format!("{}", err));
                push_err!(
                    self.errors,
                    format!("Noop: Failed to receive version info - {}", err).to_owned()
                );
            }
        }

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
