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
  * @defgroup SPI
  * @addtogroup SPI
  * @{
  */

/**
  *
  * @file       spi.c
  * @brief      Kubos-HAL-MSP430F5529 - SPI module
  *
  * @author     kubos.co
  */

#if (defined YOTTA_CFG_HARDWARE_SPI) && (YOTTA_CFG_HARDWARE_SPI_COUNT > 0)
#include "kubos-hal/spi.h"
#include "msp430f5529-hal/spi.h"
#include <msp430.h>

/**
  * @brief Helper function to convert spi bus option.
  */
static inline hal_spi_bus spi_bus(KSPINum spi)
{
    switch(spi)
    {
        case K_SPI1: return HAL_SPI_B0;
        case K_SPI2: return HAL_SPI_B1;
        default: return 0;
    }
}

/**
  * @brief Helper function to convert spi roles
  */
static inline hal_spi_role spi_role(SPIRole spi)
{
    switch(spi)
    {
        case K_SPI_MASTER: return HAL_SPI_MASTER;
        case K_SPI_SLAVE: return HAL_SPI_SLAVE;
        default: return 0;
    }
}

/**
  * @brief Helper function to get spi handle.
  */
static inline hal_spi_handle * spi_handle(KSPINum spi)
{
    switch(spi)
    {
        case K_SPI1: return &hal_spi_buses[HAL_SPI_B0];
        case K_SPI2: return &hal_spi_buses[HAL_SPI_B1];
        default: return NULL;
    }
}

/**
  * @brief Helper function to convert spi direction
  */
static inline hal_spi_direction spi_direction(SPIDirection spi)
{
    switch(spi)
    {
        case K_SPI_DIRECTION_2LINES: return HAL_SPI_DIRECTION_2LINES;
        case K_SPI_DIRECTION_2LINES_RXONLY: return HAL_SPI_DIRECTION_2LINES_RXONLY;
        case K_SPI_DIRECTION_1LINE: return HAL_SPI_DIRECTION_1LINE;
        default: return 0;
    }
}

/**
  * @brief Helper function to convert spi data size
  */
static inline hal_spi_data_size spi_data_size(SPIDataSize spi)
{
    switch(spi)
    {
        case K_SPI_DATASIZE_8BIT: return HAL_SPI_DATASIZE_8BIT;
        case K_SPI_DATASIZE_16BIT: return HAL_SPI_DATASIZE_16BIT;
        default: return 0;
    }
}

/**
  * @brief Creates and sets up specified spi bus option.
  * @param spi Number of spi bus to setup.
  */
void kprv_spi_dev_init(KSPINum spi)
{
    KSPI *k_spi = kprv_spi_get(spi);

    hal_spi_conf config = {
            .role = spi_role(k_spi->config.role),
            .data_size = spi_data_size(k_spi->config.data_size),
            .direction = spi_direction(k_spi->config.direction),
            .speed = k_spi->config.speed,
    };

    hal_spi_handle * handle = hal_spi_init(config, spi);
    handle->bus_num = spi_bus(spi);
    hal_spi_setup(handle);
}

void kprv_spi_dev_terminate(KSPINum spi)
{
    hal_spi_dev_terminate(spi_handle(spi));
}

KSPIStatus kprv_spi_write(KSPINum spi, uint8_t *buffer, uint32_t len)
{
    hal_spi_status ret = hal_spi_master_write(spi_handle(spi), buffer, len);
    return (KSPIStatus)ret;
}

KSPIStatus kprv_spi_read(KSPINum spi, uint8_t *buffer, uint32_t len)
{
    hal_spi_status ret = hal_spi_master_read(spi_handle(spi), buffer, len);
    return (KSPIStatus)ret;
}

KSPIStatus kprv_spi_write_read(KSPINum spi, uint8_t *txBuffer, uint8_t *rxBuffer, uint32_t len)
{
    hal_spi_status ret = hal_spi_master_write_read(spi_handle(spi), txBuffer, rxBuffer, len);
    return (KSPIStatus)ret;
}

#endif
