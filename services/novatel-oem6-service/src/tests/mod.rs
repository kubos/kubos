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

use novatel_oem6_api::mock::*;
use novatel_oem6_api::*;

#[macro_export]
/// Mock a service for the OEM6.
macro_rules! service_new {
    ($mock:ident) => {{
        use crate::objects::AckCommand;
        use novatel_oem6_api::Connection;
        use std::sync::{Arc, Mutex, RwLock};
        use std::thread;
        use std::time::Duration;

        let (log_send, log_recv) = sync_channel(5);
        let (response_send, response_recv) = sync_channel(5);
        let (response_abbrv_send, response_abbrv_recv) = sync_channel(5);

        $mock.read.set_result(Err(UartError::IoError {
            cause: ::std::io::ErrorKind::TimedOut,
            description: "Mock Timeout".to_owned(),
        }));

        let oem = OEM6 {
            conn: Arc::new(Mutex::new(Connection {
                stream: Box::new($mock),
            })),
            log_recv: Arc::new(Mutex::new(log_recv)),
            response_recv: Arc::new(Mutex::new(response_recv)),
            response_abbrv_recv: Arc::new(Mutex::new(response_abbrv_recv)),
        };

        let rx_conn = oem.conn.clone();

        thread::spawn(move || {
            read_thread(&rx_conn, &log_send, &response_send, &response_abbrv_send)
        });

        let data = Arc::new(LockData::new());
        let (error_send, error_recv) = sync_channel(10);
        let (version_send, version_recv) = sync_channel(1);

        let data_ref = data.clone();
        let oem_ref = oem.clone();
        thread::spawn(move || log_thread(&oem_ref, &data_ref, &error_send, &version_send));

        // The read thread needs some time to intake and process the
        // sample data we give it.
        // Note: If run locally, this delay can be 50ms. However, when
        // run on CircleCI, it needs to be 500ms
        thread::sleep(Duration::from_millis(500));

        let config = r#"
            [novatel-oem6-service.addr]
            ip = "127.0.0.1"
            port = 9999"#;

        Service::new(
            Config::new_from_str("novatel-oem6-service", &config).unwrap(),
            Subsystem {
                oem,
                last_cmd: Arc::new(RwLock::new(AckCommand::None)),
                errors: Arc::new(RwLock::new(vec![])),
                lock_data: data.clone(),
                error_recv: Arc::new(Mutex::new(error_recv)),
                version_recv: Arc::new(Mutex::new(version_recv)),
            },
            QueryRoot,
            MutationRoot,
        )
    }};
}

mod schema;
