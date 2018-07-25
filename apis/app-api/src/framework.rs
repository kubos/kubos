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

//! A simple API to make standalone Rust applications with high-level hooks
//! for mission life-cycle management
#![deny(missing_docs)]
#![deny(warnings)]

use std::fmt;

/// The RunLevel type
/// The different RunLevels supported by KubOS applications
#[derive(Clone, Debug, PartialEq)]
pub enum RunLevel {
    /// An application will start at system boot time, and is managed automatically by the
    /// Application Service
    OnBoot,
    /// An application will start when commanded through the `start_app` GraphQL mutation
    OnCommand,
}

impl fmt::Display for RunLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RunLevel::OnBoot => write!(f, "OnBoot"),
            RunLevel::OnCommand => write!(f, "OnCommand"),
        }
    }
}

/// The trait that should be implemented by KubOS Applications to be notified when the application
/// goes through one of three lifecycle events:
///
/// 1. Start on system boot-up
/// 2. Start on demand when being commanded (i.e. through the `start_app` GraphQL mutation)
/// 3. When the app is shutting down, to clean up any resources initialized in on_boot or
///    on_command
pub trait AppHandler {
    /// Called when the Application is started at system boot time
    fn on_boot(&self);

    /// Called when the Application is started on demand through the `start_app` GraphQL mutation
    fn on_command(&self);
}

/// A helper macro that can be called from a KubOS application's `main` function.
///
/// # Arguments
///
/// * `handler` - An implementation of `AppHandler`
///
/// # Examples
///
/// ```
/// extern crate getopts;
/// #[macro_use]
/// extern crate kubos_app;
///
/// use kubos_app::AppHandler;
///
/// struct MyApp;
///
/// impl AppHandler for MyApp {
///   fn on_boot(&self) {
///     println!("OnBoot logic");
///   }
///   fn on_command(&self) {
///     println!("OnCommand logic");
///   }
/// }
///
/// fn main() {
///     let app = MyApp { };
///     app_main!(&app);
/// }
/// ```
#[macro_export]
macro_rules! app_main {
    ($handler:expr) => {{
        use getopts::Options;
        use std::env;

        let name = option_env!("CARGO_PKG_NAME").unwrap_or("Unknown");
        let version = option_env!("CARGO_PKG_VERSION").unwrap_or("Unknown");
        let authors = option_env!("CARGO_PKG_AUTHORS").unwrap_or("Unknown");

        let _pid = std::process::id();

        let args: Vec<String> = env::args().collect();
        let program = args[0].clone();

        let mut opts = Options::new();
        opts.optflag("m", "metadata", "Print app metadata and immediately exit");
        opts.optflag("h", "help", "Print this help menu");

        let matches = match opts.parse(&args[1..]) {
            Ok(m) => m,
            Err(f) => panic!(f.to_string()),
        };

        if matches.opt_present("h") {
            let brief = format!("Usage: {} [options]", program);
            print!("{}", opts.usage(&brief));
            return;
        }

        if matches.opt_present("m") {
            println!("name = \"{}\"", name);
            println!("version = \"{}\"", version);
            println!("author = \"{}\"", authors);
            return;
        }

        let _uuid = env::var_os("KUBOS_APP_UUID");
        let run_level = env::var_os("KUBOS_APP_RUN_LEVEL");

        match run_level {
            Some(ref level) if level == "OnBoot" => {
                $handler.on_boot();
            }
            Some(ref level) if level == "OnCommand" => {
                $handler.on_command();
            }
            _ => {
                eprintln!(
                    "Warning: Unknown or missing KUBOS_APP_RUN_LEVEL. Set to OnBoot or OnCommand"
                );
                $handler.on_command();
            }
        }
    }};
}
