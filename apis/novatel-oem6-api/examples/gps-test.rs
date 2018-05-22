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

extern crate novatel_oem6_api;

use novatel_oem6_api::*;
use std::thread;
use std::time::Duration;
use std::sync::mpsc::sync_channel;

fn get_version(oem: &OEM6) -> OEMResult<()> {
    // Send the request for info
    oem.request_version().map_err(|err| {
        println!("{}: Failed to request version information", line!());
        err
    })?;

    loop {
        // Read the next log message, which should have the reply
        let entry = oem.get_log().map_err(|err| {
            println!("{}: Failed to get version log from the OEM", line!());
            err
        })?;

        match entry {
            Log::Version(log) => {
                println!("Version Info ({}):\n", log.num_components);
                for component in log.components.iter() {
                    println!(
                        "Type: {} Model: {} SN: {}",
                        component.comp_type, component.model, component.serial_num
                    );
                    println!("    HW Version: {}", component.hw_version);
                    println!("    SW Version: {}", component.sw_version);
                    println!("    Boot Version: {}", component.boot_version);
                    println!(
                        "    Compiled: {} {}",
                        component.compile_date, component.compile_time
                    );
                    println!("");
                }
                break;
            }
            _ => continue,
        }
    }

    Ok(())
}

fn get_position(oem: &OEM6) -> OEMResult<()> {
    // Send the request for info
    oem.request_position(1.0, 0.0, false).map_err(|err| {
        println!("{}: Failed to request position information", line!());
        err
    })?;

    loop {
        // Read the next log message, which should have the reply
        let entry = oem.get_log().map_err(|err| {
            println!("{}: Failed to get position log from the OEM", line!());
            err
        })?;

        match entry {
            Log::BestXYZ(log) => {
                println!("Best XYZ Data:");
                println!("pos_status: {}", log.pos_status);
                println!("pos_type: {}", log.pos_type);
                println!("position: {:?}", log.position);
                println!("pos_deviation: {:?}", log.pos_deviation);
                println!("vel_status: {}", log.vel_status);
                println!("vel_type: {}", log.vel_type);
                println!("velocity: {:?}", log.velocity);
                println!("vel_deviation: {:?}", log.vel_deviation);
                println!("station_id: {}", log.station_id);
                println!("vel_time_latency: {}", log.vel_time_latency);
                println!("diff_age: {}", log.diff_age);
                println!("sol_age: {}", log.sol_age);
                println!("num_sats: {}", log.num_sats);
                println!("num_sat_vehicles: {}", log.num_sat_vehicles);
                println!("num_gg_l1: {}", log.num_gg_l1);
                println!("num_multi_sats: {}", log.num_multi_sats);
                println!("ext_sol_stat: {}", log.ext_sol_stat);
                println!("gal_beidou_sig: {}", log.gal_beidou_sig);
                println!("gps_glonass_sig: {}", log.gps_glonass_sig);
                break;
            }
            _ => continue,
        }
    }
    Ok(())
}

fn get_errors(oem: &OEM6) -> OEMResult<()> {
    oem.request_errors().map_err(|err| {
        println!("{}: Failed to request error messages", line!());
        err
    })
}

fn unlog(oem: &OEM6) -> OEMResult<()> {
    oem.request_unlog(MessageID::BestXYZ).map_err(|err| {
        println!("{}: Failed to unlog message", line!());
        err
    })
}

fn unlog_all(oem: &OEM6) -> OEMResult<()> {
    oem.request_unlog_all(false).map_err(|err| {
        println!("{}: Failed to unlog everything", line!());
        err
    })
}

fn main() -> OEMResult<()> {
    println!("OEM6 GPS Test");

    let bus = "/dev/ttyS5";

    let (log_send, log_recv) = sync_channel(5);
    let (response_send, response_recv) = sync_channel(5);

    let oem = OEM6::new(bus, BaudRate::Baud9600, log_recv, response_recv)?;

    let rx_conn = oem.conn.clone();

    thread::spawn(move || read_thread(rx_conn, log_send, response_send));

    get_version(&oem)?;

    thread::sleep(Duration::from_millis(100));

    get_position(&oem)?;

    thread::sleep(Duration::from_millis(100));

    unlog(&oem)?;

    thread::sleep(Duration::from_millis(100));

    get_errors(&oem)?;

    thread::sleep(Duration::from_millis(100));

    unlog_all(&oem)?;

    println!("End of test");

    Ok(())
}
