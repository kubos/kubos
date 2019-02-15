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

#[test]
fn integration_test_good() {
    let mut mock = mock_new!();
    mock.state = true;

    let service = service_new!(mock);

    let query = r#"
        mutation {
            testHardware(test: INTEGRATION) {
            ... on IntegrationTestResults {
                errors,
                success,
                telemetryDebug {
                     ant1ActivationCount,
                     ant1ActivationTime,
                     ant2ActivationCount,
                     ant2ActivationTime,
                     ant3ActivationCount,
                     ant3ActivationTime,
                     ant4ActivationCount,
                     ant4ActivationTime,
                },
                telemetryNominal {
                     ant1Active,
                     ant1NotDeployed,
                     ant1StoppedTime,
                     ant2Active,
                     ant2NotDeployed,
                     ant2StoppedTime,
                     ant3Active,
                     ant3NotDeployed,
                     ant3StoppedTime,
                     ant4Active,
                     ant4NotDeployed,
                     ant4StoppedTime,
                     rawTemp,
                     sysArmed,
                     sysBurnActive,
                     sysIgnoreDeploy,
                     uptime,
                },
            }}
        }"#;

    let expected = json!({
        "testHardware": {
            "errors": "",
            "success": true,
            "telemetryDebug": {
                 "ant1ActivationCount": 1,
                 "ant1ActivationTime": 11,
                 "ant2ActivationCount": 2,
                 "ant2ActivationTime": 22,
                 "ant3ActivationCount": 3,
                 "ant3ActivationTime": 33,
                 "ant4ActivationCount": 4,
                 "ant4ActivationTime": 44,
            },
            "telemetryNominal": {
                 "ant1Active": true,
                 "ant1NotDeployed": false,
                 "ant1StoppedTime": false,
                 "ant2Active": false,
                 "ant2NotDeployed": false,
                 "ant2StoppedTime": false,
                 "ant3Active": false,
                 "ant3NotDeployed": false,
                 "ant3StoppedTime": false,
                 "ant4Active": false,
                 "ant4NotDeployed": false,
                 "ant4StoppedTime": false,
                 "rawTemp": 15,
                 "sysArmed": true,
                 "sysBurnActive": false,
                 "sysIgnoreDeploy": false,
                 "uptime": 35,
            },
        }
    });

    test!(service, query, expected);
}

#[test]
fn integration_test_bad() {
    let mut mock = mock_new!();
    mock.state = false;

    let service = service_new!(mock);

    let query = r#"
        mutation {
            testHardware(test: INTEGRATION) {
            ... on IntegrationTestResults {
                errors,
                success,
                telemetryDebug {
                     ant1ActivationCount,
                     ant1ActivationTime,
                     ant2ActivationCount,
                     ant2ActivationTime,
                     ant3ActivationCount,
                     ant3ActivationTime,
                     ant4ActivationCount,
                     ant4ActivationTime,
                },
                telemetryNominal {
                     ant1Active,
                     ant1NotDeployed,
                     ant1StoppedTime,
                     ant2Active,
                     ant2NotDeployed,
                     ant2StoppedTime,
                     ant3Active,
                     ant3NotDeployed,
                     ant3StoppedTime,
                     ant4Active,
                     ant4NotDeployed,
                     ant4StoppedTime,
                     rawTemp,
                     sysArmed,
                     sysBurnActive,
                     sysIgnoreDeploy,
                     uptime,
                },
            }}
        }"#;

    let expected = json!({
        "testHardware": {
            "errors": "Nominal: Generic error; Debug: get_activation_count (services/isis-ants-service/src/model.rs:311): Generic error, \
                get_activation_time (services/isis-ants-service/src/model.rs:313): Generic error, \
                get_activation_count (services/isis-ants-service/src/model.rs:317): Generic error, \
                get_activation_time (services/isis-ants-service/src/model.rs:319): Generic error, \
                get_activation_count (services/isis-ants-service/src/model.rs:323): Generic error, \
                get_activation_time (services/isis-ants-service/src/model.rs:325): Generic error, \
                get_activation_count (services/isis-ants-service/src/model.rs:329): Generic error, \
                get_activation_time (services/isis-ants-service/src/model.rs:331): Generic error",
            "success": false,
            "telemetryDebug": {
                 "ant1ActivationCount": 0,
                 "ant1ActivationTime": 0,
                 "ant2ActivationCount": 0,
                 "ant2ActivationTime": 0,
                 "ant3ActivationCount": 0,
                 "ant3ActivationTime": 0,
                 "ant4ActivationCount": 0,
                 "ant4ActivationTime": 0,
            },
            "telemetryNominal": {
                 "ant1Active": false,
                 "ant1NotDeployed": false,
                 "ant1StoppedTime": false,
                 "ant2Active": false,
                 "ant2NotDeployed": false,
                 "ant2StoppedTime": false,
                 "ant3Active": false,
                 "ant3NotDeployed": false,
                 "ant3StoppedTime": false,
                 "ant4Active": false,
                 "ant4NotDeployed": false,
                 "ant4StoppedTime": false,
                 "rawTemp": 0,
                 "sysArmed": false,
                 "sysBurnActive": false,
                 "sysIgnoreDeploy": false,
                 "uptime": 0,
            },
        }
    });

    test!(service, query, expected);
}

#[test]
fn hardware_test() {
    let mock = mock_new!();
    let service = service_new!(mock);

    let query = r#"
        mutation {
            testHardware(test: HARDWARE) {
            ... on HardwareTestResults {
                data,
                errors,
                success,
                
            }}
        }"#;

    let expected = json!({
        "testHardware": {
            "data": "",
            "errors": "Not Implemented",
            "success": true,

        }
    });

    test!(service, query, expected);
}
