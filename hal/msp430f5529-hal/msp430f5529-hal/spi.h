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
 * @file      spi.h
 * @brief      MSP430F5529 HAL - SPI module
 *
 * @author     kubos.co
 */

#if (defined YOTTA_CFG_HARDWARE_SPI) && (YOTTA_CFG_HARDWARE_SPI_COUNT > 0)
#ifndef HAL_SPI_H
#define HAL_SPI_H

#include <stdint.h>

/**
  * @brief Type by which SPI devices are numbered.
  */
typedef enum
{
    HAL_SPI_B0 = 0,
    HAL_SPI_B1
} hal_spi_bus;

/**
 * @brief Expected role of spi bus.
 */
typedef enum {
    HAL_SPI_MASTER = 0,
    HAL_SPI_SLAVE
} hal_spi_role;

/**
 * @brief Directionality of spi bus.
 */
typedef enum {
    HAL_SPI_DIRECTION_2LINES = 0,
    HAL_SPI_DIRECTION_2LINES_RXONLY,
    HAL_SPI_DIRECTION_1LINE
} hal_spi_direction;

/**
 * @brief Size of spi data.
 */
typedef enum {
    HAL_SPI_DATASIZE_8BIT = 0,
    HAL_SPI_DATASIZE_16BIT
} hal_spi_data_size;

typedef enum {
    HAL_SPI_CPOL_LOW = 0,
    HAL_SPI_CPOL_HIGH
} hal_spi_clock_polarity;

typedef enum {
    HAL_SPI_CPHA_1EDGE = 0,
    HAL_SPI_CPHA_2EDGE
} hal_spi_clock_phase;

typedef enum {
    HAL_SPI_FIRSTBIT_MSB = 0,
    HAL_SPI_FIRSTBIT_LSB
} hal_spi_first_bit;

typedef enum {
    HAL_SPI_OK,
    HAL_SPI_ERROR,
    HAL_SPI_ERROR_TIMEOUT
} hal_spi_status;

typedef struct {
    hal_spi_role role;
    hal_spi_direction direction;
    hal_spi_data_size data_size;
    hal_spi_clock_polarity clock_polarity;
    hal_spi_clock_phase clock_phase;
    hal_spi_first_bit first_bit;
    uint32_t speed;
} hal_spi_conf;

/**
 * @brief This type is a map of the USCI_Bx spi registers.
 */
typedef struct
{
    /** UCBxCTL1 */
    volatile uint8_t control1;
    /** UCBxCTL0 */
    volatile uint8_t control0;
    uint8_t padding1[4];
    /** UCBxBR0 */
    volatile uint8_t baudrate0;
    /** UCBxBR1 */
    volatile uint8_t baudrate1;
    uint8_t padding2[2];
    /** UCBxSTAT */
    volatile uint8_t status;
    uint8_t padding3;
    /** UCBxRXBUF */
    volatile uint8_t rx_buffer;
    uint8_t padding4;
    /** UCBxTXBUF */
    volatile uint8_t tx_buffer;
    uint8_t padding5;
    /** UCBxI2COA */
    volatile uint8_t own_address;
    uint8_t padding6;
    /** UCBxI2CSA */
    volatile uint8_t slave_address;
    uint8_t padding7[9];
    /** UCAxIE */
    volatile uint8_t interrupt_enable;
    /** UCAxIFG */
    volatile uint8_t interrupt_flags;
    /** UCAxIV */
    volatile uint8_t interrupt_vector;
} hal_spi_mem_reg;

/**
 * @brief This type contains all spi config and register details for this layer.
 */
typedef struct
{
    volatile hal_spi_mem_reg* reg;
    hal_spi_bus bus_num;
    hal_spi_conf conf;
    volatile uint8_t * select;
    uint8_t select_val;
} hal_spi_handle;

/**
 * @brief Static array of available spi handles.
 */
extern hal_spi_handle hal_spi_buses[];

/**
 * @brief Creates a spi handle and fills in details associated with
 *        the specified bus.
 * @param spi Instance of hal_spi_bus, specifying bus to use.
 * @retval hal_spi_handle *
 */
hal_spi_handle * hal_spi_device_init(hal_spi_bus spi);

/**
 * @brief Creates a spi handle according to details specified in config.
 * @param config Instance of hal_spi_conf with init details.
 * @param spi Instance of hal_spi_bus, specifying bus to use.
 * @retval hal_spi_handle *
 */
hal_spi_handle * hal_spi_init(hal_spi_conf config, hal_spi_bus spi);

/**
 * @brief Low level hardware de-init of spi.
 * @param handle Instance of initilized hal_spi_handle containing hardware
 *               registers and config values.
 */
void hal_spi_dev_terminate(hal_spi_handle * handle);

/**
 * @brief Low level hardware setup of spi.
 * @param handle Instance of initilaized hal_spi_handle containing hardware
 *               registers and config values.
 */
void hal_spi_setup(hal_spi_handle * handle);

/**
 * @brief Writes a buffer to a slave device over spi.
 * @param handle spi bus handle to transmit on
 * @param buffer buffer pointer to write
 * @param len number of characters to write
 */
hal_spi_status hal_spi_master_write(hal_spi_handle * handle, uint8_t *buffer, int len);

/**
 * @brief Reads a buffer from a slave device over spi.
 * @param handle spi bus handle to read from
 * @param buffer buffer pointer to read to
 * @param len number of characters to read
 */
hal_spi_status hal_spi_master_read(hal_spi_handle * handle, uint8_t *buffer, int len);

/**
 * @brief Reads a buffer from a slave device over spi.
 * @param handle spi bus handle to read from
 * @param tx_buffer buffer pointer to write
 * @param rx_buffer buffer pointer to read to
 * @param len number of characters to read
 */
hal_spi_status hal_spi_master_write_read(hal_spi_handle * handle, uint8_t *tx_buffer, uint8_t *rx_buffer, int len);

#endif
#endif
