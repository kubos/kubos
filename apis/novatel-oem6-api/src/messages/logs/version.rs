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

const COMPONENT_SIZE: usize = 108;

/// Log message containing version information
#[derive(Clone, Default, Debug, PartialEq)]
pub struct VersionLog {
    /// Number of components present in this structure
    pub num_components: u32,
    /// Version information for each component present in the system
    pub components: Vec<Component>,
}

impl VersionLog {
    /// Convert a raw data buffer into a useable struct
    pub fn new(mut raw: Vec<u8>) -> Option<Self> {
        let raw_comp = raw.split_off(4);

        let mut log = VersionLog {
            num_components: {
                match le_u32(&raw) {
                    Ok(v) => v.1,
                    Err(_) => return None,
                }
            },
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

/// Version information about a specific system component
#[derive(Clone, Default, Debug, PartialEq)]
pub struct Component {
    /// The component type which the version info is about
    pub comp_type: u32,
    /// Model number
    pub model: String,
    /// Serial number
    pub serial_num: String,
    /// Hardware version
    pub hw_version: String,
    /// Software version
    pub sw_version: String,
    /// Boot code version
    pub boot_version: String,
    /// Software compilation date stamp
    pub compile_date: String,
    /// Software compilation time stamp
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
            model: String::from_utf8_lossy(model)
                            .trim_right_matches('\u{0}').to_owned(),
            serial_num: String::from_utf8_lossy(serial_num)
                            .trim_right_matches('\u{0}').to_owned(),
            hw_version: String::from_utf8_lossy(hw_version)
                            .trim_right_matches('\u{0}').to_owned(),
            sw_version: String::from_utf8_lossy(sw_version)
                            .trim_right_matches('\u{0}').to_owned(),
            boot_version: String::from_utf8_lossy(boot_version)
                            .trim_right_matches('\u{0}').to_owned(),
            compile_date: String::from_utf8_lossy(compile_date)
                            .trim_right_matches('\u{0}').to_owned(),
            compile_time: String::from_utf8_lossy(compile_time)
                            .trim_right_matches('\u{0}').to_owned(),
            }
        )
    )
);
