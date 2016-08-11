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
 * @defgroup HTU21D HTU21D Temperature and Humidity Sensor
 * @addtogroup HTU21D
 * @{
 */

/**
 *
 * @file       htu21d.h
 * @brief HTU21D Temperature and Humidity Sensor
 *
 * Enabling this sensor code requires certain configuration values to be present
 * in the application's configuration json. An example is given below:
 *
 *  {
 *      "sensors": {
 *          "htu21d": {
 *              "i2c_bus": "K_I2C1"
 *          }
 *      }
 *  }
 *
 * This would enable the htu21d sensor code and configure it for the I2C bus K_I2C1.
 */

#ifdef YOTTA_CFG_SENSORS_HTU21D
#ifndef HTU21D_H
#define HTU21D_H

#include "kubos-hal/i2c.h"
#include "kubos-core/modules/sensors/sensors.h"

/**
 * Setup the I2C interface for talking with the HTU21D
 */
KSensorStatus htu21d_setup(void);

/**
 * Sends temperature command and reads back temperature data
 * @return float temperature in celsius (-40.0 to 125.0)
 */
KSensorStatus htu21d_read_temperature(float * temp);

/**
 * Sends humidity command and reads back humidity data
 * @return float relative humidity in percentage (0.0 - 100.0)
 */
KSensorStatus htu21d_read_humidity(float * hum);

/**
 * Sends reset command which powers sensor off and on again
 */
KSensorStatus htu21d_reset(void);

#endif
#endif

/* @} */
