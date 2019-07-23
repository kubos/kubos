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

#[allow(dead_code)]
pub struct UnlogCmd {
    hdr: Header,
    port: Port,
    msg_id: MessageID,
    msg_type: u8,
}

impl UnlogCmd {
    pub fn new(port: Port, msg_id: MessageID) -> Self {
        UnlogCmd {
            hdr: Header::new(MessageID::Unlog, 8),
            port,
            msg_id,
            msg_type: 0,
        }
    }
}
#[cfg(not(feature = "nos3"))]
impl Message for UnlogCmd {
    fn serialize(&self) -> Vec<u8> {
        let mut vec = vec![];

        //Add header
        vec.append(&mut self.hdr.serialize());

        // Add Unlog message
        vec.write_u32::<LittleEndian>(self.port as u32).unwrap();
        vec.write_u16::<LittleEndian>(self.msg_id as u16).unwrap();
        vec.push(self.msg_type);
        vec.push(0x00); // Reserved byte

        vec
    }
}

#[cfg(feature = "nos3")]
impl Message for UnlogCmd {
    fn serialize(&self) -> Vec<u8> {
        let mut vec = vec![];

        // Header is always command name (UNLOG) for abbrv. ASCII commands
        let mut header = String::from("UNLOG ").into_bytes();

        let mut port = {
            let from = match self.port {
                Port::COM1 => "COM1 ",
                Port::ThisPort => "THISPORT ",
            };
            String::from(from).into_bytes()
        };

        let mut message = {
            let from = match self.msg_id {
                MessageID::BestXYZ => "BESTXYZB ",
                MessageID::RxStatusEvent => "RXSTATUSEVENT ",
                MessageID::Version => "VERSION ",
                _ => "UNKNOWN ",
            };
            String::from(from).into_bytes()
        };

        vec.append(&mut header);
        vec.append(&mut port);
        vec.append(&mut message);

        vec
    }
}
