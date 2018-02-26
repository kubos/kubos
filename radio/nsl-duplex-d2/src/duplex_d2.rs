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

use radio_api::Connection;
use messages::{File, FileCount, StateOfHealth};

/// Structure for interacting with Duplex-D2 Radio API
pub struct DuplexD2 {
    conn: Connection,
}

impl DuplexD2 {
    /// Constructor for DuplexD2 structure
    pub fn new(conn: Connection) -> DuplexD2 {
        DuplexD2 { conn }
    }

    /// Helper function for generating Command<File>
    pub fn get_file(&self) -> Result<File, String> {
        self.conn.write(b"GUGET_UF")?;
        let result = self.conn.read(File::parse)?;
        self.conn.write(b"GU\x06")?;
        Ok(result)
    }

    /// Helper function for generating Command<FileCount>
    pub fn get_file_count(&self) -> Result<FileCount, String> {
        self.conn.write(b"GUGETUFC")?;
        self.conn.read(FileCount::parse)
    }

    /// Helper function for generating Command<StateOfHealth>
    pub fn get_state_of_health(&self) -> Result<StateOfHealth, String> {
        self.conn.write(b"GUGETSOH")?;
        self.conn.read(StateOfHealth::parse)
    }
}
