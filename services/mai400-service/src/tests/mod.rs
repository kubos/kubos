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
use mai400_api::mock::*;
use mai400_api::*;
use std::sync::mpsc::channel;

#[macro_export]
macro_rules! service_new {
    ($mock:ident) => {{
        let data = Arc::new(ReadData {
            std_telem: Mutex::new(STD),
            irehs_telem: Mutex::new(IREHS),
            imu: Mutex::new(IMU),
            rotating: Mutex::new(ROTATING),
        });
        service_new!($mock, data)
    }};
    ($mock:ident, $data:ident) => {{
        use crate::objects::AckCommand;
        use mai400_api::Connection;
        use std::sync::{Arc, Mutex, RwLock};
        use std::thread;

        let (sender, receiver) = channel();

        let mai = MAI400 {
            conn: Arc::new(Mutex::new(Connection {
                stream: Box::new($mock),
            })),
        };

        // We don't actually want to do anything with this thread, the channel
        // sender just needs to live through the lifetime of each test
        thread::spawn(move || {
            let _send = sender;
            thread::sleep(Duration::from_secs(2))
        });

        Service::new(
            Config::new("mai400-service"),
            Subsystem {
                mai,
                last_cmd: Arc::new(RwLock::new(AckCommand::None)),
                errors: Arc::new(RwLock::new(vec![])),
                persistent: $data.clone(),
                receiver: Arc::new(Mutex::new(receiver)),
            },
            QueryRoot,
            MutationRoot,
        )
    }};
}

#[macro_export]
macro_rules! service_new_with_read {
    ($mock:ident, $data:ident) => {{
        use crate::objects::AckCommand;
        use mai400_api::Connection;
        use std::sync::{Arc, Mutex, RwLock};
        use std::thread;

        let (sender, receiver) = channel();

        let mai = MAI400 {
            conn: Arc::new(Mutex::new(Connection {
                stream: Box::new($mock),
            })),
        };

        let mai_ref = mai.clone();
        let data_ref = $data.clone();

        thread::spawn(move || read_thread(mai_ref, data_ref, sender));

        Service::new(
            Config::new("mai400-service"),
            Subsystem {
                mai,
                last_cmd: Arc::new(RwLock::new(AckCommand::None)),
                errors: Arc::new(RwLock::new(vec![])),
                persistent: $data.clone(),
                receiver: Arc::new(Mutex::new(receiver)),
            },
            QueryRoot,
            MutationRoot,
        )
    }};
}

macro_rules! request {
    ($service:ident, $query:ident) => {{
        // Warp doesn't like control characters (ie. new line characters)
        // so we need to remove them before we send the request
        let query = $query.replace("\n", "");
        warp::test::request()
            .header("Content-Type", "application/json")
            .method("POST")
            .body(format!("{{\"query\": \"{}\"}}", query))
            .reply(&$service.filter)
    }};
}

macro_rules! wrap {
    ($result:ident) => {{
        &json!({ "data": $result }).to_string()
    }};
}

macro_rules! test {
    ($service:ident, $query:ident, $expected:ident) => {{
        let res = request!($service, $query);

        assert_eq!(res.body(), wrap!($expected));
    }};
}

mod read;
mod schema;
mod test_data;
