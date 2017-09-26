/*
 * KubOS Linux
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

#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include "kubos-hal/i2c.h"

#define I2C_BUS K_I2C1

#define LSM303DLHC_ADDRESS_A 0x19
#define LSM303DLHC_NAME "LSM303DLHC"

#define msleep(time_ms) usleep(time_ms * 1000);

#define POWER_DOWN 0x00
#define SPEED_1Hz 0x10
#define SPEED_10Hz 0x20
#define SPEED_25Hz 0x30
#define SPEED_50Hz 0x40
#define SPEED_100Hz 0x50
#define SPEED_200Hz 0x60
#define SPEED_400Hz 0x70
#define SPEED_L1_6kHz 0x80
#define SPEED_N1_3kHz_L5_3kHz 0x90

#define NORMAL_POWER 0x00
#define LOW_POWER 0x08

#define X_AXIS_ENABLE 0x01
#define Y_AXIS_ENABLE 0x02
#define Z_AXIS_ENABLE 0x04

#define TEST_MODE \
    SPEED_100Hz + NORMAL_POWER + X_AXIS_ENABLE + Y_AXIS_ENABLE + Z_AXIS_ENABLE

typedef enum { CTRL_REG1_A = 0x20 } LSM303DLHC_reg_t;

static int write_byte(LSM303DLHC_reg_t reg, uint8_t value)
{
    /* Write buffer: reg and write value */
    uint8_t buffer[2] = { (uint8_t) reg, value };

    /* Transmit reg and value */
    if (k_i2c_write(I2C_BUS, LSM303DLHC_ADDRESS_A, buffer, sizeof(buffer)) != I2C_OK)
    {
        printf("Write failed\n");
        return -1;
    }

    return 0;
}

static int read_byte(LSM303DLHC_reg_t reg, uint8_t * value)
{
    if (value == NULL)
    {
        return -1;
    }
    /* Transmit reg */
    if (k_i2c_write(I2C_BUS, LSM303DLHC_ADDRESS_A, (uint8_t *) &reg, 1) != I2C_OK)
    {
        printf("Write (read) failed\n");
        return -1;
    }
    msleep(5);

    /* Receive value */
    if (k_i2c_read(I2C_BUS, LSM303DLHC_ADDRESS_A, value, 1) != I2C_OK)
    {
        printf("Read failed\n");
        return -1;
    }
    return 0;
}

int init_sensor(void)
{

    /* Prep power/frequency mode */
    uint8_t mode = TEST_MODE;

    int ret = -1;

    /* Set the requested operating mode */
    if ((ret = write_byte(CTRL_REG1_A, mode)) != 0)
    {
        return ret;
    }
    msleep(20);

    /* Fetch the operating mode */
    if ((ret = read_byte(CTRL_REG1_A, &mode)) != 0)
    {
        return ret;
    }

    /* Verify that the mode received matches the mode sent */
    if (mode != TEST_MODE)
    {
        printf("Incorrect mode received: %x\n", mode);
        return -1;
    }

    printf("Mode: %x\n", mode);
    return ret;
}

int main(void)
{

    int  status = 0;
    /*
     * The iOBC only has one i2c bus, so this will always be i2c-0
     * If a future board has more than one i2c bus, change this to take
     * an input parameter
     */

    KI2CConf conf = k_i2c_conf_defaults();

    if (k_i2c_init(I2C_BUS, &conf) != I2C_OK)
    {
        return -1;
    }

    if (init_sensor() != 0)
    {
        printf("Something went wrong\n");
        status = -1;
    }

    k_i2c_terminate(I2C_BUS);

    return status;
}
