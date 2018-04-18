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

use i2c_hal::Command;

/// Sends a reset command to the EPS TTC node
///
/// If required the user can reset the TTC node using this command. When issued,
/// the board will reset within 1 second. This command will result in the board
/// being brought up in its defined initial condition. Resetting the board in
/// this fashion will increment the Manual Reset Counter.
    pub fn command() -> Command {
        Command {
            cmd: 0x80,
            data: vec![0x00],
        }
    }
