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

use ffi::*;

#[derive(Fail, Display, Debug)]
pub enum AdcsError {
    #[display(fmt = "Generic error")] Generic,
    #[display(fmt = "Configuration error")] Config,
    #[display(fmt = "No response from ADCS")] NoResponse,
    #[display(fmt = "Internal ADCS error")] Internal,
    #[display(fmt = "ADCS mutex error")] Mutex,
    #[display(fmt = "Not implemented error")] NotImplemented,
}

pub type AdcsResult<T> = Result<T, AdcsError>;

/// Structure for interacting with the ISIS Imtq
pub struct Imtq<T: ImtqFFI> {
    handle: T,
}

impl Imtq<ImtqRaw> {
    /// Constructor - Returns an `AdcsResult<Imtq>`
    ///
    /// Opens a connection to the underlying Imtq device.
    ///
    /// # Example
    /// ```
    /// use isis_imtq_api::*;
    /// let imtq = Imtq::imtq().unwrap();
    /// ```
    pub fn imtq() -> AdcsResult<Self> {
        let handle = ImtqRaw {};
        Imtq::new(&handle)
    }
}

impl<T: ImtqFFI> Imtq<T> {
    /// Private Constructor - returns `AdcsResult<Imtq>`
    /// Used by Imtq::imtq and tests to inject
    /// appropriate ImtqFFI object.
    ///
    /// The one argument *must* implement the `ImtqFFI` trait.
    fn new(handle: &T) -> AdcsResult<Self> {
        adcs_status_to_err(handle.k_adcs_init())?;
        Ok(Imtq {
            handle: handle.clone(),
        })
    }

    /// Passes a command directly to the Imtq device and returns back the response
    /// Useful for executing commands which have not been implemented in the API
    ///
    /// # Arguments
    ///
    /// * `command` - A string slice containing the command to be sent
    /// * `rx_len` - Expected length of command response
    /// * `delay_secs` - Delay between sending command and requesting response (seconds)
    /// * `delay_nsecs` - Delay between sending command and requesting response (nano seconds)
    ///
    /// # Example
    /// ```
    /// use isis_imtq_api::*;
    /// let imtq = Imtq::imtq().unwrap();
    /// let cmd = vec![10, 10, 10, 10];
    /// let result = imtq.passthrough(&cmd, 10, 0, 0).unwrap();
    /// ```
    pub fn passthrough(
        &self,
        command: &[u8],
        rx_len: i32,
        delay_secs: i32,
        delay_nsecs: i64,
    ) -> AdcsResult<Vec<u8>> {
        let mut rx_buffer = vec![0; rx_len as usize];
        let tspec = timespec {
            tv_sec: delay_secs,
            tv_nsec: delay_nsecs,
        };

        adcs_status_to_err(self.handle.k_adcs_passthrough(
            command.as_ptr(),
            command.len() as i32,
            rx_buffer.as_mut_ptr(),
            rx_len,
            &tspec,
        ))?;

        Ok(rx_buffer)
    }
}

impl<T: ImtqFFI> Drop for Imtq<T> {
    fn drop(&mut self) {
        self.handle.k_adcs_terminate();
    }
}

#[cfg(test)]
mod tests {
    use double;
    use super::*;

    mock_trait!(
        MockImtq,
        k_adcs_init() -> KADCSStatus,
        k_adcs_terminate() -> (),
        k_adcs_passthrough(*const u8, i32, *mut u8, i32, *const timespec) -> KADCSStatus
    );

    impl ImtqFFI for MockImtq {
        mock_method!(k_adcs_init(&self) -> KADCSStatus);
        mock_method!(k_adcs_terminate(&self));
        mock_method!(k_adcs_passthrough(&self, tx: *const u8,
        len: i32,
        rx: *mut u8,
        rx_len: i32,
        delay: *const timespec) -> KADCSStatus);
    }

    #[test]
    fn test_new_good() {
        let mock = MockImtq::default();
        mock.k_adcs_init.return_value(KADCSStatus::Ok);

        let imtq = Imtq::new(&mock);
        assert!(imtq.is_ok());
    }

    #[test]
    fn test_new_err() {
        let mock = MockImtq::default();
        mock.k_adcs_init.return_value(KADCSStatus::Error);

        let imtq = Imtq::new(&mock);
        assert!(imtq.is_err());
    }

    #[test]
    fn test_terminate_on_drop() {
        let mock = MockImtq::default();

        let imtq = Imtq::new(&mock);
        drop(imtq);
        assert_eq!(1, mock.k_adcs_terminate.num_calls());
    }

    #[test]
    fn test_passthrough() {
        let mock = MockImtq::default();
        let mock_result = vec![1, 1, 2, 3];
        mock.k_adcs_passthrough.use_closure(Box::new(
            |(_tx, tx_len, rx, _rx_len, _delay): (
                *const u8,
                i32,
                *mut u8,
                i32,
                *const timespec,
            )| {
                assert_eq!(4, tx_len);
                unsafe {
                    *rx = 1;
                    *rx.offset(1) = 1;
                    *rx.offset(2) = 2;
                    *rx.offset(3) = 3;
                }
                KADCSStatus::Ok
            },
        ));
        let imtq = Imtq::new(&mock).unwrap();

        let cmd = vec![0, 1, 1, 1];
        let result = imtq.passthrough(&cmd, 4, 0, 100);
        assert!(result.is_ok());
        assert_eq!(mock_result, result.unwrap());
    }
}
