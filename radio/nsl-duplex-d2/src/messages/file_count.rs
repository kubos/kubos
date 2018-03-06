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

use nom::{IResult, be_u32};

#[derive(Debug, PartialEq)]
pub struct FileCount(u32);

impl FileCount {
    pub fn parse(input: &[u8]) -> IResult<&[u8], FileCount> {
        let (input, _) = tag!(input, b"GU")?;
        let (input, file_count) = be_u32(input)?;
        Ok((input, FileCount(file_count)))
    }
}

#[cfg(test)]
mod tests {
    use super::FileCount;
    #[test]
    fn it_parses() {
        assert_eq!(
            Ok((&b"extra"[..], FileCount(0x12345678))),
            FileCount::parse(b"GU\x12\x34\x56\x78extra")
        );
    }
}
