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

use i2c_api;

#[derive(Debug, Eq, PartialEq)]
pub enum Flags {
    LastCommandFailed = 0b0000001,
    WatchdogError = 0b0000010,
    BadCommandData = 0b0000100,
    BadCommandChannel = 0b0001000,
    ErrorReadingEeprom = 0b0010000,
    PowerOnReset = 0b0100000,
    BrownOutReset = 0b1000000,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Status(Vec<Flags>);

impl Status {
    pub fn parse(data: &[u8]) -> Self {
        Status::default()
    }

    pub fn command() -> (Vec<u8>) {
        (vec![0x01, 0x00])
    }
}

impl Default for Status {
    fn default() -> Self {
        Status { 0: vec![] }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_flags() {
        assert_eq!(Status { 0: vec![] }, Status::parse(&vec![]));
    }

    #[test]
    fn test_last_cmd_failed() {
        assert_eq!(
            Status {
                0: vec![Flags::LastCommandFailed],
            },
            Status::parse(&vec![Flags::LastCommandFailed as u8])
        );
    }
}
