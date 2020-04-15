//
// Copyright (C) 2020 Kubos Corporation
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

// Contributed by Xueliang Bai <x.bai@sydney.edu.au> on behalf of the
// ARC Training Centre for CubeSats, UAVs & Their Applications (CUAVA) team (www.cuava.com.au)
// at the University of Sydney

//! Kubos API for interacting with [GomSpace EPS Systems]

#![deny(missing_docs)]

pub use crate::eps::*;
pub use crate::object::{EpsBatteryConfig, EpsHk, EpsSystemConfig};

mod eps;
mod ffi;
mod object;
