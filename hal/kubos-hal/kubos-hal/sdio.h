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
 * @defgroup SDIO
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

typedef struct
{
    uint32_t capacity;
} k_sdio_card_info_t;


// Public HAL interface

KSDIOStatus k_sdio_init();

void k_sdio_terminate();

KSDIOStatus k_sdio_write_blocks(uint32_t * buffer, uint64_t addr, uint32_t block_size, uint32_t count);

KSDIOStatus k_sdio_read_blocks(uint32_t * buffer, uint64_t addr, uint32_t block_size, uint32_t count);

KSDIOStatus k_sdio_card_status();

k_sdio_card_info_t k_sdio_card_info();

// Private hardware specific interfaces

KSDIOStatus kprv_sdio_init();

void kprv_sdio_terminate();

KSDIOStatus kprv_sdio_write_blocks(uint32_t * buffer, uint64_t addr, uint32_t block_size, uint32_t count);

KSDIOStatus kprv_sdio_read_blocks(uint32_t * buffer, uint64_t addr, uint32_t block_size, uint32_t count);

KSDIOStatus kprv_sdio_card_status();

k_sdio_card_info_t kprv_sdio_card_info();

#endif
#endif
