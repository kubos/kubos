/*
 * Copyright (C) 2017 Kubos Corporation
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

#include <pthread.h>

/**
 *  @name Command Response Flags
 */
/**@{*/
#define RESP_NEW 0x80           /**< First time retrieving this response */
#define RESP_IVA_X 0x40         /**< X-axis measurement might be invalid */
#define RESP_IVA_Y 0x20         /**< Y-axis measurement might be invalid */
#define RESP_IVA_Z 0x10         /**< Z-axis measurement might be invalid */
/**@}*/

/**
 * iMTQ Return Values
 * Error codes which may be returned in the ::imtq_resp_header.status byte of
 * a response message
 */
typedef enum {
    IMTQ_OK,
    IMTQ_ERROR           = 0x01, /**< Generic error */
    IMTQ_ERROR_BAD_CMD   = 0x02, /**< Invalid command */
    IMTQ_ERROR_NO_PARAM  = 0x03, /**< Parameter missing */
    IMTQ_ERROR_BAD_PARAM = 0x04, /**< Parameter invalid */
    IMTQ_ERROR_MODE      = 0x05, /**< Command unavailable in current mode */
    IMTQ_ERROR_RESERVED  = 0x06, /**< (Internal reserved value) */
    IMTQ_ERROR_INTERNAL  = 0x07  /**< Internal error */
} KIMTQStatus;

/**
 * Response header structure
 */
typedef struct {
    uint8_t cmd;                /**< Command which produced this response */
    /**
     * Status byte
     *
     * Contains command response flags, like ::RESP_IVA_X, and a return code
     * which can be extracted with ::kprv_imtq_check_error
     */
    uint8_t status;
} __attribute__((packed)) imtq_resp_header;

/**
 * Generic structure for data relating to the axes
 */
typedef struct {
    int16_t x;                  /**< X-axis */
    int16_t y;                  /**< Y-axis */
    int16_t z;                  /**< Z-axis */
} __attribute__((packed)) imtq_axis_data;

/**
 * System mutex to preserve iMTQ command/response ordering
 */
extern pthread_mutex_t imtq_mutex;

/* Public Functions */
/**
 * Start a thread to kick the iMTQ's watchdog at an interval of
 * (::IMTQ_WD_TIMEOUT/3) seconds
 * @return KADCSStatus `ADCS_OK` if OK, error otherwise
 */
KADCSStatus k_imtq_watchdog_start(void);
/**
 * Stop the watchdog thread
 * @return KADCSStatus `ADCS_OK` if OK, error otherwise
 */
KADCSStatus k_imtq_watchdog_stop(void);
/**
 * Reboot the iMTQ
 * @note All configuration options will be reset to their default values
 * @return KADCSStatus `ADCS_OK` if OK, error otherwise
 */
KADCSStatus k_imtq_reset(void);

/* Private Functions */
/**
 * Send an iMTQ request and fetch the response
 * @param [in] tx Pointer to data to send
 * @param [in] tx_len Length of data to send
 * @param [out] rx Pointer to buffer for response data
 * @param [in] rx_len Length of data to read for response
 * @param [in] delay Delay between sending data to the iMTQ and reading the response. A value of `NULL` indicates that the default should be used.
 * @return KADCSStatus `ADCS_OK` if OK, error otherwise
 */
KADCSStatus kprv_imtq_transfer(const uint8_t * tx, int tx_len, uint8_t * rx,
                               int rx_len, const struct timespec * delay);
/**
 * Extract the return code in a response status byte
 * @param [in] status A ::imtq_resp_header.status byte returned in a response
 * message structure
 * @return Converted ::KIMTQStatus value
 */
static inline KIMTQStatus kprv_imtq_check_error(uint8_t status) { return (KIMTQStatus) status & 0x0F; }

/* @} */
