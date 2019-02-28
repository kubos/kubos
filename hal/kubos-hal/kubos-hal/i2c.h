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

#ifndef K_I2C_H
#define K_I2C_H

#include <pthread.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

/**
 * IOCTL master role value
 */
#define I2C_MASTER  0
/**
 * IOCTL slave role value
 */
#define I2C_SLAVE   1

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
 * takes an I2C device name and a pointer to where the returned file descriptor
 * should be stored
 * After correctly calling k_i2c_init, the returned file descriptor may be used with
 * the k_i2c_read/k_i2c_write/k_i2c_terminate functions.
 *
 * Example usage:
 * @code
int bus = 0;
k_i2c_init("/dev/i2c-1", &bus);
 * @endcode
 *
 * @param device I2C device name to initialize
 * @param fp Pointer to storage for file descriptor of I2C bus
 * @return KI2CStatus I2C_OK on success, otherwise return I2C_ERROR_*
 */
KI2CStatus k_i2c_init(char * device, int * fp);

/**
 * @brief Terminates an I2C bus
 *
 * This fuction is used to terminate an active I2C bus connection.
 * It takes a pointer to the file descriptor to be closed.
 * After calling this function the device will *not* be available for usage in the reading/writing functions.
 *
 * Example usage:
 * @code
// initialize bus
int bus = 0;
k_i2c_init("/dev/i2c-1", &bus);
// read some data
k_i2c_read(bus, addr, buffer, 10);
// shut down bus
k_i2C_terminate(bus);
 * @endcode
 * @param fp Pointer to file descriptor of I2C bus which should be closed
 */
void k_i2c_terminate(int * fp);

/**
 * @brief Write data over the I2C bus to specified address
 *
 * This function writes data over the specified I2C bus to the specified slave address.
 * The actual low-level I2C writing is delegated to the hardware specific kprv_i2c_*_write functions.
 * This function is intended to be used on an I2C bus which has already been initialized.
 * 
 * Example usage:
 * @code
int bus = 0;
k_i2c_init("/dev/i2c-1", &bus);
uint8_t cmd = 0x40;
uint16_t slave_addr = 0x80;
KI2CStatus write_status;
write_status = k_i2c_write(bus, slave_addr, &cmd, 1);
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
KI2CStatus k_i2c_write(int i2c, uint16_t addr, uint8_t *ptr, int len);

/**
 * @brief Read data over the I2C bus from specified address
 *
 * This function reads data from the specified I2C bus from the specified slave address.
 * The actual low-level I2C reading is delegated to the hardware specific kprv_i2c_*_read functions.
 * This function is intended to be used on an I2C bus which has already been initialized.
 *
 * Example usage:
 * @code
int bus = 0;
k_i2c_init("/dev/i2c-1", &bus);
uint8_t buffer[10];
int read_len = 10;
uint16_t slave_addr = 0x80;
KI2CStatus read_status;
read_status = k_i2c_read(bus, slave_addr, buffer, read_len);
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
KI2CStatus k_i2c_read(int i2c, uint16_t addr, uint8_t *ptr, int len);

#endif
/* @} */
