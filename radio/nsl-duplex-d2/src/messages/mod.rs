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

//! This module contains structs and parsers for messages received on
//! serial connection.

mod file;
mod file_count;
mod state_of_health;

use nom::IResult;

pub type ParseFn<T> = fn(input: &[u8]) -> IResult<&[u8], T>;

pub use messages::file::File;
pub use messages::file_count::FileCount;
pub use messages::state_of_health::StateOfHealth;
