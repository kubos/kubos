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

//! Daughterboard Telemetry
//!
//! This module provides the enum, commands and parsers necessary for working
//! with telemetry from the EPS daughterboard.
//!
//! The macro `make_telemetry!` is responsibly for generating the enum `Type`, the
//! `parse` function and the `command` function.

use eps_api::EpsResult;
use rust_i2c::Command;
use crate::telemetry::lib::get_adc_result;

const TELEM_CMD: u8 = 0x10;

make_telemetry!(
    /// VBCR4 - Voltage feeding BRC4 (V)
    VoltageFeedingBcr4 => {vec![0xE1, 0x40], |d| (0.032_233_143 * d) + 0.022_840_592},
    /// IBCR4A - Current BCR4, Connector SA4A (A)
    CurrentBcr4Sa4a => {vec![0xE1, 0x44], |d| (0.977_821_782 * d) - 3.020_660_066},
    /// IBCR4B - Current BCR4, Connection SA4B (A)
    CurrentBcr4Sa4b => {vec![0xE1, 0x45], |d| (0.982_567_819 * d) + 0.388_437_306},
    /// TBCR4A - Array Temp, Connector SA4A (*C)
    ArrayTempSa4a => {vec![0xE1, 0x48], |d| (0.356_573_44 * d) - 273.884_02},
    /// TBCR4B - Array Temp, Connector SA4B (*C)
    ArrayTempSa4b => {vec![0xE1, 0x49], |d| (0.356_573_44 * d) - 273.884_02},
    /// SDBCR4A - Sun Detector, Connector SA4A (W/m^2)
    SunDetectorSa4a => {vec![0xE1, 0x4C], |d| d},
    /// SDBCR4B - Sun Detector, Connector SA4B (W/m^2)
    SunDetectorSa4b => {vec![0xE1, 0x4D], |d| d},

    /// VBCR5 - Voltage Feeding BCR5 (V)
    VoltageFeedingBcr5 => {vec![0xE1, 0x50], |d| (0.032_515_932 * d) - 0.085_250_182},
    /// IBCR5A - Current BCR5, Connection SA5A (A)
    CurrentBcr5Sa5a => {vec![0xE1, 0x54], |d| (0.977_821_782 * d) - 1.065_016_502},
    /// IBCR5B - Current BCR5, Connector SA5B (A)
    CurrentBcr5Sa5b => {vec![0xE1, 0x55], |d| (0.980_922_045 * d) - 2.734_752_724},
    /// TBCR5A - Array Temp, Connector SA5A (*C)
    ArrayTempSa5a => {vec![0xE1, 0x58], |d| (0.356_573_44 * d) - 273.884_02},
    /// TBCR5B - Array Temp, Connector SA5B (*C)
    ArrayTempSa5b => {vec![0xE1, 0x59], |d| (0.356_573_44 * d) - 273.884_02},
    /// SDBCR5A - Sun Detector, Connector SA5A (W/m^2)
    SunDetectorSa5a => {vec![0xE1, 0x5C], |d| d},
    /// SDBCR5B - Sun Detector, Connector SA5B (W/m^2)
    SunDetectorSa5b => {vec![0xE1, 0x5D], |d| d},

    /// VBCR6 - Voltage Feeding BCR6 (V)
    VoltageFeedingBcr6 => {vec![0xE1, 0x60], |d| (0.032_338_203 * d) + 0.063_747_86},
    /// IBCR6A - Current BCR6, Connection SA6A (A)
    CurrentBcr6Sa6a => {vec![0xE1, 0x64], |d| (0.967_231_18 * d) - 4.607_992_112},
    /// IBCR6B - Current BCR6, Connector SA6B (A)
    CurrentBcr6Sa6b => {vec![0xE1, 0x65], |d| (0.985_693_552 * d) - 10.183_288_41},
    /// TBCR6A - Array Temp, Connector SA6A (*C)
    ArrayTempSa6a => {vec![0xE1, 0x68], |d| (0.356_573_44 * d) - 273.884_02},
    /// TBCR6B - Array Temp, Connector SA6B (*C)
    ArrayTempSa6b => {vec![0xE1, 0x69], |d| (0.356_573_44 * d) - 273.884_02},
    /// SDBCR6A - Sun Detector, Connector SA6A (W/m^2)
    SunDetectorSa6a => {vec![0xE1, 0x6C], |d| d},
    /// SDBCR6B - Sun Detector, Connector SA6B (W/m^2)
    SunDetectorSa6b => {vec![0xE1, 0x6D], |d| d},

    /// VBCR7 - Voltage Feeding BCR7 (V)
    VoltageFeedingBcr7 => {vec![0xE1, 0x70], |d| (0.032_110_025 * d) + 0.035_328_493},
    /// IBCR7A - Current BCR7, Connection SA7A (A)
    CurrentBcr7Sa7a => {vec![0xE1, 0x74], |d| (0.984_719_536 * d) - 4.746_679_562},
    /// IBCR7B - Current BCR7, Connector SA7B (A)
    CurrentBcr7Sa7b => {vec![0xE1, 0x75], |d| (0.972_638_482 * d) - 3.537_251_86},
    /// TBCR7A - Array Temp, Connector SA7A (*C)
    ArrayTempSa7a => {vec![0xE1, 0x78], |d| (0.356_573_44 * d) - 273.884_02},
    /// TBCR7B - Array Temp, Connector SA7B (*C)
    ArrayTempSa7b => {vec![0xE1, 0x79], |d| (0.356_573_44 * d) - 273.884_02},
    /// SDBCR7A - Sun Detector, Connector SA7A (W/m^2)
    SunDetectorSa7a => {vec![0xE1, 0x7C], |d| d},
    /// SDBCR7B - Sun Detector, Connector SA7B (W/m^2)
    SunDetectorSa7b => {vec![0xE1, 0x7D], |d| d},

    /// VBCR8 - Voltage Feeding BCR8 (V)
    VoltageFeedingBcr8 => {vec![0xE1, 0x80], |d| (0.032_396_988 * d) - 0.081_490_692},
    /// IBCR8A - Current BCR8, Connection SA8A (A)
    CurrentBcr8Sa8a => {vec![0xE1, 0x84], |d| (0.977_621_05 * d) - 8.569_171_301},
    /// IBCR8B - Current BCR8, Connector SA8B (A)
    CurrentBcr8Sa8b => {vec![0xE1, 0x85], |d| (0.987_950_139 * d) - 4.917_313_019},
    /// TBCR8A - Array Temp, Connector SA8A (*C)
    ArrayTempSa8a => {vec![0xE1, 0x88], |d| (0.356_573_44 * d) - 273.884_02},
    /// TBCR8B - Array Temp, Connector SA8B (*C)
    ArrayTempSa8b => {vec![0xE1, 0x89], |d| (0.356_573_44 * d) - 273.884_02},
    /// SDBCR8A - Sun Detector, Connector SA8A (W/m^2)
    SunDetectorSa8a => {vec![0xE1, 0x8C], |d| d},
    /// SDBCR8B - Sun Detector, Connector SA8B (W/m^2)
    SunDetectorSa8b => {vec![0xE1, 0x8D], |d| d},

    /// VBCR9 - Voltage Feeding BCR9 (V)
    VoltageFeedingBcr9 => {vec![0xE1, 0x90], |d| (0.032_258_137 * d) + 0.020_293_952},
    /// IBCR9A - Current BC96, Connection SA9A (A)
    CurrentBcr9Sa9a => {vec![0xE1, 0x94], |d| (0.964_867_436 * d) - 3.896_088_456},
    /// IBCR9B - Current BCR9, Connector SA9B (A)
    CurrentBcr9Sa9b => {vec![0xE1, 0x95], |d| (0.991_113_02 * d) - 10.112_293_11},
    /// TBCR9A - Array Temp, Connector SA9A (*C)
    ArrayTempSa9a => {vec![0xE1, 0x98], |d| (0.356_573_44 * d) - 273.884_02},
    /// TBCR9B - Array Temp, Connector SA9B (*C)
    ArrayTempSa9b => {vec![0xE1, 0x99], |d| (0.356_573_44 * d) - 273.884_02},
    /// SDBCR9A - Sun Detector, Connector SA9A (W/m^2)
    SunDetectorSa9a => {vec![0xE1, 0x9C], |d| d},
    /// SDBCR9B - Sun Detector, Connector SA9B (W/m^2)
    SunDetectorSa9b => {vec![0xE1, 0x9D], |d| d},

    /// TLM_TBRD_DB - Board Temperature (*C)
    BoardTemperature => {vec![0xE3, 0x88], |d|  (0.356_573_44 * d) - 273.884_02},
);
