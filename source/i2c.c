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

#include "kubos-hal/i2c.h"
#include <string.h>

static KI2C k_i2cs[K_NUM_I2CS];

void k_i2c_init(KI2CNum i2c, KI2CConf *conf)
{
    KI2C *k_i2c = kprv_i2c_get(i2c);
    // Need to prevent re-initialization
    memcpy(&k_i2c->conf, conf, sizeof(KI2CConf));

    k_i2c->bus_num = i2c;

    k_i2c->i2c_lock = xSemaphoreCreateMutex();

    kprv_i2c_dev_init(i2c);
}

void k_i2c_terminate(KI2CNum i2c)
{
    kprv_i2c_dev_terminate(i2c);
}

KI2CConf k_i2c_conf_defaults(void)
{
    return (KI2CConf) {
        .addressing_mode = K_ADDRESSINGMODE_7BIT,
        .role = K_MASTER,
        .clock_speed = 100000
    };
}

void k_i2c_default_init()
{
    KI2CConf conf = k_i2c_conf_defaults();
    k_i2c_init(DEFAULT_I2C, &conf);
}

void k_i2c_default_dev_init(KI2CNum i2c)
{
    KI2CConf conf = k_i2c_conf_defaults();
    k_i2c_init(i2c, &conf);
}

KI2CStatus k_i2c_write(KI2CNum i2c, uint16_t addr, uint8_t* ptr, int len)
{
    KI2C * ki2c = kprv_i2c_get(i2c);
    KI2CStatus ret = I2C_ERROR;
    if (ki2c->i2c_lock != NULL)
    {
        // Today...block indefinitely
        if (xSemaphoreTake(ki2c->i2c_lock, (TickType_t)portMAX_DELAY) == pdTRUE)
        {
            ret = kprv_i2c_master_write(i2c, addr, ptr, len);
            xSemaphoreGive(ki2c->i2c_lock);
        }
    }
    return ret;
}

KI2CStatus k_i2c_read(KI2CNum i2c, uint16_t addr, uint8_t* ptr, int len)
{
    KI2C * ki2c = kprv_i2c_get(i2c);
    KI2CStatus ret = I2C_ERROR;
    if (ki2c->i2c_lock != NULL)
    {
        // Today...block indefinitely
        if (xSemaphoreTake(ki2c->i2c_lock, (TickType_t)portMAX_DELAY) == pdTRUE)
        {
            ret = kprv_i2c_master_read(i2c, addr, ptr, len);
            xSemaphoreGive(ki2c->i2c_lock);
        }
    }
    return ret;
}

KI2C* kprv_i2c_get(KI2CNum i2c)
{
    return &k_i2cs[i2c];
}
