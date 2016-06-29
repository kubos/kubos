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
/**
 * @defgroup SENSORS
 * @addtogroup SENSORS
 * @{
 */
/**
 * @brief HTU21D Temperature and Humidty Sensor
 */
#ifdef YOTTA_CFG_SENSORS_HTU21D
#ifndef HTU21D_H
#define HTU21D_H

#include "kubos-hal/i2c.h"

/**
 * I2C bus that the sensor is wired into. Defined in the application
 * config.json file
 */
#ifndef I2C_BUS
#define I2C_BUS YOTTA_CFG_SENSORS_HTU21D_I2C_BUS
#endif

/**
 * Setup the I2C interface for talking with the HTU21D
 */
void htu21d_setup(void);

/**
 * Sends temperature command and reads back temperature data
 * @return float temperature in celsius (-40.0 to 125.0)
 */
float htu21d_read_temperature(void);

/**
 * Sends humidity command and reads back humidity data
 * @return float relative humidity in percentage (0.0 - 100.0)
 */
float htu21d_read_humidity(void);

/**
 * Sends reset command which powers sensor off and on again
 */
void htu21d_reset(void);

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
static int read_value(uint8_t cmd);

#endif
#endif

/* @} */
