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
 * @defgroup LINUX_HAL_I2C KubOS Linux HAL I2C Interface
 * @addtogroup LINUX_HAL_I2C
 * @{
 */

#if (defined YOTTA_CFG_HARDWARE_I2C) && (YOTTA_CFG_HARDWARE_I2C_COUNT > 0)

#include <errno.h>
#include <fcntl.h>
#include <linux/i2c-dev.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/ioctl.h>
#include <sys/types.h>
#include <unistd.h>

#include "kubos-hal/i2c.h"

 #define STRINGIFY(s) TOSTRING(s)
 #define TOSTRING(s) #s

/**
 * Static array of I2C bus file descriptors
 */
static int hal_i2c_bus[K_NUM_I2CS];

/**
 * Low level hal device initialization
 * @param i2c I2C bus to initialize
 * @return KI2CStatus I2C_OK on success, I2C_ERROR_* on error
 */
KI2CStatus kprv_i2c_dev_init(KI2CNum i2c)
{
    if (i2c == K_I2C_NO_BUS)
    {
        return I2C_ERROR;
    }

    if (hal_i2c_bus[i2c - 1] != 0)
    {
        fprintf(stderr, "I2C bus %d already initialized!\n", i2c);
        return I2C_ERROR;
    }

    char filename[] = "/dev/i2c-n";

    switch(i2c)
    {
#ifdef YOTTA_CFG_HARDWARE_I2C_I2C1
        case K_I2C1:
            sprintf(filename, STRINGIFY(YOTTA_CFG_HARDWARE_I2C_I2C1_DEVICE));
            break;
#endif
#ifdef YOTTA_CFG_HARDWARE_I2C_I2C2
        case K_I2C2:
            sprintf(filename, STRINGIFY(YOTTA_CFG_HARDWARE_I2C_I2C2_DEVICE));
            break;
#endif
#ifdef YOTTA_CFG_HARDWARE_I2C_I2C3
        case K_I2C3:
            sprintf(filename, STRINGIFY(YOTTA_CFG_HARDWARE_I2C_I2C3_DEVICE));
            break;
#endif
        default:
            fprintf(stderr, "Error: Bus number unknown\n");
            return I2C_ERROR_CONFIG;
    }

    hal_i2c_bus[i2c - 1] = open(filename, O_RDWR);

    if (hal_i2c_bus[i2c - 1] < 0)
    {
        perror("Couldn't open I2C bus");
        hal_i2c_bus[i2c - 1] = 0;
        return I2C_ERROR_CONFIG;
    }

    return I2C_OK;
}

/**
 * Low level HAL I2C termination
 * @param i2c I2C bus to terminate
 * @return KI2CStatus I2C_OK on success, I2C_ERROR on error
 */
KI2CStatus kprv_i2c_dev_terminate(KI2CNum i2c)
{
    if (i2c == K_I2C_NO_BUS || hal_i2c_bus[i2c - 1] == 0)
    {
        return I2C_ERROR;
    }

    close(hal_i2c_bus[i2c - 1]);
    hal_i2c_bus[i2c - 1] = 0;

    return I2C_OK;
}

/**
 * Low level HAL I2C write (as master)
 * @param i2c I2C bus to write from
 * @param addr I2C addr to write to
 * @param ptr data buffer
 * @param len length of data in buffer
 * @return KI2CStatus I2C_OK on success, I2C_ERROR on error
 */
KI2CStatus kprv_i2c_master_write(KI2CNum i2c, uint16_t addr, uint8_t * ptr,
                                 int len)
{
    if (i2c == K_I2C_NO_BUS || ptr == NULL || hal_i2c_bus[i2c - 1] == 0)
    {
        return I2C_ERROR;
    }

    /* Set the desired slave's address */
    if (ioctl(hal_i2c_bus[i2c - 1], I2C_SLAVE, addr) < 0)
    {
        perror("Couldn't reach requested address");
        return I2C_ERROR_ADDR_TIMEOUT;
    }

    /* Transmit buffer */
    if (write(hal_i2c_bus[i2c - 1], ptr, len) != len)
    {
        perror("I2C write failed");
        return I2C_ERROR;
    }

    return I2C_OK;
}

/**
 * Low level HAL I2C read (as master)
 * This is implemented by the device specific hal
 * @param i2c I2C bus to read from
 * @param addr I2C addr to read from
 * @param ptr data buffer
 * @param len length of data expected to read
 * @return KI2CStatus I2C_OK on success, I2C_ERROR on error
 */
KI2CStatus kprv_i2c_master_read(KI2CNum i2c, uint16_t addr, uint8_t * ptr,
                                int len)
{
    if (i2c == K_I2C_NO_BUS || ptr == NULL || hal_i2c_bus[i2c - 1] == 0)
    {
        return I2C_ERROR;
    }

    /* Set the desired slave's address */
    if (ioctl(hal_i2c_bus[i2c - 1], I2C_SLAVE, addr) < 0)
    {
        perror("Couldn't reach requested address");
        return I2C_ERROR_ADDR_TIMEOUT;
    }

    /* Read in data */
    if (read(hal_i2c_bus[i2c - 1], ptr, len) != len)
    {
        perror("I2C read failed");
        return I2C_ERROR;
    }

    return I2C_OK;
}

/**
 * Low level HAL I2C write (as slave)
 * @warning Not currently implemented
 * @param i2c I2C bus to write from
 * @param addr I2C addr to write to
 * @param ptr data buffer
 * @param len length of data in buffer
 * @return KI2CStatus I2C_OK on success, I2C_ERROR on error
 */
KI2CStatus kprv_i2c_slave_write(KI2CNum i2c, uint16_t addr, uint8_t * ptr,
                                int len)
{
    fprintf(stderr, "Unsupported function: Kubos I2C slave write\n");
    return I2C_ERROR;
}

/**
 * Low level HAL I2C read (as slave)
 * @warning Not currently implemented
 * @param i2c I2C bus to read from
 * @param addr I2C addr to read from
 * @param ptr data buffer
 * @param len length of data expected to read
 * @return KI2CStatus I2C_OK on success, I2C_ERROR on error
 */
KI2CStatus kprv_i2c_slave_read(KI2CNum i2c, uint16_t addr, uint8_t * ptr,
                               int len)
{
    fprintf(stderr, "Unsupported function: Kubos I2C slave read\n");
    return I2C_ERROR;
}

#endif

/* @} */

