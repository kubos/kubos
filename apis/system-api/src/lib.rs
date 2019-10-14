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
#![deny(warnings)]
#![deny(missing_docs)]

//! KubOS System level APIs

mod config;
pub mod logger;
mod uboot;

pub use crate::config::DEFAULT_PATH as DEFAULT_CONFIG_PATH;
pub use crate::config::*;
pub use crate::uboot::UBootVars;

/// The name of the KubOS app service that can be used to derive service configuration
pub const SERVICE_APP: &str = "app-service";
/// The name of the KubOS telemetry db service that can be used to dervice service configuration
pub const SERVICE_TELEMETRY: &str = "telemetry-service";

/// Information about the version(s) of KubOS installed in the system
pub struct KubosVersions {
    /// The current or "active" version of KubOS
    pub curr: Option<String>,
    /// The previous or "inactive" version of KubOS. If there is no previous version, this will be
    /// None
    pub prev: Option<String>,
}

/// Fetch information about the version(s) of KubOS installed in the system
///
/// Returns the current and previous version(s) of KubOS.
pub fn kubos_versions() -> KubosVersions {
    let vars = UBootVars::new();
    KubosVersions {
        curr: vars.get_str(uboot::VAR_KUBOS_CURR_VERSION),
        prev: vars.get_str(uboot::VAR_KUBOS_PREV_VERSION),
    }
}

/// Whether or not the system has been marked as deployed
pub fn initial_deploy() -> Option<bool> {
    let vars = UBootVars::new();
    vars.get_bool(uboot::VAR_KUBOS_INITIAL_DEPLOY)
}
