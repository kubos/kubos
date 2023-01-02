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

//!
//! Wrapping radio uplink/downlink functionality in a way that can be
//! consumed by the communications service library.
//!

use crate::NslDuplexCommsResult;
use kubos_comms::CommsServiceError;
use nsl_duplex_d2::{serial_connection, DuplexD2, File};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

/// Default ping frequency in seconds
pub const DEFAULT_PING_FREQ: u64 = 10;

/// Struct for wrapping around radio interface
pub struct DuplexComms {
    /// Actual radio interface
    pub radio: DuplexD2,
    /// Count of downlinked packets
    /// Used to create unique filenames for duplex downlink queue
    downlink_counter: u16,
    /// Ping Frequency - How often to check and queue up a ping
    /// in the radio's downlink queue (in seconds)
    pub ping_freq: u64,
}

impl DuplexComms {
    pub fn new(path: &str, ping_freq: u64) -> Self {
        let serial_conn = serial_connection(path);
        let radio = DuplexD2::new(serial_conn);

        DuplexComms {
            radio,
            downlink_counter: 0,
            ping_freq,
        }
    }

    pub fn read(&self) -> NslDuplexCommsResult<Vec<u8>> {
        let count = self.radio.get_uploaded_file_count()?;

        if count > 0 {
            let file = self.radio.get_uploaded_file()?;
            Ok(file.body)
        } else {
            bail!(CommsServiceError::NoReadData);
        }
    }

    pub fn write(&mut self, data: &[u8]) -> NslDuplexCommsResult<()> {
        let file_name = format!("udp{:03}", self.downlink_counter);
        self.downlink_counter += 1;
        let file = File::new(&file_name, data);
        match self.radio.put_download_file(&file) {
            Ok(true) => Ok(()),
            Ok(false) => {
                warn!("Failed to downlink file");
                bail!("Failed to downlink file")
            }
            Err(e) => {
                warn!("Duplex write failed {:?}", e);
                bail!(e.to_string())
            }
        }
    }

    pub fn download_ping(&mut self) -> NslDuplexCommsResult<()> {
        let count = self.radio.get_download_file_count()?;
        if count == 0 {
            let file = File::new("ping", &[]);
            self.radio.put_download_file(&file)?;
        }
        Ok(())
    }
}

// The duplex radio must have a file queued up to download in order
// to make contact with the ground. This loop is used to ensure
// there is always at least one generic file queued up.
pub fn ping_loop(radio: Arc<Mutex<DuplexComms>>) -> NslDuplexCommsResult<()> {
    let ping_freq = radio
        .lock()
        .map(|radio| radio.ping_freq)
        .unwrap_or(DEFAULT_PING_FREQ);
    loop {
        if let Ok(mut radio) = radio.lock() {
            match radio.download_ping() {
                Ok(_) => {}
                Err(e) => {
                    warn!("Failed to send ping to radio {}", e);
                }
            }
        }
        thread::sleep(Duration::from_secs(ping_freq));
    }
}

// Read wrapper used by comms service
pub fn read(radio: &Arc<Mutex<DuplexComms>>) -> NslDuplexCommsResult<Vec<u8>> {
    if let Ok(radio) = radio.lock() {
        radio.read()
    } else {
        warn!("Failed to lock radio");
        panic!("Failed to lock radio");
    }
}

// Write wrapper used by comms service
pub fn write(radio: &Arc<Mutex<DuplexComms>>, data: &[u8]) -> NslDuplexCommsResult<()> {
    if let Ok(mut radio) = radio.lock() {
        radio.write(data)
    } else {
        warn!("Failed to lock radio");
        panic!("Failed to lock radio");
    }
}
