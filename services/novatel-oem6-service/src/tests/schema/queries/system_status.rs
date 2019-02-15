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

use super::*;
use serde_json::json;

#[test]
fn get_system_status_good() {
    let mut mock = MockStream::default();

    mock.write.set_input(LOG_VERSION_COMMAND.to_vec());

    let mut output = LOG_RESPONSE_GOOD.to_vec();
    output.extend_from_slice(&VERSION_LOG);
    mock.read.set_output(output);

    let service = service_new!(mock);

    let query = r#"{
            systemStatus {
                errors,
                status
            }
        }"#;

    let expected = json!({
            "systemStatus": {
                "errors": [],
                "status": ["POSITION_SOLUTION_INVALID", "CLOCK_MODEL_INVALID"]
            }
    });

    test!(service, query, expected);
}

#[test]
fn get_system_status_good_with_error() {
    let mut mock = MockStream::default();

    mock.write.set_input(LOG_VERSION_COMMAND.to_vec());

    let mut output = ERROR_LOG.to_vec();
    output.extend_from_slice(&LOG_RESPONSE_GOOD);
    output.extend_from_slice(&VERSION_LOG);
    mock.read.set_output(output);

    let service = service_new!(mock);

    let query = r#"{
            systemStatus {
                errors,
                status
            }
        }"#;

    let expected = json!({
            "systemStatus": {
                "errors": ["RxStatusEvent(1, 19, 1): No Valid Position Calculated"],
                "status": ["POSITION_SOLUTION_INVALID", "CLOCK_MODEL_INVALID"]
            }
    });

    test!(service, query, expected);
}

#[test]
fn get_system_status_bad() {
    let mut mock = MockStream::default();

    mock.write.set_input(LOG_VERSION_COMMAND.to_vec());

    mock.read.set_output(LOG_RESPONSE_GOOD.to_vec());

    let service = service_new!(mock);

    let query = r#"{
            systemStatus {
                errors,
                status
            }
        }"#;

    let expected = json!({
            "systemStatus": {
                "errors": ["System Status: Failed to receive version info - timed out waiting on channel"],
                "status": [
                           "ERROR_PRESENT",
                           "TEMPERATURE_WARNING",
                           "VOLTAGE_SUPPLY_WARNING",
                           "ANTENNA_NOT_POWERED",
                           "LNA_FAILURE",
                           "ANTENNA_OPEN",
                           "ANTENNA_SHORTENED",
                           "CPU_OVERLOAD",
                           "COM1_BUFFER_OVERRUN",
                           "COM2_BUFFER_OVERRUN",
                           "COM3_BUFFER_OVERRUN",
                           "LINK_OVERRUN",
                           "AUX_TRANSMIT_OVERRUN",
                           "AGC_OUT_OF_RANGE",
                           "INS_RESET",
                           "GPS_ALMANAC_INVALID",
                           "POSITION_SOLUTION_INVALID",
                           "POSITION_FIXED",
                           "CLOCK_STEERING_DISABLED",
                           "CLOCK_MODEL_INVALID",
                           "EXTERNAL_OSCILLATOR_LOCKED",
                           "SOFTWARE_RESOURCE_WARNING",
                           "AUX3_STATUS_EVENT",
                           "AUX2_STATUS_EVENT",
                           "AUX1_STATUS_EVENT"
                           ]
            }
    });

    test!(service, query, expected);
}
