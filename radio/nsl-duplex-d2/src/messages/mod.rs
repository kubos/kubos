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

use nom::{IResult, be_u32};

mod file;
mod state_of_health;

pub use messages::file::File;
pub type Message = File;
pub use messages::state_of_health::StateOfHealth;

pub fn parse_u32(input: &[u8]) -> IResult<&[u8], u32> {
    let (input, _) = tag!(input, b"GU")?;
    let (input, file_count) = be_u32(input)?;
    Ok((input, file_count))
}

pub fn parse_ack(input: &[u8]) -> IResult<&[u8], ()> {
    let (input, _) = tag!(input, b"GU\x06")?;
    Ok((input, ()))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_parses_u32() {
        assert_eq!(
            Ok((&b"extra"[..], 0x12345678)),
            parse_u32(b"GU\x12\x34\x56\x78extra")
        );
    }

    #[test]
    fn it_parses_ack() {
        assert_eq!(Ok((&b"extra"[..], ())), parse_ack(b"GU\x06extra"));
    }
}
