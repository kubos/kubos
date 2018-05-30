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

// TODO: Add telemetryNominal results

use super::*;

#[test]
fn test_hardware_integration_good() {
    let mut mock = MockStream::default();

    mock.write.set_input(LOG_VERSION_COMMAND.to_vec());

    let mut output = LOG_RESPONSE_GOOD.to_vec();
    output.extend_from_slice(&VERSION_LOG);
    mock.read.set_output(output);

    let service = service_new!(mock);

    let query = r#"mutation {
            testHardware(test: INTEGRATION) {
                ... on IntegrationTestResults {
                    errors,
                    success,
                    telemetryDebug {
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
            }
        }"#;

    let expected = json!({
            "testHardware": {
                "errors": "",
                "success": true,
                "telemetryDebug": {
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

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}

#[test]
fn test_hardware_integration_no_response() {
    let mut mock = MockStream::default();

    mock.write.set_input(LOG_VERSION_COMMAND.to_vec());

    let service = service_new!(mock);

    let query = r#"mutation {
            testHardware(test: INTEGRATION) {
                ... on IntegrationTestResults {
                    errors,
                    success,
                    telemetryDebug {
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
            }
        }"#;

    let expected = json!({
            "testHardware": {
                "errors": "Failed to get command response",
                "success": false,
                "telemetryDebug": null,
            }
    });

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}

#[test]
fn test_hardware_integration_no_log() {
    let mut mock = MockStream::default();

    mock.write.set_input(LOG_VERSION_COMMAND.to_vec());

    mock.read.set_output(LOG_RESPONSE_GOOD.to_vec());

    let service = service_new!(mock);

    let query = r#"mutation {
            testHardware(test: INTEGRATION) {
                ... on IntegrationTestResults {
                    errors,
                    success,
                    telemetryDebug {
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
            }
        }"#;

    let expected = json!({
            "testHardware": {
                "errors": "Failed to receive version info - timed out waiting on channel",
                "success": false,
                "telemetryDebug": null,
            }
    });

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}

#[test]
fn test_hardware_hardware() {
    let mut mock = MockStream::default();

    let service = service_new!(mock);

    let query = r#"mutation {
            testHardware(test: HARDWARE) {
                ... on HardwareTestResults {
                    data,
                    errors,
                    success
                }
            }
        }"#;

    let expected = json!({
            "testHardware": {
                "data": "",
                "errors": "Not Implemented",
                "success": true
            }
    });

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}
