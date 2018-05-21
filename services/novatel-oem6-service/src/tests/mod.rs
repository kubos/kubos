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
macro_rules! service_new {
    ($mock:ident) => {{
        use novatel_oem6_api::Connection;
        use objects::AckCommand;
        use std::cell::{Cell, RefCell};
        use std::sync::{Arc, Mutex};
        use std::thread;
        use std::time::Duration;

        let (log_send, log_recv) = sync_channel(5);
        let (response_send, response_recv) = sync_channel(5);

        let oem = OEM6 {
            conn: Arc::new(Mutex::new(Connection {
                stream: Box::new($mock),
            })),
            log_recv,
            response_recv,
        };

        // We don't actually want to do anything with this thread, the channel
        // senders just need to live through the lifetime of each test
        thread::spawn(move || {
            let _log_sender = log_send;
            let _response_sender = response_send;
            thread::sleep(Duration::from_secs(2))
        });

        Service::new(
            Config::new("novatel-oem6-service"),
            Subsystem {
                oem,
                last_cmd: Cell::new(AckCommand::None),
                errors: RefCell::new(vec![]),
            },
            QueryRoot,
            MutationRoot,
        )
    }};
}

mod schema;
