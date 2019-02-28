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

#include "i2c.h"
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

KI2CStatus k_i2c_init(char * device, int * fp)
{
    if (device == NULL || fp == NULL)
    {
        return I2C_ERROR;
    }

    char bus[] = "/dev/i2c-n\0";
    // Make sure the device name is null terminated
    snprintf(bus, 11, "%s", device);
    *fp = open(bus, O_RDWR);

    if (*fp <= 0)
    {
        perror("Couldn't open I2C bus");
        *fp = 0;
        return I2C_ERROR_CONFIG;
    }

    return I2C_OK;
}

void k_i2c_terminate(int * fp)
{
    if (fp == NULL || *fp == 0)
    {
        return;
    }

    close(*fp);
    *fp = 0;

    return;
}

KI2CStatus k_i2c_write(int i2c, uint16_t addr, uint8_t* ptr, int len)
{
    if (i2c == 0 || ptr == NULL)
    {
        return I2C_ERROR;
    }

    /* Set the desired slave's address */
    if (ioctl(i2c, I2C_SLAVE, addr) < 0)
    {
        perror("Couldn't reach requested address");
        return I2C_ERROR_ADDR_TIMEOUT;
    }

    /* Transmit buffer */
    if (write(i2c, ptr, len) != len)
    {
        perror("I2C write failed");
        return I2C_ERROR;
    }

    return I2C_OK;
}

KI2CStatus k_i2c_read(int i2c, uint16_t addr, uint8_t* ptr, int len)
{
    if (i2c == 0 || ptr == NULL)
    {
        return I2C_ERROR;
    }

    /* Set the desired slave's address */
    if (ioctl(i2c, I2C_SLAVE, addr) < 0)
    {
        perror("Couldn't reach requested address");
        return I2C_ERROR_ADDR_TIMEOUT;
    }

    /* Read in data */
    if (read(i2c, ptr, len) != len)
    {
        perror("I2C read failed");
        return I2C_ERROR;
    }

    return I2C_OK;
}
