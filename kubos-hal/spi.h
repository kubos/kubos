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
 * @brief KubOS-HAL SPI Interface
 * @author kubos.co
 */

#if (defined YOTTA_CFG_HARDWARE_SPI) && (YOTTA_CFG_HARDWARE_SPI_COUNT > 0)
#ifndef K_SPI_H
#define K_SPI_H

#include "FreeRTOS.h"
#include "semphr.h"

/**
 * Number of spi buses available. Derived from value in target.json
 * @code
 * "config": {
 *   "hardware": {
 *     "spi": {
 *       "count": 2
 *     }
 *   }
 * }
 * @endcode
 */
#ifndef K_NUM_SPI
#define K_NUM_SPI YOTTA_CFG_HARDWARE_SPI_COUNT
#endif

/**
 * Default spi bus. Derived from value in target.json
 * @code
 * "config": {
 *   "hardware": {
 *     "spi": {
 *       "defaults": {
 *          "bus": "K_SPI1"
 *        }
 *     }
 *   }
 * }
 * @endcode
 */
#define DEFAULT_SPI YOTTA_CFG_HARDWARE_SPI_DEFAULTS_BUS

/**
 * Available spi buses
 */
typedef enum {
#ifdef YOTTA_CFG_HARDWARE_SPI_SPI1
    K_SPI1 = 0,
#endif
#ifdef YOTTA_CFG_HARDWARE_SPI_SPI2
    K_SPI2,
#endif
#ifdef YOTTA_CFG_HARDWARE_SPI_SPI3
    K_SPI3
#endif
} KSPINum;

/**
 * Expected role of spi bus
 */
typedef enum {
    K_SPI_MASTER = 0,
    K_SPI_SLAVE
} SPIRole;

/**
 * Spi direction mode
 */
typedef enum {
    K_SPI_DIRECTION_2LINES = 0,
    K_SPI_DIRECTION_2LINES_RXONLY,
    K_SPI_DIRECTION_1LINE
} SPIDirection;

/**
 * Spi data size
 */
typedef enum {
    K_SPI_DATASIZE_8BIT = 0,
    K_SPI_DATASIZE_16BIT
} SPIDataSize;

/**
 * Spi clock polarity
 */
typedef enum {
    K_SPI_CPOL_LOW = 0,
    K_SPI_CPOL_HIGH
} SPIClockPolarity;

/**
 * Spi clock phase
 */
typedef enum {
    K_SPI_CPHA_1EDGE = 0,
    K_SPI_CPHA_2EDGE
} SPIClockPhase;

/**
 * Spi first bit order/endianess
 */
typedef enum {
    K_SPI_FIRSTBIT_MSB = 0,
    K_SPI_FIRSTBIT_LSB
} SPIFirstBit;

/**
 * Spi status values
 */
typedef enum {
    SPI_OK,
    SPI_ERROR,
    SPI_ERROR_TIMEOUT
} KSPIStatus;

/**
 * Spi configuration structure
 */
typedef struct {
    SPIRole role;
    SPIDirection direction;
    SPIDataSize data_size;
    SPIClockPhase clock_phase;
    SPIClockPolarity clock_polarity;
    SPIFirstBit first_bit;
    uint32_t speed;
} KSPIConf;

/**
 * Spi bus data structure
 */
typedef struct {
    KSPIConf config;
    KSPINum bus_num;
    SemaphoreHandle_t spi_lock;
} KSPI;

/**
 * Setup and enable spi bus
 * @param spi spi bus to initialize
 * @param conf config values to initialize with
 */
void k_spi_init(KSPINum spi, KSPIConf * conf);

/**
 * Terminate spi bus
 * @param spi spi bus to terminate
 */
void k_spi_terminate(KSPINum spi);

/**
 * Write data over spi bus
 *
 * In order to ensure safe spi sharing, this function is semaphore locked.
 * There is one semaphore per bus. This function will block indefinitely
 * while waiting for the semaphore.
 *
 * @param spi spi bus to write to
 * @param buffer pointer to data buffer
 * @param len length of data to write
 * @return KSPIStatus SPI_OK on success, otherwise failure
 */
KSPIStatus k_spi_write(KSPINum spi, uint8_t * buffer, uint32_t len);

/**
 * Read data over spi bus
 *
 * In order to ensure safe spi sharing, this function is semaphore locked.
 * There is one semaphore per bus. This function will block indefinitely
 * while waiting for the semaphore.
 *
 * @param spi spi bus to read from
 * @param buffer pointer to data buffer
 * @param len length of data to read
 * @return KSPIStatus SPI_OK on success, otherwise failure
 */
KSPIStatus k_spi_read(KSPINum spi, uint8_t * buffer, uint32_t len);

/**
 * Write and read data over spi bus
 *
 * In order to ensure safe spi sharing, this function is semaphore locked.
 * There is one semaphore per bus. This function will block indefinitely
 * while waiting for the semaphore.
 *
 * @param spi spi bus to write to
 * @param txBuffer pointer to data buffer to write from
 * @param rxBuffer pointer to data buffer to read into
 * @param len length of data to write and read
 * @return KSPIStatus SPI_OK on success, otherwise failure
 */
KSPIStatus k_spi_write_read(KSPINum spi, uint8_t * txBuffer, uint8_t * rxBuffer, uint32_t len);

/**
 * Fetches spi bus data structure
 * @param spi number of spi bus to fetch
 * @return KSPI* pointer to data structure
 */
KSPI * kprv_spi_get(KSPINum spi);

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
KSPIStatus kprv_spi_write(KSPINum spi, uint8_t * buffer, uint32_t len);

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
KSPIStatus kprv_spi_read(KSPINum spi, uint8_t * buffer, uint32_t len);

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
KSPIStatus kprv_spi_write_read(KSPINum spi, uint8_t * txBuffer, uint8_t * rxBuffer, uint32_t len);

void kprv_spi_dev_init(KSPINum spi);

void kprv_spi_dev_terminate(KSPINum spi);

#endif
#endif

/* @} */
