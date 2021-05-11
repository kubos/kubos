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

use crate::messages::*;
use byteorder::{LittleEndian, WriteBytesExt};
use failure::Fail;
use rust_uart::UartError;
use rust_uart::*;
use serial;
use std::sync::{Arc, Mutex};
use std::time::Duration;

const TIMEOUT: Duration = Duration::from_millis(60);

/// Structure for MAI-400 device instance
#[derive(Clone)]
pub struct MAI400 {
    /// Device connection structure
    pub conn: Arc<Mutex<Connection>>,
}

impl MAI400 {
    /// Constructor for MAI400 structure
    ///
    /// # Arguments
    ///
    /// * conn - The underlying connection stream to use for communication with the device
    ///
    /// # Examples
    ///
    /// ```
    /// use mai400_api::*;
    ///
    /// # fn func() -> MAIResult<()> {
    /// let mai = MAI400::new("/dev/ttyS5")?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    pub fn new(bus: &str) -> MAIResult<MAI400> {
        let settings = serial::PortSettings {
            baud_rate: serial::Baud115200,
            char_size: serial::Bits8,
            parity: serial::ParityNone,
            stop_bits: serial::Stop1,
            flow_control: serial::FlowNone,
        };

        let conn = Connection::from_path(bus, settings, TIMEOUT)?;

        Ok(MAI400 {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    /// Request a hardware reset of the MAI-400
    ///
    /// # Errors
    ///
    /// If this function encounters any errors, an [`MAIError`] variant will be returned.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mai400_api::*;
    /// # fn func() -> MAIResult<()> {
    /// let mai = MAI400::new("/dev/ttyS5")?;
    /// mai.reset()?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`MAIError`]: enum.MAIError.html
    // Resetting requires a pair of commands: request, then confirm
    pub fn reset(&self) -> MAIResult<()> {
        let request = RequestReset::default();
        self.send_message(&request)?;

        let request = ConfirmReset::default();
        self.send_message(&request)
    }

    /// Set the ACS mode
    ///
    /// # Arguments
    ///
    /// *Note: Arguments should be set to `0x00` when not needed for desired mode*
    ///
    /// * mode - ACS mode to enter
    /// * params - Array of signed shorts containing the arguments for configuring the requested mode
    ///
    /// # Errors
    ///
    /// If this function encounters any errors, an [`MAIError`] variant will be returned.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mai400_api::*;
    /// # fn func() -> MAIResult<()> {
    /// let mai = MAI400::new("/dev/ttyS5")?;
    /// mai.set_mode(9, [1, -1, -3, 0])?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`MAIError`]: enum.MAIError.html
    // TODO: Get good values for examples
    pub fn set_mode(&self, mode: u8, params: [i16; 4]) -> MAIResult<()> {
        let request = SetAcsMode {
            mode,
            qbi_cmd: params,
            ..Default::default()
        };

        self.send_message(&request)
    }

    /// Set the ACS mode (Normal-Sun or Lat/Long-Sun)
    ///
    /// # Arguments
    ///
    /// *Note: Arguments should be set to `0x00` when not needed for desired mode*
    ///
    /// * mode - ACS mode to enter. Should be either Normal-Sun or Lat/Long-Sun
    /// * sun_angle_enable - Sun angle enable (0 = no update, 1 = update)
    /// * sun_rot_angle - Sun rotation angle, in degrees
    ///
    /// # Errors
    ///
    /// If this function encounters any errors, an [`MAIError`] variant will be returned.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mai400_api::*;
    /// # fn func() -> MAIResult<()> {
    /// let mai = MAI400::new("/dev/ttyS5")?;
    /// mai.set_mode_sun(7, 1, 45.0)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`MAIError`]: enum.MAIError.html
    // TODO: Get good values for examples
    pub fn set_mode_sun(
        &self,
        mode: u8,
        sun_angle_enable: i16,
        sun_rot_angle: f32,
    ) -> MAIResult<()> {
        let request = SetAcsModeSun {
            mode,
            sun_angle_enable,
            sun_rot_angle,
            ..Default::default()
        };

        self.send_message(&request)
    }

    /// Set the ADACS clock with the desired GPS time
    ///
    /// # Arguments
    ///
    /// * gps_time - New clock time (elapsed seconds after Jan 6, 1980 00:00:00)
    ///
    /// # Errors
    ///
    /// If this function encounters any errors, an [`MAIError`] variant will be returned.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mai400_api::*;
    /// # fn func() -> MAIResult<()> {
    /// let mai = MAI400::new("/dev/ttyS5")?;
    /// // Jan 01, 2018
    /// mai.set_gps_time(1198800018)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`MAIError`]: enum.MAIError.html
    pub fn set_gps_time(&self, gps_time: u32) -> MAIResult<()> {
        let request = SetGPSTime {
            gps_time,
            ..Default::default()
        };

        self.send_message(&request)
    }

    /// Set orbital position and velocity at epoch for RK4 integration method of orbit propagation
    ///
    /// # Arguments
    ///
    /// * eci_pos - ECI position [X, Y, Z]
    /// * eci_vel - ECI velocity [X, Y, Z]
    /// * time_epoch - GPS time at Epoch
    ///
    /// # Errors
    ///
    /// If this function encounters any errors, an [`MAIError`] variant will be returned.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mai400_api::*;
    /// # fn func() -> MAIResult<()> {
    /// let mai = MAI400::new("/dev/ttyS5")?;
    /// mai.set_rv([0.0, 0.0, 0.0], [0.0, 0.0, 0.0], 1198800018)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`MAIError`]: enum.MAIError.html
    pub fn set_rv(&self, eci_pos: [f32; 3], eci_vel: [f32; 3], time_epoch: u32) -> MAIResult<()> {
        let request = SetRV {
            eci_pos,
            eci_vel,
            time_epoch,
            ..Default::default()
        };

        self.send_message(&request)
    }

    /// Directly send a message without formatting or checksum calculation
    ///
    /// # Arguments
    ///
    /// * msg - Message to send
    ///
    /// # Errors
    ///
    /// If this function encounters any errors, an [`MAIError`] variant will be returned.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mai400_api::*;
    /// # fn func() -> MAIResult<()> {
    /// let mai = MAI400::new("/dev/ttyS5")?;
    ///
    /// let mut array = [0; 8];
    /// array[0] = 0x90; // SYNC byte 1
    /// array[1] = 0xEB; // SYNC byte 2
    /// array[2] = 0x0;  // Data_len byte 1
    /// array[3] = 0x0;  // Data_len byte 2
    /// array[4] = 0x5A; // Msg_id
    /// array[5] = 0x0;  // Addr
    /// array[6] = 0x00; // CRC byte 1
    /// array[7] = 0x00; // CRC byte 2
    ///
    /// mai.passthrough(&array)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`MAIError`]: enum.MAIError.html
    pub fn passthrough(&self, msg: &[u8]) -> MAIResult<()> {
        // If the mutex has been poisoned by the read thread,
        // go ahead and attempt the write anyways, to try to
        // preserve functionality, but inform the caller afterwards
        match self.conn.lock() {
            Ok(conn) => conn.write(msg).map_err(|err| err.into()),
            Err(conn) => conn
                .into_inner()
                .write(msg)
                .map_err(|err| err.into())
                .and(Err(MAIError::ThreadCommError)),
        }
    }

    fn send_message<T: Message>(&self, msg: &T) -> MAIResult<()> {
        let mut raw = msg.serialize();

        // Get the calculated CRC
        let mut crc: u16 = 0;
        for byte in raw.iter() {
            crc += u16::from(*byte);
        }
        raw.write_u16::<LittleEndian>(crc).unwrap();

        // If the mutex has been poisoned by the read thread,
        // go ahead and attempt the write anyways, to try to
        // preserve functionality, but inform the caller afterwards
        match self.conn.lock() {
            Ok(conn) => conn.write(raw.as_slice()).map_err(|err| err.into()),
            Err(conn) => conn
                .into_inner()
                .write(raw.as_slice())
                .map_err(|err| err.into())
                .and(Err(MAIError::ThreadCommError)),
        }
    }

    /// Wait for and read a message set from the MAI-400.
    ///
    /// Returns a tuple potentially containing a standard telemetry message, a raw IMU telemetry message,
    /// and an IREHS telemetry packet.
    ///
    /// *Note*: Messages are sent every 250ms, so, to the human eye, this should
    /// appear to be instantaneous
    ///
    /// # Errors
    ///
    /// If this function encounters any errors, an [`MAIError`] variant will be returned.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mai400_api::*;
    /// # fn func() -> MAIResult<()> {
    /// let mai = MAI400::new("/dev/ttyS5")?;
    /// let (std, imu, irehs) = mai.get_message()?;
    ///
    /// if let Some(telem) = std {
    ///     println!("Num successful commands: {}", telem.cmd_valid_cntr);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`MAIError`]: enum.MAIError.html
    pub fn get_message(
        &self,
    ) -> MAIResult<(
        Option<StandardTelemetry>,
        Option<RawIMU>,
        Option<IREHSTelemetry>,
    )> {
        let mut msg = vec![];
        loop {
            {
                // Take the stream connection mutex
                // If the lock() call fails, it means that a different thread poisoned
                // the mutex. We want to maintain our ability to read messages from the
                // device for as long as possible, so we'll go ahead and just ignore the
                // poisoned status. Ideally, the master thread will have detected whatever
                // error caused the problem and will take error handling measures.
                let conn = self.conn.lock().unwrap_or_else(|err| err.into_inner());

                // Read SYNC bytes
                let mut sync = match conn.read(2, Duration::from_millis(250)) {
                    Ok(v) => v,
                    Err(err) => match err {
                        #[cfg(test)]
                        UartError::GenericError => return Err(MAIError::GenericError),
                        UartError::IoError {
                            cause: ::std::io::ErrorKind::TimedOut,
                            ..
                        } => continue,
                        _ => panic!("{}", err),
                    },
                };

                if sync != SYNC {
                    continue;
                }

                msg.append(&mut sync);

                // Read the rest of the message
                let mut body = match conn.read(236, TIMEOUT) {
                    Ok(v) => v,
                    Err(err) => match err {
                        UartError::IoError {
                            cause: ::std::io::ErrorKind::TimedOut,
                            ..
                        } => continue,
                        _ => panic!("{}", err),
                    },
                };

                msg.append(&mut body);
                break;
            }
        }

        // Pull out raw IMU message
        let len = msg.len();
        let imu = msg.split_off(len - 21);
        let imu = RawIMU::new(imu);

        // Pull out IREHS telemetry message
        let len = msg.len();
        let irehs = msg.split_off(len - 56);
        let irehs = IREHSTelemetry::new(irehs);

        // Process remaining bytes as standard telemetry message
        let std = StandardTelemetry::new(msg);

        Ok((std, imu, irehs))
    }
}

/// Common Error for MAI Actions
#[derive(Fail, Debug, Clone, PartialEq)]
pub enum MAIError {
    /// Catch-all error
    #[fail(display = "Generic Error")]
    GenericError,
    /// The thread reading messages from the device is no longer working
    #[fail(display = "Failed to communicate with read thread")]
    ThreadCommError,
    /// Received a valid message, but the message ID doesn't match any known message type
    #[fail(display = "Unknown Message Received: {:X}", id)]
    UnknownMessage {
        /// ID of message received
        id: u16,
    },
    /// An error was thrown by the serial communication driver
    #[fail(display = "UART Error")]
    UartError {
        /// The underlying error
        #[fail(cause)]
        cause: UartError,
    },
}

impl From<UartError> for MAIError {
    fn from(error: UartError) -> Self {
        MAIError::UartError { cause: error }
    }
}

/// Custom error type for MAI400 operations.
pub type MAIResult<T> = Result<T, MAIError>;
