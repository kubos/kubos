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
 * @addtogroup MAI400
 * @{
 */

#pragma once

#include <stdint.h>

#define SYNC 0xEB90 /* Sync byte for all message headers */
#define HDR_SZ 6
#define FRAME_SZ HDR_SZ+2

/* Command Message IDs */
#define SET_ACS_MODE        0x00 //Yes
#define LAT_LONG_DATA       0x51
#define Q_TABLE_LOAD        0x60
#define RWS_TORQUE          0x01
#define RWS_SPEED           0x02
#define RWS_RAW_TORQUE      0x5D
#define SYNC_WHEEL_TACH     0x5E
#define RW_ZERO_START       0x5F
#define IREHS_MODE          0x10
#define IREHS_PARAM1        0x11
#define IREHS_PARAM2        0x18
#define FILTER_PARAM        0x19
#define SET_DIPOLE          0x04
#define SET_QBO             0x06
#define SET_EXTERNAL        0x07
#define SET_EXT_RAW_EHS     0x5C
#define SET_KEPLER          0x08
#define SET_ACQ_GAIN        0x09
#define SET_NORMAL_GAIN     0x0A
#define SET_UNLOAD_GAIN     0x0B
#define SET_QBX             0x0C
#define SET_MAG_BIAS        0x0D
#define SET_CSS_BIAS        0x0E
#define SET_MAG_GAIN        0x0F
#define SET_CSS_GAIN        0x40
#define SET_RV1             0x41 //TODO: If we end up using this, need to get the message IDs checked,
#define SET_RV2             0x41 //since they're the same...
#define SET_PROP_MODE       0x42
#define SET_ATT_DET_MODE    0x43
#define SET_GPS_TIME        0x44 //Yes
#define SET_QBI             0x45
#define SET_DIPOLE_GAIN     0x46
#define SET_NORMAL_PARAMS   0x47
#define REQUEST_RESET       0x5A //Yes
#define RESET_CONFIRM       0xF1 //Yes
#define REQUEST_ISP         0x58 //Almost certainly don't need/want this one
#define ISP_CONFIRM         0xF0 //Or this one
#define GET_INFO            0x1D //Yes

/* Telemetry Message IDs */
#define CONFIG_INFO         0x06 //query config. Returned after GET_INFO is sent
#define STANDARD_TELEM      0x01 //nominal telem
#define RAW_IMU_TELEM       0x03 //debug telem
#define IREHS_TELEM         0x02 //debug telem

/**
 * ADCS function return values
 */
typedef enum {
    ADCS_OK,
    ADCS_ERROR,                  /**< Generic error */
    ADCS_ERROR_CONFIG,           /**< Configuration error */
    ADCS_ERROR_NO_RESPONSE,      /**< No response received from subsystem */
    ADCS_ERROR_INTERNAL,         /**< An error was thrown by the subsystem */
    ADCS_ERROR_CRC               /**< Error with CRC calculcation/verification */
} KADCSStatus;

typedef enum {
    TEST,
    ACQUISITION,
    RESERVED,
    NORMAL,
    LAT_LONG,
    QBX,
    RESERVED,
    NORMAL_SUN,
    LAT_LONG_SUN,
    QINERTIAL,
    NOOP,
    QTABLE,
    SUN_RAM,
    OBJECT_TRACK
} ACSMode;

/**
 * Message header structure
 */
typedef struct {
    uint16_t sync;
    uint16_t data_len;
    uint8_t  msg_id;
    uint8_t  addr;
} __attribute__((packed)) adcs_msg_header;

typedef struct {
    adcs_msg_header hdr;
    uint8_t mode;
    int32_t sec_vec;
    int32_t pri_axis;
    int32_t sec_axis;
    int32_t qbi_cmd4;
    uint16_t crc;
} __attribute__((packed)) adcs_set_mode_msg;

typedef struct {
    adcs_msg_header hdr;
    uint16_t crc;
} __attribute__((packed)) adcs_get_info_msg;

/**
 * Initialize the ADCS interface
 * @return KADCSStatus ADCS_OK if OK, error otherwise
 */
KADCSStatus k_adcs_init(void);
/**
 * Terminate the ADCS interface
 */
void k_adcs_terminate(void);
/**
 * Execute ADCS no-op command
 * @note This function might not be implemented for all ADCSs.
 * @return KADCSStatus ADCS_OK if OK, error otherwise
 */
KADCSStatus k_adcs_noop(void);
/**
 * Configure the ADCS
 * @param [in] config ADCS configuration structure
 * @return KADCSStatus ADCS_OK if OK, error otherwise
 */
KADCSStatus k_adcs_configure(const JsonNode * config);
/**
 * Reset the ADCS
 * @note This function might not be implemented for all ADCSs
 * @param [in] type Type of reset to perform (hard, soft, etc)
 * @return KADCSStatus ADCS_OK if OK, error otherwise
 */
KADCSStatus k_adcs_reset(KADCSReset type);
/**
 * Set the ADCS's operating mode
 * @note See specific ADCS API documentation for available modes
 * @param [in] mode Operating mode to change to
 * @param [in] params Pointer to optional parameters which may be needed to configure the new operational mode
 * @return KADCSStatus ADCS_OK if OK, error otherwise
 */
KADCSStatus k_adcs_set_mode(ADCSMode mode, const adcs_mode_param * params);
/**
 * Run an ADCS self-test
 * @note This function might not be implemented for all ADCSs.
 * See specific ADCS API documentation for available self-tests.
 * @param [in] test Type of self-test to run
 * @param [out] buffer (Pointer to) structure which the test-results should be copied to
 * @return KADCSStatus ADCS_OK if OK, error otherwise
 */
KADCSStatus k_adcs_run_test(ADCSTestType test, adcs_test_results buffer);
/**
 * Get the ADCS's power status
 * @param [out] data Pointer to storage for returned system uptime. If the value is zero, then the ADCS is offline.
 * @return KADCSStatus ADCS_OK if OK, error otherwise
 */
KADCSStatus k_adcs_get_power_status(adcs_power_status * data);
/**
 * Get the ADCS's current operating mode
 * @param [out] mode Pointer to storage which the mode value should be copied to
 * @return KADCSStatus ADCS_OK if OK, error otherwise
 */
KADCSStatus k_adcs_get_mode(ADCSMode * mode);
/**
 * Get the ADCS's current orientation
 * @note This function might not be implemented for all ADCSs.
 * @param [out] data Pointer to storage for returned data.
 * @return KADCSStatus ADCS_OK if OK, error otherwise
 */
KADCSStatus k_adcs_get_orientation(adcs_orient * data);
/**
 * Get the ADCS's current spin
 * @note This function might not be implemented for all ADCSs.
 * @param [out] data Pointer to storage for returned data.
 * @return KADCSStatus ADCS_OK if OK, error otherwise
 */
KADCSStatus k_adcs_get_spin(adcs_spin * data);
/**
 * Read ADCS telemetry values
 * @note See specific ADCS API documentation for available telemetry types
 * @param [in] type Telemetry packet to read
 * @param [out] buffer (Pointer to) structure which data should be copied to
 * @return KADCSStatus ADCS_OK if OK, error otherwise
 */
KADCSStatus k_adcs_get_telemetry(ADCSTelemType type, JsonNode * buffer);
/**
 * Pass a command packet directly through to the ADCS.
 * Useful for executing commands which have not been implemented in either the generic or specific ADCS APIs.
 * @param [in] tx Pointer to command packet to send
 * @param [in] tx_len Size of command packet
 * @param [out] rx Pointer to storage for command response
 * @param [in] rx_len Expected length of command response
 * @param [in] delay Time to wait inbetween sending the command packet and requesting a response
 * @return KADCSStatus ADCS_OK if OK, error otherwise
 */
KADCSStatus k_adcs_passthrough(const uint8_t * tx, int tx_len, uint8_t * rx, int rx_len, const struct timespec * delay);

/* @} */
