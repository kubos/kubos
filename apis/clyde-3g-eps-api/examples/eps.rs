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

extern crate clyde_3g_eps_api;
extern crate rust_i2c;

use clyde_3g_eps_api::*;
use rust_i2c::*;
use std::thread;
use std::time::Duration;

macro_rules! print_result {
    ($p:expr, $r:expr) => {
        match $r {
            Ok(v) => println!("{} - {:#?}", $p, v),
            Err(e) => println!("{} Err - {:#?}", $p, e),
        }
    };
}

macro_rules! dump_mother_telem {
    (
        $eps:expr,
        $($type:ident,)+
    ) => {
        $(
            print_result!(
                format!("{:?}", MotherboardTelemetry::$type),
                $eps.get_motherboard_telemetry(MotherboardTelemetry::$type)
            );
            thread::sleep(Duration::from_millis(100));
        )+
    };
}

macro_rules! dump_daughter_telem {
    (
        $eps:expr,
        $($type:ident,)+
    ) => {
        $(
            print_result!(
                format!("{:?}", DaughterboardTelemetry::$type),
                $eps.get_daughterboard_telemetry(DaughterboardTelemetry::$type)
            );
            thread::sleep(Duration::from_millis(100));
        )+
    };
}

macro_rules! dump_reset_telem {
    (
        $eps:expr,
        $($type:ident,)+
    ) => {
        $(
            print_result!(
                format!("{:?}", ResetTelemetry::$type),
                $eps.get_reset_telemetry(ResetTelemetry::$type)
            );
            thread::sleep(Duration::from_millis(100));
        )+
    };
}

pub fn main() {
    let eps = Eps::new(Connection::from_path("/dev/i2c-1", 0x2B));

    loop {
        eps.get_version_info();
        thread::sleep(Duration::from_millis(50000));
    }
}
