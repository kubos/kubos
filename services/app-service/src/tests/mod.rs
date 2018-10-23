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

macro_rules! mock_service {
    ($registry_dir:ident) => {{
        let registry = AppRegistry::new_from_dir(&$registry_dir.path().to_string_lossy()).unwrap();

        let config = format!(
            r#"
            [app-service]
            registry-dir = "{}"
            [app-service.addr]
            ip = "127.0.0.1"
            port = 9999"#,
            $registry_dir.path().to_str().unwrap(),
        );

        Service::new(
            Config::new_from_str("app-service", &config),
            registry,
            schema::QueryRoot,
            schema::MutationRoot,
        )
    }};
}

mod register_app;
mod registry_onboot;
mod registry_start_app;
mod registry_test;
mod upgrade_app;
