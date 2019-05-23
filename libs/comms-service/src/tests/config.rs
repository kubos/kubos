//
// Copyright (C) 2019 Kubos Corporation
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

use crate::config::*;
use crate::errors::*;
use crate::service::*;
use std::sync::Arc;

fn test_read(_read_conn: &u8) -> CommsResult<Vec<u8>> {
    Ok(vec![])
}
fn test_write(_write_conn: &u8, _data: &[u8]) -> CommsResult<()> {
    Ok(())
}

#[test]
fn config_full_good() {
    let config = kubos_system::Config::new_from_str(
        "comms-service",
        r#"
        [comms-service.comms]
        handler_port_min = 1300
        handler_port_max = 1310
        downlink_ports = [14011]
        timeout = 1400
        ip = "0.0.0.0"
        "#,
    );

    let config = CommsConfig::new(config).unwrap();

    let read_conn = 1;
    let write_conn = 2;

    let result = CommsControlBlock::new(
        Some(Arc::new(test_read)),
        vec![Arc::new(test_write)],
        read_conn,
        write_conn,
        config,
    );

    assert!(result.is_ok());
}

#[test]
fn config_no_comms_ip() {
    let config = kubos_system::Config::new_from_str(
        "comms-service",
        r#"
        [comms-service.comms]
        downlink_ports = [14011, 14000]
        "#,
    );

    let result = CommsConfig::new(config);

    assert_eq!(
        format!("{}", result.unwrap_err()),
        "Config error: Failed to parse config: missing field `ip`"
    );
}

#[test]
fn config_write_downlink_mismatch() {
    let config = kubos_system::Config::new_from_str(
        "comms-service",
        r#"
        [comms-service.comms]
        downlink_ports = [14011, 14012]
        ip = "0.0.0.0"
        "#,
    );

    let config = CommsConfig::new(config).unwrap();

    let read_conn = 1;
    let write_conn = 2;

    let result = CommsControlBlock::new(
        Some(Arc::new(test_read)),
        vec![Arc::new(test_write)],
        read_conn,
        write_conn,
        config,
    );

    assert_eq!(
        format!("{}", result.unwrap_err()),
        "Config error: There must be a unique write function for each downlink port"
    );
}
