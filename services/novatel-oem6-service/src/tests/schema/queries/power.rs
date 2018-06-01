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
fn get_power_on() {
    let mut mock = MockStream::default();

    mock.write.set_input(LOG_VERSION_COMMAND.to_vec());

    let mut output = LOG_RESPONSE_GOOD.to_vec();
    output.extend_from_slice(&VERSION_LOG);
    mock.read.set_output(output);

    let service = service_new!(mock);

    let query = r#"{
            power{
                state,
                uptime
            }
        }"#;

    let expected = json!({
            "power": {
                "state": "ON",
                "uptime": 1
            }
    });

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}

#[test]
fn get_power_off() {
    let mut mock = MockStream::default();

    mock.write.set_input(LOG_VERSION_COMMAND.to_vec());

    let service = service_new!(mock);

    let query = r#"{
            power{
                state,
                uptime
            }
        }"#;

    let expected = json!({
            "power": {
                "state": "OFF",
                "uptime": 0
            }
    });

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}
