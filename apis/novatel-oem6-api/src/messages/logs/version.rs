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

use nom::*;
use super::*;

const COMPONENT_SIZE: usize = 108;

#[derive(Clone, Default, Debug, PartialEq)]
pub struct VersionLog {
    pub num_components: u32,
    pub components: Vec<Component>,
}

impl VersionLog {
    pub fn new(mut raw: Vec<u8>) -> Option<Self> {
        let raw_comp = raw.split_off(4);

        let mut log = VersionLog {
            num_components: le_u32(&raw).unwrap().1,
            components: vec![],
        };

        for elem in raw_comp.chunks(COMPONENT_SIZE) {
            match parse_component(elem) {
                Ok(conv) => log.components.push(conv.1),
                _ => {}
            }
        }

        Some(log)
    }
}

#[derive(Clone, Default, Debug, PartialEq)]
pub struct Component {
    pub comp_type: u32, //TODO: enum
    pub model: String,
    pub serial_num: String,
    pub hw_version: String,
    pub sw_version: String,
    pub boot_version: String,
    pub compile_date: String,
    pub compile_time: String,
}

named!(parse_component(&[u8]) -> Component,
    do_parse!(
        comp_type: le_u32 >>
        model: take!(16) >>
        serial_num: take!(16) >>
        hw_version: take!(16) >>
        sw_version: take!(16) >>
        boot_version: take!(16) >>
        compile_date: take!(12) >>
        compile_time: take!(12) >>
        (Component {
            comp_type,
            model: ::std::str::from_utf8(model).unwrap().to_owned(),
            serial_num: ::std::str::from_utf8(serial_num).unwrap().to_owned(),
            hw_version: ::std::str::from_utf8(hw_version).unwrap().to_owned(),
            sw_version: ::std::str::from_utf8(sw_version).unwrap().to_owned(),
            boot_version: ::std::str::from_utf8(boot_version).unwrap().to_owned(),
            compile_date: ::std::str::from_utf8(compile_date).unwrap().to_owned(),
            compile_time: ::std::str::from_utf8(compile_time).unwrap().to_owned(),
            }
            
        )
    )
);
