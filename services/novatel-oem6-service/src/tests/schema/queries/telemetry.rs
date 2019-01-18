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
fn get_telemetry_nominal() {
    let mut mock = MockStream::default();

    mock.write.set_input(LOG_VERSION_COMMAND.to_vec());

    let mut output = POSITION_LOG_GOOD.to_vec();
    output.extend_from_slice(&LOG_RESPONSE_GOOD.to_vec());
    output.extend_from_slice(&VERSION_LOG);
    mock.read.set_output(output);

    let service = service_new!(mock);

    let query = r#"{
            telemetry{
                nominal{
                    lockInfo {
                        position,
                        time {
                            ms,
                            week
                        },
                        velocity
                    },
                    lockStatus {
                        positionStatus,
                        positionType,
                        time {
                            ms,
                            week
                        },
                        timeStatus,
                        velocityStatus,
                        velocityType
                    },
                    systemStatus {
                        errors,
                        status
                    }
                }
            }
        }"#;

    let expected = json!({
            "telemetry": {
                "nominal": {
                    "lockInfo": {
                        "position": [1.1, 2.2, 3.3],
                        "time": {
                            "ms": 164195000,
                            "week": 3025
                        },
                        "velocity": [4.4, 5.5, 6.6],
                    },
                    "lockStatus": {
                        "positionStatus": "SOL_COMPUTED",
                        "positionType": "PSRDIFF",
                        "time": {
                            "ms": 164195000,
                            "week": 3025
                        },
                        "timeStatus": "FINE_STEERING",
                        "velocityStatus": "SOL_COMPUTED",
                        "velocityType": "PSRDIFF"
                    },
                    "systemStatus": {
                        "errors": [],
                        "status": ["POSITION_SOLUTION_INVALID", "CLOCK_MODEL_INVALID"]
                    }
                }
            }
    });

    assert_eq!(service.process(&query.to_owned()), wrap!(expected));
}

#[test]
fn get_telemetry_debug() {
    let mut mock = MockStream::default();

    mock.write.set_input(LOG_VERSION_COMMAND.to_vec());

    let mut output = LOG_RESPONSE_GOOD.to_vec();
    output.extend_from_slice(&VERSION_LOG);
    mock.read.set_output(output);

    let service = service_new!(mock);

    let query = r#"{
            telemetry{
                debug {
                    components {
                        bootVersion,
                        compType,
                        compileDate,
                        compileTime, 
                        hwVersion,
                        model,
                        serialNum,
                        swVersion,
                    },
                    numComponents
                }
            }
        }"#;

    let expected = json!({
            "telemetry": {
                "debug": {
                    "components": [{
                        "bootVersion": "OEM060201RB0000",
                        "compType": 1,
                        "compileDate": "2015/Jan/28",
                        "compileTime": "15:27:29",
                        "hwVersion": "OEM615-2.00",
                        "model": "G1SB0GTT0",
                        "serialNum": "BJYA15120038H",
                        "swVersion": "OEM060600RN0000",
                    }],
                    "numComponents": 1
                }
            }
    });

    assert_eq!(service.process(&query.to_owned()), wrap!(expected));
}

#[test]
fn get_telemetry_both() {
    let mut mock = MockStream::default();

    mock.write.set_input(LOG_VERSION_COMMAND.to_vec());

    let mut output = POSITION_LOG_GOOD.to_vec();
    output.extend_from_slice(&LOG_RESPONSE_GOOD.to_vec());
    output.extend_from_slice(&VERSION_LOG);
    mock.read.set_output(output);

    let service = service_new!(mock);

    let query = r#"{
            telemetry{
                debug {
                    components {
                        bootVersion,
                        compType,
                        compileDate,
                        compileTime, 
                        hwVersion,
                        model,
                        serialNum,
                        swVersion,
                    },
                    numComponents
                },
                nominal{
                    lockInfo {
                        position,
                        time {
                            ms,
                            week
                        },
                        velocity
                    },
                    lockStatus {
                        positionStatus,
                        positionType,
                        time {
                            ms,
                            week
                        },
                        timeStatus,
                        velocityStatus,
                        velocityType
                    },
                    systemStatus {
                        errors,
                        status
                    }
                }
            }
        }"#;

    let expected = json!({
            "telemetry": {
                "debug": {
                    "components": [{
                        "bootVersion": "OEM060201RB0000",
                        "compType": 1,
                        "compileDate": "2015/Jan/28",
                        "compileTime": "15:27:29",
                        "hwVersion": "OEM615-2.00",
                        "model": "G1SB0GTT0",
                        "serialNum": "BJYA15120038H",
                        "swVersion": "OEM060600RN0000",
                    }],
                    "numComponents": 1
                },
                "nominal": {
                    "lockInfo": {
                        "position": [1.1, 2.2, 3.3],
                        "time": {
                            "ms": 164195000,
                            "week": 3025
                        },
                        "velocity": [4.4, 5.5, 6.6],
                    },
                    "lockStatus": {
                        "positionStatus": "SOL_COMPUTED",
                        "positionType": "PSRDIFF",
                        "time": {
                            "ms": 164195000,
                            "week": 3025
                        },
                        "timeStatus": "FINE_STEERING",
                        "velocityStatus": "SOL_COMPUTED",
                        "velocityType": "PSRDIFF"
                    },
                    "systemStatus": {
                        "errors": [],
                        "status": ["POSITION_SOLUTION_INVALID", "CLOCK_MODEL_INVALID"]
                    }
                }
            }
    });

    assert_eq!(service.process(&query.to_owned()), wrap!(expected));
}

#[test]
fn get_telemetry_empty() {
    let mut mock = MockStream::default();

    mock.write.set_input(LOG_VERSION_COMMAND.to_vec());

    mock.read.set_output(LOG_RESPONSE_GOOD.to_vec());

    let service = service_new!(mock);

    let query = r#"{
            telemetry{
                debug {
                    components {
                        bootVersion,
                        compType,
                        compileDate,
                        compileTime, 
                        hwVersion,
                        model,
                        serialNum,
                        swVersion,
                    },
                    numComponents
                },
                nominal{
                    lockInfo {
                        position,
                        time {
                            ms,
                            week
                        },
                        velocity
                    },
                    lockStatus {
                        positionStatus,
                        positionType,
                        time {
                            ms,
                            week
                        },
                        timeStatus,
                        velocityStatus,
                        velocityType
                    },
                    systemStatus {
                        errors,
                        status
                    }
                }
            }
        }"#;

    let expected = json!({
            "telemetry": {
                "debug":null,
                "nominal": {
                    "lockInfo": {
                        "position": [0.0, 0.0, 0.0],
                        "time": {
                            "ms": 0,
                            "week": 0,
                        },
                        "velocity": [0.0, 0.0, 0.0],
                    },
                    "lockStatus": {
                        "positionStatus": "INSUFFICIENT_OBSERVATIONS",
                        "positionType": "NONE",
                        "time": {
                            "ms": 0,
                            "week": 0
                        },
                        "timeStatus": "UNKNOWN",
                        "velocityStatus": "INSUFFICIENT_OBSERVATIONS",
                        "velocityType": "NONE"
                    },
                    "systemStatus": {
                        "errors": ["Get Telemetry: Failed to receive version info - timed out waiting on channel"],
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
                }
            }
    });

    assert_eq!(service.process(&query.to_owned()), wrap!(expected));
}
