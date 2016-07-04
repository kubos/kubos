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

#include "kubos-hal-stm32f4/spi.h"
#include "stm32cubef4/stm32f4xx_hal_spi.h"

static k_spi_handle k_spi_dev[K_NUM_SPI];
static uint32_t spi_timeout = 256;

k_spi_handle * k_prv_spi_get_handle(KSPINum spi)
{
    return &k_spi_dev[spi];
}

KSPIStatus k_spi_write(KSPINum spi, uint8_t * buffer, uint32_t len)
{
    k_spi_handle * handle = k_prv_spi_get_handle(spi);
    HAL_StatusTypeDef status = HAL_SPI_Transmit(handle->hal_handle, buffer, len, spi_timeout);

}
