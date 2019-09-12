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

// Clean out all telemetry database entries older than the requested age
//
// During normal operations, the default threshold will be a week.
// If the system is low on disk space, the default threshold will be a day.

use super::*;

pub fn clean_db(age: f64) -> Result<(), Error> {
    // Get the current timestamp
    let time = time::now_utc().to_timespec();
    // Convert it to fractional seconds and calculate the timestamp for the requested age
    let timestamp = time.sec as f64 + (f64::from(time.nsec) / 1_000_000_000.0) - age;

    // Request that the telemetry service remove everything older than a week
    let telem_service = ServiceConfig::new("telemetry-service")?;

    let request = format!(
        r#"mutation {{
        delete(timestampLe: {}) {{
            success,
            errors,
            entriesDeleted
        }}
    }}"#,
        timestamp
    );

    let response = query(&telem_service, &request, Some(QUERY_TIMEOUT))?;

    // Check the results
    let data = response
        .get("delete")
        .ok_or_else(|| err_msg("Failed to get delete response"))?;
    let success = data.get("success").and_then(|val| val.as_bool());

    if success == Some(true) {
        let count = data
            .get("entriesDeleted")
            .and_then(|val| val.as_u64())
            .unwrap_or(0);
        info!("Deleted {} telemetry entries", count);
    } else {
        match data.get("errors") {
            Some(errors) => bail!("Failed to delete telemetry entries: {}", errors),
            None => bail!("Failed to delete telemetry entries"),
        };
    }

    Ok(())
}
