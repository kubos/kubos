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

use nom::*;

pub struct StandardTelemetry {
    pub sync: u16,
    pub tlm_counter: u8,
    pub gps_time: u32,
    pub time_subsec: u8,
    pub cmd_valid_cntr: u16,
    pub cmd_invalid_cntr: u16,
    pub cmd_invalid_chksum_cntr: u16,
    pub last_command: u8,
    pub acs_mode: u8, //TODO: enum
    pub css_0: u16,
    pub css_1: u16,
    pub css_2: u16,
    pub css_3: u16,
    pub css_4: u16,
    pub css_5: u16,
    pub eclipse_flag: u8, //TODO: bool
    pub sun_vec_b_0: i16,
    pub sun_vec_b_1: i16,
    pub sun_vec_b_2: i16,
    pub i_b_field_meas_0: i16,
    pub i_b_field_meas_1: i16,
    pub i_b_field_meas_2: i16,
    pub bd_0: f32,
    pub bd_1: f32,
    pub bd_2: f32,
    pub rws_speed_cmd_0: i16,
    pub rws_speed_cmd_1: i16,
    pub rws_speed_cmd_2: i16,
    pub rws_speed_tach_0: i16,
    pub rws_speed_tach_1: i16,
    pub rws_speed_tach_2: i16,
    pub rwa_torque_cmd_0: f32,
    pub rwa_torque_cmd_1: f32,
    pub rwa_torque_cmd_2: f32,
    pub gc_rwa_torque_cmd_0: char,
    pub gc_rwa_torque_cmd_1: char,
    pub gc_rwa_torque_cmd_2: char,
    pub torque_coil_cmd_0: f32,
    pub torque_coil_cmd_1: f32,
    pub torque_coil_cmd_2: f32,
    pub gc_torque_coil_cmd_0: char,
    pub gc_torque_coil_cmd_1: char,
    pub gc_torque_coil_cmd_2: char,
    pub qbo_cmd_0: i16,
    pub qbo_cmd_1: i16,
    pub qbo_cmd_2: i16,
    pub qbo_cmd_3: i16,
    pub qbo_hat_0: i16,
    pub qbo_hat_1: i16,
    pub qbo_hat_2: i16,
    pub qbo_hat_3: i16,
    pub angle_to_go: f32,
    pub q_error_0: i16,
    pub q_error_1: i16,
    pub q_error_2: i16,
    pub q_error_3: i16,
    pub omega_b_0: f32,
    pub omega_b_1: f32,
    pub omega_b_2: f32,
    pub rotating_variable_a: u32,
    pub rotating_variable_b: u32,
    pub rotating_variable_c: u32,
    pub nb_0: i16,
    pub nb_1: i16,
    pub nb_2: i16,
    pub neci_0: i16,
    pub neci_1: i16,
    pub neci_2: i16,
    pub crc: u16,
}

impl StandardTelemetry {
    pub fn new(msg: &[u8]) -> Self {
        standardtelem(msg).unwrap().1
    }
}

named!(standardtelem(&[u8]) -> StandardTelemetry,
    do_parse!(
        sync: le_u16 >>
        tlm_counter: le_u8 >>
        gps_time: le_u32 >>
        time_subsec: le_u8 >>
        cmd_valid_cntr: le_u16 >>
        cmd_invalid_cntr: le_u16 >>
        cmd_invalid_chksum_cntr: le_u16 >>
        last_command: le_u8 >>
        acs_mode: le_u8 >> //TODO: le_enum
        css_0: le_u16 >>
        css_1: le_u16 >>
        css_2: le_u16 >>
        css_3: le_u16 >>
        css_4: le_u16 >>
        css_5: le_u16 >>
        eclipse_flag: le_u8 >> //TODO: le_bool
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
        gc_rwa_torque_cmd_0: le_u8 >>
        gc_rwa_torque_cmd_1: le_u8 >>
        gc_rwa_torque_cmd_2: le_u8 >>
        torque_coil_cmd_0: le_f32 >>
        torque_coil_cmd_1: le_f32 >>
        torque_coil_cmd_2: le_f32 >>
        gc_torque_coil_cmd_0: le_u8 >>
        gc_torque_coil_cmd_1: le_u8 >>
        gc_torque_coil_cmd_2: le_u8 >>
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
        crc: le_u16 >>
        (StandardTelemetry{
            sync,
            tlm_counter,
            gps_time,
            time_subsec,
            cmd_valid_cntr,
            cmd_invalid_cntr,
            cmd_invalid_chksum_cntr,
            last_command,
            acs_mode, //TODO: enum
            css_0,
            css_1,
            css_2,
            css_3,
            css_4,
            css_5,
            eclipse_flag, //TODO: bool
            sun_vec_b_0,
            sun_vec_b_1,
            sun_vec_b_2,
            i_b_field_meas_0,
            i_b_field_meas_1,
            i_b_field_meas_2,
            bd_0,
            bd_1,
            bd_2,
            rws_speed_cmd_0,
            rws_speed_cmd_1,
            rws_speed_cmd_2,
            rws_speed_tach_0,
            rws_speed_tach_1,
            rws_speed_tach_2,
            rwa_torque_cmd_0,
            rwa_torque_cmd_1,
            rwa_torque_cmd_2,
            gc_rwa_torque_cmd_0: gc_rwa_torque_cmd_0 as char,
            gc_rwa_torque_cmd_1: gc_rwa_torque_cmd_1 as char,
            gc_rwa_torque_cmd_2: gc_rwa_torque_cmd_2 as char,
            torque_coil_cmd_0,
            torque_coil_cmd_1,
            torque_coil_cmd_2,
            gc_torque_coil_cmd_0: gc_torque_coil_cmd_0 as char,
            gc_torque_coil_cmd_1: gc_torque_coil_cmd_1 as char,
            gc_torque_coil_cmd_2: gc_torque_coil_cmd_2 as char,
            qbo_cmd_0,
            qbo_cmd_1,
            qbo_cmd_2,
            qbo_cmd_3,
            qbo_hat_0,
            qbo_hat_1,
            qbo_hat_2,
            qbo_hat_3,
            angle_to_go,
            q_error_0,
            q_error_1,
            q_error_2,
            q_error_3,
            omega_b_0,
            omega_b_1,
            omega_b_2,
            rotating_variable_a,
            rotating_variable_b,
            rotating_variable_c,
            nb_0,
            nb_1,
            nb_2,
            neci_0,
            neci_1,
            neci_2,
            crc
        })
    )
);

pub struct RawIMU {
    pub sync: u16,
    pub data_len: u16,
    pub packet_type: u16,
    pub accel: [i16; 3],
    pub gyro: [i16; 3],
    pub gyro_temp: u8,
    pub crc: u16,
}

impl RawIMU {
    pub fn new(msg: &[u8]) -> Self {
        raw_imu(msg).unwrap().1
    }
}

named!(raw_imu(&[u8]) -> RawIMU,
    do_parse!(
        sync: le_u16 >>
        data_len: le_u16 >>
        packet_type: le_u16 >>
        accel_x: le_i16 >>
        accel_y: le_i16 >>
        accel_z: le_i16 >>
        gyro_x: le_i16 >>
        gyro_y: le_i16 >>
        gyro_z: le_i16 >>
        gyro_temp: le_u8 >>
        crc: le_u16 >>
        (RawIMU {
                sync,
                data_len,
                packet_type,
                accel: [accel_x ,accel_y, accel_z],
                gyro: [gyro_x, gyro_y, gyro_z],
                gyro_temp,
                crc
        })
    )
);

pub struct IREHSTelemetry {
    pub sync: u16,
    pub data_len: u16,
    pub packet_type: u16,
    pub thermopiles_a: [u16; 4],
    pub thermopiles_b: [u16; 4],
    pub temp_a: [u16; 4],
    pub temp_b: [u16; 4],
    pub dip_angle_a: u32,
    pub dip_angle_b: u32,
    pub solution_degraded: [u8; 8],
    pub crc: u16,
}

impl IREHSTelemetry {
    pub fn new(msg: &[u8]) -> Self {
        irehs_telem(msg).unwrap().1
    }
}

named!(irehs_telem(&[u8]) -> IREHSTelemetry,
    do_parse!(
        sync: le_u16 >>
        data_len: le_u16 >>
        packet_type: le_u16 >>
        thermopiles_a_earth_limb: le_u16 >>
        thermopiles_a_earth_ref: le_u16 >>
        thermopiles_a_space_ref: le_u16 >>
        thermopiles_a_wide_fov: le_u16 >>
        thermopiles_b_earth_limb: le_u16 >>
        thermopiles_b_earth_ref: le_u16 >>
        thermopiles_b_space_ref: le_u16 >>
        thermopiles_b_wide_fov: le_u16 >>
        temp_a_earth_limb: le_u16 >>
        temp_a_earth_ref: le_u16 >>
        temp_a_space_ref: le_u16 >>
        temp_a_wide_fov: le_u16 >>
        temp_b_earth_limb: le_u16 >>
        temp_b_earth_ref: le_u16 >>
        temp_b_space_ref: le_u16 >>
        temp_b_wide_fov: le_u16 >>
        dip_angle_a: le_u32 >>
        dip_angle_b: le_u32 >>
        solution_degraded_earth_limb_a: le_u8 >>
        solution_degraded_earth_ref_a: le_u8 >>
        solution_degraded_space_ref_a: le_u8 >>
        solution_degraded_wide_fov_a: le_u8 >>
        solution_degraded_earth_limb_b: le_u8 >>
        solution_degraded_earth_ref_b: le_u8 >>
        solution_degraded_space_ref_b: le_u8 >>
        solution_degraded_wide_fov_b: le_u8 >>
        crc: le_u16 >>
        (IREHSTelemetry {
                sync,
                data_len,
                packet_type,
                thermopiles_a: [
                    thermopiles_a_earth_limb,
                    thermopiles_a_earth_ref,
                    thermopiles_a_space_ref,
                    thermopiles_a_wide_fov
                ],
                thermopiles_b: [
                    thermopiles_b_earth_limb,
                    thermopiles_b_earth_ref,
                    thermopiles_b_space_ref,
                    thermopiles_b_wide_fov
                ],
                temp_a: [
                    temp_a_earth_limb,
                    temp_a_earth_ref,
                    temp_a_space_ref,
                    temp_a_wide_fov
                ],
                temp_b: [
                    temp_b_earth_limb,
                    temp_b_earth_ref,
                    temp_b_space_ref,
                    temp_b_wide_fov
                ],
                dip_angle_a,
                dip_angle_b,
                solution_degraded: [
                    solution_degraded_earth_limb_a,
                    solution_degraded_earth_ref_a,
                    solution_degraded_space_ref_a,
                    solution_degraded_wide_fov_a,
                    solution_degraded_earth_limb_b,
                    solution_degraded_earth_ref_b,
                    solution_degraded_space_ref_b,
                    solution_degraded_wide_fov_b
                ],
                crc,
        })
    )
);

pub enum Response {
    StdTelem(StandardTelemetry),
    IMU(RawIMU),
    IREHS(IREHSTelemetry),
}
