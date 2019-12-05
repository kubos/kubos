//! Kubos API for interacting with [GomSpace EPS Systems]

#![deny(missing_docs)]

pub use crate::eps::*;
pub use crate::object::{EpsSystemConfig, EpsBatteryConfig,EpsHk};

mod eps;
mod ffi;
mod object;