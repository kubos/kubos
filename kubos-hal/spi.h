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

#include <csp/arch/csp_semaphore.h>

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
    K_SPI_NO_BUS = 0,
#ifdef YOTTA_CFG_HARDWARE_SPI_SPI1
    K_SPI1,
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
 * @warning Only the Master role is available as of v0.1.0
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
    SPI_ERROR_BUSY,
    SPI_ERROR_TIMEOUT,
    SPI_ERROR_NULL_HANDLE
} KSPIStatus;

/**
 * Spi configuration structure
 */
typedef struct {
    /**
     * The role of the SPI bus.
     * Should be either master or slave, as specified by the SPIRole enumerator
     * @warning Only the Master role is available as of v0.1.0
     */
    SPIRole role;
    /**
     * The communication mode of the SPI bus.
     * Can be 2-wire Rx/Tx, 2-wire Rx only, or 1-wire bidirectional, as specified by the SPIDirection enumerator
     */
    SPIDirection direction;
    /**
     * The amount of data in each transmit/receive of the SPI bus.
     * Can either send 8-bits at a time or 16-bits, as specified by the SPIDataSize enumerator
     */
    SPIDataSize data_size;
    /**
     * The clock phase of the SPI bus.
     * Can either be low (idle state = 0, active state = 1), or high (idle state = 1, active state = 0),
     * as specified by the SPIClockPhase enumerator
     */
    SPIClockPhase clock_phase;
    /**
     * The clock polarity of the SPI bus.
     * Can either be the first edge (falling if clock phase is high, rising if clock phase is low), or
     * second edge (rising if clock phase is high, falling if clock phase is low), as specified by the
     * SPIClockPolarity enumerator
     */
    SPIClockPolarity clock_polarity;
    /**
     * The bit ordering of the SPI bus communication.
     * Can be either least-significant bit first, or most-significant, as specified by the SPIFirstBit enumerator
     */
    SPIFirstBit first_bit;
    /**
     * The baud rate of the SPI bus
     * @warning For the <b>STM32F4 microcontroller</b>, the speed of the SPI bus can only be defined as a factor of the
     * peripheral clock to which it's connected (PCLK1 for SPI bus 2 and 3, PCLK2 for SPI bus 1).  For example,
     * PCLK_speed / 2.  To make things easier, this speed field will take a normal baud rate number and then it
     * will automatically be converted to the nearest available system speed without exceeding the original
     * value. <br />
     * For example:  <br />
     * Given conf.speed = 10MHz, PCLK_speed = 84MHz <br />
     * The closest speed without going over is 5.25Mhz (PCLK_speed / 16), so this is what the SPI bus speed will be
     * set to.
     */
    uint32_t speed;
} KSPIConf;

/**
 * Spi bus data structure
 */
typedef struct {
    KSPIConf config;
    KSPINum bus_num;
    csp_mutex_t spi_lock;
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

KSPIConf k_spi_conf_defaults(void);

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

/**
 * Low level spi initialization
 *
 * This is implemented by the hardware specific hal
 *
 * @param spi spi bus to initialize
 * @return KSPIStatus SPI_OK on success, otherwise failure
 */
KSPIStatus kprv_spi_dev_init(KSPINum spi);

/**
 * Low level spi termination
 *
 * This is implemented by the hardware specific hal
 *
 * @param spi spi bus to terminate
 * @return KSPIStatus SPI_OK on success, otherwise failure
 */
KSPIStatus kprv_spi_dev_terminate(KSPINum spi);

#endif
#endif

/* @} */
