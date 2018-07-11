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
fn debug_telem_good() {
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
        {
            debug: telemetry(telem: DEBUG) {
            ... on TelemetryDebug {
                 ant1ActivationCount,
                 ant1ActivationTime,
                 ant2ActivationCount,
                 ant2ActivationTime,
                 ant3ActivationCount,
                 ant3ActivationTime,
                 ant4ActivationCount,
                 ant4ActivationTime,
            }
            }
        }"#;

    let expected = json!({
            "debug": {
                 "ant1ActivationCount": 1,
                 "ant1ActivationTime": 11,
                 "ant2ActivationCount": 2,
                 "ant2ActivationTime": 22,
                 "ant3ActivationCount": 3,
                 "ant3ActivationTime": 33,
                 "ant4ActivationCount": 4,
                 "ant4ActivationTime": 44,
            }
    });

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}

#[test]
fn debug_telem_bad() {
    let mock = mock_new!();

    mock.get_activation_count
        .return_value(Err(AntsError::GenericError));
    mock.get_activation_time
        .return_value(Err(AntsError::GenericError));

    let service = service_new!(mock);

    let query = r#"
        {
            debug: telemetry(telem: DEBUG) {
            ... on TelemetryDebug {
                 ant1ActivationCount,
                 ant1ActivationTime,
                 ant2ActivationCount,
                 ant2ActivationTime,
                 ant3ActivationCount,
                 ant3ActivationTime,
                 ant4ActivationCount,
                 ant4ActivationTime,
            }}
        }"#;

    let expected = json!({
            "debug": {
                 "ant1ActivationCount": 0,
                 "ant1ActivationTime": 0,
                 "ant2ActivationCount": 0,
                 "ant2ActivationTime": 0,
                 "ant3ActivationCount": 0,
                 "ant3ActivationTime": 0,
                 "ant4ActivationCount": 0,
                 "ant4ActivationTime": 0,
            }
    });

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}
