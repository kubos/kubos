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
 * @defgroup SPI HAL SPI Interface
 * @addtogroup SPI
 * @{
 */

#if (defined YOTTA_CFG_HARDWARE_SPI) && (YOTTA_CFG_HARDWARE_SPI_COUNT > 0)
#ifndef K_SPI_H
#define K_SPI_H

#include <csp/arch/csp_semaphore.h>

/**
 * @brief Number of SPI buses available. 
 * 
 * This value is derived from the platform specific target.json file.
 *
 * Snippet from target.json:
 * @code{.json}
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
 * @brief Default SPI bus. 
 * 
 * This value is derived from the platform specific target.json file. It is possible to override it
 * in a project's config.json.
 *
 * Example of overriding via config.json:
 * @code{.json}
 * "hardware": {
 *   "spi": {
 *     "defaults": {
 *        "bus": "K_SPI1"
 *      }
 *   }
 * }
 * @endcode
 */
#define DEFAULT_SPI YOTTA_CFG_HARDWARE_SPI_DEFAULTS_BUS

/**
 * @brief Available SPI buses
 *
 * The number of buses defined in this enumeration is controlled by the number of SPI buses
 * defined in the platform's target.json file.
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
 * @brief SPI bus roles
 * 
 * @warning Only the Master role is available as of v0.1.0
 */
typedef enum {
    K_SPI_MASTER = 0,
    K_SPI_SLAVE
} SPIRole;

/**
 * @brief SPI bus direction modes
 *
 * @note MSP430F5 does not support 1-line mode
 */
typedef enum {
    K_SPI_DIRECTION_2LINES = 0,
    K_SPI_DIRECTION_2LINES_RXONLY,
    K_SPI_DIRECTION_1LINE
} SPIDirection;

/**
 * @brief SPI bus data sizes
 *
 * @note MSP430F5 does not support 16-bit mode
 */
typedef enum {
    K_SPI_DATASIZE_8BIT = 0,
    K_SPI_DATASIZE_16BIT
} SPIDataSize;

/**
 * @brief SPI bus clock polarities
 */
typedef enum {
    K_SPI_CPOL_LOW = 0,
    K_SPI_CPOL_HIGH
} SPIClockPolarity;

/**
 * @brief SPI bus clock phases
 */
typedef enum {
    K_SPI_CPHA_1EDGE = 0,
    K_SPI_CPHA_2EDGE
} SPIClockPhase;

/**
 * @brief SPI bus first bit order/endianess
 */
typedef enum {
    K_SPI_FIRSTBIT_MSB = 0,
    K_SPI_FIRSTBIT_LSB
} SPIFirstBit;

/**
 * @brief SPI return status values
 *
 * This enumeration is intended to be the primary return status of the functions in this SPI interface.
 */
typedef enum {
    SPI_OK,
    SPI_ERROR,
    SPI_ERROR_BUSY,
    SPI_ERROR_TIMEOUT,
    SPI_ERROR_NULL_HANDLE,
    SPI_ERROR_CONFIG
} KSPIStatus;

/**
 * @brief SPI configuration structure
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
     * @warning For the <b>STM32F4  and MSP430F5 microcontrollers</b>, the speed of the SPI bus can only be defined
     * as a factor of the peripheral clock to which it's connected (PCLK1 for STM32F4 SPI bus 2 and 3, PCLK2 for
     * STM32F4 SPI bus 1, SMCLK for MSP430F5 SPI buses).
     * For example, PCLK_speed / 2.  To make things easier, this speed field will take a normal baud rate number and
     * then it will automatically be converted to the nearest available system speed without exceeding the original
     * value. <br />
     * For example:  <br />
     * Given conf.speed = 10MHz, PCLK_speed = 84MHz <br />
     * The closest speed without going over is 5.25Mhz (PCLK_speed / 16), so this is what the SPI bus speed will be
     * set to.
     */
    uint32_t speed;
} KSPIConf;

/**
 * @brief SPI bus data structure
 *
 * The SPI interface holds a static array of these structures, one for each
 * possible SPI bus.
 */
typedef struct {
    /**
     * SPI interface configuration values
     */
    KSPIConf config;
    /**
     * Number of SPI interface
     */
    KSPINum bus_num;
    /**
     * Mutex used to lock access to SPI device
     */
    csp_mutex_t spi_lock;
} KSPI;

/**
 * @brief Configures and enables a SPI bus
 *
 * This function is used to configure and enable SPI buses for further usage (reading/writing).
 * Calling this function is always the first step before using any SPI peripheral. This function
 * takes a SPI bus number (KSPINum) and SPI configuration structure (KSPIConf). The SPI bus number
 * *must* be a valid value from the KSPINum enum. The configuration can either be created manually or
 * k_spi_conf_defaults can be used to retreive the default configuration.
 *
 * After calling k_spi_init, the initalized bus can be used with the k_spi_read/k_spi_write/k_spi_terminate functions.
 *
 * Example usage:
 * @code
KSPIConf conf = {
    .role = K_SPI_MASTER,
    .direction = K_SPI_DIRECTION_2LINES,
    .data_size = K_SPI_DATASIZE_8BIT,
    .clock_polarity = K_SPI_CPOL_HIGH,
    .clock_phase = K_SPI_CPHA_1EDGE,
    .first_bit = K_SPI_FIRSTBIT_LSB,
    .speed = 100000
};

k_spi_init(K_SPI1, &conf);
 * @endcode
 *
 * @note This function delegates the low-level initialization to the platform specific @ref kprv_spi_dev_init
 *
 * @param spi number spi bus to initialize
 * @param conf config values to initialize with
 */
void k_spi_init(KSPINum spi, KSPIConf * conf);

/**
 * @brief Terminates a SPI bus
 *
 * This function is used to terminate an active SPI bus. After calling this function, the bus number
 * used will *not* be available for usage in reading/writing functions.
 *
 * Example usage:
 * @code
// init bus
k_spi_init(K_SPI1, &conf);

// read some data
k_spi_read(K_SPI1, buffer, length);

// shut down bus
k_spi_terminate(K_SPI1);
 * @endcode
 *
 * @note This function delegates the low-level termination to the platform specific @ref kprv_spi_dev_terminate
 *
 * @param spi spi bus to terminate
 */
void k_spi_terminate(KSPINum spi);

/**
 * @brief Returns a copy of the default SPI configuration
 *
 * This function returns a SPIConf structure with the default SPI configuration values already set.
 * These default values are derived from the platform's target.json. New default values can easily be
 * set by creating a config.json in the project directory.
 *
 * Example contents of config.json overriding defaults:
 * @code{.json}
 "hardware": {
    "spi": {
      "defaults": {
        "bus": "K_SPI1",
        "role": "K_SPI_MASTER",
        "direction": "K_SPI_DIRECTION_2LINES",
        "dataSize": "K_SPI_DATASIZE_8BIT",
        "clockPolarity": "K_SPI_CPOL_HIGH",
        "clockPhase": "K_SPI_CPHA_1EDGE",
        "firstBit": "K_SPI_FIRSTBIT_LSB",
        "speed": "10000"
      }
    }
 }
 * @endcode
 *
 * @return KSPIConf structure containing default config
 */
KSPIConf k_spi_conf_defaults(void);

/**
 * @brief Write data over specified SPI bus
 *
 * This function writes data over the specified SPI bus. 
 *
 * @warning This function does not handle the chip select line. The user is responsible for selecting the correct line.
 *
 * @note In order to ensure safe spi sharing, this function is semaphore locked.
 * There is one semaphore per bus. This function will block indefinitely while waiting for the semaphore.
 *
 * Example usage:
 * @code{.c}
uint8_t buffer[10];
KSPIStatus write_status;

// init SPI bus
k_spi_init(K_SPI1, &conf);

// custom chip select function
chip_select(SPI_CS1);

write_status = k_spi_write(K_SPI1, buffer, 10);

// custom chip select function
chip_deselect(SPI_CS1);
 * @endcode
 *
 * @note This function delegates the low-level actions to the platform specific @ref kprv_spi_write.
 *
 * @param spi spi bus to write to
 * @param buffer pointer to data buffer
 * @param len length of data to write
 * @return KSPIStatus SPI_OK on success, otherwise failure
 */
KSPIStatus k_spi_write(KSPINum spi, uint8_t * buffer, uint32_t len);

/**
 * @brief Reads data over specified SPI bus
 *
 * @warning This function does not handle the chip select line. The user is responsible for selecting the correct line.
 *
 * @note In order to ensure safe spi sharing, this function is semaphore locked.
 * There is one semaphore per bus. This function will block indefinitely while waiting for the semaphore.
 *
 * Example usage:
 * @code{.c}
uint8_t buffer[10];
KSPIStatus read_status;

// init SPI bus
k_spi_init(K_SPI1, &conf);

// custom chip select function
chip_select(SPI_CS1);

read_status = k_spi_read(K_SPI1, buffer, 10);

// custom chip select function
chip_deselect(SPI_CS1);
 * @endcode
 *
 * @note This function delegates the low-level actions to the platform specific @ref kprv_spi_read.
 *
 * @param spi spi bus to read from
 * @param buffer pointer to data buffer
 * @param len length of data to read
 * @return KSPIStatus SPI_OK on success, otherwise failure
 */
KSPIStatus k_spi_read(KSPINum spi, uint8_t * buffer, uint32_t len);

/**
 * @brief Writes and reads data over SPI bus
 *
 * @warning This function does not handle the chip select line. The user is responsible for selecting the correct line.
 *
 * @note In order to ensure safe spi sharing, this function is semaphore locked.
 * There is one semaphore per bus. This function will block indefinitely while waiting for the semaphore.
 *
 * Example usage:
 * @code{.c}
uint8_t read_buffer[10];
uint8_t write_buffer[10];
KSPIStatus read_status;

// init SPI bus
k_spi_init(K_SPI1, &conf);

// custom chip select function
chip_select(SPI_CS1);

read_status = k_spi_write_read(K_SPI1, write_buffer, read_buffer, 10);

// custom chip select function
chip_deselect(SPI_CS1);
 * @endcode
 *
 * @note This function delegates the low-level actions to the platform specific kprv_spi_write_read.
 *
 * @param spi spi bus to write to
 * @param txBuffer pointer to data buffer to write from
 * @param rxBuffer pointer to data buffer to read into
 * @param len length of data to write and read
 * @return KSPIStatus SPI_OK on success, otherwise failure
 */
KSPIStatus k_spi_write_read(KSPINum spi, uint8_t * txBuffer, uint8_t * rxBuffer, uint32_t len);

/**
 * @brief Fetches SPI bus data structure
 *
 * This function takes a SPI bus number and retrieves the bus data/config structure from the 
 * HAL's internal static array.
 *
 * @param spi number of spi bus to fetch
 * @return KSPI* pointer to data structure
 */
KSPI * kprv_spi_get(KSPINum spi);

/**
 * @brief Low-level SPI write
 *
 * This function is called by k_spi_write and is intended to perform the necessary low-level
 * actions for a SPI write.
 *
 * @note This function must be implemented by each platform specific HAL
 *
 * @param spi spi bus to write to
 * @param buffer pointer to data buffer
 * @param len length of data to write
 * @return KSPIStatus SPI_OK on success, otherwise failure
 */
KSPIStatus kprv_spi_write(KSPINum spi, uint8_t * buffer, uint32_t len);

/**
 * @brief Low-level SPI read
 *
 * This function is called by k_spi_read and is intended to perform the necessary low-level
 * actions for a SPI read.
 *
 * @note This function must be implemented by each platform specific HAL
 *
 * @param spi spi bus to read from
 * @param buffer pointer to data buffer
 * @param len length of data to read
 * @return KSPIStatus SPI_OK on success, otherwise failure
 */
KSPIStatus kprv_spi_read(KSPINum spi, uint8_t * buffer, uint32_t len);

/**
 * @brief Low-level SPI write and read
 *
 * This function is called by k_spi_write_read and is intended to perform the necessary low-level
 * actions for a SPI write and read.
 *
 * @note This function must be implemented by each platform specific HAL
 *
 * @param spi spi bus to write to
 * @param txBuffer pointer to data buffer to write from
 * @param rxBuffer pointer to data buffer to read into
 * @param len length of data to write and read
 * @return KSPIStatus SPI_OK on success, otherwise failure
 */
KSPIStatus kprv_spi_write_read(KSPINum spi, uint8_t * txBuffer, uint8_t * rxBuffer, uint32_t len);

/**
 * @brief Low-level SPI bus initialization
 *
 * This function is called by k_spi_init and is intended to perform the necessary low-level
 * initialization and configuration of a SPI bus.
 *
 * @note This function must be implemented by each platform specific HAL
 *
 * @param spi spi bus to initialize
 * @return KSPIStatus SPI_OK on success, otherwise failure
 */
KSPIStatus kprv_spi_dev_init(KSPINum spi);

/**
 * @brief Low-level SPI bus termination
 *
 * This function is called by k_spi_terminate and is intended to perform the necessary low-level
 * actios to power-down and disable a SPI bus.
 *
 * @note This function must be implemented by each platform specific HAL
 *
 * @param spi spi bus to terminate
 * @return KSPIStatus SPI_OK on success, otherwise failure
 */
KSPIStatus kprv_spi_dev_terminate(KSPINum spi);

#endif
#endif

/* @} */
