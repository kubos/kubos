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

// Check for OBC resets. Issue error message if reset is discovered
//
// Note: This will issue an error message whenever the system is first started, even if
// power-on/reboot was done on purpose. There's not an easy way to tell the difference between
// intentional and spurious reboots

use super::*;
use std::process::Command;

pub fn check_reset() -> Result<(), Error> {
    // If we're here, that means one of two things:
    // 1. The system just started up
    // 2. This app was restarted

    // Get the current uptime
    let uptime = if let Ok(output) = Command::new("cat").arg("/proc/uptime").output() {
        if !output.stderr.is_empty() {
            bail!(
                "Failed to get system uptime: {}",
                ::std::str::from_utf8(&output.stderr).unwrap_or("n/a")
            );
        }

        let mut slices = output.stdout.split(|&elem| elem == b' ');

        // The first entry is the overall system uptime
        let temp = slices
            .next()
            .ok_or_else(|| err_msg("Failed to get system uptime"))?;
        // Convert it to a useable number
        let uptime = ::std::str::from_utf8(&temp)?;
        uptime.parse::<f32>()?
    } else {
        bail!("Failed to get system uptime");
    };

    // If the uptime is less than 30 seconds, we'll assume that the entire system was restarted,
    // rather than just this app
    if uptime < 30.0 {
        error!("System reset observed");
    }

    Ok(())
}
