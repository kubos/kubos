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

use byteorder::{LittleEndian, WriteBytesExt};
use messages::*;
use serial;
use serial_comm::Connection;
use std::io;

/// Structure for MAI-400 device instance
pub struct MAI400 {
    /// Device connection structure
    pub conn: Connection,
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
    /// let connection = Connection::new("/dev/ttyS5".to_owned());
    /// let mai = MAI400::new(connection);
    /// # Ok(())
    /// # }
    /// ```
    ///
    pub fn new(conn: Connection) -> MAI400 {
        MAI400 { conn }
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
    /// # let connection = Connection::new("/dev/ttyS5".to_owned());
    /// let mai = MAI400::new(connection);
    /// mai.reset()?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`MAIError`]: enum.MAIError.html
    // Resetting requires a pair of commands: request, then confirm
    pub fn reset(&self) -> MAIResult<()> {
        let request = RequestReset::default();
        self.send_message(request.serialize())?;

        let request = ConfirmReset::default();
        self.send_message(request.serialize())
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
    /// # let connection = Connection::new("/dev/ttyS5".to_owned());
    /// let mai = MAI400::new(connection);
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

        self.send_message(request.serialize())
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
    /// # let connection = Connection::new("/dev/ttyS5".to_owned());
    /// let mai = MAI400::new(connection);
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

        self.send_message(request.serialize())
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
    /// # let connection = Connection::new("/dev/ttyS5".to_owned());
    /// let mai = MAI400::new(connection);
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

        self.send_message(request.serialize())
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
    /// # let connection = Connection::new("/dev/ttyS5".to_owned());
    /// let mai = MAI400::new(connection);
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

        self.send_message(request.serialize())
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
    /// # let connection = Connection::new("/dev/ttyS5".to_owned());
    /// let mai = MAI400::new(connection);
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
        self.conn.write(msg)
    }

    fn send_message(&self, mut msg: Vec<u8>) -> MAIResult<()> {
        // Get the calculated CRC
        //let calc = State::<AUG_CCITT>::calculate(&msg[1..]);
        let mut crc: u16 = 0;
        for byte in msg.iter() {
            crc += *byte as u16;
        }
        msg.write_u16::<LittleEndian>(crc).unwrap();

        self.conn.write(msg.as_slice())?;
        Ok(())
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
    /// # let connection = Connection::new("/dev/ttyS5".to_owned());
    /// let mai = MAI400::new(connection);
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
    ) -> MAIResult<
        (
            Option<StandardTelemetry>,
            Option<RawIMU>,
            Option<IREHSTelemetry>,
        ),
    > {
        let mut msg = self.conn.read()?;

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
#[derive(Fail, Display, Debug, Clone, PartialEq)]
pub enum MAIError {
    /// Catch-all error
    #[display(fmt = "Generic Error")]
    GenericError,
    /// Software attempted to send a command packet which was not exactly 40 characters long
    #[display(fmt = "Bad Command Length")]
    BadCommandLen,
    /// Received a valid message, but the message ID doesn't match any known message type
    #[display(fmt = "Unknown Message Received: {:X}", id)]
    UnknownMessage {
        /// ID of message received
        id: u16,
    },
    #[display(fmt = "Serial Error: {}", cause)]
    /// An error was thrown by the serial driver
    SerialError {
        /// The underlying error
        cause: String,
    },
    #[display(fmt = "IO Error: {}", cause)]
    /// An I/O error was thrown by the kernel
    IoError {
        /// The underlying error
        cause: String,
    },
}

impl From<io::Error> for MAIError {
    fn from(error: io::Error) -> Self {
        MAIError::IoError {
            cause: format!("{}", error),
        }
    }
}

impl From<serial::Error> for MAIError {
    fn from(error: serial::Error) -> Self {
        MAIError::SerialError {
            cause: format!("{}", error),
        }
    }
}

/// Custom error type for MAI400 operations.
pub type MAIResult<T> = Result<T, MAIError>;
