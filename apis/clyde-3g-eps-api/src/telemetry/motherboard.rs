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

//! Motherboard Telemetry
//!
//! This module provides the enum, commands and parsers necessary for working
//! with telemetry from the EPS motherboard.
//!
//! The macro `make_telemetry!` is responsibly for generating the enum `Type`, the
//! `parse` function and the `command` function.

use eps_api::EpsResult;
use rust_i2c::Command;
use crate::telemetry::lib::get_adc_result;

const TELEM_CMD: u8 = 0x10;

make_telemetry!(
    /// VBCR1 - Voltage feeding BRC1 (V)
    VoltageFeedingBcr1 => {vec![0xE1, 0x10], |d| (0.032_253_7 * d) - 0.051_236_678},
    /// IBCR1A - Current BCR1, Connector SA1A (A)
    CurrentBcr1Sa1a => {vec![0xE1, 0x14], |d| (0.978_131_613 * d) + 16.108_602_91},
    /// IBCR1B - Current BCR1, Connection SA1B (A)
    CurrentBcr1Sa1b => {vec![0xE1, 0x15], |d| (0.999_566_912 * d) + 2.087_050_671},
    /// TBCR1A - Array Temp, Connector SA1A (*C)
    ArrayTempSa1a => {vec![0xE1, 0x18], |d| (0.356_573_44 * d) - 273.884_02},
    /// TBCR1B - Array Temp, Connector SA1B (*C)
    ArrayTempSa1b => {vec![0xE1, 0x19], |d| (0.356_573_44 * d) - 273.884_02},
    /// SDBCR1A - Sun Detector, Connector SA1A (W/m^2)
    SunDetectorSa1a => {vec![0xE1, 0x1C], |d| d},
    /// SDBCR1B - Sun Detector, Connector SA1B (W/m^2)
    SunDetectorSa1b => {vec![0xE1, 0x1D], |d| d},

    /// VBCR2 - Voltage Feeding BCR2 (V)
    VoltageFeedingBcr2 => {vec![0xE1, 0x20], |d| (0.032_096_106 * d) + 0.056_830_684},
    /// IBCR2A - Current BCR2, Connection SA2A (A)
    CurrentBcr2Sa2a => {vec![0xE1, 0x24], |d| (0.976_340_483 * d) + 18.248_726_54},
    /// IBCR2B - Current BCR2, Connector SA2B (A)
    CurrentBcr2Sa2b => {vec![0xE1, 0x25], |d| (0.971_278_847 * d) + 10.953_978_97},
    /// TBCR2A - Array Temp, Connector SA2A (*C)
    ArrayTempSa2a => {vec![0xE1, 0x28], |d| (0.356_573_44 * d) - 273.884_02},
    /// TBCR2B - Array Temp, Connector SA2B (*C)
    ArrayTempSa2b => {vec![0xE1, 0x29], |d| (0.356_573_44 * d) - 273.884_02},
    /// SDBCR2A - Sun Detector, Connector SA2A (W/m^2)
    SunDetectorSa2a => {vec![0xE1, 0x2C], |d| d},
    /// SDBCR2B - Sun Detector, Connector SA2B (W/m^2)
    SunDetectorSa2b => {vec![0xE1, 0x2D], |d| d},

    /// VBCR3 - Voltage Feeding BCR3 (V)
    VoltageFeedingBcr3 => {vec![0xE1, 0x30], |d| (0.010_021_041 * d) + 0.019_172_526},
    /// IBCR3A - Current BCR3, Connection SA3A (A)
    CurrentBcr3Sa3a => {vec![0xE1, 0x34], |d| (0.979_728_933 * d) + 3.627_460_224},
    /// IBCR3B - Current BCR3, Connector SA3B (A)
    CurrentBcr3Sa3b => {vec![0xE1, 0x35], |d| (0.982_520_653 * d) + 7.656_558_533},
    /// TBCR3A - Array Temp, Connector SA3A (*C)
    ArrayTempSa3a => {vec![0xE1, 0x38], |d| (0.356_573_44 * d) - 273.884_02},
    /// TBCR3B - Array Temp, Connector SA3B (*C)
    ArrayTempSa3b => {vec![0xE1, 0x39], |d| (0.356_573_44 * d) - 273.884_02},
    /// SDBCR3A - Sun Detector, Connector SA3A (W/m^2)
    SunDetectorSa3a => {vec![0xE1, 0x3C], |d| d},
    /// SDBCR3B - Sun Detector, Connector SA3B (W/m^2)
    SunDetectorSa3b => {vec![0xE1, 0x3D], |d| d},

    /// IIDIODE_OUT - BCR Output Current (mA)
    BcrOutputCurrent => {vec![0xE2, 0x84], |d| (14.315_340_23 * d) + 25.431_078_9},
    /// VIDIODE_OUT - BCR Output Voltage (V)
    BcrOutputVoltage => {vec![0xE2, 0x80], |d| (0.009_049_712 * d) - 0.008_697_551},
    /// I3V3_DRW - 3V3 Current Draw of EPS (A)
    CurrentDraw3V3 => {vec![0xE2, 0x05], |d| 0.001_327_547 * d},
    /// I5V_DRW - 5V Current Draw of EPS (A)
    CurrentDraw5V => {vec![0xE2, 0x15], |d| 0.001_327_547 * d},

    /// IPCM12V - Output Current of 12V bus (A)
    OutputCurrent12V => {vec![0xE2, 0x34], |d| (2.069_546_232 * d) + 9.158_584_601},
    /// VPCM12V - Output Voltage of 12V bus (V)
    OutputVoltage12V => {vec![0xE2, 0x30], |d| 0.013_468_447 * d},
    /// IPCMBATV - Output Current of Battery Bus (A)
    OutputCurrentBattery => {vec![0xE2, 0x24], |d| (5.277_754_989 * d) + 7.128_757_361},
    /// VPCMBATV - Output Voltage of Battery Bus (V)
    OutputVoltageBattery => {vec![0xE2, 0x20], |d| 0.000_901_295_7 * d},
    /// IPCM5V - Output Current of 5V bus (A)
    OutputCurrent5v => {vec![0xE2, 0x14], |d| (5.244_380_133 * d) + 4.352_775_681},
    /// VPCM5V - Output Voltage of 5V bus (V)
    OutputVoltage5v => {vec![0xE2, 0x10], |d| 0.005_846_589 * d},
    /// IPCM3V3 - Output Current of 3.3V Bus (A)
    OutputCurrent33v => {vec![0xE2, 0x04], |d| (5.255_897_808 * d) + 32.568_735_93},
    /// VPCM3V3 - Output Voltage of 3.3V Bus (V)
    OutputVoltage33v => {vec![0xE2, 0x00], |d| 0.004_288_677 * d},

    /// VSW1 - Output Voltage Switch 1 (V)
    OutputVoltageSwitch1 => {vec![0xE4, 0x10], |d| 0.013_458_119 * d},
    /// ISW1 - Output Current Switch 1 (A)
    OutputCurrentSwitch1 => {vec![0xE4, 0x14], |d| (1.337_509_933 * d) + 1.643_798_992},
    /// VSW2 - Output Voltage Switch 2 (V)
    OutputVoltageSwitch2 => {vec![0xE4, 0x20], |d| 0.013_447_282 * d},
    /// ISW2 - Output Current Switch 2 (A)
    OutputCurrentSwitch2 => {vec![0xE4, 0x24], |d| (1.337_575_789 * d) + 1.084_230_776},
    /// VSW3 - Output Voltage Switch 3 (V)
    OutputVoltageSwitch3 => {vec![0xE4, 0x30], |d| 0.008_997_091 * d},
    /// ISW3 - Output Current Switch 3 (A)
    OutputCurrentSwitch3 => {vec![0xE4, 0x34], |d| (6.233_851_836 * d) + 12.648_034_55},
    /// VSW4 - Output Voltage Switch 4 (V)
    OutputVoltageSwitch4 => {vec![0xE4, 0x40], |d| 0.008_991_18 * d},
    /// ISW4 - Output Current Switch 4 (A)
    OutputCurrentSwitch4 => {vec![0xE4, 0x44], |d| (6.211_055_525 * d) - 3.686_512_211},
    /// VSW5 - Output Voltage Switch 5 (V)
    OutputVoltageSwitch5 => {vec![0xE4, 0x50], |d| 0.005_854_875 * d},
    /// ISW5 - Output Current Switch 5 (A)
    OutputCurrentSwitch5 => {vec![0xE4, 0x54], |d| (1.334_023_552 * d) + 0.541_582_24},
    /// VSW6 - Output Voltage Switch 6 (V)
    OutputVoltageSwitch6 => {vec![0xE4, 0x60], |d| 0.005_855_28 * d},
    /// ISW6 - Output Current Switch 6 (A)
    OutputCurrentSwitch6 => {vec![0xE4, 0x64], |d| (1.339_674_94 * d) + 2.712_662_269},
    /// VSW7 - Output Voltage Switch 7 (V)
    OutputVoltageSwitch7 => {vec![0xE4, 0x70], |d| 0.005_852_332 * d},
    /// ISW7 - Output Current Switch 7 (A)
    OutputCurrentSwitch7 => {vec![0xE4, 0x74], |d| (1.330_454_623 * d) + 1.345_607_22},
    /// VSW8 - Output Voltage Switch 8 (V)
    OutputVoltageSwitch8 => {vec![0xE4, 0x80], |d| 0.004_297_822 * d},
    /// ISW8 - Output Current Switch 8 (A)
    OutputCurrentSwitch8 => {vec![0xE4, 0x84], |d| (1.339_713_413 * d) + 0.018_954_145},
    /// VSW9 - Output Voltage Switch 9 (V)
    OutputVoltageSwitch9 => {vec![0xE4, 0x90], |d| 0.004_297_481 * d},
    /// ISW9 - Output Current Switch 9 (A)
    OutputCurrentSwitch9 => {vec![0xE4, 0x94], |d| (1.330_468_189 * d) - 1.054_319_922},
    /// VSW10 - Output Voltage Switch 10 (V)
    OutputVoltageSwitch10 => {vec![0xE4, 0xA0], |d| 0.004_295_835 * d},
    /// ISW10 - Output Current Switch 10 (A)
    OutputCurrentSwitch10 => {vec![0xE4, 0xA4], |d| (1.337_542_094 * d) + 2.166_832_506},

    /// TBRD - Board Temperature (*C)
    BoardTemperature => {vec![0xE3, 0x08], |d|  (0.356_573_44 * d) - 273.884_02},
);
