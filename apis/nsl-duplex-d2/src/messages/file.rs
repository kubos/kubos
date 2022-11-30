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

use nom::{be_u16, map_res, take, take_str, take_until_and_consume, IResult};
use std::io::Write;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
/// Structure for files
pub struct File {
    /// Name of file
    pub name: String,
    /// Contents of file
    pub body: Vec<u8>,
}

impl File {
    /// Create a new file object by copying name and body.
    pub fn new(name: &str, body: &[u8]) -> Self {
        File {
            name: name.to_string(),
            body: body.to_vec(),
        }
    }

    /// Create a new file object by parsing raw serial data.
    pub fn parse(input: &[u8]) -> IResult<&[u8], File> {
        let (input, _) = take_until_and_consume!(input, "GU")?;
        let (input, name_length) = map_res!(input, take_str!(3), usize::from_str)?;
        let (input, body_length) = map_res!(input, take_str!(6), usize::from_str)?;
        let (input, name) = take_str!(input, name_length)?;
        let name = String::from(name);
        let (input, body) = take!(input, body_length)?;
        let body = Vec::from(body);
        let (input, crc) = be_u16(input)?;
        println!("TODO: check crc: {}", crc);
        Ok((input, File { name, body }))
    }

    /// Encode a file object to raw serial data.
    pub fn encode(&self) -> Vec<u8> {
        let mut output: Vec<u8> = Vec::new();
        let name = self.name.as_bytes();
        write!(&mut output, "GU{:03}{:06}", name.len(), self.body.len(),)
            .expect("Problem encoding lengths");
        output.extend_from_slice(name);
        output.extend_from_slice(&self.body);
        let crc = crc16::State::<crc16::XMODEM>::calculate(&output);
        output.push((crc >> 8) as u8);
        output.push(crc as u8);
        output
    }
}

#[cfg(test)]
mod tests {
    use super::File;
    #[test]
    fn it_parses() {
        assert_eq!(
            Ok((
                &b"extra"[..],
                File {
                    name: String::from("test.txt"),
                    body: b"Hello World\n".to_vec(),
                }
            )),
            File::parse(b"GU008000012test.txtHello World\n\x42\x24extra")
        );
    }

    #[test]
    fn it_encodes() {
        let file = File {
            name: String::from("test.txt"),
            body: b"Hello World\n".to_vec(),
        };
        let expected: &[u8] = b"GU008000012test.txtHello World\n\x15\xac";
        let actual: &[u8] = &file.encode();
        assert_eq!(expected, actual);
    }

    #[test]
    fn it_roundtrips() {
        let file = File::new("test.txt", b"Hello World\n");
        let encoded = file.encode();
        let expected = Ok((&b""[..], file));
        let actual = File::parse(&encoded);
        assert_eq!(expected, actual);
    }
}
