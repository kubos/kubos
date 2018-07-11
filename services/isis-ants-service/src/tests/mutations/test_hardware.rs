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
    let mock = mock_new!();

    let nominal = AntsTelemetry {
        raw_temp: 15,
        uptime: 35,
        deploy_status: DeployStatus {
            sys_armed: true,
            ant_1_active: true,
            ant_4_not_deployed: false,
            ..Default::default()
        },
    };
    mock.get_system_telemetry.return_value(Ok(nominal.clone()));

    mock.get_activation_count
        .return_value_for(KANTSAnt::Ant1, Ok(1));
    mock.get_activation_time
        .return_value_for(KANTSAnt::Ant1, Ok(11));
    mock.get_activation_count
        .return_value_for(KANTSAnt::Ant2, Ok(2));
    mock.get_activation_time
        .return_value_for(KANTSAnt::Ant2, Ok(22));
    mock.get_activation_count
        .return_value_for(KANTSAnt::Ant3, Ok(3));
    mock.get_activation_time
        .return_value_for(KANTSAnt::Ant3, Ok(33));
    mock.get_activation_count
        .return_value_for(KANTSAnt::Ant4, Ok(4));
    mock.get_activation_time
        .return_value_for(KANTSAnt::Ant4, Ok(44));
    let service = service_new!(mock);

    let query = r#"
        mutation {
            testHardware(test: INTEGRATION) {
            ... on IntegrationTestResults {
                errors,
                success,
                telemetryNominal {
                     rawTemp,
                     uptime,
                     sysBurnActive,
                     sysIgnoreDeploy,
                     sysArmed,
                     ant1NotDeployed,
                     ant1StoppedTime,
                     ant1Active,
                     ant2NotDeployed,
                     ant2StoppedTime,
                     ant2Active,
                     ant3NotDeployed,
                     ant3StoppedTime,
                     ant3Active,
                     ant4NotDeployed,
                     ant4StoppedTime,
                     ant4Active
                },
                telemetryDebug {
                     ant1ActivationCount,
                     ant1ActivationTime,
                     ant2ActivationCount,
                     ant2ActivationTime,
                     ant3ActivationCount,
                     ant3ActivationTime,
                     ant4ActivationCount,
                     ant4ActivationTime,
                }
            }}
        }"#;

    let expected = json!({
        "testHardware": {
            "errors": "",
            "success": true,
            "telemetryNominal": {
                 "rawTemp": 15,
                 "uptime": 35,
                 "sysBurnActive": false,
                 "sysIgnoreDeploy": false,
                 "sysArmed": true,
                 "ant1NotDeployed": false,
                 "ant1StoppedTime": false,
                 "ant1Active": true,
                 "ant2NotDeployed": false,
                 "ant2StoppedTime": false,
                 "ant2Active": false,
                 "ant3NotDeployed": false,
                 "ant3StoppedTime": false,
                 "ant3Active": false,
                 "ant4NotDeployed": false,
                 "ant4StoppedTime": false,
                 "ant4Active": false
            },
            "telemetryDebug": {
                 "ant1ActivationCount": 1,
                 "ant1ActivationTime": 11,
                 "ant2ActivationCount": 2,
                 "ant2ActivationTime": 22,
                 "ant3ActivationCount": 3,
                 "ant3ActivationTime": 33,
                 "ant4ActivationCount": 4,
                 "ant4ActivationTime": 44,
            }
        }
    });

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}

#[test]
fn integration_test_bad() {
    let mock = mock_new!();

    mock.get_activation_count
        .return_value_for(KANTSAnt::Ant1, Ok(1));
    mock.get_activation_time
        .return_value_for(KANTSAnt::Ant1, Ok(11));
    mock.get_activation_count
        .return_value_for(KANTSAnt::Ant2, Ok(2));
    mock.get_activation_time
        .return_value_for(KANTSAnt::Ant2, Ok(22));
    mock.get_activation_count
        .return_value_for(KANTSAnt::Ant3, Ok(3));
    mock.get_activation_time
        .return_value_for(KANTSAnt::Ant3, Ok(33));
    mock.get_activation_count
        .return_value_for(KANTSAnt::Ant4, Ok(4));
    mock.get_activation_time
        .return_value_for(KANTSAnt::Ant4, Ok(44));

    let service = service_new!(mock);

    let query = r#"
        mutation {
            testHardware(test: INTEGRATION) {
            ... on IntegrationTestResults {
                errors,
                success,
                telemetryNominal {
                     rawTemp,
                     uptime,
                     sysBurnActive,
                     sysIgnoreDeploy,
                     sysArmed,
                     ant1NotDeployed,
                     ant1StoppedTime,
                     ant1Active,
                     ant2NotDeployed,
                     ant2StoppedTime,
                     ant2Active,
                     ant3NotDeployed,
                     ant3StoppedTime,
                     ant3Active,
                     ant4NotDeployed,
                     ant4StoppedTime,
                     ant4Active
                },
                telemetryDebug {
                     ant1ActivationCount,
                     ant1ActivationTime,
                     ant2ActivationCount,
                     ant2ActivationTime,
                     ant3ActivationCount,
                     ant3ActivationTime,
                     ant4ActivationCount,
                     ant4ActivationTime,
                }
            }}
        }"#;

    let expected = json!({
        "testHardware": {
            "errors": "Nominal: Configuration error",
            "success": false,
            "telemetryNominal": {
                 "rawTemp": 0,
                 "uptime": 0,
                 "sysBurnActive": false,
                 "sysIgnoreDeploy": false,
                 "sysArmed": false,
                 "ant1NotDeployed": false,
                 "ant1StoppedTime": false,
                 "ant1Active": false,
                 "ant2NotDeployed": false,
                 "ant2StoppedTime": false,
                 "ant2Active": false,
                 "ant3NotDeployed": false,
                 "ant3StoppedTime": false,
                 "ant3Active": false,
                 "ant4NotDeployed": false,
                 "ant4StoppedTime": false,
                 "ant4Active": false
            },
            "telemetryDebug": {
                 "ant1ActivationCount": 1,
                 "ant1ActivationTime": 11,
                 "ant2ActivationCount": 2,
                 "ant2ActivationTime": 22,
                 "ant3ActivationCount": 3,
                 "ant3ActivationTime": 33,
                 "ant4ActivationCount": 4,
                 "ant4ActivationTime": 44,
            }
        }
    });

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}

#[test]
fn hardware_test() {
    let mock = mock_new!();
    let service = service_new!(mock);

    let query = r#"
        mutation {
            testHardware(test: HARDWARE) {
            ... on HardwareTestResults {
                errors,
                success,
                data
            }}
        }"#;

    let expected = json!({
        "testHardware": {
            "errors": "Not Implemented",
            "success": true,
            "data": ""
        }
    });

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}
