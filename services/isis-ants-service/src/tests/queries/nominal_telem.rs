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
fn nominal_telem_good() {
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

    let service = service_new!(mock);

    let query = r#"
        {
            nominal: telemetry(telem: NOMINAL) {
            ... on TelemetryNominal {
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
                }
            }
        }"#;

    let expected = json!({
            "nominal": {
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
            }
    });

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}

#[test]
fn nominal_telem_nondefault() {
    let mock = mock_new!();

    let nominal = AntsTelemetry {
        raw_temp: 15,
        uptime: 35,
        deploy_status: DeployStatus {
            sys_burn_active: true,
            sys_ignore_deploy: true,
            sys_armed: true,
            ant_1_not_deployed: true,
            ant_1_stopped_time: true,
            ant_1_active: true,
            ant_2_not_deployed: true,
            ant_2_stopped_time: true,
            ant_2_active: true,
            ant_3_not_deployed: true,
            ant_3_stopped_time: true,
            ant_3_active: true,
            ant_4_not_deployed: true,
            ant_4_stopped_time: true,
            ant_4_active: true,
        },
    };
    mock.get_system_telemetry.return_value(Ok(nominal.clone()));

    let service = service_new!(mock);

    let query = r#"
        {
            nominal: telemetry(telem: NOMINAL) {
            ... on TelemetryNominal {
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
                }
            }
        }"#;

    let expected = json!({
            "nominal": {
                 "rawTemp": 15,
                 "uptime": 35,
                 "sysBurnActive": true,
                 "sysIgnoreDeploy": true,
                 "sysArmed": true,
                 "ant1NotDeployed": true,
                 "ant1StoppedTime": true,
                 "ant1Active": true,
                 "ant2NotDeployed": true,
                 "ant2StoppedTime": true,
                 "ant2Active": true,
                 "ant3NotDeployed": true,
                 "ant3StoppedTime": true,
                 "ant3Active": true,
                 "ant4NotDeployed": true,
                 "ant4StoppedTime": true,
                 "ant4Active": true
            }
    });

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}

#[test]
fn nominal_telem_bad() {
    let mock = mock_new!();

    mock.get_system_telemetry
        .return_value(Err(AntsError::GenericError));

    let service = service_new!(mock);

    let query = r#"
        {
            nominal: telemetry(telem: NOMINAL) {
            ... on TelemetryNominal {
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
                }
            }
        }"#;

    let expected = json!({
            "nominal": {
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
                }
    });

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}
