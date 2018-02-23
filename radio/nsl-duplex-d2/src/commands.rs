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

use messages::{File, FileCount, ParseFn, StateOfHealth};

/// Structure for abstracting away command to radio
/// This structure is generic over the expected type T
/// which will be parsed out of the radio response
pub struct Command<T> {
    /// Byte array containing raw radio command
    pub request: Vec<u8>,
    /// Function for parsing out response from radio
    pub parse: ParseFn<T>,
}

/// Helper function for generating Command<File>
pub fn get_file() -> Command<File> {
    Command {
        request: b"GUGET_UF".to_vec(),
        parse: File::parse,
    }
}

/// Helper function for generating Command<FileCount>
pub fn get_file_count() -> Command<FileCount> {
    Command {
        request: b"GUGETUFC".to_vec(),
        parse: FileCount::parse,
    }
}

/// Helper function for generating Command<StateOfHealth>
pub fn get_state_of_health() -> Command<StateOfHealth> {
    Command {
        request: b"GUGETSOH".to_vec(),
        parse: StateOfHealth::parse,
    }
}
