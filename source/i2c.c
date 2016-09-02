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

#if (defined YOTTA_CFG_HARDWARE_I2C) && (YOTTA_CFG_HARDWARE_I2C_COUNT > 0)

#include "kubos-hal/i2c.h"
#include <stdio.h>

static char buffer[1024];
static int buffer_len;

/**
 * Low level hal device initialization
 * This is implemented by the device specific hal
 * @param i2c i2c bus to initialize
 */
KI2CStatus kprv_i2c_dev_init(KI2CNum i2c)
{
    return I2C_OK;
}

/**
 * Low level hal i2c termination
 * This is implemented by the device specific hal
 * @param i2c i2c bus to terminate
 */
KI2CStatus kprv_i2c_dev_terminate(KI2CNum i2c)
{
    buffer_len = 0;
    *buffer = 0;
    return I2C_OK;
}

/**
 * Low level hal i2c write (as master)
 * This is implemented by the device specific hal
 * @param i2c i2c bus to write from
 * @param addr i2c addr to write to
 * @param ptr data buffer
 * @param len length of data in buffer
 * @return KI2CStatus I2C_OK on success, I2C_ERROR on error
 */
KI2CStatus kprv_i2c_master_write(KI2CNum i2c, uint16_t addr, uint8_t *ptr, int len)
{
    int i = 0;
    for (i = 0; i < len; i++)
    {
        buffer[i] = *ptr++;
    }
    buffer_len = len;
    return I2C_OK;
}

/**
 * Low level hal i2c read (as master)
 * This is implemented by the device specific hal
 * @param i2c i2c bus to read from
 * @param addr i2c addr to read from
 * @param ptr data buffer
 * @param len length of data expected to read
 * @return KI2CStatus I2C_OK on success, I2C_ERROR on error
 */
KI2CStatus kprv_i2c_master_read(KI2CNum i2c, uint16_t addr, uint8_t *ptr, int len)
{
    if (buffer_len != 0)
    {
        int i = 0;
        for (i = 0; i < len; i++)
        {
            *ptr++ = buffer[i];
        }
        buffer_len = 0;
        return I2C_OK;
    }
    return I2C_ERROR;
}

/**
 * Low level hal i2c write (as slave)
 * This is implemented by the device specific hal
 * @warning Not implemented as of v0.0.4
 * @param i2c i2c bus to write from
 * @param addr i2c addr to write to
 * @param ptr data buffer
 * @param len length of data in buffer
 * @return KI2CStatus I2C_OK on success, I2C_ERROR on error
 */
KI2CStatus kprv_i2c_slave_write(KI2CNum i2c, uint16_t addr, uint8_t *ptr, int len)
{
    return I2C_OK;
}

/**
 * Low level hal i2c read (as slave)
 * This is implemented by the device specific hal
 * @warning Not implemented as of v0.0.4
 * @param i2c i2c bus to read from
 * @param addr i2c addr to read from
 * @param ptr data buffer
 * @param len length of data expected to read
 * @return KI2CStatus I2C_OK on success, I2C_ERROR on error
 */
KI2CStatus kprv_i2c_slave_read(KI2CNum i2c, uint16_t addr, uint8_t *ptr, int len)
{
    return I2C_OK;
}

#endif
