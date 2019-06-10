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
pub struct LogCmd {
    hdr: Header,
    port: Port,
    msg_id: MessageID,
    msg_type: u8,
    trigger: LogTrigger,
    period: f64,
    offset: f64,
    hold: bool,
}

#[allow(dead_code)]
impl LogCmd {
    pub fn new(
        port: Port,
        msg_id: MessageID,
        trigger: LogTrigger,
        period: f64,
        offset: f64,
        hold: bool,
    ) -> Self {
        LogCmd {
            hdr: Header::new(MessageID::Log, 32),
            port,
            msg_id,
            msg_type: 0,
            trigger,
            period,
            offset,
            hold,
        }
    }
}

#[cfg(not(feature = "nos3"))]
impl Message for LogCmd {
    fn serialize(&self) -> Vec<u8> {
        let mut vec = vec![];

        //Add header
        vec.append(&mut self.hdr.serialize());

        // Add LOG message
        vec.write_u32::<LittleEndian>(self.port as u32).unwrap();
        vec.write_u16::<LittleEndian>(self.msg_id as u16).unwrap();
        vec.push(self.msg_type);
        vec.push(0x00); // Reserved byte
        vec.write_u32::<LittleEndian>(self.trigger as u32).unwrap();
        vec.write_f64::<LittleEndian>(self.period).unwrap();
        vec.write_f64::<LittleEndian>(self.offset).unwrap();
        vec.write_u32::<LittleEndian>(self.hold as u32).unwrap();

        vec
    }
}

#[cfg(feature = "nos3")]
impl Message for LogCmd {
    fn serialize(&self) -> Vec<u8> {
        let mut vec = vec![];

        // Header is always command name (LOG) for abbrv. ASCII commands
        let mut header = String::from("LOG ").into_bytes();

        // Lets appropriately set the corresponding command arguments to bytes
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
                MessageID::Log => "LOG ",
                MessageID::RxStatusEvent => "RXSTATUSEVENT ",
                MessageID::Unlog => "UNLOG ",
                MessageID::UnlogAll => "UNLOGALL ",
                MessageID::Version => "VERSION ",
                _ => "UNKNOWN ",
            };
            String::from(from).into_bytes()
        };

        let mut trigger = {
            let from = match self.trigger {
                LogTrigger::OnChanged => "ONCHANGED ",
                LogTrigger::OnTime => "ONTIME ",
                LogTrigger::Once => "ONCE ",
            };
            String::from(from).into_bytes()
        };

        let mut period = format!("{} ", self.period.to_string()).into_bytes();

        let mut offset = format!("{} ", self.offset.to_string()).into_bytes();

        let mut hold = {
            let from = match self.hold {
                true => String::from("HOLD "),
                false => String::from("NOHOLD "),
            };
            String::from(from).into_bytes()
        };

        // Add LOG message
        vec.append(&mut header);
        vec.append(&mut port);
        vec.append(&mut message);
        vec.append(&mut trigger);
        vec.append(&mut period);
        vec.append(&mut offset);
        vec.append(&mut hold);

        vec
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum LogTrigger {
    OnChanged = 1,
    OnTime = 2,
    Once = 4,
}
