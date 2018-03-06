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

use nom::IResult;
use std::str::FromStr;
use std::io::Write;

#[derive(Debug, PartialEq)]
pub struct File {
    pub name: String,
    pub body: Vec<u8>,
}

impl File {
    pub fn parse(input: &[u8]) -> IResult<&[u8], File> {
        let (input, _) = tag!(input, b"GU")?;
        let (input, name_length) = map_res!(input, take_str!(3), usize::from_str)?;
        let (input, body_length) = map_res!(input, take_str!(6), usize::from_str)?;
        let (input, name) = take_str!(input, name_length)?;
        let name = String::from(name);
        let (input, body) = take!(input, body_length)?;
        let body = Vec::from(body);
        Ok((input, File { name, body }))
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut output: Vec<u8> = Vec::new();
        let name = self.name.as_bytes();
        write!(&mut output, "GU{:03}{:06}", name.len(), self.body.len(),)
            .expect("Problem encoding lengths");
        output.extend_from_slice(&name);
        output.extend_from_slice(&self.body);
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
            File::parse(b"GU008000012test.txtHello World\nextra")
        );
    }

    #[test]
    fn it_encodes() {
        let file = File {
            name: String::from("test.txt"),
            body: b"Hello World\n".to_vec(),
        };
        let expected: &[u8] = b"GU008000012test.txtHello World\n";
        let actual: &[u8] = &file.encode();
        assert_eq!(expected, actual);
    }

    #[test]
    fn it_roundtrips() {
        let file = File {
            name: String::from("test.txt"),
            body: b"Hello World\n".to_vec(),
        };
        let encoded = file.encode();
        let expected = Ok((&b""[..], file));
        let actual = File::parse(&encoded);
        assert_eq!(expected, actual);
    }
}
