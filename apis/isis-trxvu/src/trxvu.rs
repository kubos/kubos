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

    /// Helper function for requesting telemetry
    fn get_telemetry(&self, telem_type: radio_telem_type) -> RadioResult<TelemRaw> {
        let mut telem: TelemRaw = unsafe { mem::uninitialized() };
        radio_status_to_err(unsafe { k_radio_get_telemetry(&mut telem, telem_type) })?;
        Ok(telem)
    }

    /// Returns the current measurements of all the transmitter's telemetry channels
    pub fn current_transmitter_telemetry(&self) -> RadioResult<TxTelemetry> {
        let telem = self.get_telemetry(radio_telem_type::TxTelemAll)?;
        Ok(unsafe { TxTelemetry::parse(&telem.tx_telem_raw) })
    }

    /// Returns the telemetry channels that were sampled during the last frame transmission
    pub fn last_transmitter_telemetry(&self) -> RadioResult<TxTelemetry> {
        let telem = self.get_telemetry(radio_telem_type::TxTelemLast)?;
        Ok(unsafe { TxTelemetry::parse(&telem.tx_telem_raw) })
    }

    /// Returns the amount of time, in seconds, that the transmitter portion of the radio has been active
    pub fn transmitter_uptime(&self) -> RadioResult<u32> {
        let telem = self.get_telemetry(radio_telem_type::TxUptime)?;
        Ok(unsafe { telem.uptime })
    }

    /// Returns the current state of the transmitter
    pub fn transmitter_state(&self) -> RadioResult<TxState> {
        let telem = self.get_telemetry(radio_telem_type::TxState)?;
        Ok(unsafe { TxState::parse(telem.tx_state) })
    }

    /// Returns the current measurements of all the receiver's telemetry channels
    pub fn receiver_telemetry(&self) -> RadioResult<RxTelemetry> {
        let telem = self.get_telemetry(radio_telem_type::RxTelemAll)?;
        Ok(unsafe { RxTelemetry::parse(&telem.rx_telem_raw) })
    }

    /// Returns the amount of time, in seconds, that the receiver portion of the radio has been active
    pub fn receiver_uptime(&self) -> RadioResult<u32> {
        let telem = self.get_telemetry(radio_telem_type::RxUptime)?;
        Ok(unsafe { telem.uptime })
    }

    /// Send a message to the radio's transmit buffer
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

    /// Attemps to read a message from the radio's receive buffer
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
