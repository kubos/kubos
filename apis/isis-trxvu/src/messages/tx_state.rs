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

#[derive(Debug, PartialEq)]
pub enum IdleState {
    Off,
    On,
}

#[derive(Debug, PartialEq)]
pub enum Rate {
    B1200,
    B2400,
    B4800,
    B9600,
}

#[derive(Debug, PartialEq)]
pub struct TxState {
    idle: IdleState,
    beacon_active: bool,
    rate: Rate,
}

impl TxState {
    pub fn parse(raw: u8) -> TxState {
        TxState {
            beacon_active: {
                let raw = raw & 0b11;
                if check_bits!(raw, RawTxStateFirstBit::BeaconActive as u8) {
                    true
                } else {
                    false
                }
            },
            idle: {
                let raw = raw & 0b11;
                if check_bits!(raw, RawTxStateFirstBit::IdleOn as u8) {
                    IdleState::On
                } else if check_bits!(raw, RawTxStateFirstBit::IdleOff as u8) {
                    IdleState::Off
                } else {
                    IdleState::Off
                }
            },
            rate: {
                let raw = (raw >> 2) & 0b11;
                if check_bits!(raw, RawTxStateSecondBit::B9600 as u8) {
                    Rate::B9600
                } else if check_bits!(raw, RawTxStateSecondBit::B4800 as u8) {
                    Rate::B4800
                } else if check_bits!(raw, RawTxStateSecondBit::B2400 as u8) {
                    Rate::B2400
                } else if check_bits!(raw, RawTxStateSecondBit::B1200 as u8) {
                    Rate::B1200
                } else {
                    Rate::B1200
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_min_values() {
        let raw = 0b0000;
        let result = TxState {
            beacon_active: false,
            idle: IdleState::Off,
            rate: Rate::B1200,
        };
        assert_eq!(result, TxState::parse(raw));
    }

    #[test]
    fn test_parse_idle_on() {
        let raw = 0b0001;
        let result = TxState {
            beacon_active: false,
            idle: IdleState::On,
            rate: Rate::B1200,
        };
        assert_eq!(result, TxState::parse(raw));
    }

    #[test]
    fn test_parse_beacon_active() {
        let raw = 0b0010;
        let result = TxState {
            beacon_active: true,
            idle: IdleState::Off,
            rate: Rate::B1200,
        };
        assert_eq!(result, TxState::parse(raw));
    }

    #[test]
    fn test_parse_rate_2400() {
        let raw = 0b0100;
        let result = TxState {
            beacon_active: false,
            idle: IdleState::Off,
            rate: Rate::B2400,
        };
        assert_eq!(result, TxState::parse(raw));
    }

    #[test]
    fn test_parse_rate_4800() {
        let raw = 0b1000;
        let result = TxState {
            beacon_active: false,
            idle: IdleState::Off,
            rate: Rate::B4800,
        };
        assert_eq!(result, TxState::parse(raw));
    }

    #[test]
    fn test_parse_rate_9600() {
        let raw = 0b1100;
        let result = TxState {
            beacon_active: false,
            idle: IdleState::Off,
            rate: Rate::B9600,
        };
        assert_eq!(result, TxState::parse(raw));
    }
}
