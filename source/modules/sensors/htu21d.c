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

static uint8_t sensor_addr = 0x40;
static uint8_t read_temp_cmd = 0xE3;
static uint8_t read_hum_cmd = 0xE5;
static uint8_t reset_cmd = 0xFE;
static uint8_t read_reg_cmd = 0xE7;

void htu21d_setup(void)
{
    KI2CConf conf = {
        .addressing_mode = K_ADDRESSINGMODE_7BIT,
        .role = K_MASTER,
        .clock_speed = 10000
    };
    k_i2c_init(I2C_BUS, &conf);
}

float htu21d_read_temperature(void)
{
    float temp = 0;
    int raw;
    raw = read_value(read_temp_cmd);
    if (raw != 0)
    {
        temp = raw;
        temp *= 175.72;
        temp /= 65536;
        temp -= 46.85;
    }
    return temp;
}

float htu21d_read_humidity(void)
{
    float hum = 0;
    int raw;

    raw = read_value(read_hum_cmd);
    if (raw != 0)
    {
        hum = raw;
        hum *= 125;
        hum /= 65536;
        hum -= 6;
    }
    return hum;
}

void htu21d_reset(void)
{
    k_i2c_write(I2C_BUS, sensor_addr, &reset_cmd, 1);
    vTaskDelay(50);
}

static int read_value(uint8_t cmd)
{
    uint8_t buffer[3];
    int raw;
    int bit0, bit1;
    memset(buffer, '\0', 3);

    if (k_i2c_write(I2C_BUS, sensor_addr, &cmd, 1) != I2C_OK)
    {
        return 0;
    }
    vTaskDelay(100);
    if (k_i2c_read(I2C_BUS, sensor_addr, &buffer, 3) != I2C_OK)
    {
        return 0;
    }

    bit0 = (int)buffer[0];
    bit1 = (int)buffer[1];
    raw = (bit0 << 8) | bit1;

    if (check_crc(raw, (int)buffer[2]) != 0)
    {
        return 0;
    }

    return raw;
}

static int check_crc(uint16_t raw_temp, uint8_t crc)
{
    uint32_t rem = (uint32_t)raw_temp << 8;
    rem |= crc;

    uint32_t divsor = (uint32_t)0x988000;

    for (uint8_t i = 0; i < 16; i++)
    {
        if (rem & (uint32_t)1<<(23 - i))
        {
            rem ^= divsor;
        }
        divsor >>= 1;
    }

    return (int)rem;
}

#endif
