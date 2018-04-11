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

/// Structure to contain all possible variables which can be returned
/// by the standard telemetry message's `rotating_variable` fields
#[derive(Default)]
pub struct RotatingTelemetry {
    /// IGRF magnetic fields [X, Y, Z] (Tesla)
    b_field_igrf: [f32; 3],
    /// ECI Sun Vector from Ephemeris [X, Y, Z] (Unit)
    sun_vec_eph: [f32; 3],
    /// ECI Spacecraft Position [X, Y, Z] (km)
    sc_pos_eci: [f32; 3],
    /// ECI Spacecraft Velocity [X, Y, Z] (km)
    sc_vel_eci: [f32; 3],
    // TODO: These are all actually different fields...
    kepler_elem: [f32; 6],
    /// Bdot Gain Acquisition Mode [X, Y, Z]
    k_bdot: [f32; 3],
    /// Proportional Gain Normal Mode [X, Y, Z]
    kp: [f32; 3],
    /// Derivative Gain Normal Mode [X, Y, Z]
    kd: [f32; 3],
    /// Unloading Gain Normal Mode [X, Y, Z]
    k_unload: [f32; 3],
    /// CSS{n} Bias [1, 2, 3, 4, 5, 6]
    css_bias: [i16; 6],
    /// MAG Bias [X, Y, Z]
    mag_bias: [i16; 3],
    /// RWS Bus Voltage (0.00483516483 v/lsb)
    rws_volt: i16,
    /// Reserved
    rws_press: i16,
    /// Attitude Determination Mode
    att_det_mode: u8, //TODO: enum
    /// RWS Reset Counter [X, Y, Z]
    rws_reset_cntr: [u8; 3],
    /// Sun and Mag Field are aligned
    sun_mag_aligned: u8, //TODO: bool
    /// Software Minor Version
    minor_version: u8,
    /// Software Unit Serial Number
    mai_sn: u8,
    /// Orbit Propagation Mode
    orbit_prop_mode: u8,
    /// ACS Mode in Operation
    acs_op_mode: u8,
    /// ADACS Processor Reset Counter
    proc_reset_cntr: u8,
    /// Software Major Version
    major_version: u8,
    /// ADS Mode in Operation
    ads_op_mode: u8, //TODO: enum
    /// CSS{n} Gain [1, 2, 3, 4, 5, 6]
    css_gain: [f32; 6],
    /// Mag Gain [X, Y, Z]
    mag_gain: [f32; 3],
    /// Epoch of Current Orbit (GPS sec)
    orbit_epoch: u32,
    /// True Anomaly at Epoch – Kepler (deg)
    true_anomoly_epoch: f32,
    /// Epoch of Next Updated RV (GPS sec)
    orbit_epoch_next: u32,
    /// ECI Position at Next Epoch [X, Y, Z] (km)
    sc_pos_eci_epoch: [f32; 3],
    /// ECI Velocity at Next Epoch [X, Y, Z] (km/sec)
    sc_vel_eci_epoch: [f32; 3],
    /// QbX Wheel Speed Command (rpm)
    qb_x_wheel_speed: i16,
    /// QbX Filter Gain
    qb_x_filter_gain: f32,
    /// QbX Dipole Gain
    qb_x_dipole_gain: f32,
    /// Dipole Gain [X, Y, Z]
    dipole_gain: [f32; 3],
    /// Wheel Speed Bias [X, Y, Z] (rpm)
    wheel_speed_bias: [i16; 3],
    /// Cosine of Sun/Mag Align Threshold Angle
    cos_sun_mag_align_thresh: f32,
    /// Max AngleToGo for Unloading (rad)
    unload_ang_thresh: f32,
    /// Quaternion feedback saturation.
    q_sat: f32,
    /// Maximum RWA Torque (mNm)
    raw_trq_max: f32,
    /// Reaction Wheel Motor Current [X, Y, Z] (A) (0.0003663003663 A/lsb)
    rws_motor_current: [u16; 3],
    /// RWS Motor Temperature (Temperature oC = rwsMotorTemp * 0.0402930 - 50)
    raw_motor_temp: i16,
}

/// Structure for MAI-400 device instance
pub struct MAI400 {
    /// Device connection structure
    pub conn: Connection,
}

impl MAI400 {
    /// Constructor for MAI400 structure
    pub fn new(conn: Connection) -> MAI400 {
        MAI400 { conn }
    }

    /// Request a hardware reset of the MAI-400
    // Resetting requires a pair of commands: request, then confirm
    pub fn reset(&self) -> MAIResult<()> {
        let request = RequestReset::default();
        self.send_message(request.serialize())?;

        let request = ConfirmReset::default();
        self.send_message(request.serialize())
    }

    /// Set the ACS mode
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
    pub fn set_gps_time(&self, gps_time: u32) -> MAIResult<()> {
        let request = SetGPSTime {
            gps_time,
            ..Default::default()
        };

        self.send_message(request.serialize())
    }

    /// Set orbital position and velocity at epoch for RK4 integration method of orbit propagation
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

    /// Directly send a message without formatting or checksum calculation
    pub fn passthrough(&self, msg: &[u8]) -> MAIResult<()> {
        self.conn.write(msg)
    }

    fn send_message(&self, mut msg: Vec<u8>) -> MAIResult<()> {
        let crc = State::<AUG_CCITT>::calculate(&msg[2..]);
        msg.write_u16::<LittleEndian>(crc).unwrap();

        self.conn.write(msg.as_slice())?;
        Ok(())
    }

    /// Wait for and read a message from the MAI-400
    /// Note: Messages are sent every 250ms, so, to the human eye, this should
    /// appear to be instantaneous
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
            // TODO: Add remaining messages
            let id = msg[5];
            match id {
                1 => {
                    let telem = StandardTelemetry::new(&msg[..]);
                    response = Response::StdTelem(telem);
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

    /// Extract the rotating variables from a standard telemetry message and update
    /// the appropriate corresponding fields in a `RotatingTelemetry` structure
    /// TODO: structure reference
    //TODO: verify the bit shifting
    // TODO: Doc says 3 MSB are used for version information. Need to extract
    pub fn update_rotating(&self, msg: &StandardTelemetry, rotating: &mut RotatingTelemetry) {
        match msg.tlm_counter {
            0 => {
                rotating.b_field_igrf[0] = msg.rotating_variable_a as f32;
                rotating.b_field_igrf[1] = msg.rotating_variable_b as f32;
                rotating.b_field_igrf[2] = msg.rotating_variable_c as f32;
            }
            1 => {
                rotating.sun_vec_eph[0] = msg.rotating_variable_a as f32;
                rotating.sun_vec_eph[1] = msg.rotating_variable_b as f32;
                rotating.sun_vec_eph[2] = msg.rotating_variable_c as f32;
            }
            2 => {
                rotating.sc_pos_eci[0] = msg.rotating_variable_a as f32;
                rotating.sc_pos_eci[1] = msg.rotating_variable_b as f32;
                rotating.sc_pos_eci[2] = msg.rotating_variable_c as f32;
            }
            3 => {
                rotating.sc_vel_eci[0] = msg.rotating_variable_a as f32;
                rotating.sc_vel_eci[1] = msg.rotating_variable_b as f32;
                rotating.sc_vel_eci[2] = msg.rotating_variable_c as f32;
            }
            4 => {
                rotating.kepler_elem[0] = msg.rotating_variable_a as f32;
                rotating.kepler_elem[1] = msg.rotating_variable_b as f32;
                rotating.kepler_elem[2] = msg.rotating_variable_c as f32;
            }
            5 => {
                rotating.kepler_elem[3] = msg.rotating_variable_a as f32;
                rotating.kepler_elem[4] = msg.rotating_variable_b as f32;
                rotating.kepler_elem[5] = msg.rotating_variable_c as f32;
            }
            6 => {
                rotating.k_bdot[0] = msg.rotating_variable_a as f32;
                rotating.k_bdot[1] = msg.rotating_variable_b as f32;
                rotating.k_bdot[2] = msg.rotating_variable_c as f32;
            }
            7 => {
                rotating.kp[0] = msg.rotating_variable_a as f32;
                rotating.kp[1] = msg.rotating_variable_b as f32;
                rotating.kp[2] = msg.rotating_variable_c as f32;
            }
            8 => {
                rotating.kd[0] = msg.rotating_variable_a as f32;
                rotating.kd[1] = msg.rotating_variable_b as f32;
                rotating.kd[2] = msg.rotating_variable_c as f32;
            }
            9 => {
                rotating.k_unload[0] = msg.rotating_variable_a as f32;
                rotating.k_unload[1] = msg.rotating_variable_b as f32;
                rotating.k_unload[2] = msg.rotating_variable_c as f32;
            }
            10 => {
                rotating.css_bias[0] = msg.rotating_variable_a.wrapping_shr(16) as i16;
                rotating.css_bias[1] = msg.rotating_variable_b.wrapping_shr(16) as i16;
                rotating.css_bias[2] = msg.rotating_variable_c.wrapping_shr(16) as i16;
                rotating.css_bias[3] = msg.rotating_variable_a as i16;
                rotating.css_bias[4] = msg.rotating_variable_b as i16;
                rotating.css_bias[5] = msg.rotating_variable_c as i16;
            }
            11 => {
                rotating.mag_bias[0] = msg.rotating_variable_a.wrapping_shr(16) as i16;
                rotating.mag_bias[1] = msg.rotating_variable_b.wrapping_shr(16) as i16;
                rotating.mag_bias[2] = msg.rotating_variable_c.wrapping_shr(16) as i16;
                rotating.rws_volt = msg.rotating_variable_a as i16;
                rotating.rws_press = msg.rotating_variable_b as i16;
            }
            12 => {
                rotating.att_det_mode = msg.rotating_variable_a.wrapping_shr(24) as u8;
                rotating.rws_reset_cntr[0] = msg.rotating_variable_a.wrapping_shr(16) as u8;
                rotating.sun_mag_aligned = msg.rotating_variable_a.wrapping_shr(8) as u8;
                rotating.minor_version = msg.rotating_variable_a as u8;
                rotating.mai_sn = msg.rotating_variable_b.wrapping_shr(24) as u8;
                rotating.rws_reset_cntr[1] = msg.rotating_variable_b.wrapping_shr(16) as u8;
                rotating.orbit_prop_mode = msg.rotating_variable_b.wrapping_shr(8) as u8;
                rotating.acs_op_mode = msg.rotating_variable_b as u8;
                rotating.proc_reset_cntr = msg.rotating_variable_c.wrapping_shr(24) as u8;
                rotating.rws_reset_cntr[2] = msg.rotating_variable_c.wrapping_shr(16) as u8;
                rotating.major_version = msg.rotating_variable_c.wrapping_shr(8) as u8;
                rotating.ads_op_mode = msg.rotating_variable_c as u8;
            }
            13 => {
                rotating.css_gain[0] = msg.rotating_variable_a as f32;
                rotating.css_gain[1] = msg.rotating_variable_b as f32;
                rotating.css_gain[2] = msg.rotating_variable_c as f32;
            }
            14 => {
                rotating.css_gain[3] = msg.rotating_variable_a as f32;
                rotating.css_gain[4] = msg.rotating_variable_b as f32;
                rotating.css_gain[5] = msg.rotating_variable_c as f32;
            }
            15 => {
                rotating.mag_gain[0] = msg.rotating_variable_a as f32;
                rotating.mag_gain[1] = msg.rotating_variable_b as f32;
                rotating.mag_gain[2] = msg.rotating_variable_c as f32;
            }
            16 => {
                rotating.orbit_epoch = msg.rotating_variable_a as u32;
                rotating.true_anomoly_epoch = msg.rotating_variable_b as f32;
                rotating.orbit_epoch_next = msg.rotating_variable_c as u32;
            }
            17 => {
                rotating.sc_pos_eci_epoch[0] = msg.rotating_variable_a as f32;
                rotating.sc_pos_eci_epoch[1] = msg.rotating_variable_b as f32;
                rotating.sc_pos_eci_epoch[2] = msg.rotating_variable_c as f32;
            }
            18 => {
                rotating.sc_vel_eci_epoch[0] = msg.rotating_variable_a as f32;
                rotating.sc_vel_eci_epoch[1] = msg.rotating_variable_b as f32;
                rotating.sc_vel_eci_epoch[2] = msg.rotating_variable_c as f32;
            }
            19 => {
                rotating.qb_x_wheel_speed = msg.rotating_variable_a.wrapping_shr(16) as i16;
                rotating.qb_x_filter_gain = msg.rotating_variable_b as f32;
                rotating.qb_x_dipole_gain = msg.rotating_variable_c as f32;
            }
            20 => {
                rotating.dipole_gain[0] = msg.rotating_variable_a as f32;
                rotating.dipole_gain[1] = msg.rotating_variable_b as f32;
                rotating.dipole_gain[2] = msg.rotating_variable_c as f32;
            }
            21 => {
                rotating.wheel_speed_bias[0] = msg.rotating_variable_a.wrapping_shr(16) as i16;
                rotating.wheel_speed_bias[1] = msg.rotating_variable_b.wrapping_shr(16) as i16;
                rotating.wheel_speed_bias[2] = msg.rotating_variable_c.wrapping_shr(16) as i16;
            }
            22 => {
                rotating.cos_sun_mag_align_thresh = msg.rotating_variable_a as f32;
                rotating.unload_ang_thresh = msg.rotating_variable_b as f32;
                rotating.q_sat = msg.rotating_variable_c as f32;
            }
            23 => {
                rotating.raw_trq_max = msg.rotating_variable_a as f32;
                rotating.rws_motor_current[0] = msg.rotating_variable_b.wrapping_shr(16) as u16;
                rotating.rws_motor_current[1] = msg.rotating_variable_b as u16;
                rotating.rws_motor_current[2] = msg.rotating_variable_c.wrapping_shr(16) as u16;
                rotating.raw_motor_temp = msg.rotating_variable_c as i16;
            }
            _ => {}
        }
    }
}

/* 
TODO: Deal with the fact that you can't clone io::error,
but double requires the ability to clone errors 

#[derive(Fail, Display, Debug)]
pub enum MAIError {
    #[display(fmt = "Generic Error")]
    GenericError,
    #[display(fmt = "Serial Error: {}", cause)]
    /// There was a problem parsing the result data
    SerialError { #[fail(cause)] cause: serial::Error },
    #[display(fmt = "IO Error: {}", cause)]
    /// There was a problem parsing the result data
    IoError {
        #[fail(cause)]
        cause: io::Error,
    },
}

impl From<io::Error> for MAIError {
    fn from(error: io::Error) -> Self {
        MAIError::IoError { cause: error }
    }
}

impl From<serial::Error> for MAIError {
    fn from(error: serial::Error) -> Self {
        MAIError::SerialError { cause: error }
    }
}
*/

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
