//
// Copyright (C) 2019 Kubos Corporation
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

// Check the current RAM and disk usage
//
// - If RAM usage is too high (> RAM_CRITICAL%), reboot the system
// - If disk usage is too high (> DISK_CRITICAL%), delete all telemetry database entries older than
//   a day (vs the usual week lifespan)

use super::*;
use std::process::Command;

const OBC_TELEMETRY: &str = r#"{
    memInfo {
        available
    }
}"#;

pub fn check_mem() -> Result<(), Error> {
    // Check RAM usage as reported by the monitor service
    let service = ServiceConfig::new("monitor-service");

    let result = query(&service, OBC_TELEMETRY, Some(QUERY_TIMEOUT))?;

    let mem = result["memInfo"]["available"].as_u64().unwrap_or_else(|| {
        error!("Failed to fetch available memory");
        0
    });

    // Convert to percentage in use, since that's an easier number to work with
    let ram_in_use = (100 - mem * 100 / MEM_TOTAL) as u8;

    // Check RAM usage with respect to thresholds
    if ram_in_use < RAM_NOMINAL {
        info!("RAM usage nominal: {}%", ram_in_use);
    } else if ram_in_use < RAM_HIGH {
        info!("RAM usage high, but acceptable: {}%", ram_in_use);
    } else if ram_in_use < RAM_CRITICAL {
        warn!("RAM usage high: {}%", ram_in_use);
    } else {
        error!("RAM usage too high: {}%. Triggering reboot", ram_in_use);
        // Reboot the system not-nicely. If we're at this point, there's probably a rogue process
        // that's hogging all the system resources and not playing nicely with others.
        Command::new("reboot").arg("-f").status()?;
    }

    // Check disk space usage
    // Get the % of the user data partition that's free
    let disk_in_use = if let Ok(output) = Command::new("df").arg(USER_PARTITION).output() {
        let stdout = if output.stderr.is_empty() {
            output.stdout
        } else {
            vec![]
        };

        let mut slices = stdout.rsplit(|&elem| elem == b' ');

        // The last entry is the mount point (/home)
        slices.next();
        // The second to last entry is the percent in use
        let temp = slices.next();
        // Convert it to a useable number
        let percent = temp
            .unwrap_or(&[])
            .iter()
            .filter_map(|&elem| {
                if elem.is_ascii_digit() {
                    Some(elem as char)
                } else {
                    None
                }
            })
            .collect::<String>();

        percent.parse::<u8>().unwrap_or_else(|err| {
            error!("Failed to parse current disk usage info: {:?}", err);
            100
        })
    } else {
        error!("Failed to get current disk usage info");
        100
    };

    // Check disk usage with respect to threshold
    if disk_in_use < DISK_NOMINAL {
        info!("Disk usage nominal: {}%", disk_in_use);
    } else if disk_in_use < DISK_HIGH {
        info!("Disk usage high, but acceptable: {}%", disk_in_use);
    } else if disk_in_use < DISK_CRITICAL {
        warn!("Disk usage high: {}%", disk_in_use);
    } else {
        error!("Disk usage too high: {}%. Triggering cleanup", disk_in_use);
        // Delete everything from the database that's more than the critical age threshold
        clean_db::clean_db(CRITICAL_AGE)?;

        // Reboot the system nicely. This will clean up any temporary storage which is being used
        // (/tmp), freeing up some additional disk space.
        Command::new("reboot").status()?;
    }

    Ok(())
}
