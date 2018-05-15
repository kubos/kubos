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

#[derive(Clone, Default, Debug, PartialEq)]
pub struct BestXYZLog {
    pub pos_status: u32, //TODO: enum
    pub pos_type: u32,   //TODO: enum
    pub position: [f64; 3],
    pub pos_deviation: [f32; 3],
    pub vel_status: u32,
    pub vel_type: u32,
    pub velocity: [f64; 3],
    pub vel_deviation: [f32; 3],
    pub station_id: String,
    pub vel_time_latency: f32,
    pub diff_age: f32,
    pub sol_age: f32,
    pub num_sats: u8,
    pub num_sat_vehicles: u8,
    pub num_gg_l1: u8,
    pub num_multi_sats: u8,
    pub ext_sol_stat: u8,
    pub gal_beidou_sig: u8,
    pub gps_glonass_sig: u8,
}

impl BestXYZLog {
    pub fn new(raw: Vec<u8>) -> Option<Self> {
        // Convert the raw data to an official struct
        match parse_bestxyz(&raw) {
            Ok(conv) => Some(conv.1),
            _ => None,
        }
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
            pos_status,
            pos_type,
            position: [pos_x, pos_y, pos_z],
            pos_deviation: [pos_dev_x, pos_dev_y, pos_dev_z],
            vel_status,
            vel_type,
            velocity: [vel_x, vel_y, vel_z],
            vel_deviation: [vel_dev_x, vel_dev_y, vel_dev_z],
            station_id: ::std::str::from_utf8(station_id).unwrap().trim_right_matches('\u{0}').to_owned(),
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
