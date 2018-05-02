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

extern crate clyde_3g_eps_api;
extern crate i2c_hal;

use i2c_hal::*;
use clyde_3g_eps_api::*;

pub fn main() {
    let eps = Eps::new(Connection::from_path("/dev/i2c-1", 0x2B));

    println!("Board Status {:#?}", eps.get_board_status());

    println!("Checksum: {:#?}", eps.get_checksum());

    println!("Version: {:#?}", eps.get_version_info());

    println!("Last Er: {:#?}", eps.get_last_error());
}
