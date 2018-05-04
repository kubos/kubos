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

use failure::Error;
use i2c_hal::Command;
use telemetry::lib::get_adc_result;

const TELEM_CMD: u8 = 0x10;

/// TODO - need to make these parsing functions generic and configurable
make_telemetry!(
    /// VBCR1 - Voltage feeding BRC1 (V)
    VoltageFeedingBcr1 => {vec![0xE1, 0x10], |d| (0.0322537 * d) - 0.051236678},
    /// IBCR1A - Current BCR1, Connector SA1A (A)
    CurrentBcr1Sa1a => {vec![0xE1, 0x14], |d| (0.978131613 * d) + 16.10860291},
    /// IBCR1B - Current BCR1, Connection SA1B (A)
    CurrentBcr1Sa1b => {vec![0xE1, 0x15], |d| (0.999566912 * d) + 2.087050671},
    /// TBCR1A - Array Temp, Connector SA1A (*C)
    ArrayTempSa1a => {vec![0xE1, 0x18], |d| (0.35657344 * d) - 273.88402},
    /// TBCR1B - Array Temp, Connector SA1B (*C)
    ArrayTempSa1b => {vec![0xE1, 0x19], |d| (0.35657344 * d) - 273.88402},
    /// SDBCR1A - Sun Detector, Connector SA1A (W/m^2)
    SunDetectorSa1a => {vec![0xE1, 0x1C], |d| d},
    /// SDBCR1B - Sun Detector, Connector SA1B (W/m^2)
    SunDetectorSa1b => {vec![0xE1, 0x1D], |d| d},

    /// VBCR2 - Voltage Feeding BCR2 (V)
    VoltageFeedingBcr2 => {vec![0xE1, 0x20], |d| (0.032096106 * d) + 0.056830684},
    /// IBCR2A - Current BCR2, Connection SA2A (A)
    CurrentBcr2Sa2a => {vec![0xE1, 0x24], |d| (0.976340483 * d) + 18.24872654},
    /// IBCR2B - Current BCR2, Connector SA2B (A)
    CurrentBcr2Sa2b => {vec![0xE1, 0x25], |d| (0.971278847 * d) + 10.95397897},
    /// TBCR2A - Array Temp, Connector SA2A (*C)
    ArrayTempSa2a => {vec![0xE1, 0x28], |d| (0.35657344 * d) - 273.88402},
    /// TBCR2B - Array Temp, Connector SA2B (*C)
    ArrayTempSa2b => {vec![0xE1, 0x29], |d| (0.35657344 * d) - 273.88402},
    /// SDBCR2A - Sun Detector, Connector SA2A (W/m^2)
    SunDetectorSa2a => {vec![0xE1, 0x2C], |d| d},
    /// SDBCR2B - Sun Detector, Connector SA2B (W/m^2)
    SunDetectorSa2b => {vec![0xE1, 0x2D], |d| d},

    /// VBCR3 - Voltage Feeding BCR3 (V)
    VoltageFeedingBcr3 => {vec![0xE1, 0x30], |d| (0.010021041 * d) + 0.019172526},
    /// IBCR3A - Current BCR3, Connection SA3A (A)
    CurrentBcr3Sa3a => {vec![0xE1, 0x34], |d| (0.979728933 * d) + 3.627460224},
    /// IBCR3B - Current BCR3, Connector SA3B (A)
    CurrentBcr3Sa3b => {vec![0xE1, 0x35], |d| (0.982520653 * d) + 7.656558533},
    /// TBCR3A - Array Temp, Connector SA3A (*C)
    ArrayTempSa3a => {vec![0xE1, 0x38], |d| (0.35657344 * d) - 273.88402},
    /// TBCR3B - Array Temp, Connector SA3B (*C)
    ArrayTempSa3b => {vec![0xE1, 0x39], |d| (0.35657344 * d) - 273.88402},
    /// SDBCR3A - Sun Detector, Connector SA3A (W/m^2)
    SunDetectorSa3a => {vec![0xE1, 0x3C], |d| d},
    /// SDBCR3B - Sun Detector, Connector SA3B (W/m^2)
    SunDetectorSa3b => {vec![0xE1, 0x3D], |d| d},

    /// IIDIODE_OUT - BCR Output Current (mA)
    BcrOutputCurrent => {vec![0xE2, 0x84], |d| (14.31534023 * d) + 25.4310789},
    /// VIDIODE_OUT - BCR Output Voltage (V)
    BcrOutputVoltage => {vec![0xE2, 0x80], |d| (0.009049712 * d) - 0.008697551},
    /// I3V3_DRW - 3V3 Current Draw of EPS (A)
    CurrentDraw3V3 => {vec![0xE2, 0x05], |d| 0.001327547 * d},
    /// I5V_DRW - 5V Current Draw of EPS (A)
    CurrentDraw5V => {vec![0xE2, 0x15], |d| 0.001327547 * d},

    /// IPCM12V - Output Current of 12V bus (A)
    OutputCurrent12V => {vec![0xE2, 0x34], |d| (2.069546232 * d) + 9.158584601},
    /// VPCM12V - Output Voltage of 12V bus (V)
    OutputVoltage12V => {vec![0xE2, 0x30], |d| 0.013468447 * d},
    /// IPCMBATV - Output Current of Battery Bus (A)
    OutputCurrentBattery => {vec![0xE2, 0x24], |d| (5.277754989 * d) + 7.128757361},
    /// VPCMBATV - Output Voltage of Battery Bus (V)
    OutputVoltageBattery => {vec![0xE2, 0x20], |d| 0.0009012957 * d},
    /// IPCM5V - Output Current of 5V bus (A)
    OutputCurrent5v => {vec![0xE2, 0x14], |d| (5.244380133 * d) + 4.352775681},
    /// VPCM5V - Output Voltage of 5V bus (V)
    OutputVoltage5v => {vec![0xE2, 0x10], |d| 0.005846589 * d},
    /// IPCM3V3 - Output Current of 3.3V Bus (A)
    OutputCurrent33v => {vec![0xE2, 0x04], |d| (5.255897808 * d) + 32.56873593},
    /// VPCM3V3 - Output Voltage of 3.3V Bus (V)
    OutputVoltage33v => {vec![0xE2, 0x00], |d| 0.004288677 * d},

    /// VSW1 - Output Voltage Switch 1 (V)
    OutputVoltageSwitch1 => {vec![0xE4, 0x10], |d| 0.013458119 * d},
    /// ISW1 - Output Current Switch 1 (A)
    OutputCurrentSwitch1 => {vec![0xE4, 0x14], |d| (1.337509933 * d) + 1.643798992},
    /// VSW2 - Output Voltage Switch 2 (V)
    OutputVoltageSwitch2 => {vec![0xE4, 0x20], |d| 0.013447282 * d},
    /// ISW2 - Output Current Switch 2 (A)
    OutputCurrentSwitch2 => {vec![0xE4, 0x24], |d| (1.337575789 * d) + 1.084230776},
    /// VSW3 - Output Voltage Switch 3 (V)
    OutputVoltageSwitch3 => {vec![0xE4, 0x30], |d| 0.008997091 * d},
    /// ISW3 - Output Current Switch 3 (A)
    OutputCurrentSwitch3 => {vec![0xE4, 0x34], |d| (6.233851836 * d) + 12.64803455},
    /// VSW4 - Output Voltage Switch 4 (V)
    OutputVoltageSwitch4 => {vec![0xE4, 0x40], |d| 0.00899118 * d},
    /// ISW4 - Output Current Switch 4 (A)
    OutputCurrentSwitch4 => {vec![0xE4, 0x44], |d| (6.211055525 * d) - 3.686512211},
    /// VSW5 - Output Voltage Switch 5 (V)
    OutputVoltageSwitch5 => {vec![0xE4, 0x50], |d| 0.005854875 * d},
    /// ISW5 - Output Current Switch 5 (A)
    OutputCurrentSwitch5 => {vec![0xE4, 0x54], |d| (1.334023552 * d) + 0.54158224},
    /// VSW6 - Output Voltage Switch 6 (V)
    OutputVoltageSwitch6 => {vec![0xE4, 0x60], |d| 0.00585528 * d},
    /// ISW6 - Output Current Switch 6 (A)
    OutputCurrentSwitch6 => {vec![0xE4, 0x64], |d| (1.33967494 * d) + 2.712662269},
    /// VSW7 - Output Voltage Switch 7 (V)
    OutputVoltageSwitch7 => {vec![0xE4, 0x70], |d| 0.005852332 * d},
    /// ISW7 - Output Current Switch 7 (A)
    OutputCurrentSwitch7 => {vec![0xE4, 0x74], |d| (1.330454623 * d) + 1.34560722},
    /// VSW8 - Output Voltage Switch 8 (V)
    OutputVoltageSwitch8 => {vec![0xE4, 0x80], |d| 0.004297822 * d},
    /// ISW8 - Output Current Switch 8 (A)
    OutputCurrentSwitch8 => {vec![0xE4, 0x84], |d| (1.339713413 * d) + 0.018954145},
    /// VSW9 - Output Voltage Switch 9 (V)
    OutputVoltageSwitch9 => {vec![0xE4, 0x90], |d| 0.004297481 * d},
    /// ISW9 - Output Current Switch 9 (A)
    OutputCurrentSwitch9 => {vec![0xE4, 0x94], |d| (1.330468189 * d) - 1.054319922},
    /// VSW10 - Output Voltage Switch 10 (V)
    OutputVoltageSwitch10 => {vec![0xE4, 0xA0], |d| 0.004295835 * d},
    /// ISW10 - Output Current Switch 10 (A)
    OutputCurrentSwitch10 => {vec![0xE4, 0xA4], |d| (1.337542094 * d) + 2.166832506},

    /// TBRD - Board Temperature (*C)
    BoardTemperature => {vec![0xE3, 0x08], |d|  (0.35657344 * d) - 273.88402},
);
