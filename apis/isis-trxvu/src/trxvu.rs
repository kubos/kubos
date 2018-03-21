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
pub struct Trxvu {
    handle: Box<TrxvuFFI>,
}

/// Helper for generating real radio connection
pub fn trxvu_raw() -> Box<TrxvuFFI> {
    Box::new(TrxvuRaw {})
}

impl Trxvu {
    /// Constructor
    pub fn new(handle: Box<TrxvuFFI>) -> RadioResult<Trxvu> {
        radio_status_to_err(handle.k_radio_init())?;
        radio_status_to_err(handle.k_radio_watchdog_start())?;
        Ok(Trxvu { handle })
    }

    /// Helper function for requesting telemetry
    fn get_telemetry(&self, telem_type: radio_telem_type) -> RadioResult<TelemRaw> {
        let mut telem: TelemRaw = Default::default();
        radio_status_to_err(self.handle.k_radio_get_telemetry(&mut telem, telem_type))?;
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
        radio_status_to_err(self.handle.k_radio_send(
            message.as_ptr(),
            message.len() as i32,
            &mut response,
        ))?;
        Ok(())
    }

    /// Attemps to read a message from the radio's receive buffer
    pub fn read(&self) -> RadioResult<Vec<u8>> {
        let mut response: Vec<u8> = Vec::new();
        let mut rx_msg: radio_rx_message = Default::default();
        let mut len: u8 = 0;
        radio_status_to_err(self.handle.k_radio_recv(&mut rx_msg, &mut len))?;
        let end = rx_msg.msg_size as usize;
        response.extend_from_slice(&rx_msg.message[0..end]);
        Ok(response)
    }
}

impl Drop for Trxvu {
    fn drop(&mut self) {
        self.handle.k_radio_watchdog_stop();
        self.handle.k_radio_terminate();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use double;
    use ffi;
    use radio_api::RadioError;

    mock_trait!(
        MockTrxvu,
        k_radio_init() -> radio_status,
        k_radio_watchdog_start() -> radio_status,
        k_radio_watchdog_stop() -> radio_status,
        k_radio_terminate() -> (),
        k_radio_get_telemetry(*mut TelemRaw, radio_telem_type) -> radio_status,
        k_radio_send(*const u8, i32, *mut u8) -> radio_status,
        k_radio_recv(*mut radio_rx_message, *mut u8) -> radio_status
    );

    impl TrxvuFFI for MockTrxvu {
        mock_method!(k_radio_init(&self) -> radio_status);
        mock_method!(k_radio_watchdog_start(&self) -> radio_status);
        mock_method!(k_radio_watchdog_stop(&self) -> radio_status);
        mock_method!(k_radio_terminate(&self));
        mock_method!(k_radio_get_telemetry(
            &self,
            buffer: *mut TelemRaw,
            telem_type: radio_telem_type) -> radio_status);
        mock_method!(k_radio_send(&self,
            buffer: *const u8, len: i32, response: *mut u8) -> radio_status);
        mock_method!(k_radio_recv(&self,
            buffer: *mut radio_rx_message, len: *mut u8) -> radio_status);
    }

    #[test]
    fn test_send_good() {
        let mock = Box::new(MockTrxvu::default());
        mock.k_radio_send.return_value(ffi::radio_status::RadioOk);

        let radio = Trxvu::new(mock).unwrap();
        let message = vec![0, 1, 2, 3, 4];

        assert_eq!((), radio.send(&message).unwrap());
    }

    #[test]
    fn test_send_get_error() {
        let mock = Box::new(MockTrxvu::default());
        mock.k_radio_send
            .return_value(ffi::radio_status::RadioError);

        let radio = Trxvu::new(mock).unwrap();
        let message = vec![0, 1, 2, 3, 4];

        let err = radio.send(&message).unwrap_err();

        assert_eq!(
            format!("{:?}", err),
            format!(
                "{:?}",
                RadioError::HardwareError {
                    message: "TRXVU radio error RadioError".to_string(),
                }
            )
        );
    }

    #[test]
    fn test_receive_empty() {
        let mock = Box::new(MockTrxvu::default());

        let radio = Trxvu::new(mock).unwrap();
        let resp = radio.read();
        assert!(resp.is_ok());
        assert_eq!(resp.unwrap(), vec![]);
    }

    #[test]
    fn test_receive_good_data() {
        fn recv((buffer, len): (*mut ffi::radio_rx_message, *mut u8)) -> ffi::radio_status {
            unsafe {
                *len = 10;
                (*buffer).msg_size = 4;
                (*buffer).message[0] = 0;
                (*buffer).message[1] = 1;
                (*buffer).message[2] = 2;
                (*buffer).message[3] = 3;
            }
            ffi::radio_status::RadioOk
        }

        let mock = MockTrxvu::default();
        mock.k_radio_recv.use_fn(recv);
        let radio = Trxvu::new(Box::new(mock)).unwrap();
        let resp = radio.read();
        assert!(resp.is_ok());
        assert_eq!(resp.unwrap(), vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_receive_empty_err() {
        let mock = MockTrxvu::default();
        mock.k_radio_recv
            .return_value(ffi::radio_status::RadioRxEmpty);
        let radio = Trxvu::new(Box::new(mock)).unwrap();
        let resp = radio.read();
        assert!(resp.is_ok());
        assert_eq!(resp.unwrap(), vec![]);
    }

    #[test]
    fn test_receive_err() {
        let mock = MockTrxvu::default();
        mock.k_radio_recv
            .return_value(ffi::radio_status::RadioError);
        let radio = Trxvu::new(Box::new(mock)).unwrap();
        let resp = radio.read();
        assert!(resp.is_err());
        assert_eq!(
            format!("{:?}", resp.unwrap_err()),
            format!(
                "{:?}",
                RadioError::HardwareError {
                    message: "TRXVU radio error RadioError".to_string(),
                }
            )
        );
    }

    #[test]
    fn test_receiver_uptime() {
        fn get_telem(
            (buffer, telem_type): (*mut ffi::TelemRaw, ffi::radio_telem_type),
        ) -> ffi::radio_status {
            unsafe {
                (*buffer).uptime = 100;
            }
            ffi::radio_status::RadioOk
        }

        let mock = MockTrxvu::default();
        mock.k_radio_get_telemetry.use_fn(get_telem);
        let radio = Trxvu::new(Box::new(mock)).unwrap();
        let resp = radio.receiver_uptime();
        assert!(resp.is_ok());
        assert_eq!(resp.unwrap(), 100);
    }
}
