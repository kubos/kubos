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

use ffi::*;

macro_rules! check_bits {
    ($bits:expr, $mask:expr) => (
        ($bits & $mask) == $mask
    );
}

#[derive(Debug)]
pub enum IdleState {
    Off,
    On,
}

#[derive(Debug)]
pub enum Rate {
    b1200,
    b2400,
    b4800,
    b9600,
}

#[derive(Debug)]
pub struct TxState {
    idle: IdleState,
    beacon_active: bool,
    rate: Rate,
}

impl TxState {
    pub fn parse(raw: u8) -> TxState {
        TxState {
            beacon_active: {
                let raw = (raw & 0b11);
                if check_bits!(raw, RawTxStateFirstBit::BeaconActive as u8) {
                    true
                } else {
                    false
                }
            },
            idle: {
                let raw = (raw & 0b11);
                if check_bits!(raw, RawTxStateFirstBit::IdleOn as u8) {
                    IdleState::On
                } else {
                    IdleState::Off
                }
            },
            rate: {
                let raw = (raw >> 2) & 0b11;
                if check_bits!(raw, RawTxStateSecondBit::B1200 as u8) {
                    Rate::b1200
                } else if check_bits!(raw, RawTxStateSecondBit::B2400 as u8) {
                    Rate::b2400
                } else if check_bits!(raw, RawTxStateSecondBit::B4800 as u8) {
                    Rate::b4800
                } else {
                    Rate::b9600
                }
            },
        }
    }
}
