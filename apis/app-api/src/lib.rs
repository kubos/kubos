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
extern crate getopts;
extern crate kubos_system;
extern crate uuid;
#[macro_use]
extern crate serde_derive;
extern crate toml;

use std::env;

use getopts::Options;

/// The registry submodule
pub mod registry;
use self::registry::*;

/// The App type
pub type App = registry::App;

/// The RunLevel type
pub type RunLevel = registry::RunLevel;

/// The trait that should be implemented by KubOS Applications to be notified when the application
/// goes through one of three lifecycle events:
///
/// 1. Start on system boot-up
/// 2. Start on demand when being commanded (i.e. through the `start_app` GraphQL mutation)
/// 3. When the app is shutting down, to clean up any resources initialized in on_boot or
///    on_command
#[allow(unused_variables)]
pub trait AppHandler {
    /// Called when the Application is started at system boot time
    fn on_boot(&self) {}

    /// Called when the Application is started on demand through the `start_app` GraphQL mutation
    fn on_command(&self) {}

    /// Called when the Application is shutting down, to clean up any resources initialized in
    /// on_boot or on_command. Note: This function is not guaranteed to be called if the process
    /// exits for unexpected reasons
    fn on_shutdown(&self) {}
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
/// #[macro_use]
/// extern crate kubos_app;
///
/// struct MyApp;
///
/// impl kubos_app::AppHandler for MyApp {
///   fn on_boot(&self) {
///     //..
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
        let name: Option<&'static str> = option_env!("CARGO_PKG_NAME");
        let version: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
        let authors: Option<&'static str> = option_env!("CARGO_PKG_AUTHORS");

        kubos_app::App::main(
            name.unwrap_or("Unknown"),
            version.unwrap_or("Unknown"),
            authors.unwrap_or("Unknown"),
            std::process::id(),
            $handler,
        )
    }};
}

impl App {
    /// The entry point for all KubOS applications. The preferred way to use this application
    /// is through the `app_main!` macro
    pub fn main(name: &str, version: &str, authors: &str, _pid: u32, handler: &AppHandler) {
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
            println!(
                "{}",
                toml::to_string(&AppMetadata::new(name, version, authors)).unwrap()
            );
            return;
        }

        let _uuid = env::var_os("KUBOS_APP_UUID");
        let run_level = env::var_os("KUBOS_APP_RUN_LEVEL");

        match run_level {
            Some(ref level) if level == "OnBoot" => {
                handler.on_boot();
            }
            Some(ref level) if level == "OnCommand" => {
                handler.on_command();
            }
            _ => {
                eprintln!(
                    "Warning, unknown or missing KUBOS_APP_RUN_LEVEL, set to OnBoot or OnCommand"
                );
                handler.on_command();
            }
        }
    }
}
