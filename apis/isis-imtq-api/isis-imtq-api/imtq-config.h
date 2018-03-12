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
/**
 * @addtogroup IMTQ_API
 * @{
 */

#pragma once

#include <json/json.h>
#include "imtq-core.h"

/** \cond WE DO NOT WANT TO HAVE THESE IN OUR GENERATED DOCS */
/* Configuration Commands */
#define GET_PARAM               0x81
#define SET_PARAM               0x82
#define RESET_PARAM             0x83
/** \endcond */

/**
 *  @name Magnetometer Configuration Parameter Codes
 */
/**@{*/
#define MTM_SELECT              0x2002 /**< Select MTM to use for measurement. 0 - Internal, 1 - External */

#define MTM_INTERNAL_TIME       0x2003 /**< Integration time selection for idle mode MTM measurements. <em>Refer to Table 3-10 of the iMTQ User Manual for more information</em> */
#define MTM_EXTERNAL_TIME       0x2004 /**< Integration time selection for idle mode MTM measurements. <em>Refer to Table 3-10 of the iMTQ User Manual for more information</em> */

#define MTM_INTERNAL_MAP_X      0x2005 /**< iMTQ axis to which the MTM x-axis is mapped */
#define MTM_INTERNAL_MAP_Y      0x2006 /**< iMTQ axis to which the MTM y-axis is mapped */
#define MTM_INTERNAL_MAP_Z      0x2007 /**< iMTQ axis to which the MTM z-axis is mapped */

#define MTM_EXTERNAL_MAP_X      0x2008 /**< iMTQ axis to which the MTM x-axis is mapped */
#define MTM_EXTERNAL_MAP_Y      0x2009 /**< iMTQ axis to which the MTM y-axis is mapped */
#define MTM_EXTERNAL_MAP_Z      0x200A /**< iMTQ axis to which the MTM z-axis is mapped */

#define MTM_MATRIX_R1_C1        0xA001 /**< MTM raw -> corrected correction matrix (Row 1, Column 1) */
#define MTM_MATRIX_R1_C2        0xA002 /**< MTM raw -> corrected correction matrix (Row 1, Column 2) */
#define MTM_MATRIX_R1_C3        0xA003 /**< MTM raw -> corrected correction matrix (Row 1, Column 3) */
#define MTM_MATRIX_R2_C1        0xA004 /**< MTM raw -> corrected correction matrix (Row 2, Column 1) */
#define MTM_MATRIX_R2_C2        0xA005 /**< MTM raw -> corrected correction matrix (Row 2, Column 2) */
#define MTM_MATRIX_R2_C3        0xA006 /**< MTM raw -> corrected correction matrix (Row 2, Column 3) */
#define MTM_MATRIX_R3_C1        0xA007 /**< MTM raw -> corrected correction matrix (Row 3, Column 1) */
#define MTM_MATRIX_R3_C2        0xA008 /**< MTM raw -> corrected correction matrix (Row 3, Column 2) */
#define MTM_MATRIX_R3_C3        0xA009 /**< MTM raw -> corrected correction matrix (Row 3, Column 3) */

#define MTM_BIAS_X              0xA00A /**< MTM raw -> corrected correction bias vector (X-axis value) */
#define MTM_BIAS_Y              0xA00B /**< MTM raw -> corrected correction bias vector (Y-axis value) */
#define MTM_BIAS_Z              0xA00C /**< MTM raw -> corrected correction bias vector (Z-axis value) */
/**@}*/

/**
 *  @name Current/Temperature Measurement Configuration Parameter Codes
 */
/**@{*/
#define ADC_COIL_CURRENT_BIAS_X 0x301C /**< X-axis voltage bias for coil current ADC -> engineering value conversion */
#define ADC_COIL_CURRENT_BIAS_Y 0x301D /**< Y-axis voltage bias for coil current ADC -> engineering value conversion */
#define ADC_COIL_CURRENT_BIAS_Z 0x301E /**< Z-axis voltage bias for coil current ADC -> engineering value conversion */

#define ADC_COIL_CURRENT_MULT_X 0x301F /**< X-axis pre-multiplier for coil current ADC -> engineering value conversion */
#define ADC_COIL_CURRENT_MULT_Y 0x3020 /**< Y-axis pre-multiplier for coil current ADC -> engineering value conversion */
#define ADC_COIL_CURRENT_MULT_Z 0x3021 /**< Z-axis pre-multiplier for coil current ADC -> engineering value conversion */

#define ADC_COIL_CURRENT_DIV_X  0x3022 /**< X-axis post-divider for coil current ADC -> engineering value conversion */
#define ADC_COIL_CURRENT_DIV_Y  0x3023 /**< Y-axis post-divider for coil current ADC -> engineering value conversion */
#define ADC_COIL_CURRENT_DIV_Z  0x3024 /**< Z-axis post-divider for coil current ADC -> engineering value conversion */

#define ADC_COIL_TEMP_BIAS_X    0x3025 /**< X-axis voltage bias for coil temperature ADC -> engineering value conversion */
#define ADC_COIL_TEMP_BIAS_Y    0x3026 /**< Y-axis voltage bias for coil temperature ADC -> engineering value conversion */
#define ADC_COIL_TEMP_BIAS_Z    0x3027 /**< Z-axis voltage bias for coil temperature ADC -> engineering value conversion */

#define ADC_COIL_TEMP_MULT_X    0x3028 /**< X-axis pre-multiplier for coil temperature ADC -> engineering value conversion */
#define ADC_COIL_TEMP_MULT_Y    0x3029 /**< Y-axis pre-multiplier for coil temperature ADC -> engineering value conversion */
#define ADC_COIL_TEMP_MULT_Z    0x302A /**< Z-axis pre-multiplier for coil temperature ADC -> engineering value conversion */

#define ADC_COIL_TEMP_DIV_X     0x302B /**< X-axis post-divider for coil temperature ADC -> engineering value conversion */
#define ADC_COIL_TEMP_DIV_Y     0x302C /**< Y-axis post-divider for coil temperature ADC -> engineering value conversion */
#define ADC_COIL_TEMP_DIV_Z     0x302D /**< Z-axis post-divider for coil temperature ADC -> engineering value conversion */
/**@}*/

/**
 *  @name Detumble Configuration Parameter Codes
 */
/**@{*/
#define DETUMBLE_FREQUENCY      0x2000 /**< Control frequency of the detumble mode control loop. <em>Values: 1, 2, 4, or 8 Hz</em> */
#define BDOT_GAIN               0xA000 /**< B-Dot algorithm gain when converting from B-Dot to dipole. Value should be negative */
#define MTM_FILTER_SENSITIVITY  0xA00D /**< Adaptive sensitivity of low-pass filter applied to calibrated MTM measurements during detumble mode */
#define MTM_FILTER_WEIGHT       0xA00E /**< Adaptive weight of low-pass filter applied to calibrated MTM measurements during detumble mode */
/**@}*/

/**
 *  @name Dipole to Current Conversion Configuration Parameter Codes
 */
/**@{*/
#define COIL_AREA_X             0xA00F /**< X-axis area of the coil used to calculate the dipole from the current flowing through the coil */
#define COIL_AREA_Y             0xA010 /**< Y-axis area of the coil used to calculate the dipole from the current flowing through the coil */
#define COIL_AREA_Z             0xA011 /**< Z-axis area of the coil used to calculate the dipole from the current flowing through the coil */

#define COIL_CURRENT_LIMIT      0x4000 /**< Maximum total coil current allowed for dipole generation (excluding idle current consumption) */
/**@}*/

/**
 *  @name Current to Actuation-percentage Conversion Configuration Parameter Codes
 */
/**@{*/
#define CURRENT_FEEDBACK_ENABLE 0x2001 /**< Current feedback control. 0 - Open-loop temperature-compensated, 1 - Software-based closed-loop */

#define CURRENT_FEEDBACK_GAIN_X 0x5000 /**< X-axis feedback gain of the proportional difference controller */
#define CURRENT_FEEDBACK_GAIN_Y 0x5001 /**< Y-axis feedback gain of the proportional difference controller */
#define CURRENT_FEEDBACK_GAIN_Z 0x5002 /**< Z-axis feedback gain of the proportional difference controller */

#define CURRENT_MAP_TEMP_T1     0x3000 /**< Current-map profile temperature 1 (lowest) */
#define CURRENT_MAP_TEMP_T2     0x3001 /**< Current-map profile temperature 2 */
#define CURRENT_MAP_TEMP_T3     0x3002 /**< Current-map profile temperature 3 */
#define CURRENT_MAP_TEMP_T4     0x3003 /**< Current-map profile temperature 4 */
#define CURRENT_MAP_TEMP_T5     0x3004 /**< Current-map profile temperature 5 */
#define CURRENT_MAP_TEMP_T6     0x3005 /**< Current-map profile temperature 6 */
#define CURRENT_MAP_TEMP_T7     0x3006 /**< Current-map profile temperature 7 (highest) */

#define CURRENT_MAX_X_T1        0x3007 /**< X-axis maximum current at temperature 1 */
#define CURRENT_MAX_X_T2        0x3008 /**< X-axis maximum current at temperature 2 */
#define CURRENT_MAX_X_T3        0x3009 /**< X-axis maximum current at temperature 3 */
#define CURRENT_MAX_X_T4        0x300A /**< X-axis maximum current at temperature 4 */
#define CURRENT_MAX_X_T5        0x300B /**< X-axis maximum current at temperature 5 */
#define CURRENT_MAX_X_T6        0x300C /**< X-axis maximum current at temperature 6 */
#define CURRENT_MAX_X_T7        0x300D /**< X-axis maximum current at temperature 7 */

#define CURRENT_MAX_Y_T1        0x300E /**< Y-axis maximum current at temperature 1 */
#define CURRENT_MAX_Y_T2        0x300F /**< Y-axis maximum current at temperature 2 */
#define CURRENT_MAX_Y_T3        0x3010 /**< Y-axis maximum current at temperature 3 */
#define CURRENT_MAX_Y_T4        0x3011 /**< Y-axis maximum current at temperature 4 */
#define CURRENT_MAX_Y_T5        0x3012 /**< Y-axis maximum current at temperature 5 */
#define CURRENT_MAX_Y_T6        0x3013 /**< Y-axis maximum current at temperature 6 */
#define CURRENT_MAX_Y_T7        0x3014 /**< Y-axis maximum current at temperature 7 */

#define CURRENT_MAX_Z_T1        0x3015 /**< Z-axis maximum current at temperature 1 */
#define CURRENT_MAX_Z_T2        0x3016 /**< Z-axis maximum current at temperature 2 */
#define CURRENT_MAX_Z_T3        0x3017 /**< Z-axis maximum current at temperature 3 */
#define CURRENT_MAX_Z_T4        0x3018 /**< Z-axis maximum current at temperature 4 */
#define CURRENT_MAX_Z_T5        0x3019 /**< Z-axis maximum current at temperature 5 */
#define CURRENT_MAX_Z_T6        0x301A /**< Z-axis maximum current at temperature 6 */
#define CURRENT_MAX_Z_T7        0x301B /**< Z-axis maximum current at temperature 7 */
/**@}*/

/**
 *  @name Read-Only Configuration Parameter Codes
 */
/**@{*/
#define HW_CONFIG               0x2800 /**< iMTQ hardware configuration. 0 - Internal config, 1 - External config */
#define WATCHDOG_TIMEOUT        0x2801 /**< I<sup>2</sup>C watchdog timeout interval in seconds. <em>Value of 0 indicates the watchdog is disabled</em> */
#define SLAVE_ADDRESS           0x4800 /**< iMTQ's I<sup>2</sup>C address*/
#define SOFTWARE_VERSION        0x6800 /**< iMTQ's software version. <em>v[second byte].[third byte].[fourth byte]; first byte is ignored</em> */
/**@}*/

/**
 * Configuration value storage union
 * Using a union allows one common variable type to be used for all configuration functions
 */
typedef union {
    int8_t int8_val;                /**< Storage for signed single-byte values */
    uint8_t uint8_val;              /**< Storage for unsigned single-byte values */
    int16_t int16_val;              /**< Storage for signed byte-pair values */
    uint16_t uint16_val;            /**< Storage for unsigned byte-pair values */
    int32_t int32_val;              /**< Storage for signed four-byte values */
    uint32_t uint32_val;            /**< Storage for unsigned four-byte values */
    float float_val;                /**< Storage for IEEE754 single-precision floating point values (four bytes) */
    int64_t int64_val;              /**< Storage for signed eight-byte values */
    uint64_t uint64_val;            /**< Storage for unsigned eight-byte values */
    double double_val;              /**< Storage for IEEE754 double-precision floating point values (eight bytes) */
} imtq_config_value;

/**
 * Message structure returned by all get/set/reset configuration requests
 */
typedef struct {
    imtq_resp_header hdr;           /**< Response message header */
    uint16_t param;                 /**< Echo of requested parameter ID */
    imtq_config_value value;        /**< Current value of requested parameter */
} __attribute__((packed)) imtq_config_resp;

/* Configuration Commands */
/**
 * Configure the ADCS
 * @param [in] config ADCS configuration structure
 * @return KADCSStatus ADCS_OK if OK, error otherwise
 */
KADCSStatus k_adcs_configure(const JsonNode * config);
/**
 * Get the current value of a configuration parameter
 * @param [in] param ID of parameter value to fetch
 * @param [out] response Pointer to storage for response message. Returns the
 * current value of the parameter.
 * @return KADCSStatus `ADCS_OK` if OK, error otherwise
 */
KADCSStatus k_imtq_get_param(uint16_t param, imtq_config_resp * response);
/**
 * Set the value of a configuration parameter
 * @param [in] param ID of parameter to update
 * @param [in] value Pointer to new parameter value
 * @param [out] response Pointer to storage for response message. Returns the
 * updated value of the parameter. If successful, the new value should match
 * the requested value.
 * @return KADCSStatus `ADCS_OK` if OK, error otherwise
 */
KADCSStatus k_imtq_set_param(uint16_t param, const imtq_config_value * value,
                             imtq_config_resp * response);
/**
 * Reset the value of a configuration parameter to its default
 * @param [in] param ID of parameter value to reset
 * @param [out] response Pointer to storage for response message. Returns the
 * new value of the parameter.
 * @return KADCSStatus `ADCS_OK` if OK, error otherwise
 */
KADCSStatus k_imtq_reset_param(uint16_t param, imtq_config_resp * response);

/* @} */
