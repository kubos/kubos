use kubos_service::{Config, Service};

mod mock_service;

use self::mock_service::*;
use std::thread;
use std::time::Duration;
use query::query;

macro_rules! mock_service {
    ($addr:expr, $port:expr) => {{
        thread::spawn(|| {
            let config = format!(
                r#"
                [mock-service.addr]
                ip = "{}"
                port = {}
                "#, $addr, $port);
            Service::new(
                Config::new_from_str(
                    "mock-service",
                    &config
                ),
                Subsystem,
                QueryRoot,
                MutationRoot,
            ).start()
        });

        thread::sleep(Duration::from_millis(100));
    }};
}

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
