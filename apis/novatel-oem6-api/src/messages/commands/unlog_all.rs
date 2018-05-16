//
// Copyright (C) 2018 Kubos Corporation
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
//

use super::*;

pub struct UnlogAllCmd {
    hdr: Header,
    port: u32,
    hold: bool,
}

impl UnlogAllCmd {
    pub fn new(port: u32, hold: bool) -> Self {
        UnlogAllCmd {
            hdr: Header::new(MessageID::UnlogAll, 8),
            port,
            hold,
        }
    }
}

impl Message for UnlogAllCmd {
    fn serialize(&self) -> Vec<u8> {
        let mut vec = vec![];

        //Add header
        vec.append(&mut self.hdr.serialize());

        // Add UnlogAll message
        vec.write_u32::<LittleEndian>(self.port).unwrap();
        vec.write_u32::<LittleEndian>(self.hold as u32).unwrap();

        vec
    }
}
