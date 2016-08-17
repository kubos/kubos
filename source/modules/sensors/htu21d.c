/*
 * KubOS Core
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

#ifdef YOTTA_CFG_SENSORS_HTU21D

#include "kubos-core/modules/sensors/htu21d.h"
#include <string.h>
#include <task.h>

/**
 * I2C bus that the sensor is wired into. Defined in the application
 * config.json file
 */
#ifndef I2C_BUS
#define I2C_BUS YOTTA_CFG_SENSORS_HTU21D_I2C_BUS
#endif

#define SENSOR_ADDR 0x40
#define READ_HUM_CMD 0xE5
#define READ_REG_CMD 0xE7
#define READ_TEMP_CMD 0xE3


/**
 * Performs crc check on raw value and crc check value
 * @param raw raw sensor data
 * @param crc third byte sent from sensor
 * @return int 0 if check passed, otherwise check failed
 */
static int check_crc(uint16_t raw, uint8_t crc);

/**
 * Sends command, creates raw value, crc checks, returns raw
 * @param cmd single byte command to send
 * @return int 999 if crc err, 998 if comm err, otherwise raw sensor value
 */
static KSensorStatus read_value(uint8_t cmd, int * raw);

KSensorStatus htu21d_setup(void)
{
    KSensorStatus ret;
    KI2CConf conf = {
        .addressing_mode = K_ADDRESSINGMODE_7BIT,
        .role = K_MASTER,
        .clock_speed = 100000
    };
    k_i2c_init(I2C_BUS, &conf);

    /* reset sensor */
    ret = htu21d_reset();
    return ret;
}

KSensorStatus htu21d_read_temperature(float * temp)
{
    KSensorStatus ret = SENSOR_ERROR;
    int raw;
    if (temp != NULL)
    {
        if ((ret = read_value(READ_TEMP_CMD, &raw)) == SENSOR_OK)
        {
            *temp = raw;
            *temp *= 175.72;
            *temp /= 65536;
            *temp -= 46.85;
        }
    }
    return ret;
}

KSensorStatus htu21d_read_humidity(float * hum)
{
    KSensorStatus ret = SENSOR_ERROR;
    int raw;
    if (hum != NULL)
    {
        if ((ret = read_value(READ_HUM_CMD, &raw)) == SENSOR_OK)
        {
            *hum = raw;
            *hum *= 125;
            *hum /= 65536;
            *hum -= 6;
        }
    }
    return ret;
}

KSensorStatus htu21d_reset(void)
{
    uint8_t reset_cmd = 0xFE;
    if (k_i2c_write(I2C_BUS, SENSOR_ADDR, &reset_cmd, 1) != I2C_OK)
    {
        return SENSOR_WRITE_ERROR;
    }
    vTaskDelay(10);

    return SENSOR_OK;
}

static KSensorStatus read_value(uint8_t cmd, int * raw)
{
    uint8_t buffer[3];
    int bit0, bit1;
    memset(buffer, '\0', 3);
    if (raw == NULL)
    {
        return SENSOR_ERROR;
    }
    if (k_i2c_write(I2C_BUS, SENSOR_ADDR, &cmd, 1) != I2C_OK)
    {
        return SENSOR_WRITE_ERROR;
    }
    vTaskDelay(30);
    if (k_i2c_read(I2C_BUS, SENSOR_ADDR, buffer, 3) != I2C_OK)
    {
        return SENSOR_READ_ERROR;
    }

    bit0 = (int)buffer[0];
    bit1 = (int)buffer[1];
    *raw = (bit0 << 8) | bit1;

    if (check_crc(*raw, (int)buffer[2]) != 0)
    {
        return SENSOR_ERROR;
    }
    return SENSOR_OK;
}

static int check_crc(uint16_t raw_temp, uint8_t crc)
{
    uint32_t divisor = (uint32_t)0x988000;
    uint32_t rem = (uint32_t)raw_temp << 8;
    rem |= crc;

    for (uint8_t i = 0; i < 16; i++)
    {
        if (rem & (uint32_t)1<<(23 - i))
        {
            rem ^= divisor;
        }
        divisor >>= 1;
    }

    return (int)rem;
}

#endif
