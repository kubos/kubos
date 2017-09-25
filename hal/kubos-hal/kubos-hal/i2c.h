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
 * @defgroup I2C HAL I2C Interface
 * @addtogroup I2C
 * @{
 */

#if (defined YOTTA_CFG_HARDWARE_I2C) && (YOTTA_CFG_HARDWARE_I2C_COUNT > 0)
#ifndef K_I2C_H
#define K_I2C_H

#include <stdint.h>
#include <csp/arch/csp_semaphore.h>

/**
 * Number of I2C buses available. Derived from value in target.json
 * @code
 * "config": {
 *   "hardware": {
 *     "i2c": {
 *       "count": 2
 *     }
 *   }
 * }
 * @endcode
 */
#ifndef K_NUM_I2CS
#define K_NUM_I2CS YOTTA_CFG_HARDWARE_I2C_COUNT
#endif

/**
 * Default I2C bus. Derived from value in target.json
 * @code
 * "config": {
 *   "hardware": {
 *     "i2c": {
 *       "defaults": {
 *         "bus": "K_I2C1"
 *       }
 *     }
 *   }
 * }
 * @endcode
 */
#ifndef DEFAULT_I2C
#define DEFAULT_I2C YOTTA_CFG_HARDWARE_I2C_DEFAULTS_BUS
#endif

/**
 * Available I2C buses
 */
typedef enum {
    K_I2C_NO_BUS = 0,
#ifdef YOTTA_CFG_HARDWARE_I2C_I2C1
    K_I2C1,
#endif
#ifdef YOTTA_CFG_HARDWARE_I2C_I2C2
    K_I2C2,
#endif
#ifdef YOTTA_CFG_HARDWARE_I2C_I2C3
    K_I2C3
#endif
} KI2CNum;

/**
 * Expected addressing mode of I2C bus
 */
typedef enum {
    K_ADDRESSINGMODE_7BIT = 0,
    K_ADDRESSINGMODE_10BIT
} I2CAddressingMode;

/**
 * Expected role of I2C bus
 * @warning Only the Master role is available as of v0.0.4
 */
typedef enum {
    K_MASTER = 0,
    K_SLAVE
} I2CRole;

/**
 * Structure used to store I2C bus configuration options
 */
typedef struct {
	/**
	 * The size of the slave address.
	 * Should be either 7-bits long or 10-bits long, as specified by the @ref I2CAddressingMode enumerator
	 */
    I2CAddressingMode addressing_mode;
	/**
	 * The role of the I2C bus.
	 * Should be either master or slave, as specified by the @ref I2CRole enumerator
	 * @warning Only the Master role is available as of v0.1.0
	 */
    I2CRole role;
    /**
     * The clock speed of the I2C bus
     */
    uint32_t clock_speed;
} KI2CConf;

/**
 * Structure used to store I2C bus description/configuration information
 */
typedef struct {
    /**
     * I2C device number
     */
    KI2CNum bus_num;
    /**
     * I2C device configuration values
     */
    KI2CConf conf;
    /**
     * Mutex for guarding device access
     */
    csp_mutex_t i2c_lock;
} KI2C;

/**
 * I2C function status
 */
typedef enum {
    I2C_OK = 0,
    I2C_ERROR,
    I2C_ERROR_AF,
    I2C_ERROR_ADDR_TIMEOUT,
    I2C_ERROR_TIMEOUT,
    I2C_ERROR_NACK,
    I2C_ERROR_TXE_TIMEOUT,
    I2C_ERROR_BTF_TIMEOUT,
    I2C_ERROR_NULL_HANDLE,
    I2C_ERROR_CONFIG
} KI2CStatus;

/**
 * @brief Configures and enables an I2C bus
 * 
 * This function is used to configure and enable I2C buses for further usage (reading/writing).
 * It is always the first required step before using any I2C peripheral. The k_i2c_init function
 * takes an I2C bus number (KI2CNum) and I2C configuration structure (KI2CConf). The I2C bus number *must* be a valid
 * value of the KI2CNum enum. The configuration can either be manually created or k_i2c_conf_defaults
 * can be used to retreive the default configuration. 
 
 * After correctly calling k_i2c_init, the bus number used may be used with the k_i2c_read/k_i2c_write/k_i2c_terminate functions.
 *
 * Example usage:
 * @code
 KI2CConf conf = {
     .addressing_mode = K_ADDRESSING_MODE_7BIT,
     .role = K_MASTER,
     .clock_speed = 100000
 };
 k_i2c_init(K_I2C1, &conf);
 * @endcode
 *
 * @note The functions k_i2c_default_init or k_i2c_default_dev_init can also be used to initialize an
 * I2C device. They provide nice convience wrappers around k_i2c_init.
 *
 * @param i2c I2C bus to initialize
 * @param conf config values to initialize with
 * @return KI2CStatus I2C_OK on success, otherwise return I2C_ERROR_*
 */
KI2CStatus k_i2c_init(KI2CNum i2c, KI2CConf *conf);

/**
 * @brief Terminates an I2C bus
 *
 * This fuction is used to terminate an active I2C bus. It takes the bus number of an active I2C bus.
 * After calling this function the bus number used will *not* be available for usage in the reading/writing functions.
 *
 * Example usage:
 * @code
// initialize bus
k_i2c_init(K_I2C1, &conf);
// read some data
k_i2c_read(K_I2C1, addr, buffer, 10);
// shut down bus
k_i2C_terminate(K_I2C1);
 * @endcode
 * @param i2c I2C bus to terminate
 */
void k_i2c_terminate(KI2CNum i2c);

/**
 * @brief Generate KI2CConf with default I2C values
 *
 * This function returns a KI2CConf structure with the default I2C values already set.
 * These default values are derived from the target.json file of the selected hardware target.
 * New default values can easily be set by creating a config.json file in the project directory.
 *
 * Example contents of config.json overriding defaults:
 * @code{.json}
"hardware": {
  "i2c": {
    "defaults": {
      "addressingmode": "K_ADDRESSING_MODE_7BIT",
      "role": "K_MASTER",
      "clockspeed": 50000
    }
  }
}
 * @endcode
 * @return KI2CConf
 */
KI2CConf k_i2c_conf_defaults(void);

/**
 * @brief Initialize default I2C configuration
 *
 * This function initializes the default I2C bus using the preset default I2C configuration.
 * The default I2C bus and configuration are set in the target.json file and can be overriden in a project's config.json.
 *
 * Calling this function is equivalent to the following code:
 * @code{.c}    
KI2CConf default_conf = k_i2c_conf_defaults();
k_i2c_init(DEFAULT_I2C, &default_conf);
 * @endcode
 * The DEFAULT_I2C define is derived from the json default values.
 *
 * Example contents of config.json overriding defaults:
 * @code
"hardware": {
  "i2c": {
    "defaults": {
      "addressingmode": "K_ADDRESSING_MODE_7BIT",
      "role": "K_MASTER",
      "clockspeed": 50000
      "bus": "K_I2C1"
    }
  }
}
 * @endcode
 */
void k_i2c_default_init();

/**
 * @brief Initialize specified I2C bus with default values.
 *
 * This function initializes a specified I2C bus with the preset default I2C configuration.
 * The default I2C configuration is derived from the target.json file and can be overriden in a project's config.json.
 *
 * Calling this function is equivalent to the following code (K_I2C3 could be any I2C bus number):
 * @code{.c}
KI2CConf default_conf = k_i2c_conf_defaults();
k_i2c_init(K_I2C3, &default_conf);
 * @endcode
 *
 * @param i2c I2C bus num to initialize
 */
void k_i2c_default_dev_init(KI2CNum i2c);

/**
 * @brief Write data over the I2C bus to specified address
 *
 * This function writes data over the specified I2C bus to the specified slave address.
 * The actual low-level I2C writing is delegated to the hardware specific kprv_i2c_*_write functions.
 * This function is intended to be used on an I2C bus which has already been initialized.
 * 
 * Example usage:
 * @code
uint8_t cmd = 0x40;
uint16_t slave_addr = 0x80;
KI2CStatus write_status;
write_status = k_i2c_write(K_I2C1, slave_addr, &cmd, 1);
 * @endcode
 *
 * In order to ensure safe I2C sharing, this function is semaphore locked.
 * There is one semaphore per bus. This function will block indefinitely
 * while waiting for the semaphore.
 *
 * @param i2c I2C bus to transmit over
 * @param addr address of target I2C device
 * @param ptr pointer to data buffer
 * @param len length of data in buffer
 * @return KI2CStatus I2C_OK on success, I2C_ERROR on error
 */
KI2CStatus k_i2c_write(KI2CNum i2c, uint16_t addr, uint8_t *ptr, int len);

/**
 * @brief Read data over the I2C bus from specified address
 *
 * This function reads data from the specified I2C bus from the specified slave address.
 * The actual low-level I2C reading is delegated to the hardware specific kprv_i2c_*_read functions.
 * This function is intended to be used on an I2C bus which has already been initialized.
 *
 * Example usage:
 * @code
uint8_t buffer[10];
int read_len = 10;
uint16_t slave_addr = 0x80;
KI2CStatus read_status;
read_status = k_i2c_read(K_I2C1, slave_addr, buffer, read_len);
 * @endcode
 *
 * In order to ensure safe I2C sharing, this function is semaphore locked.
 * There is one semaphore per bus. This function will block indefinitely
 * while waiting for the semaphore.
 *
 * @param i2c I2C bus to read from
 * @param addr address of target I2C device
 * @param ptr pointer to data buffer
 * @param len length of data to read
 * @return KI2CStatus I2C_OK on success, I2C_ERROR on error
 */
KI2CStatus k_i2c_read(KI2CNum i2c, uint16_t addr, uint8_t *ptr, int len);

/**
 * Fetches I2C bus data structure
 * @param i2c number of I2C bus to fetch
 * @return KI2C* pointer to data structure
 */
KI2C* kprv_i2c_get(KI2CNum i2c);

/**
 * Low-level HAL device initialization
 * This is implemented by the device specific HAL
 * @param i2c I2C bus to initialize
 * @return KI2CStatus I2C_OK if success, otherwise specific error
 */
KI2CStatus kprv_i2c_dev_init(KI2CNum i2c);

/**
 * Low-level HAL I2C termination
 * This is implemented by the device specific HAL
 * @param i2c I2C bus to terminate
 * @return KI2CStatus I2C_OK if success, otherwise specific error
 */
KI2CStatus kprv_i2c_dev_terminate(KI2CNum i2c);

/**
 * @brief Low-level HAL I2C write (as master)
 *
 * This function is called by k_i2c_write and is intended to perform the neccesary low-level
 * actions for an I2C write (as a master).
 *
 * ** This function must be implemented for each platform specific HAL. **
 *
 * @param i2c I2C bus to write from
 * @param addr I2C addr to write to
 * @param ptr data buffer
 * @param len length of data in buffer
 * @return KI2CStatus I2C_OK on success, I2C_ERROR on error
 */
KI2CStatus kprv_i2c_master_write(KI2CNum i2c, uint16_t addr, uint8_t *ptr, int len);

/**
 * @brief Low-level HAL I2C read (as master)
 *
 * This function is called by k_i2c_read and is intended to perform the neccesary low-level
 * actions for an I2C read (as a master).
 *
 * ** This function must be implemented for each platform specific HAL. **
 *
 * @param i2c I2C bus to read from
 * @param addr I2C addr to read from
 * @param ptr data buffer
 * @param len length of data expected to read
 * @return KI2CStatus I2C_OK on success, I2C_ERROR on error
 */
KI2CStatus kprv_i2c_master_read(KI2CNum i2c, uint16_t addr, uint8_t *ptr, int len);

/**
 * @brief Low-level HAL I2C write (as slave)
 *
 * This function will be called by k_i2c_write and is intended to perform the neccesary low-level
 * actions for an I2C write (as a slave).
 *
 * ** This function must be implemented for each platform specific HAL. **
 *
 * @warning I2C slave functionality is not implemented as of v0.0.4
 *
 * @param i2c I2C bus to write from
 * @param addr I2C addr to write to
 * @param ptr data buffer
 * @param len length of data in buffer
 * @return KI2CStatus I2C_OK on success, I2C_ERROR on error
 */
KI2CStatus kprv_i2c_slave_write(KI2CNum i2c, uint16_t addr, uint8_t *ptr, int len);

/**
 * @brief Low-level HAL I2C read (as slave)
 *
 * This function will be called by k_i2c_read and is intended to perform the neccesary low-level
 * actions for an I2C read (as a slave). 
 *
 * ** This function must be implemented for each platform specific HAL. **
 *
 * @warning I2C slave functionality is not implemented as of v0.0.4
 *
 * @param i2c I2C bus to read from
 * @param addr I2C addr to read from
 * @param ptr data buffer
 * @param len length of data expected to read
 * @return KI2CStatus I2C_OK on success, I2C_ERROR on error
 */
KI2CStatus kprv_i2c_slave_read(KI2CNum i2c, uint16_t addr, uint8_t *ptr, int len);

#endif
#endif

/* @} */
