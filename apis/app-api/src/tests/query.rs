/*
 * Copyright (C) 2018 Kubos Corporation
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#[test]
fn query_good() {
    mock_service!("0.0.0.0", 8765);

    let request = r#"{
            ping
        }"#;

    let expected = json!({
            "ping": "query"
        });

    let result = query("0.0.0.0:8765", request, Some(Duration::from_secs(1))).unwrap();

    assert_eq!(result, expected);
}

#[test]
fn query_error() {
    mock_service!("0.0.0.0", 8764);

    let request = r#"{
            ping(fail: true)
        }"#;

    let result = query("0.0.0.0:8765", request, Some(Duration::from_secs(1))).unwrap_err();

    let result_str = format!("{}", result);

    assert_eq!(result_str, "{\"message\":\"Query failed\",\"locations\":[{\"line\":2,\"column\":13}],\"path\":[\"ping\"]}");
}

#[test]
fn query_bad_addr() {
    let request = r#"{
            ping
        }"#;

    let result = query("0.0.0.0:1234", request, Some(Duration::from_secs(1))).unwrap_err();

    let result_str = format!("{}", result);

    assert_eq!(result_str, "Connection refused (os error 111)");
}

#[test]
fn query_mutation() {
    mock_service!("0.0.0.0", 8763);

    let request = r#"mutation {
            ping
        }"#;

    let expected = json!({
            "ping": "mutation"
        });

    let result = query("0.0.0.0:8763", request, Some(Duration::from_secs(1))).unwrap();

    assert_eq!(result, expected);
}
