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
//TODO: remove before publishing
#![allow(unused)]

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use crc16::*;
use failure::Error;
use messages::*;
use serial;
use serial_comm::Connection;
use std::io;
use std::io::Cursor;

#[derive(Default)]
pub struct RotatingTelemetry {
    b_field_igrf: [f32; 3],
    sun_vec_eph: [f32; 3],
    sc_pos_eci: [f32; 3],
    sc_vel_eci: [f32; 3],
    kepler_elem: [f32; 6],
    k_bdot: [f32; 3],
    kp: [f32; 3],
    kd: [f32; 3],
    k_unload: [f32; 3],
    css_bias: [i16; 6],
    mag_bias: [i16; 3],
    rws_volt: i16,
    rws_press: i16,
    att_det_mode: u8, //TODO: enum
    rws_reset_cntr: [u8; 3],
    sun_mag_aligned: u8, //TODO: bool
    minor_version: u8,
    mai_sn: u8,
    orbit_prop_mode: u8,
    acs_op_mode: u8,
    proc_reset_cntr: u8,
    major_version: u8,
    ads_op_mode: u8, //TODO: enum
    css_gain: [f32; 6],
    mag_gain: [f32; 3],
    orbit_epoch: u32,
    true_anomoly_epoch: f32,
    orbit_epoch_next: u32,
    sc_pos_eci_epoch: [f32; 3],
    sc_vel_eci_epoch: [f32; 3],
    qb_x_wheel_speed: i16,
    qb_x_filter_gain: f32,
    qb_x_dipole_gain: f32,
    dipole_gain: [f32; 3],
    wheel_speed_bias: [i16; 3],
    cos_sun_mag_align_thresh: f32,
    unload_ang_thresh: f32,
    q_sat: f32,
    raw_trq_max: f32,
    rws_motor_current: [u16; 3],
    raw_motor_temp: i16, //TODO: Conversion formula
}

pub struct MAI400 {
    pub conn: Connection,
}

impl MAI400 {
    /// Constructor for MAI400 structure
    pub fn new(conn: Connection) -> MAI400 {
        MAI400 { conn }
    }

    pub fn reset() {
        //REQUEST_RESET
        //CONFIRM_RESET
    }

    //SetAcsMode
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

    pub fn set_gps_time(&self) -> MAIResult<()> {
        unimplemented!()
    }

    pub fn set_rv(&self) -> MAIResult<()> {
        unimplemented!()
    }

    //Option 2
    //Don't actually merge this. Need to figure out which way is preferable
    /*
    pub fn get_info_alt(&self) -> MAIResult<()> {
        //Create packet
        let packet = GetInfoMessage::default();
        let slice = unsafe {
            ::std::mem::transmute::<GetInfoMessage, [u8; ::std::mem::size_of::<GetInfoMessage>()]>(
                packet,
            )
        };

        //send packet
        self.conn.write(&slice)?;
        Ok(())
    }
    */

    fn send_message(&self, mut msg: Vec<u8>) -> MAIResult<()> {
        let crc = State::<AUG_CCITT>::calculate(&msg[2..]);
        msg.write_u16::<LittleEndian>(crc).unwrap();

        //send packet
        self.conn.write(msg.as_slice())?;
        Ok(())
    }

    pub fn get_message(&self) -> MAIResult<Response> {
        let mut msg = vec![];
        let mut response: Response;

        loop {
            msg = self.conn.read()?;

            // Pull out raw IMU message
            let len = msg.len();
            let imu = msg.split_off(len - 21);

            // Pull out IREHS telemetry message
            let len = msg.len();
            let irehs = msg.split_off(len - 56);

            // Get the CRC bytes
            let len = msg.len();
            let mut raw = msg.split_off(len - 2);
            let mut crc = Cursor::new(raw.to_vec());
            let crc = crc.read_u16::<LittleEndian>()?;

            // Get the calculated CRC
            //let calc = State::<AUG_CCITT>::calculate(&msg[1..]);
            let mut calc: u16 = 0;
            for byte in msg.iter() {
                calc += *byte as u16;
            }

            // Make sure they match
            // If not, pretend this never happened and go get another message
            if calc != crc {
                continue;
            }

            // Put the CRC bytes back and translate the vector into a useable structure
            msg.append(&mut raw);

            let telem = StandardTelemetry::new(&msg[..]);
            response = Response::StdTelem(telem);
            break;
        }

        Ok(response)

    }

    pub fn update_rotating(&self, msg: &StandardTelemetry, rotating: &mut RotatingTelemetry) {
        match msg.tlm_counter {
            0 => {
                rotating.b_field_igrf[0] = msg.rotating_variable_a as f32;
                rotating.b_field_igrf[1] = msg.rotating_variable_b as f32;
                rotating.b_field_igrf[2] = msg.rotating_variable_c as f32;
            }
            1 => {
                rotating.b_field_igrf[0] = msg.rotating_variable_a as f32;
                rotating.b_field_igrf[1] = msg.rotating_variable_b as f32;
                rotating.b_field_igrf[2] = msg.rotating_variable_c as f32;
            }
            2 => {
                rotating.sun_vec_eph[0] = msg.rotating_variable_a as f32;
                rotating.sun_vec_eph[1] = msg.rotating_variable_b as f32;
                rotating.sun_vec_eph[2] = msg.rotating_variable_c as f32;
            }
            3 => {
                rotating.sc_pos_eci[0] = msg.rotating_variable_a as f32;
                rotating.sc_pos_eci[1] = msg.rotating_variable_b as f32;
                rotating.sc_pos_eci[2] = msg.rotating_variable_c as f32;
            }
            4 => {
                rotating.sc_vel_eci[0] = msg.rotating_variable_a as f32;
                rotating.sc_vel_eci[1] = msg.rotating_variable_b as f32;
                rotating.sc_vel_eci[2] = msg.rotating_variable_c as f32;
            }
            5 => {
                rotating.kepler_elem[0] = msg.rotating_variable_a as f32;
                rotating.kepler_elem[1] = msg.rotating_variable_b as f32;
                rotating.kepler_elem[2] = msg.rotating_variable_c as f32;
            }
            6 => {
                rotating.kepler_elem[3] = msg.rotating_variable_a as f32;
                rotating.kepler_elem[4] = msg.rotating_variable_b as f32;
                rotating.kepler_elem[5] = msg.rotating_variable_c as f32;
            }
            7 => {
                rotating.k_bdot[0] = msg.rotating_variable_a as f32;
                rotating.k_bdot[1] = msg.rotating_variable_b as f32;
                rotating.k_bdot[2] = msg.rotating_variable_c as f32;
            }
            8 => {
                rotating.kp[0] = msg.rotating_variable_a as f32;
                rotating.kp[1] = msg.rotating_variable_b as f32;
                rotating.kp[2] = msg.rotating_variable_c as f32;
            }
            8 => {
                rotating.kd[3] = msg.rotating_variable_a as f32;
                rotating.kd[4] = msg.rotating_variable_b as f32;
                rotating.kd[5] = msg.rotating_variable_c as f32;
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
    #[display(fmt = "Generic Error")]
    GenericError,
    #[display(fmt = "Serial Error: {}", cause)]
    /// There was a problem parsing the result data
    SerialError { cause: String },
    #[display(fmt = "IO Error: {}", cause)]
    /// There was a problem parsing the result data
    IoError { cause: String },
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
