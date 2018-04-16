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

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use crc16::*;
use messages::*;
use serial;
use serial_comm::Connection;
use std::io;
use std::io::Cursor;

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
    /// * sec_vec - Secondary vector, when entering Object Track mode,
    ///   or Qbi_cmd[0] when entering Qinertial mode
    /// * pri_axis - Primary pointing axis, or Qbi_cmd[1] when entering Qinertial mode
    /// * sec_axis - Secondary pointing axis, or Qbi_cmd[2] when entering Qinertial mode
    /// * qbi_cmd4 - Qbi_cmd[3] when entering Qinertial mode
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
    /// mai.set_mode(13, 1, -1, -3, 0)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`MAIError`]: enum.MAIError.html
    // TODO: Get good values for examples
    pub fn set_mode(
        &self,
        mode: u8,
        sec_vec: i32,
        pri_axis: i32,
        sec_axis: i32,
        qbi_cmd4: i32,
    ) -> MAIResult<()> {
        let request = SetAcsMode {
            mode,
            sec_vec,
            pri_axis,
            sec_axis,
            qbi_cmd4,
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
    /// * eci_pos_x - X-axis ECI position
    /// * eci_pos_y - Y-axis ECI position
    /// * eci_pos_z - Z-axis ECI position
    /// * eci_vel_x - X-axis ECI velocity
    /// * eci_vel_y - Y-axis ECI velocity
    /// * eci_vel_z - Z-axis ECI velocity
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
    /// mai.set_rv(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1198800018)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`MAIError`]: enum.MAIError.html
    pub fn set_rv(
        &self,
        eci_pos_x: f32,
        eci_pos_y: f32,
        eci_pos_z: f32,
        eci_vel_x: f32,
        eci_vel_y: f32,
        eci_vel_z: f32,
        time_epoch: u32,
    ) -> MAIResult<()> {
        let request = SetRV {
            eci_pos_x,
            eci_pos_y,
            eci_pos_z,
            eci_vel_x,
            eci_vel_y,
            eci_vel_z,
            time_epoch,
            ..Default::default()
        };

        self.send_message(request.serialize())
    }

    /// Request the device configuration information
    ///
    /// *Note: The configuration information should be fetched with a separate `read` call*
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
    /// // Request configuration information
    /// mai.get_info()?;
    ///
    /// // Grab returned config message
    /// loop {
    /// 	match mai.get_message()? {
    ///     	Response::Config(config) => {
    ///         	println!("FW Version: {}.{}.{}", config.major, config.minor, config.build);
    ///         	break;
    ///     	}
    ///     	_ => continue
    /// 	}
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`MAIError`]: enum.MAIError.html
    pub fn get_info(&self) -> MAIResult<()> {
        self.send_message(GetInfo::default().serialize())
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
        let crc = State::<AUG_CCITT>::calculate(&msg[2..]);
        msg.write_u16::<LittleEndian>(crc).unwrap();

        self.conn.write(msg.as_slice())?;
        Ok(())
    }

    /// Wait for and read a message from the MAI-400.
    ///
    /// Returns a `Response` enum containing a received message
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
    /// let msg = mai.get_message()?;
    ///
    /// match msg {
    ///     Response::StdTelem(telem) => {
    ///         println!("Num successful commands: {}", telem.cmd_valid_cntr);
    ///     }
    ///     _ => {}
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`MAIError`]: enum.MAIError.html
    pub fn get_message(&self) -> MAIResult<Response> {

        let response: Response;

        loop {
            let mut msg = self.conn.read()?;

            // Get the CRC bytes
            let len = msg.len();
            let mut raw = msg.split_off(len - 2);
            let mut crc = Cursor::new(raw.to_vec());
            let crc = crc.read_u16::<LittleEndian>()?;

            // Get the calculated CRC
            let calc = State::<AUG_CCITT>::calculate(&msg[1..]);

            // Make sure they match
            // If not, pretend this never happened and go get another message
            if calc != crc {
                continue;
            }

            // Put the CRC bytes back and translate the vector into a useable structure
            msg.append(&mut raw);

            // Identify message type and convert to usable structure
            let id = msg[5];
            match id {
                1 => {
                    let telem = StandardTelemetry::new(&msg[..]);
                    response = Response::StdTelem(telem);
                    break;
                }
                2 => {
                    let irehs = IREHSTelemetry::new(&msg[..]);
                    response = Response::IREHS(irehs);
                    break;
                }
                3 => {
                    let imu = RawIMU::new(&msg[..]);
                    response = Response::IMU(imu);
                    break;
                }
                6 => {
                    let info = ConfigInfo::new(&msg[..]);
                    response = Response::Config(info);
                    break;
                }
                _ => {
                    throw!(MAIError::GenericError);
                }
            }
        }

        Ok(response)

    }
}

/// Common Error for MAI Actions
#[derive(Fail, Display, Debug, Clone, PartialEq)]
pub enum MAIError {
    /// Catch-all error
    #[display(fmt = "Generic Error")]
    GenericError,
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
        MAIError::IoError { cause: format!("{}", error) }
    }
}

impl From<serial::Error> for MAIError {
    fn from(error: serial::Error) -> Self {
        MAIError::SerialError { cause: format!("{}", error) }
    }
}

/// Custom error type for MAI400 operations.
pub type MAIResult<T> = Result<T, MAIError>;
