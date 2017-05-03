/*
 * KubOS HAL
 * Copyright (C) 2016 Kubos Corporation
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
 * @defgroup SDIO HAL SDIO Interface
 * @addtogroup SDIO
 * @{
 */
/**
 * @brief KubOS-HAL SDIO Interface
 * @author kubos.co
 */

#ifdef YOTTA_CFG_HARDWARE_SDIO
#ifndef K_SDIO_H
#define K_SDIO_H

#include <stdint.h>

typedef enum
{
    SDIO_OK,
    SDIO_ERROR,
    SDIO_WRITE_ERROR,
    SDIO_READ_ERROR,
    SDIO_INIT_ERROR
} KSDIOStatus;

/**
 * Structure for SDIO card info
 */
typedef struct
{
    /**
     * Capacity of current card
     */
    uint32_t capacity;
} k_sdio_card_info_t;


// Public HAL interface

/**
 * Performs low-level SDIO hardware initialization
 * @return KSDIOStatus SDIO_OK if successful, otherwise error
 */
KSDIOStatus k_sdio_init();

/**
 * Performs low-level SDIO hardware termination
 */
void k_sdio_terminate();

/**
 * Writes blocks of data via SDIO
 * @param buffer [in] data buffer to write data from
 * @param addr [in] address on sdcard to write to
 * @param block_size [in] size of each block to write
 * @param count [in] number of blocks to write
 * @return KSDIOStatus SDIO_OK if successful, otherwise error code
 */
KSDIOStatus k_sdio_write_blocks(uint32_t * buffer, uint64_t addr, uint32_t block_size, uint32_t count);

/**
 * Reads blocks of data via SDIO
 * @param buffer [out] data buffer to read data into
 * @param addr [in] address on sdcard to read from
 * @param block_size [in] size of each block to read
 * @param count [in] number of blocks to read
 * @return KSDIOStatus SDIO_OK if successful, otherwise error code
 */
KSDIOStatus k_sdio_read_blocks(uint32_t * buffer, uint64_t addr, uint32_t block_size, uint32_t count);

/**
 * Reads status of SDIO device
 * @return KSDIOStatus indication of current SDIO device status
 */
KSDIOStatus k_sdio_card_status();

/**
 * Queries SDIO device for card info
 * @return k_sdio_card_info_t structure of SDIO card info
 */
k_sdio_card_info_t k_sdio_card_info();

// Private hardware specific interfaces

/**
 * Performs low-level SDIO hardware initialization - *Private interface for platform specific implementation*
 * @return KSDIOStatus SDIO_OK if successful, otherwise error
 */
KSDIOStatus kprv_sdio_init();

/**
 * Performs low-level SDIO hardware termination - *Private interface for platform specific implementation*
 */
void kprv_sdio_terminate();

/**
 * Writes blocks of data via SDIO - *Private interface for platform specific implementation*
 * @param buffer [in] data buffer to write data from
 * @param addr [in] address on sdcard to write to
 * @param block_size [in] size of each block to write
 * @param count [in] number of blocks to write
 * @return KSDIOStatus SDIO_OK if successful, otherwise error code
 */
KSDIOStatus kprv_sdio_write_blocks(uint32_t * buffer, uint64_t addr, uint32_t block_size, uint32_t count);

/**
 * Reads blocks of data via SDIO - *Private interface for platform specific implementation*
 * @param buffer [out] data buffer to read data into
 * @param addr [in] address on sdcard to read from
 * @param block_size [in] size of each block to write
 * @param count [in] number of blocks to write
 * @return KSDIOStatus SDIO_OK if successful, otherwise error code
 */
KSDIOStatus kprv_sdio_read_blocks(uint32_t * buffer, uint64_t addr, uint32_t block_size, uint32_t count);

/**
 * Reads status of SDIO device - *Private interface for platform specific implementation*
 * @return KSDIOStatus indication of current SDIO device status
 */
KSDIOStatus kprv_sdio_card_status();

/**
 * Queries SDIO device for card info - *Private interface for platform specific implementation*
 * @return k_sdio_card_info_t structure of SDIO card info
 */
k_sdio_card_info_t kprv_sdio_card_info();

#endif
#endif

/* @} */