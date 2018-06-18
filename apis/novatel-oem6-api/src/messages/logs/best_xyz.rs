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
use super::*;

/// Log message containing position information
#[derive(Clone, Default, Debug, PartialEq)]
pub struct BestXYZLog {
    /// Current status of receiver
    pub recv_status: ReceiverStatusFlags,
    /// Validity of the time information
    pub time_status: u8,
    /// GPS reference week
    pub week: u16,
    /// Milliseconds into GPS reference week
    pub ms: i32,
    /// Position solution status
    pub pos_status: u32,
    /// Position type
    pub pos_type: u32,
    /// Position coordinates {X, Y, Z} (m)
    pub position: [f64; 3],
    /// Standard deviation of position coordinates {X, Y, Z} (m)
    pub pos_deviation: [f32; 3],
    /// Velocity solution status
    pub vel_status: u32,
    /// Velocity type
    pub vel_type: u32,
    /// Velocity vector along {X, Y, Z}-axis (m/s)
    pub velocity: [f64; 3],
    /// Standard deviation of velocity vector along {X, Y, Z}-axis (m/s)
    pub vel_deviation: [f32; 3],
    /// Base station ID
    pub station_id: String,
    /// Latency of the velocity time tag
    pub vel_time_latency: f32,
    /// Differential age (seconds)
    pub diff_age: f32,
    /// Solution age (seconds)
    pub sol_age: f32,
    /// Number of satellites tracked
    pub num_sats: u8,
    /// Number of satellite vehicles used in solution
    pub num_sat_vehicles: u8,
    /// Number of GPS plus GLONASS plus BDS L1/B1 used in solution
    pub num_gg_l1: u8,
    /// Number of satellites with L1/E1/B1 signals used in solution
    pub num_multi_sats: u8,
    /// Extended solution status
    pub ext_sol_stat: u8,
    /// Galileo and BeiDou signals used mask
    pub gal_beidou_sig: u8,
    /// GPS and GLONASS signals used mask
    pub gps_glonass_sig: u8,
}

impl BestXYZLog {
    /// Convert a raw data buffer into a useable struct
    pub fn new(
        recv_status: ReceiverStatusFlags,
        time_status: u8,
        week: u16,
        ms: i32,
        raw: Vec<u8>,
    ) -> Option<Self> {
        let mut log = match parse_bestxyz(&raw) {
            Ok(conv) => conv.1,
            _ => return None,
        };

        log.recv_status = recv_status;
        log.time_status = time_status;
        log.week = week;
        log.ms = ms;

        Some(log)
    }
}

named!(parse_bestxyz(&[u8]) -> BestXYZLog,
    do_parse!(
        pos_status: le_u32 >>
        pos_type: le_u32 >>
        pos_x: le_f64 >>
        pos_y: le_f64 >>
        pos_z: le_f64 >>
        pos_dev_x: le_f32 >>
        pos_dev_y: le_f32 >>
        pos_dev_z: le_f32 >>
        vel_status: le_u32 >>
        vel_type: le_u32 >>
        vel_x: le_f64 >>
        vel_y: le_f64 >>
        vel_z: le_f64 >>
        vel_dev_x: le_f32 >>
        vel_dev_y: le_f32 >>
        vel_dev_z: le_f32 >>
        station_id: take!(4) >>
        vel_time_latency: le_f32 >>
        diff_age: le_f32 >>
        sol_age: le_f32 >>
        num_sats: le_u8 >>
        num_sat_vehicles: le_u8 >>
        num_gg_l1: le_u8 >>
        num_multi_sats: le_u8 >>
        le_u8 >>
        ext_sol_stat: le_u8 >>
        gal_beidou_sig: le_u8 >>
        gps_glonass_sig: le_u8 >>
        (BestXYZLog {
            recv_status: ReceiverStatusFlags::empty(),
            time_status: 0,
            week: 0,
            ms: 0,
            pos_status,
            pos_type,
            position: [pos_x, pos_y, pos_z],
            pos_deviation: [pos_dev_x, pos_dev_y, pos_dev_z],
            vel_status,
            vel_type,
            velocity: [vel_x, vel_y, vel_z],
            vel_deviation: [vel_dev_x, vel_dev_y, vel_dev_z],
            station_id: String::from_utf8_lossy(station_id).trim_right_matches('\u{0}').to_owned(),
            vel_time_latency,
            diff_age,
            sol_age,
            num_sats,
            num_sat_vehicles,
            num_gg_l1,
            num_multi_sats,
            ext_sol_stat,
            gal_beidou_sig,
            gps_glonass_sig,
            }
        )
    )
);
