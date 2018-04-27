//
// Copyright (C) 2018 Kubos Corporation
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

use byteorder::{LittleEndian, ReadBytesExt};
use nom::*;
use std::io::Cursor;

/// Standard telemetry packet sent by the MAI-400 every 250ms
#[derive(Clone, Debug, Default, PartialEq)]
pub struct StandardTelemetry {
    /// Rotating variable indicator
    pub tlm_counter: u8,
    /// UTC Time in Seconds
    pub gps_time: u32,
    /// 4 Hz Subsecond counter
    pub time_subsec: u8,
    /// Valid Command Counter
    pub cmd_valid_cntr: u16,
    /// Invalid Command Counter. Number of commands which did not pass command verification.
    pub cmd_invalid_cntr: u16,
    /// Invalid Command CRC Counter. Number of command messages received with invalid checksums
    pub cmd_invalid_chksum_cntr: u16,
    /// Last valid CCT command received
    pub last_command: u8,
    /// Commanded ACS Mode
    pub acs_mode: u8,
    /// Raw sun sensor outputs
    pub css: [u16; 6],
    /// Whether the device is eclipsed or not
    pub eclipse_flag: u8,
    /// Unit Sun Vector in Body Frame
    pub sun_vec_b: [i16; 3],
    /// Magnetometer Reading (inc. bias and gain)
    pub i_b_field_meas: [i16; 3],
    /// Rate of Change of Magnetic field Vector in Body Frame
    pub bd: [f32; 3],
    /// RWS Commanded Wheel Speed, lsb: 1 RPM
    pub rws_speed_cmd: [i16; 3],
    /// RWS Wheel Speeds, lsb: 1 RPM
    pub rws_speed_tach: [i16; 3],
    /// Commanded Wheel Torque computed by ADACS (mNm)
    pub rwa_torque_cmd: [f32; 3],
    /// RWS Torque Command to wheel
    pub gc_rwa_torque_cmd: [i8; 3],
    /// Torque Coil Command computed by ADACS (Am<sup>2</sup>)
    pub torque_coil_cmd: [f32; 3],
    /// Torque Coil Command (lsb)
    pub gc_torque_coil_cmd: [i8; 3],
    /// Commanded orbit-to-body quaternion
    pub qbo_cmd: [i16; 4],
    /// Current Estimated Orbit-to-Body Quaternion
    pub qbo_hat: [i16; 4],
    /// Angle to Go in radians
    pub angle_to_go: f32,
    /// Error between command and estimate Qbo
    pub q_error: [i16; 4],
    /// Body rate in body frame (rad/sec).
    pub omega_b: [f32; 3],
    /// Rotating variable A. Use [RotatingTelemetry](struct.RotatingTelemetry.html) struct if interaction is needed
    pub rotating_variable_a: u32,
    /// Rotating variable B. Use [RotatingTelemetry](struct.RotatingTelemetry.html) struct if interaction is needed
    pub rotating_variable_b: u32,
    /// Rotating variable C. Use [RotatingTelemetry](struct.RotatingTelemetry.html) struct if interaction is needed
    pub rotating_variable_c: u32,
    /// Nadir vectors in Body
    pub nb: [i16; 3],
    /// Nadir vectors in ECI frame
    pub neci: [i16; 3],
}

impl StandardTelemetry {
    /// Constructor. Converts a raw data array received from the MAI-400 into a usable structure
    pub fn new(mut msg: Vec<u8>) -> Option<Self> {
        // Get the CRC bytes
        let len = msg.len() - 2;

        let mut crc = Cursor::new(msg.split_off(len));
        let crc = crc.read_u16::<LittleEndian>().unwrap_or(0);

        // Get the calculated CRC
        let mut calc: u16 = 0;
        for byte in msg.iter() {
            calc += *byte as u16;
        }

        // Make sure they match
        match calc == crc {
            true => {
                // Convert the raw data to an official struct
                match standardtelem(&msg) {
                    Ok(conv) => Some(conv.1),
                    _ => None,
                }
            }
            false => None,
        }
    }
}

named!(standardtelem(&[u8]) -> StandardTelemetry,
    do_parse!(
        le_u16 >>
        tlm_counter: le_u8 >>
        gps_time: le_u32 >>
        time_subsec: le_u8 >>
        cmd_valid_cntr: le_u16 >>
        cmd_invalid_cntr: le_u16 >>
        cmd_invalid_chksum_cntr: le_u16 >>
        last_command: le_u8 >>
        acs_mode: le_u8 >>
        css_0: le_u16 >>
        css_1: le_u16 >>
        css_2: le_u16 >>
        css_3: le_u16 >>
        css_4: le_u16 >>
        css_5: le_u16 >>
        eclipse_flag: le_u8 >>
        sun_vec_b_0: le_i16 >>
        sun_vec_b_1: le_i16 >>
        sun_vec_b_2: le_i16 >>
        i_b_field_meas_0: le_i16 >>
        i_b_field_meas_1: le_i16 >>
        i_b_field_meas_2: le_i16 >>
        bd_0: le_f32 >>
        bd_1: le_f32 >>
        bd_2: le_f32 >>
        rws_speed_cmd_0: le_i16 >>
        rws_speed_cmd_1: le_i16 >>
        rws_speed_cmd_2: le_i16 >>
        rws_speed_tach_0: le_i16 >>
        rws_speed_tach_1: le_i16 >>
        rws_speed_tach_2: le_i16 >>
        rwa_torque_cmd_0: le_f32 >>
        rwa_torque_cmd_1: le_f32 >>
        rwa_torque_cmd_2: le_f32 >>
        gc_rwa_torque_cmd_0: le_i8 >>
        gc_rwa_torque_cmd_1: le_i8 >>
        gc_rwa_torque_cmd_2: le_i8 >>
        torque_coil_cmd_0: le_f32 >>
        torque_coil_cmd_1: le_f32 >>
        torque_coil_cmd_2: le_f32 >>
        gc_torque_coil_cmd_0: le_i8 >>
        gc_torque_coil_cmd_1: le_i8 >>
        gc_torque_coil_cmd_2: le_i8 >>
        qbo_cmd_0: le_i16 >>
        qbo_cmd_1: le_i16 >>
        qbo_cmd_2: le_i16 >>
        qbo_cmd_3: le_i16 >>
        qbo_hat_0: le_i16 >>
        qbo_hat_1: le_i16 >>
        qbo_hat_2: le_i16 >>
        qbo_hat_3: le_i16 >>
        angle_to_go: le_f32 >>
        q_error_0: le_i16 >>
        q_error_1: le_i16 >>
        q_error_2: le_i16 >>
        q_error_3: le_i16 >>
        omega_b_0: le_f32 >>
        omega_b_1: le_f32 >>
        omega_b_2: le_f32 >>
        rotating_variable_a: le_u32 >>
        rotating_variable_b: le_u32 >>
        rotating_variable_c: le_u32 >>
        nb_0: le_i16 >>
        nb_1: le_i16 >>
        nb_2: le_i16 >>
        neci_0: le_i16 >>
        neci_1: le_i16 >>
        neci_2: le_i16 >>
        (StandardTelemetry{
            tlm_counter,
            gps_time,
            time_subsec,
            cmd_valid_cntr,
            cmd_invalid_cntr,
            cmd_invalid_chksum_cntr,
            last_command,
            acs_mode,
            css: [
                css_0,
                css_1,
                css_2,
                css_3,
                css_4,
                css_5
            ],
            eclipse_flag,
            sun_vec_b: [
                sun_vec_b_0,
                sun_vec_b_1,
                sun_vec_b_2
            ],
            i_b_field_meas: [
                i_b_field_meas_0,
                i_b_field_meas_1,
                i_b_field_meas_2
            ],
            bd: [
                bd_0,
                bd_1,
                bd_2
            ],
            rws_speed_cmd: [
                rws_speed_cmd_0,
                rws_speed_cmd_1,
                rws_speed_cmd_2
            ],
            rws_speed_tach: [
                rws_speed_tach_0,
                rws_speed_tach_1,
                rws_speed_tach_2
            ],
            rwa_torque_cmd: [
                rwa_torque_cmd_0,
                rwa_torque_cmd_1,
                rwa_torque_cmd_2
            ],
            gc_rwa_torque_cmd: [
                gc_rwa_torque_cmd_0,
                gc_rwa_torque_cmd_1,
                gc_rwa_torque_cmd_2
            ],
            torque_coil_cmd: [
                torque_coil_cmd_0,
                torque_coil_cmd_1,
                torque_coil_cmd_2
            ],
            gc_torque_coil_cmd: [
                gc_torque_coil_cmd_0,
                gc_torque_coil_cmd_1,
                gc_torque_coil_cmd_2
            ],
            qbo_cmd: [
                qbo_cmd_0,
                qbo_cmd_1,
                qbo_cmd_2,
                qbo_cmd_3
            ],
            qbo_hat: [
                qbo_hat_0,
                qbo_hat_1,
                qbo_hat_2,
                qbo_hat_3
            ],
            angle_to_go,
            q_error: [
                q_error_0,
                q_error_1,
                q_error_2,
                q_error_3
            ],
            omega_b: [
                omega_b_0,
                omega_b_1,
                omega_b_2
            ],
            rotating_variable_a,
            rotating_variable_b,
            rotating_variable_c,
            nb: [
                nb_0,
                nb_1,
                nb_2
            ],
            neci: [
                neci_0,
                neci_1,
                neci_2
            ]
        })
    )
);
