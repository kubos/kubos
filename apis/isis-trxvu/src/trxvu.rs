/*
 * Copyright (C) 2018 Kubos Corporation
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use std::mem;

use radio_api::RadioResult;
use messages::{RxTelemetry, TxState, TxTelemetry};
use ffi::*;

/// Structure for interacting with the TRXVU Radio API
pub struct Trxvu {}

impl Trxvu {
    /// Constructor
    pub fn new() -> RadioResult<Trxvu> {
        unsafe {
            radio_status_to_err(k_radio_init())?;
            radio_status_to_err(k_radio_watchdog_start())?;
        };
        Ok(Trxvu {})
    }

    fn get_telemetry(&self, telem_type: radio_telem_type) -> RadioResult<TelemRaw> {
        let mut telem: TelemRaw = unsafe { mem::uninitialized() };
        radio_status_to_err(unsafe { k_radio_get_telemetry(&mut telem, telem_type) })?;
        Ok(telem)
    }

    /// Retrieves the tx telemetry
    pub fn get_tx_telemetry(&self) -> RadioResult<TxTelemetry> {
        let telem = self.get_telemetry(radio_telem_type::TxTelemAll)?;
        Ok(unsafe { TxTelemetry::parse(&telem.tx_telem_raw) })
    }

    pub fn get_tx_uptime(&self) -> RadioResult<u32> {
        let telem = self.get_telemetry(radio_telem_type::TxUptime)?;
        Ok(unsafe { telem.uptime })
    }

    pub fn get_tx_state(&self) -> RadioResult<TxState> {
        let telem = self.get_telemetry(radio_telem_type::TxState)?;
        Ok(unsafe { TxState::parse(telem.tx_state) })
    }

    /// Retrieves the rx telemetry
    pub fn get_rx_telemetry(&self) -> RadioResult<RxTelemetry> {
        let telem = self.get_telemetry(radio_telem_type::RxTelemAll)?;
        Ok(unsafe { RxTelemetry::parse(&telem.rx_telem_raw) })
    }

    pub fn get_rx_uptime(&self) -> RadioResult<u32> {
        let telem = self.get_telemetry(radio_telem_type::RxUptime)?;
        Ok(unsafe { telem.uptime })
    }

    pub fn send(&self, message: &[u8]) -> RadioResult<()> {
        let mut response: u8 = 0;
        unsafe {
            radio_status_to_err(k_radio_send(
                message.as_ptr(),
                message.len() as i32,
                &mut response,
            ))?;
        };
        Ok(())
    }

    pub fn read(&self) -> RadioResult<Vec<u8>> {
        let mut response: Vec<u8> = Vec::new();
        let mut rx_msg: radio_rx_message = unsafe { mem::uninitialized() };
        let mut len: u8 = 0;
        unsafe {
            radio_status_to_err(k_radio_recv(&mut rx_msg, &mut len))?;
        };
        let end = len as usize;
        response.extend_from_slice(&rx_msg.message[0..end]);
        Ok(response)
    }
}

impl Drop for Trxvu {
    fn drop(&mut self) {
        unsafe {
            k_radio_watchdog_stop();
            k_radio_terminate();
        }
    }
}
