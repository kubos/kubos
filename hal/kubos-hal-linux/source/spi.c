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
#if (defined YOTTA_CFG_HARDWARE_SPI) && (YOTTA_CFG_HARDWARE_SPI_COUNT > 0)

#include "kubos-hal/spi.h"
#include <stdio.h>

static uint8_t buffer[1024];
static uint32_t buffer_len;

KSPIStatus kprv_spi_dev_init(KSPINum spi)
{
    buffer_len = 0;
    *buffer = 0;
    return SPI_OK;
}

KSPIStatus kprv_spi_dev_terminate(KSPINum spi)
{
    buffer_len = 0;
    *buffer = 0;
    return SPI_OK;
}

/**
 * Low level hal spi write
 *
 * This is implemented by the hardware specific hal
 *
 * @param spi spi bus to write to
 * @param buffer pointer to data buffer
 * @param len length of data to write
 * @return KSPIStatus SPI_OK on success, otherwise failure
 */
KSPIStatus kprv_spi_write(KSPINum spi, uint8_t * ptr, uint32_t len)
{
    uint32_t i = 0;
    for (i = 0; i < len; i++)
    {
        buffer[i] = *ptr++;
    }
    buffer_len = len;
    return SPI_OK;
}

/**
 * Low level hal spi read
 *
 * This is implemented by the hardware specific hal
 *
 * @param spi spi bus to read from
 * @param buffer pointer to data buffer
 * @param len length of data to read
 * @return KSPIStatus SPI_OK on success, otherwise failure
 */
KSPIStatus kprv_spi_read(KSPINum spi, uint8_t * ptr, uint32_t len)
{
    if (buffer_len != 0)
    {
        uint32_t i = 0;
        for (i = 0; i < len; i++)
        {
            *ptr++ = buffer[i];
        }
        buffer_len = 0;
        return SPI_OK;
    }
    return SPI_ERROR;
}

/**
 * Low level spi write & read
 *
 * This is implemented by the hardware specific hal
 *
 * @param spi spi bus to write to
 * @param txBuffer pointer to data buffer to write from
 * @param rxBuffer pointer to data buffer to read into
 * @param len length of data to write and read
 * @return KSPIStatus SPI_OK on success, otherwise failure
 */
KSPIStatus kprv_spi_write_read(KSPINum spi, uint8_t * txBuffer, uint8_t * rxBuffer, uint32_t len)
{
    memcpy(rxBuffer, txBuffer, len);
    return SPI_OK;
}

#endif
