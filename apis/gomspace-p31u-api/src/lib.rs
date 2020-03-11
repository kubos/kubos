//! Kubos API for interacting with [GomSpace EPS Systems]

#![deny(missing_docs)]

pub use crate::eps::*;
pub use crate::object::{EpsBatteryConfig, EpsHk, EpsSystemConfig};

mod eps;
mod ffi;
mod object;
