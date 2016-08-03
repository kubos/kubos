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
 
#ifdef YOTTA_CFG_HARDWARE_SDIO

#include "kubos-hal/sdio.h"

KSDIOStatus k_sdio_init()
{
    return kprv_sdio_init();
}

void k_sdio_terminate()
{
    return kprv_sdio_terminate();
}

KSDIOStatus k_sdio_write_blocks(uint32_t * buffer, uint64_t addr, uint32_t block_size, uint32_t count)
{
    return kprv_sdio_write_blocks(buffer, addr, block_size, count);
}

KSDIOStatus k_sdio_read_blocks(uint32_t * buffer, uint64_t addr, uint32_t block_size, uint32_t count)
{
    return kprv_sdio_read_blocks(buffer, addr, block_size, count);
}

KSDIOStatus k_sdio_card_status()
{
    return kprv_sdio_card_status();
}

k_sdio_card_info_t k_sdio_card_info()
{
    return kprv_sdio_card_info();
}

#endif
