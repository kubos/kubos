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
 * @defgroup MSP430F5529_HAL_SPI MSP430F5529 HAL SPI Interface
 * @addtogroup MSP430F5529_HAL_SPI
 * @{
 */

#if (defined YOTTA_CFG_HARDWARE_SPI) && (YOTTA_CFG_HARDWARE_SPI_COUNT > 0)
#include "kubos-hal/spi.h"
#include "msp430f5529-hal/spi.h"
#include <msp430.h>

/**
  * Helper function to convert SPI bus option.
  * @param[in] spi SPI bus number
  * @return hal_spi_bus HAL specific SPI bus number
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
  * Helper function to convert SPI roles
  * @param[in] spi SPI bus role
  * @return hal_spi_role HAL specific SPI role option
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
  * Helper function to get SPI handle.
  * @param[in] spi SPI bus number
  * @return hal_spi_handle* pointer to SPI hardware handle
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
  * Helper function to convert SPI direction
  * @param[in] spi SPI bus direction option
  * @return hal_spi_direction HAL specific SPI bus direction option
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
  * Helper function to convert SPI data size
  * @param[in] spi SPI bus data size (width) option
  * @return hal_spi_data_size HAL specific data size (width) option
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
 * Helper function to convert SPI clock phase
 * @param[in] phase SPI bus clock phase option
 * @return hal_spi_clock_phase HAL specific SPI bus clock phase option
 */
static inline hal_spi_clock_phase spi_clock_phase(SPIClockPhase phase)
{
    switch(phase)
    {
        case K_SPI_CPOL_LOW: return HAL_SPI_CPOL_LOW;
        case K_SPI_CPOL_HIGH: return HAL_SPI_CPOL_HIGH;
        default: return -1;
    }
}

/**
 * Helper function to convert SPI clock polarity option
 * @param[in] polarity SPI clock polarity option
 * @return hal_spi_clock_polarity HAL specific clock polarity option
 */
static inline hal_spi_clock_polarity spi_clock_polarity(SPIClockPolarity polarity)
{
    switch(polarity)
    {
        case K_SPI_CPHA_1EDGE: return HAL_SPI_CPHA_1EDGE;
        case K_SPI_CPHA_2EDGE: return HAL_SPI_CPHA_2EDGE;
        default: return -1;
    }
}

/**
 * Helper function to convert SPI first bit option
 * @param[in] firstbit SPI bus first bit option
 * @return hal_spi_first_bit HAL specific SPI first bit option
 */
static inline hal_spi_first_bit spi_first_bit(SPIFirstBit firstbit)
{
    switch(firstbit)
    {
        case K_SPI_FIRSTBIT_LSB: return HAL_SPI_FIRSTBIT_LSB;
        case K_SPI_FIRSTBIT_MSB: return HAL_SPI_FIRSTBIT_MSB;
        default: return -1;
    }
}

/**
 * Setup and enable SPI bus
 * @param[in] spi SPI bus to initialize
 * @return KSPIStatus SPI_OK if success, otherwise a specific error flag
 */
KSPIStatus kprv_spi_dev_init(KSPINum spi)
{
    KSPI *k_spi = kprv_spi_get(spi);
    if (k_spi == NULL)
    {
        return SPI_ERROR_NULL_HANDLE;
    }

    hal_spi_conf config = {
            .role = spi_role(k_spi->config.role),
            .data_size = spi_data_size(k_spi->config.data_size),
            .direction = spi_direction(k_spi->config.direction),
            .clock_phase = spi_clock_phase(k_spi->config.clock_phase),
            .clock_polarity = spi_clock_polarity(k_spi->config.clock_polarity),
            .first_bit = spi_first_bit(k_spi->config.first_bit),
            .speed = k_spi->config.speed,
    };

    //Reject configuration options that the MSP430F5529 doesn't support.
    //Note: Slave mode will be supported once we implement it.
    if(config.role == HAL_SPI_SLAVE || config.direction == K_SPI_DIRECTION_1LINE || config.data_size == K_SPI_DATASIZE_16BIT)
    {
        return SPI_ERROR_CONFIG;
    }

    hal_spi_handle * handle = hal_spi_init(config, spi_bus(spi));
    if (handle != NULL)
    {
        handle->bus_num = spi_bus(spi);
        hal_spi_setup(handle);
        return SPI_OK;
    }
    return SPI_ERROR_NULL_HANDLE;
}

/**
 * SPI hardware cleanup and disabling
 * @param[in] spi bus num to terminate
 * @return KSPIStatus SPI_OK if success, otherwise a specific error flag
 */
KSPIStatus kprv_spi_dev_terminate(KSPINum spi)
{
    hal_spi_handle * handle = spi_handle(spi);
    if (handle != NULL)
    {
        hal_spi_dev_terminate(handle);
        return SPI_OK;
    }
    return SPI_ERROR_NULL_HANDLE;
}

/**
 * Write data over SPI bus
 * @param[in] spi SPI bus to write to
 * @param[in] buffer pointer to data buffer
 * @param[in] len length of data to write
 * @return KSPIStatus SPI_OK on success, otherwise failure
 */
KSPIStatus kprv_spi_write(KSPINum spi, uint8_t *buffer, uint32_t len)
{
    hal_spi_handle * handle = spi_handle(spi);
    if(handle->conf.direction == K_SPI_DIRECTION_2LINES_RXONLY)
    {
        return SPI_ERROR;
    }

    hal_spi_status ret = hal_spi_master_write(handle, buffer, len);
    return (KSPIStatus)ret;
}

/**
 * Read data over SPI bus
 * @param[in] spi SPI bus to read from
 * @param[out] buffer pointer to data buffer
 * @param[in] len length of data to read
 * @return KSPIStatus SPI_OK on success, otherwise failure
 */
KSPIStatus kprv_spi_read(KSPINum spi, uint8_t *buffer, uint32_t len)
{
    hal_spi_status ret = hal_spi_master_read(spi_handle(spi), buffer, len);
    return (KSPIStatus)ret;
}

/**
 * Write and read data over SPI bus
 * @param[in] spi SPI bus to write to
 * @param[in] txBuffer pointer to data buffer to write from
 * @param[out] rxBuffer pointer to data buffer to read into
 * @param[in] len length of data to write and read
 * @return KSPIStatus SPI_OK on success, otherwise failure
 */
KSPIStatus kprv_spi_write_read(KSPINum spi, uint8_t *txBuffer, uint8_t *rxBuffer, uint32_t len)
{
    hal_spi_handle * handle = spi_handle(spi);
    if(handle->conf.direction == K_SPI_DIRECTION_2LINES_RXONLY)
    {
        return SPI_ERROR;
    }

    hal_spi_status ret = hal_spi_master_write_read(handle, txBuffer, rxBuffer, len);
    return (KSPIStatus)ret;
}

#endif

/* @} */