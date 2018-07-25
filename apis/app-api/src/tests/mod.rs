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

mod mock_service;

macro_rules! mock_service {
    ($addr:expr, $port:expr) => {{
        thread::spawn(|| {
            let config = format!(
                r#"
                [mock-service.addr]
                ip = "{}"
                port = {}
                "#,
                $addr, $port
            );
            Service::new(
                Config::new_from_str("mock-service", &config),
                Subsystem,
                QueryRoot,
                MutationRoot,
            ).start()
        });

        thread::sleep(Duration::from_millis(100));
    }};
}

mod query;
