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

pub trait Message {
    fn serialize(&self) -> Vec<u8>;
}

pub struct LogCmd {
    hdr: Header,
    port: u32, //Hardcode?
    msg_id: u16,
    msg_type: u8, //Bit field
    trigger: u32, //enum
    period: f64,
    offset: f64,
    hold: bool, //bool...
}

impl LogCmd {
    fn new(
        port: u32, //Hardcode?
        msg_id: u16,
        msg_type: u8, //Bit field
        trigger: u32, //enum
        period: f64,
        offset: f64,
        hold: bool,
    ) -> Self {
        LogCmd {
            hdr: Header::new(1, 32),
            port,
            msg_id,
            msg_type,
            trigger,
            period,
            offset,
            hold,
        }
    }
}

impl Message for LogCmd {
    fn serialize(&self) -> Vec<u8> {
        let mut vec = vec![];

        //Add header
        vec.append(&mut self.hdr.serialize());

        // Add LOG message
        vec.write_u32::<LittleEndian>(self.port).unwrap();
        vec.write_u16::<LittleEndian>(self.msg_id).unwrap();
        vec.push(self.msg_type);
        vec.write_u32::<LittleEndian>(self.trigger).unwrap();
        vec.write_f64::<LittleEndian>(self.period).unwrap();
        vec.write_f64::<LittleEndian>(self.offset).unwrap();
        vec.push(self.hold as u8);

        vec
    }
}
