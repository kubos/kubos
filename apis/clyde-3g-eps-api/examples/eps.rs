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
extern crate i2c_hal;

use i2c_hal::*;
use clyde_3g_eps_api::*;
use std::thread;
use std::time::Duration;

pub fn main() {
    let eps = Eps::new(Connection::from_path("/dev/i2c-1", 0x2B));

    // println!("Board Status {:#?}", eps.get_board_status().unwrap());

    // println!("Last Er: {:#?}", eps.get_last_error().unwrap());

    println!("Checksum: {:#?}", eps.get_checksum().unwrap().motherboard);
    thread::sleep(Duration::from_millis(100));

    println!("Checksum: {:#?}", eps.get_checksum().unwrap().motherboard);
    thread::sleep(Duration::from_millis(100));

    println!("Checksum: {:#?}", eps.get_checksum().unwrap().motherboard);
    thread::sleep(Duration::from_millis(100));

    match eps.get_version_info() {
        Ok(v) => println!(
            "Motherboard: {:?}\nDaughterboard: {:?}",
            v.motherboard,
            v.daughterboard.unwrap()
        ),
        Err(_) => println!("Err fetching version"),
    };

    // println!("Last Er: {:#?}", eps.get_last_error().unwrap());

    // println!(
    //     "Brown Out Resets {:#?}",
    //     eps.get_reset_telemetry(ResetTelemetry::BrownOut).unwrap()
    // );

    // println!("Last Er: {:#?}", eps.get_last_error().unwrap());

    // println!(
    //     "Automatic Software Resets {:#?}",
    //     eps.get_reset_telemetry(ResetTelemetry::AutomaticSoftware)
    //         .unwrap()
    // );

    // println!("Last Er: {:#?}", eps.get_last_error().unwrap());

    // println!(
    //     "Manual Resets {:#?}",
    //     eps.get_reset_telemetry(ResetTelemetry::Manual).unwrap()
    // );

    // println!("Last Er: {:#?}", eps.get_last_error().unwrap());

    // println!(
    //     "Watchdog Resets {:#?}",
    //     eps.get_reset_telemetry(ResetTelemetry::Watchdog).unwrap()
    // );

    // println!("Last Er: {:#?}", eps.get_last_error().unwrap());

    // println!(
    //     "Motherboard VoltageFeedingBcr1 {:#?}",
    //     eps.get_motherboard_telemetry(MotherboardTelemetry::VoltageFeedingBcr1)
    //         .unwrap()
    // );
}
