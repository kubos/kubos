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
fn issue_raw_command_good() {
    let mut mock = MockStream::default();

    mock.write.set_input(vec![1, 2, 3, 4, 0xFA]);

    let service = service_new!(mock);

    let query = r#"mutation {
            issueRawCommand(command: "01020304FA"){
                errors,
                success
            }
        }"#;

    let expected = json!({
            "issueRawCommand": {
                "errors": "",
                "success": true
            }
    });

    assert_eq!(service.process(&query.to_owned()), wrap!(expected));
}

#[test]
fn issue_raw_command_bad() {
    let mut mock = MockStream::default();

    let service = service_new!(mock);

    let query = r#"mutation {
            issueRawCommand(command: "01020304FA"){
                errors,
                success
            }
        }"#;

    let expected = json!({
            "issueRawCommand": {
                "errors": "UART Error, Generic Error",
                "success": false
            }
    });

    assert_eq!(service.process(&query.to_owned()), wrap!(expected));
}
