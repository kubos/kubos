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

use serde_json;
use ffi::*;

#[derive(Debug, Serialize)]
pub enum Config {
    /// Select MTM to use for measurement. 0 - Internal, 1 - External
    MtmSelect(u8),
    /// Integration time selection for idle mode MTM measurements. <em>Refer to Table 3-10 of the iMTQ User Manual for more information</em>
    MtmInternalTime(u8),
    /// Integration time selection for idle mode MTM measurements. <em>Refer to Table 3-10 of the iMTQ User Manual for more information</em>
    MtmExternalTime(u8),
    /// iMTQ axis to which the MTM x-axis is mapped
    MtmInternalMapX(u8),
    /// iMTQ axis to which the MTM y-axis is mapped
    MtmInternalMapY(u8),
    /// iMTQ axis to which the MTM z-axis is mapped
    MtmInternalMapZ(u8),
    /// iMTQ axis to which the MTM x-axis is mapped
    MtmExternalMapX(u8),
    /// iMTQ axis to which the MTM y-axis is mapped
    MtmExternalMapY(u8),
    /// iMTQ axis to which the MTM z-axis is mapped
    MtmExternalMapZ(u8),
    /// MTM raw -> corrected correction matrix (Row 1, Column 1)
    MtmMatrixR1C1(f64),
    /// MTM raw -> corrected correction matrix (Row 1, Column 2)
    MtmMatrixR1C2(f64),
    /// MTM raw -> corrected correction matrix (Row 1, Column 3)
    MtmMatrixR1C3(f64),
    /// MTM raw -> corrected correction matrix (Row 2, Column 1)
    MtmMatrixR2C1(f64),
    /// MTM raw -> corrected correction matrix (Row 2, Column 2)
    MtmMatrixR2C2(f64),
    /// MTM raw -> corrected correction matrix (Row 2, Column 3)
    MtmMatrixR2C3(f64),
    /// MTM raw -> corrected correction matrix (Row 3, Column 1)
    MtmMatrixR3C1(f64),
    /// MTM raw -> corrected correction matrix (Row 3, Column 2)
    MtmMatrixR3C2(f64),
    /// MTM raw -> corrected correction matrix (Row 3, Column 3)
    MtmMatrixR3C3(f64),
    /// MTM raw -> corrected correction bias vector (X-axis value)
    MtmBiasX(f64),
    /// MTM raw -> corrected correction bias vector (Y-axis value)
    MtmBiasY(f64),
    /// MTM raw -> corrected correction bias vector (Z-axis value)
    MtmBiasZ(f64),
    /// X-axis voltage bias for coil current ADC -> engineering value conversion
    AdcCoilCurrentBiasX(i16),
    /// Y-axis voltage bias for coil current ADC -> engineering value conversion
    AdcCoilCurrentBiasY(i16),
    /// Z-axis voltage bias for coil current ADC -> engineering value conversion
    AdcCoilCurrentBiasZ(i16),
    /// X-axis pre-multiplier for coil current ADC -> engineering value conversion
    AdcCoilCurrentMultX(i16),
    /// Y-axis pre-multiplier for coil current ADC -> engineering value conversion
    AdcCoilCurrentMultY(i16),
    /// Z-axis pre-multiplier for coil current ADC -> engineering value conversion
    AdcCoilCurrentMultZ(i16),
    /// X-axis post-divider for coil current ADC -> engineering value conversion
    AdcCoilCurrentDivX(i16),
    /// Y-axis post-divider for coil current ADC -> engineering value conversion
    AdcCoilCurrentDivY(i16),
    /// Z-axis post-divider for coil current ADC -> engineering value conversion
    AdcCoilCurrentDivZ(i16),
    /// X-axis voltage bias for coil temperature ADC -> engineering value conversion
    AdcCoilTempBiasX(i16),
    /// Y-axis voltage bias for coil temperature ADC -> engineering value conversion
    AdcCoilTempBiasY(i16),
    /// Z-axis voltage bias for coil temperature ADC -> engineering value conversion
    AdcCoilTempBiasZ(i16),
    /// X-axis pre-multiplier for coil temperature ADC -> engineering value conversion
    AdcCoilTempMultX(i16),
    /// Y-axis pre-multiplier for coil temperature ADC -> engineering value conversion
    AdcCoilTempMultY(i16),
    /// Z-axis pre-multiplier for coil temperature ADC -> engineering value conversion
    AdcCoilTempMultZ(i16),
    /// X-axis post-divider for coil temperature ADC -> engineering value conversion
    AdcCoilTempDivX(i16),
    /// Y-axis post-divider for coil temperature ADC -> engineering value conversion
    AdcCoilTempDivY(i16),
    /// Z-axis post-divider for coil temperature ADC -> engineering value conversion
    AdcCoilTempDivZ(i16),
    /// Control frequency of the detumble mode control loop. <em>Values: 1, 2, 4, or 8 Hz</em>
    DetumbleFrequency(u8),
    /// B-Dot algorithm gain when converting from B-Dot to dipole. Value should be negative
    BdotGain(f64),
    /// Adaptive sensitivity of low-pass filter applied to calibrated MTM measurements during detumble mode
    MtmFilterSensitivity(f64),
    /// Adaptive weight of low-pass filter applied to calibrated MTM measurements during detumble mode
    MtmFilterWeight(f64),
    /// X-axis area of the coil used to calculate the dipole from the current flowing through the coil
    CoilAreaX(f64),
    /// Y-axis area of the coil used to calculate the dipole from the current flowing through the coil
    CoilAreaY(f64),
    /// Z-axis area of the coil used to calculate the dipole from the current flowing through the coil
    CoilAreaZ(f64),
    /// Maximum total coil current allowed for dipole generation (excluding idle current consumption)
    CoilCurrentLimit(u16),
    /// Current feedback control. 0 - Open-loop temperature-compensated, 1 - Software-based closed-loop
    CurrentFeedbackEnable(u8),
    /// X-axis feedback gain of the proportional difference controller
    CurrentFeedbackGainX(i32),
    /// Y-axis feedback gain of the proportional difference controller
    CurrentFeedbackGainY(i32),
    /// Z-axis feedback gain of the proportional difference controller
    CurrentFeedbackGainZ(i32),
    /// Current-map profile temperature 1 (lowest)
    CurrentMapTempT1(i16),
    /// Current-map profile temperature 2
    CurrentMapTempT2(i16),
    /// Current-map profile temperature 3
    CurrentMapTempT3(i16),
    /// Current-map profile temperature 4
    CurrentMapTempT4(i16),
    /// Current-map profile temperature 5
    CurrentMapTempT5(i16),
    /// Current-map profile temperature 6
    CurrentMapTempT6(i16),
    /// Current-map profile temperature 7 (highest)
    CurrentMapTempT7(i16),
    /// X-axis maximum current at temperature 1
    CurrentMaxXT1(i16),
    /// X-axis maximum current at temperature 2
    CurrentMaxXT2(i16),
    /// X-axis maximum current at temperature 3
    CurrentMaxXT3(i16),
    /// X-axis maximum current at temperature 4
    CurrentMaxXT4(i16),
    /// X-axis maximum current at temperature 5
    CurrentMaxXT5(i16),
    /// X-axis maximum current at temperature 6
    CurrentMaxXT6(i16),
    /// X-axis maximum current at temperature 7
    CurrentMaxXT7(i16),
    /// Y-axis maximum current at temperature 1
    CurrentMaxYT1(i16),
    /// Y-axis maximum current at temperature 2
    CurrentMaxYT2(i16),
    /// Y-axis maximum current at temperature 3
    CurrentMaxYT3(i16),
    /// Y-axis maximum current at temperature 4
    CurrentMaxYT4(i16),
    /// Y-axis maximum current at temperature 5
    CurrentMaxYT5(i16),
    /// Y-axis maximum current at temperature 6
    CurrentMaxYT6(i16),
    /// Y-axis maximum current at temperature 7
    CurrentMaxYT7(i16),
    /// Z-axis maximum current at temperature 1
    CurrentMaxZT1(i16),
    /// Z-axis maximum current at temperature 2
    CurrentMaxZT2(i16),
    /// Z-axis maximum current at temperature 3
    CurrentMaxZT3(i16),
    /// Z-axis maximum current at temperature 4
    CurrentMaxZT4(i16),
    /// Z-axis maximum current at temperature 5
    CurrentMaxZT5(i16),
    /// Z-axis maximum current at temperature 6
    CurrentMaxZT6(i16),
    /// Z-axis maximum current at temperature 7
    CurrentMaxZT7(i16),
}

impl Config {
    pub fn parse(&self) -> (u16, FFIConfigValue) {
        match *self {
            Config::MtmSelect(n) => (0x2002, FFIConfigValue { u8_val: n }),
            Config::MtmInternalTime(n) => (0x2003, FFIConfigValue { u8_val: n }),
            Config::MtmExternalTime(n) => (0x2004, FFIConfigValue { u8_val: n }),
            Config::MtmInternalMapX(n) => (0x2005, FFIConfigValue { u8_val: n }),
            Config::MtmInternalMapY(n) => (0x2006, FFIConfigValue { u8_val: n }),
            Config::MtmInternalMapZ(n) => (0x2007, FFIConfigValue { u8_val: n }),
            Config::MtmExternalMapX(n) => (0x2008, FFIConfigValue { u8_val: n }),
            Config::MtmExternalMapY(n) => (0x2009, FFIConfigValue { u8_val: n }),
            Config::MtmExternalMapZ(n) => (0x200A, FFIConfigValue { u8_val: n }),
            Config::MtmMatrixR1C1(n) => (0xA001, FFIConfigValue { f64_val: n }),
            Config::MtmMatrixR1C2(n) => (0xA002, FFIConfigValue { f64_val: n }),
            Config::MtmMatrixR1C3(n) => (0xA003, FFIConfigValue { f64_val: n }),
            Config::MtmMatrixR2C1(n) => (0xA004, FFIConfigValue { f64_val: n }),
            Config::MtmMatrixR2C2(n) => (0xA005, FFIConfigValue { f64_val: n }),
            Config::MtmMatrixR2C3(n) => (0xA006, FFIConfigValue { f64_val: n }),
            Config::MtmMatrixR3C1(n) => (0xA007, FFIConfigValue { f64_val: n }),
            Config::MtmMatrixR3C2(n) => (0xA008, FFIConfigValue { f64_val: n }),
            Config::MtmMatrixR3C3(n) => (0xA009, FFIConfigValue { f64_val: n }),
            Config::MtmBiasX(n) => (0xA00A, FFIConfigValue { f64_val: n }),
            Config::MtmBiasY(n) => (0xA00B, FFIConfigValue { f64_val: n }),
            Config::MtmBiasZ(n) => (0xA00C, FFIConfigValue { f64_val: n }),
            Config::AdcCoilCurrentBiasX(n) => (0x301C, FFIConfigValue { i16_val: n }),
            Config::AdcCoilCurrentBiasY(n) => (0x301D, FFIConfigValue { i16_val: n }),
            Config::AdcCoilCurrentBiasZ(n) => (0x301E, FFIConfigValue { i16_val: n }),
            Config::AdcCoilCurrentMultX(n) => (0x301F, FFIConfigValue { i16_val: n }),
            Config::AdcCoilCurrentMultY(n) => (0x3020, FFIConfigValue { i16_val: n }),
            Config::AdcCoilCurrentMultZ(n) => (0x3021, FFIConfigValue { i16_val: n }),
            Config::AdcCoilCurrentDivX(n) => (0x3022, FFIConfigValue { i16_val: n }),
            Config::AdcCoilCurrentDivY(n) => (0x3023, FFIConfigValue { i16_val: n }),
            Config::AdcCoilCurrentDivZ(n) => (0x3024, FFIConfigValue { i16_val: n }),
            Config::AdcCoilTempBiasX(n) => (0x3025, FFIConfigValue { i16_val: n }),
            Config::AdcCoilTempBiasY(n) => (0x3026, FFIConfigValue { i16_val: n }),
            Config::AdcCoilTempBiasZ(n) => (0x3027, FFIConfigValue { i16_val: n }),
            Config::AdcCoilTempMultX(n) => (0x3028, FFIConfigValue { i16_val: n }),
            Config::AdcCoilTempMultY(n) => (0x3029, FFIConfigValue { i16_val: n }),
            Config::AdcCoilTempMultZ(n) => (0x302A, FFIConfigValue { i16_val: n }),
            Config::AdcCoilTempDivX(n) => (0x302B, FFIConfigValue { i16_val: n }),
            Config::AdcCoilTempDivY(n) => (0x302C, FFIConfigValue { i16_val: n }),
            Config::AdcCoilTempDivZ(n) => (0x302D, FFIConfigValue { i16_val: n }),
            Config::DetumbleFrequency(n) => (0x2000, FFIConfigValue { u8_val: n }),
            Config::BdotGain(n) => (0xA000, FFIConfigValue { f64_val: n }),
            Config::MtmFilterSensitivity(n) => (0xA00D, FFIConfigValue { f64_val: n }),
            Config::MtmFilterWeight(n) => (0xA00E, FFIConfigValue { f64_val: n }),
            Config::CoilAreaX(n) => (0xA00F, FFIConfigValue { f64_val: n }),
            Config::CoilAreaY(n) => (0xA010, FFIConfigValue { f64_val: n }),
            Config::CoilAreaZ(n) => (0xA011, FFIConfigValue { f64_val: n }),
            Config::CoilCurrentLimit(n) => (0x4000, FFIConfigValue { u16_val: n }),
            Config::CurrentFeedbackEnable(n) => (0x2001, FFIConfigValue { u8_val: n }),
            Config::CurrentFeedbackGainX(n) => (0x5000, FFIConfigValue { i32_val: n }),
            Config::CurrentFeedbackGainY(n) => (0x5001, FFIConfigValue { i32_val: n }),
            Config::CurrentFeedbackGainZ(n) => (0x5002, FFIConfigValue { i32_val: n }),
            Config::CurrentMapTempT1(n) => (0x3000, FFIConfigValue { i16_val: n }),
            Config::CurrentMapTempT2(n) => (0x3001, FFIConfigValue { i16_val: n }),
            Config::CurrentMapTempT3(n) => (0x3002, FFIConfigValue { i16_val: n }),
            Config::CurrentMapTempT4(n) => (0x3003, FFIConfigValue { i16_val: n }),
            Config::CurrentMapTempT5(n) => (0x3004, FFIConfigValue { i16_val: n }),
            Config::CurrentMapTempT6(n) => (0x3005, FFIConfigValue { i16_val: n }),
            Config::CurrentMapTempT7(n) => (0x3006, FFIConfigValue { i16_val: n }),
            Config::CurrentMaxXT1(n) => (0x3007, FFIConfigValue { i16_val: n }),
            Config::CurrentMaxXT2(n) => (0x3008, FFIConfigValue { i16_val: n }),
            Config::CurrentMaxXT3(n) => (0x3009, FFIConfigValue { i16_val: n }),
            Config::CurrentMaxXT4(n) => (0x300A, FFIConfigValue { i16_val: n }),
            Config::CurrentMaxXT5(n) => (0x300B, FFIConfigValue { i16_val: n }),
            Config::CurrentMaxXT6(n) => (0x300C, FFIConfigValue { i16_val: n }),
            Config::CurrentMaxXT7(n) => (0x300D, FFIConfigValue { i16_val: n }),
            Config::CurrentMaxYT1(n) => (0x300E, FFIConfigValue { i16_val: n }),
            Config::CurrentMaxYT2(n) => (0x300F, FFIConfigValue { i16_val: n }),
            Config::CurrentMaxYT3(n) => (0x3010, FFIConfigValue { i16_val: n }),
            Config::CurrentMaxYT4(n) => (0x3011, FFIConfigValue { i16_val: n }),
            Config::CurrentMaxYT5(n) => (0x3012, FFIConfigValue { i16_val: n }),
            Config::CurrentMaxYT6(n) => (0x3013, FFIConfigValue { i16_val: n }),
            Config::CurrentMaxYT7(n) => (0x3014, FFIConfigValue { i16_val: n }),
            Config::CurrentMaxZT1(n) => (0x3015, FFIConfigValue { i16_val: n }),
            Config::CurrentMaxZT2(n) => (0x3016, FFIConfigValue { i16_val: n }),
            Config::CurrentMaxZT3(n) => (0x3017, FFIConfigValue { i16_val: n }),
            Config::CurrentMaxZT4(n) => (0x3018, FFIConfigValue { i16_val: n }),
            Config::CurrentMaxZT5(n) => (0x3019, FFIConfigValue { i16_val: n }),
            Config::CurrentMaxZT6(n) => (0x301A, FFIConfigValue { i16_val: n }),
            Config::CurrentMaxZT7(n) => (0x301B, FFIConfigValue { i16_val: n }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64;

    #[test]
    fn test_serialize_mtm_select() {
        let (p, v) = Config::MtmSelect(u8::max_value()).parse();
        assert_eq!(0x2002, p);
        assert_eq!(u8::max_value(), unsafe { v.u8_val });
    }

    #[test]
    fn test_serialize_mtm_internal_time() {
        let (p, v) = Config::MtmInternalTime(u8::max_value()).parse();
        assert_eq!(0x2003, p);
        assert_eq!(u8::max_value(), unsafe { v.u8_val });
    }

    #[test]
    fn test_serialize_mtm_external_time() {
        let (p, v) = Config::MtmExternalTime(u8::max_value()).parse();
        assert_eq!(0x2004, p);
        assert_eq!(u8::max_value(), unsafe { v.u8_val });
    }

    #[test]
    fn test_serialize_mtm_internal_map_x() {
        let (p, v) = Config::MtmInternalMapX(u8::max_value()).parse();
        assert_eq!(0x2005, p);
        assert_eq!(u8::max_value(), unsafe { v.u8_val });
    }

    #[test]
    fn test_serialize_mtm_internal_map_y() {
        let (p, v) = Config::MtmInternalMapY(u8::max_value()).parse();
        assert_eq!(0x2006, p);
        assert_eq!(u8::max_value(), unsafe { v.u8_val });
    }

    #[test]
    fn test_serialize_mtm_internal_map_z() {
        let (p, v) = Config::MtmInternalMapZ(u8::max_value()).parse();
        assert_eq!(0x2007, p);
        assert_eq!(u8::max_value(), unsafe { v.u8_val });
    }

    #[test]
    fn test_serialize_mtm_external_map_x() {
        let (p, v) = Config::MtmExternalMapX(u8::max_value()).parse();
        assert_eq!(0x2008, p);
        assert_eq!(u8::max_value(), unsafe { v.u8_val });
    }

    #[test]
    fn test_serialize_mtm_external_map_y() {
        let (p, v) = Config::MtmExternalMapY(u8::max_value()).parse();
        assert_eq!(0x2009, p);
        assert_eq!(u8::max_value(), unsafe { v.u8_val });
    }

    #[test]
    fn test_serialize_mtm_external_map_z() {
        let (p, v) = Config::MtmExternalMapZ(u8::max_value()).parse();
        assert_eq!(0x200A, p);
        assert_eq!(u8::max_value(), unsafe { v.u8_val });
    }

    #[test]
    fn test_serialize_mtm_matrix_r1_c1() {
        let (p, v) = Config::MtmMatrixR1C1(f64::MAX).parse();
        assert_eq!(0xA001, p);
        assert_eq!(f64::MAX, unsafe { v.f64_val });
    }

    #[test]
    fn test_serialize_mtm_matrix_r1_c2() {
        let (p, v) = Config::MtmMatrixR1C2(f64::MAX).parse();
        assert_eq!(0xA002, p);
        assert_eq!(f64::MAX, unsafe { v.f64_val });
    }

    #[test]
    fn test_serialize_mtm_matrix_r1_c3() {
        let (p, v) = Config::MtmMatrixR1C3(f64::MAX).parse();
        assert_eq!(0xA003, p);
        assert_eq!(f64::MAX, unsafe { v.f64_val });
    }

    #[test]
    fn test_serialize_mtm_matrix_r2_c1() {
        let (p, v) = Config::MtmMatrixR2C1(f64::MAX).parse();
        assert_eq!(0xA004, p);
        assert_eq!(f64::MAX, unsafe { v.f64_val });
    }

    #[test]
    fn test_serialize_mtm_matrix_r2_c2() {
        let (p, v) = Config::MtmMatrixR2C2(f64::MAX).parse();
        assert_eq!(0xA005, p);
        assert_eq!(f64::MAX, unsafe { v.f64_val });
    }

    #[test]
    fn test_serialize_mtm_matrix_r2_c3() {
        let (p, v) = Config::MtmMatrixR2C3(f64::MAX).parse();
        assert_eq!(0xA006, p);
        assert_eq!(f64::MAX, unsafe { v.f64_val });
    }

    #[test]
    fn test_serialize_mtm_matrix_r3_c1() {
        let (p, v) = Config::MtmMatrixR3C1(f64::MAX).parse();
        assert_eq!(0xA007, p);
        assert_eq!(f64::MAX, unsafe { v.f64_val });
    }

    #[test]
    fn test_serialize_mtm_matrix_r3_c2() {
        let (p, v) = Config::MtmMatrixR3C2(f64::MAX).parse();
        assert_eq!(0xA008, p);
        assert_eq!(f64::MAX, unsafe { v.f64_val });
    }

    #[test]
    fn test_serialize_mtm_matrix_r3_c3() {
        let (p, v) = Config::MtmMatrixR3C3(f64::MAX).parse();
        assert_eq!(0xA009, p);
        assert_eq!(f64::MAX, unsafe { v.f64_val });
    }

    #[test]
    fn test_serialize_mtm_bias_x() {
        let (p, v) = Config::MtmBiasX(f64::MAX).parse();
        assert_eq!(0xA00A, p);
        assert_eq!(f64::MAX, unsafe { v.f64_val });
    }

    #[test]
    fn test_serialize_mtm_bias_y() {
        let (p, v) = Config::MtmBiasY(f64::MAX).parse();
        assert_eq!(0xA00B, p);
        assert_eq!(f64::MAX, unsafe { v.f64_val });
    }

    #[test]
    fn test_serialize_mtm_bias_z() {
        let (p, v) = Config::MtmBiasZ(f64::MAX).parse();
        assert_eq!(0xA00C, p);
        assert_eq!(f64::MAX, unsafe { v.f64_val });
    }

    #[test]
    fn test_serialize_adc_coil_current_bias_x() {
        let (p, v) = Config::AdcCoilCurrentBiasX(i16::max_value()).parse();
        assert_eq!(0x301C, p);
        assert_eq!(i16::max_value(), unsafe { v.i16_val });
    }

    #[test]
    fn test_serialize_adc_coil_current_bias_y() {
        let (p, v) = Config::AdcCoilCurrentBiasY(i16::max_value()).parse();
        assert_eq!(0x301D, p);
        assert_eq!(i16::max_value(), unsafe { v.i16_val });
    }

    #[test]
    fn test_serialize_adc_coil_current_bias_z() {
        let (p, v) = Config::AdcCoilCurrentBiasZ(i16::max_value()).parse();
        assert_eq!(0x301E, p);
        assert_eq!(i16::max_value(), unsafe { v.i16_val });
    }

    #[test]
    fn test_serialize_adc_coil_current_mult_x() {
        let (p, v) = Config::AdcCoilCurrentMultX(i16::max_value()).parse();
        assert_eq!(0x301F, p);
        assert_eq!(i16::max_value(), unsafe { v.i16_val });
    }

    #[test]
    fn test_serialize_adc_coil_current_mult_y() {
        let (p, v) = Config::AdcCoilCurrentMultY(i16::max_value()).parse();
        assert_eq!(0x3020, p);
        assert_eq!(i16::max_value(), unsafe { v.i16_val });
    }

    #[test]
    fn test_serialize_adc_coil_current_mult_z() {
        let (p, v) = Config::AdcCoilCurrentMultZ(i16::max_value()).parse();
        assert_eq!(0x3021, p);
        assert_eq!(i16::max_value(), unsafe { v.i16_val });
    }

    #[test]
    fn test_serialize_adc_coil_current_div_x() {
        let (p, v) = Config::AdcCoilCurrentDivX(i16::max_value()).parse();
        assert_eq!(0x3022, p);
        assert_eq!(i16::max_value(), unsafe { v.i16_val });
    }

    #[test]
    fn test_serialize_adc_coil_current_div_y() {
        let (p, v) = Config::AdcCoilCurrentDivY(i16::max_value()).parse();
        assert_eq!(0x3023, p);
        assert_eq!(i16::max_value(), unsafe { v.i16_val });
    }

    #[test]
    fn test_serialize_adc_coil_current_div_z() {
        let (p, v) = Config::AdcCoilCurrentDivZ(i16::max_value()).parse();
        assert_eq!(0x3024, p);
        assert_eq!(i16::max_value(), unsafe { v.i16_val });
    }

    #[test]
    fn test_serialize_adc_coil_temp_bias_x() {
        let (p, v) = Config::AdcCoilTempBiasX(i16::max_value()).parse();
        assert_eq!(0x3025, p);
        assert_eq!(i16::max_value(), unsafe { v.i16_val });
    }

    #[test]
    fn test_serialize_adc_coil_temp_bias_y() {
        let (p, v) = Config::AdcCoilTempBiasY(i16::max_value()).parse();
        assert_eq!(0x3026, p);
        assert_eq!(i16::max_value(), unsafe { v.i16_val });
    }

    #[test]
    fn test_serialize_adc_coil_temp_bias_z() {
        let (p, v) = Config::AdcCoilTempBiasZ(i16::max_value()).parse();
        assert_eq!(0x3027, p);
        assert_eq!(i16::max_value(), unsafe { v.i16_val });
    }

    #[test]
    fn test_serialize_adc_coil_temp_mult_x() {
        let (p, v) = Config::AdcCoilTempMultX(i16::max_value()).parse();
        assert_eq!(0x3028, p);
        assert_eq!(i16::max_value(), unsafe { v.i16_val });
    }

    #[test]
    fn test_serialize_adc_coil_temp_mult_y() {
        let (p, v) = Config::AdcCoilTempMultY(i16::max_value()).parse();
        assert_eq!(0x3029, p);
        assert_eq!(i16::max_value(), unsafe { v.i16_val });
    }

    #[test]
    fn test_serialize_adc_coil_temp_mult_z() {
        let (p, v) = Config::AdcCoilTempMultZ(i16::max_value()).parse();
        assert_eq!(0x302A, p);
        assert_eq!(i16::max_value(), unsafe { v.i16_val });
    }

    #[test]
    fn test_serialize_adc_coil_temp_div_x() {
        let (p, v) = Config::AdcCoilTempDivX(i16::max_value()).parse();
        assert_eq!(0x302B, p);
        assert_eq!(i16::max_value(), unsafe { v.i16_val });
    }

    #[test]
    fn test_serialize_adc_coil_temp_div_y() {
        let (p, v) = Config::AdcCoilTempDivY(i16::max_value()).parse();
        assert_eq!(0x302C, p);
        assert_eq!(i16::max_value(), unsafe { v.i16_val });
    }

    #[test]
    fn test_serialize_adc_coil_temp_div_z() {
        let (p, v) = Config::AdcCoilTempDivZ(i16::max_value()).parse();
        assert_eq!(0x302D, p);
        assert_eq!(i16::max_value(), unsafe { v.i16_val });
    }

    #[test]
    fn test_serialize_detumble_frequency() {
        let (p, v) = Config::DetumbleFrequency(u8::max_value()).parse();
        assert_eq!(0x2000, p);
        assert_eq!(u8::max_value(), unsafe { v.u8_val });
    }

    #[test]
    fn test_serialize_bdot_gain() {
        let (p, v) = Config::BdotGain(f64::MAX).parse();
        assert_eq!(0xA000, p);
        assert_eq!(f64::MAX, unsafe { v.f64_val });
    }

    #[test]
    fn test_serialize_mtm_filter_sensitivity() {
        let (p, v) = Config::MtmFilterSensitivity(f64::MAX).parse();
        assert_eq!(0xA00D, p);
        assert_eq!(f64::MAX, unsafe { v.f64_val });
    }

    #[test]
    fn test_serialize_mtm_filter_weight() {
        let (p, v) = Config::MtmFilterWeight(f64::MAX).parse();
        assert_eq!(0xA00E, p);
        assert_eq!(f64::MAX, unsafe { v.f64_val });
    }

    #[test]
    fn test_serialize_coil_area_x() {
        let (p, v) = Config::CoilAreaX(f64::MAX).parse();
        assert_eq!(0xA00F, p);
        assert_eq!(f64::MAX, unsafe { v.f64_val });
    }

    #[test]
    fn test_serialize_coil_area_y() {
        let (p, v) = Config::CoilAreaY(f64::MAX).parse();
        assert_eq!(0xA010, p);
        assert_eq!(f64::MAX, unsafe { v.f64_val });
    }

    #[test]
    fn test_serialize_coil_area_z() {
        let (p, v) = Config::CoilAreaZ(f64::MAX).parse();
        assert_eq!(0xA011, p);
        assert_eq!(f64::MAX, unsafe { v.f64_val });
    }

    #[test]
    fn test_serialize_coil_current_limit() {
        let (p, v) = Config::CoilCurrentLimit(u16::max_value()).parse();
        assert_eq!(0x4000, p);
        assert_eq!(u16::max_value(), unsafe { v.u16_val });
    }

    #[test]
    fn test_serialize_current_feedback_enable() {
        let (p, v) = Config::CurrentFeedbackEnable(u8::max_value()).parse();
        assert_eq!(0x2001, p);
        assert_eq!(u8::max_value(), unsafe { v.u8_val });
    }

    #[test]
    fn test_serialize_current_feedback_gain_x() {
        let (p, v) = Config::CurrentFeedbackGainX(i32::max_value()).parse();
        assert_eq!(0x5000, p);
        assert_eq!(i32::max_value(), unsafe { v.i32_val });
    }

    #[test]
    fn test_serialize_current_feedback_gain_y() {
        let (p, v) = Config::CurrentFeedbackGainY(i32::max_value()).parse();
        assert_eq!(0x5001, p);
        assert_eq!(i32::max_value(), unsafe { v.i32_val });
    }

    #[test]
    fn test_serialize_current_feedback_gain_z() {
        let (p, v) = Config::CurrentFeedbackGainZ(i32::max_value()).parse();
        assert_eq!(0x5002, p);
        assert_eq!(i32::max_value(), unsafe { v.i32_val });
    }

    #[test]
    fn test_serialize_current_map_temp_t1() {
        let (p, v) = Config::CurrentMapTempT1(i16::max_value()).parse();
        assert_eq!(0x3000, p);
        assert_eq!(i16::max_value(), unsafe { v.i16_val });
    }

    #[test]
    fn test_serialize_current_map_temp_t2() {
        let (p, v) = Config::CurrentMapTempT2(i16::max_value()).parse();
        assert_eq!(0x3001, p);
        assert_eq!(i16::max_value(), unsafe { v.i16_val });
    }

    #[test]
    fn test_serialize_current_map_temp_t3() {
        let (p, v) = Config::CurrentMapTempT3(i16::max_value()).parse();
        assert_eq!(0x3002, p);
        assert_eq!(i16::max_value(), unsafe { v.i16_val });
    }

    #[test]
    fn test_serialize_current_map_temp_t4() {
        let (p, v) = Config::CurrentMapTempT4(i16::max_value()).parse();
        assert_eq!(0x3003, p);
        assert_eq!(i16::max_value(), unsafe { v.i16_val });
    }

    #[test]
    fn test_serialize_current_map_temp_t5() {
        let (p, v) = Config::CurrentMapTempT5(i16::max_value()).parse();
        assert_eq!(0x3004, p);
        assert_eq!(i16::max_value(), unsafe { v.i16_val });
    }

    #[test]
    fn test_serialize_current_map_temp_t6() {
        let (p, v) = Config::CurrentMapTempT6(i16::max_value()).parse();
        assert_eq!(0x3005, p);
        assert_eq!(i16::max_value(), unsafe { v.i16_val });
    }

    #[test]
    fn test_serialize_current_map_temp_t7() {
        let (p, v) = Config::CurrentMapTempT7(i16::max_value()).parse();
        assert_eq!(0x3006, p);
        assert_eq!(i16::max_value(), unsafe { v.i16_val });
    }

    #[test]
    fn test_serialize_current_max_x_t1() {
        let (p, v) = Config::CurrentMaxXT1(i16::max_value()).parse();
        assert_eq!(0x3007, p);
        assert_eq!(i16::max_value(), unsafe { v.i16_val });
    }

    #[test]
    fn test_serialize_current_max_x_t2() {
        let (p, v) = Config::CurrentMaxXT2(i16::max_value()).parse();
        assert_eq!(0x3008, p);
        assert_eq!(i16::max_value(), unsafe { v.i16_val });
    }

    #[test]
    fn test_serialize_current_max_x_t3() {
        let (p, v) = Config::CurrentMaxXT3(i16::max_value()).parse();
        assert_eq!(0x3009, p);
        assert_eq!(i16::max_value(), unsafe { v.i16_val });
    }

    #[test]
    fn test_serialize_current_max_x_t4() {
        let (p, v) = Config::CurrentMaxXT4(i16::max_value()).parse();
        assert_eq!(0x300A, p);
        assert_eq!(i16::max_value(), unsafe { v.i16_val });
    }

    #[test]
    fn test_serialize_current_max_x_t5() {
        let (p, v) = Config::CurrentMaxXT5(i16::max_value()).parse();
        assert_eq!(0x300B, p);
        assert_eq!(i16::max_value(), unsafe { v.i16_val });
    }

    #[test]
    fn test_serialize_current_max_x_t6() {
        let (p, v) = Config::CurrentMaxXT6(i16::max_value()).parse();
        assert_eq!(0x300C, p);
        assert_eq!(i16::max_value(), unsafe { v.i16_val });
    }

    #[test]
    fn test_serialize_current_max_x_t7() {
        let (p, v) = Config::CurrentMaxXT7(i16::max_value()).parse();
        assert_eq!(0x300D, p);
        assert_eq!(i16::max_value(), unsafe { v.i16_val });
    }

    #[test]
    fn test_serialize_current_max_y_t1() {
        let (p, v) = Config::CurrentMaxYT1(i16::max_value()).parse();
        assert_eq!(0x300E, p);
        assert_eq!(i16::max_value(), unsafe { v.i16_val });
    }

    #[test]
    fn test_serialize_current_max_y_t2() {
        let (p, v) = Config::CurrentMaxYT2(i16::max_value()).parse();
        assert_eq!(0x300F, p);
        assert_eq!(i16::max_value(), unsafe { v.i16_val });
    }

    #[test]
    fn test_serialize_current_max_y_t3() {
        let (p, v) = Config::CurrentMaxYT3(i16::max_value()).parse();
        assert_eq!(0x3010, p);
        assert_eq!(i16::max_value(), unsafe { v.i16_val });
    }

    #[test]
    fn test_serialize_current_max_y_t4() {
        let (p, v) = Config::CurrentMaxYT4(i16::max_value()).parse();
        assert_eq!(0x3011, p);
        assert_eq!(i16::max_value(), unsafe { v.i16_val });
    }

    #[test]
    fn test_serialize_current_max_y_t5() {
        let (p, v) = Config::CurrentMaxYT5(i16::max_value()).parse();
        assert_eq!(0x3012, p);
        assert_eq!(i16::max_value(), unsafe { v.i16_val });
    }

    #[test]
    fn test_serialize_current_max_y_t6() {
        let (p, v) = Config::CurrentMaxYT6(i16::max_value()).parse();
        assert_eq!(0x3013, p);
        assert_eq!(i16::max_value(), unsafe { v.i16_val });
    }

    #[test]
    fn test_serialize_current_max_y_t7() {
        let (p, v) = Config::CurrentMaxYT7(i16::max_value()).parse();
        assert_eq!(0x3014, p);
        assert_eq!(i16::max_value(), unsafe { v.i16_val });
    }

    #[test]
    fn test_serialize_current_max_z_t1() {
        let (p, v) = Config::CurrentMaxZT1(i16::max_value()).parse();
        assert_eq!(0x3015, p);
        assert_eq!(i16::max_value(), unsafe { v.i16_val });
    }

    #[test]
    fn test_serialize_current_max_z_t2() {
        let (p, v) = Config::CurrentMaxZT2(i16::max_value()).parse();
        assert_eq!(0x3016, p);
        assert_eq!(i16::max_value(), unsafe { v.i16_val });
    }

    #[test]
    fn test_serialize_current_max_z_t3() {
        let (p, v) = Config::CurrentMaxZT3(i16::max_value()).parse();
        assert_eq!(0x3017, p);
        assert_eq!(i16::max_value(), unsafe { v.i16_val });
    }

    #[test]
    fn test_serialize_current_max_z_t4() {
        let (p, v) = Config::CurrentMaxZT4(i16::max_value()).parse();
        assert_eq!(0x3018, p);
        assert_eq!(i16::max_value(), unsafe { v.i16_val });
    }

    #[test]
    fn test_serialize_current_max_z_t5() {
        let (p, v) = Config::CurrentMaxZT5(i16::max_value()).parse();
        assert_eq!(0x3019, p);
        assert_eq!(i16::max_value(), unsafe { v.i16_val });
    }

    #[test]
    fn test_serialize_current_max_z_t6() {
        let (p, v) = Config::CurrentMaxZT6(i16::max_value()).parse();
        assert_eq!(0x301A, p);
        assert_eq!(i16::max_value(), unsafe { v.i16_val });
    }

    #[test]
    fn test_serialize_current_max_z_t7() {
        let (p, v) = Config::CurrentMaxZT7(i16::max_value()).parse();
        assert_eq!(0x301B, p);
        assert_eq!(i16::max_value(), unsafe { v.i16_val });
    }
}
