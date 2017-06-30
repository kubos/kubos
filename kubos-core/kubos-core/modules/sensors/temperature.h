/*
 * KubOS Core Flight Services
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
 * @defgroup KUBOS_CORE_TEMPERATURE Kubos Core Temperature Sensor Interface
 * @addtogroup KUBOS_CORE_TEMPERATURE
 * @{
 */

/*
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
 * This would enable the sensor API and the htu21d sensor code and configure
 * it for the I2C bus K_I2C1.
 */

#ifndef TEMPERATURE_H
#define TEMPERATURE_H

#include "kubos-core/modules/sensors/sensors.h"

/**
 * Setup the temperature sensor interface and any related sensors
 * @return KSensorStatus - SENSOR_OK on success or SENSOR_WRITE_ERROR on error
 */
KSensorStatus k_initialize_temperature_sensor(void);

/**
 * Reads back temperature data from related sensor
 * @param[out] temp pointer to temperature in celsius (-40.0 to 125.0)
 * @return KSensorStatus - SENSOR_OK on success, SENSOR_ERROR,
 * SENSOR_READ_ERROR, SENSOR_WRITE_ERROR on error
 */
KSensorStatus k_get_temperature(float * temp);

/**
 * Reads back humidity data from related sensor
 * @param[out] hum pointer to relative humidity in percentage (0.0 - 100.0)
 * @return KSensorStatus - SENSOR_OK on success, SENSOR_ERROR,
 * SENSOR_READ_ERROR, SENSOR_WRITE_ERROR on error
 */
KSensorStatus k_get_humidity(float * hum);

#endif

/* @} */
