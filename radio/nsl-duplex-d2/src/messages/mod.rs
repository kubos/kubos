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
mod geo_record;

pub use messages::file::File;
pub type Message = File;
pub use messages::state_of_health::StateOfHealth;
pub use messages::geo_record::GeoRecord;

/// Parse out initial sync code
pub fn parse_sync(input: &[u8]) -> IResult<&[u8], ()> {
    let (input, _) = take_until_and_consume!(input, "GU")?;
    Ok((input, ()))
}

/// Parse 4 byte integer
pub fn parse_u32(input: &[u8]) -> IResult<&[u8], u32> {
    be_u32(input)
}

/// Parse ACK or NAK byte and converts to boolean.
pub fn parse_ack_or_nak(input: &[u8]) -> IResult<&[u8], bool> {
    let (input, code) = one_of!(input, "\x06\x0f")?;
    Ok((input, code == '\x06'))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_parses_sync() {
        assert_eq!(Ok((&b"extra"[..], ())), parse_sync(b"GUextra"));
    }

    #[test]
    fn it_parses_sync_and_skips_garbage() {
        assert_eq!(Ok((&b"extraGU"[..], ())), parse_sync(b"garbageGUextraGU"));
    }

    #[test]
    fn it_parses_u32() {
        assert_eq!(
            Ok((&b"extra"[..], 0x12345678)),
            parse_u32(b"\x12\x34\x56\x78extra")
        );
    }

    #[test]
    fn it_parses_ack() {
        assert_eq!(Ok((&b"extra"[..], true)), parse_ack_or_nak(b"\x06extra"));
    }

    #[test]
    fn it_parses_nak() {
        assert_eq!(Ok((&b"extra"[..], false)), parse_ack_or_nak(b"\x0fextra"));
    }
}
