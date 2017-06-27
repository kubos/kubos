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
 * @defgroup MSP430F5529_HAL_I2C MSP430F5529 HAL I2C Interface
 * @addtogroup MSP430F5529_HAL_I2C
 * @{
 */

#if (defined YOTTA_CFG_HARDWARE_I2C) && (YOTTA_CFG_HARDWARE_I2C_COUNT > 0)
#include "kubos-hal/i2c.h"
#include "msp430f5529-hal/i2c.h"
#include <msp430.h>

/**
  * Helper function to convert I2C bus option.
  * @param[in] i2c number of I2C bus
  * @return hal_i2c_bus HAL I2C bus number
  */
static inline hal_i2c_bus i2c_bus(KI2CNum i2c)
{
    switch(i2c)
    {
        case K_I2C1: return HAL_I2C_B0;
        case K_I2C2: return HAL_I2C_B1;
        default: return 0;
    }
}

/**
  * Helper function to get I2C handle
  * @param[in] i2c number of I2C bus
  * @return hal_i2c_handle* pointer to I2C handle, NULL if number is invalid
  */
static inline hal_i2c_handle * i2c_handle(KI2CNum i2c)
{
    switch(i2c)
    {
        case K_I2C1: return &hal_i2c_buses[HAL_I2C_B0];
        case K_I2C2: return &hal_i2c_buses[HAL_I2C_B1];
        default: return NULL;
    }
}

/**
  * Helper function to convert I2C addressing option from msp-specific hal to generic hal.
  * @param[in] mode i2c addressing mode option
  * @return hal_i2c_addressing_mode HAL specific I2C addressing mode option
  */
static inline hal_i2c_addressing_mode i2c_addressing(I2CAddressingMode mode)
{
    switch(mode)
    {
        case K_ADDRESSINGMODE_7BIT: return HAL_I2C_ADDRESSINGMODE_7BIT;
        case K_ADDRESSINGMODE_10BIT: return HAL_I2C_ADDRESSINGMODE_10BIT;
        default: return 0;
    }
}

/**
  * Helper function to convert I2C role option from msp-specific hal to generic hal.
  * @param[in] role i2c bus role
  * @return hal_i2c_role HAL specific I2C role option
  */
static inline hal_i2c_role i2c_role(I2CRole role)
{
    switch(role)
    {
        case K_MASTER: return HAL_I2C_MASTER;
        case K_SLAVE: return HAL_I2C_SLAVE;
        default: return 0;
    }
}

/**
 * Setup and enable I2C bus
 * @param[in] i2c I2C bus to initialize
 * @return KI2CStatus I2C_OK if success, otherwise a specific error flag
 */
KI2CStatus kprv_i2c_dev_init(KI2CNum i2c)
{
    KI2C *k_i2c = kprv_i2c_get(i2c);
    if (k_i2c == NULL)
    {
        return I2C_ERROR_NULL_HANDLE;
    }

    /*
     * We don't currently support slave mode.  Initialization should fail
     * if it's specified.
     * Remove this once slave mode is implemented.
     */
    if(k_i2c->conf.role == K_SLAVE)
    {
        return I2C_ERROR_CONFIG;
    }

    hal_i2c_config config = {
            .addressing_mode = i2c_addressing(k_i2c->conf.addressing_mode),
            .clock_speed = k_i2c->conf.clock_speed,
            .role = i2c_role(k_i2c->conf.role)
    };

    hal_i2c_handle * handle = hal_i2c_init(config, i2c_bus(i2c));
    if (handle != NULL)
    {
        handle->bus_num = i2c_bus(i2c);
        hal_i2c_setup(handle);
        return I2C_OK;
    }
    return I2C_ERROR_NULL_HANDLE;
}

/**
 * I2C hardware cleanup and disabling
 * @param[in] i2c bus num to terminate
 * @return KI2CStatus I2C_OK if success, otherwise a specific error flag
 */
KI2CStatus kprv_i2c_dev_terminate(KI2CNum i2c)
{
    hal_i2c_handle * handle = i2c_handle(i2c);
    if (handle != NULL)
    {
        hal_i2c_dev_terminate(handle);
        return I2C_OK;
    }
    return I2C_ERROR_NULL_HANDLE;
}

/**
 * Write data over I2C bus as master
 * @param[in] i2c I2C bus to write to
 * @param[in] addr I2C address to write to
 * @param[in] ptr pointer to data buffer
 * @param[in] len length of data to write
 * @return KI2CStatus I2C_OK on success, otherwise failure
 */
KI2CStatus kprv_i2c_master_write(KI2CNum i2c, uint16_t addr, uint8_t *ptr, int len)
{
    hal_i2c_status ret = HAL_I2C_ERROR;
    ret = hal_i2c_master_write(i2c_handle(i2c), addr, ptr, len);

    return (KI2CStatus)ret;
}

/**
 * Read data over I2C bus as master
 * @param[in] i2c I2C bus to read from
 * @param[in] addr I2C address to write to
 * @param[out] ptr pointer to data buffer
 * @param[in] len length of data to read
 * @return KI2CStatus I2C_OK on success, otherwise failure
 */
KI2CStatus kprv_i2c_master_read(KI2CNum i2c, uint16_t addr, uint8_t *ptr, int len)
{
    hal_i2c_status ret = HAL_I2C_ERROR;
    ret = hal_i2c_master_read(i2c_handle(i2c), addr, ptr, len);

    return (KI2CStatus)ret;
}

#endif

/* @} */
