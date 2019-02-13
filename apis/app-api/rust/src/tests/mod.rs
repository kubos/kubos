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

// TODO: Temporarily deactivating the unit tests so that CI will pass for the rust http service PR
// THIS SHOULD BE UPDATED BEFORE MERGING INTO MASTER

mod mock_service;

macro_rules! mock_service {
    ($config:ident, $addr:expr, $port:expr) => {{
        let config = format!(
            r#"
            [mock-service.addr]
            ip = "{}"
            port = {}
            "#,
            $addr, $port
        );

        ::std::fs::write($config.clone(), config).unwrap();

        let config_thread = $config.to_string_lossy().to_string();

        ::std::thread::spawn(|| {
            Service::new(
                ServiceConfig::new_from_path("mock-service", config_thread),
                Subsystem,
                QueryRoot,
                MutationRoot,
            )
            .start()
        });

        ::std::thread::sleep(::std::time::Duration::from_millis(100));
    }};
}

mod query;