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
fn config_default() {
    let mock = mock_new!();
    let service = service_new!(mock);

    let query = r#"
        {
            config
        }"#;

    let expected = json!({
            "config": "PRIMARY"
    });

    assert_eq!(service.process(&query.to_owned()), wrap!(expected));
}

#[test]
fn config_primary() {
    let mock = mock_new!();

    mock.configure.return_value(Ok(()));

    let service = service_new!(mock);

    let mutation = r#"mutation {
            configureHardware(config: SECONDARY) {
                success
            }
        }"#;

    service.process(&mutation.to_owned());

    let mutation = r#"mutation {
            configureHardware(config: PRIMARY) {
                success
            }
        }"#;

    service.process(&mutation.to_owned());

    let query = r#"
        {
            config
        }"#;

    let expected = json!({
            "config": "PRIMARY"
    });

    assert_eq!(service.process(&query.to_owned()), wrap!(expected));
}

#[test]
fn config_secondary() {
    let mock = mock_new!();

    mock.configure.return_value(Ok(()));

    let service = service_new!(mock);

    let mutation = r#"mutation {
            configureHardware(config: SECONDARY) {
                success
            }
        }"#;

    service.process(&mutation.to_owned());

    let query = r#"
        {
            config
        }"#;

    let expected = json!({
            "config": "SECONDARY"
    });

    assert_eq!(service.process(&query.to_owned()), wrap!(expected));
}
