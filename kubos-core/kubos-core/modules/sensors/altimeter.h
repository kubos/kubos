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
 * @defgroup KUBOS_CORE_ALTIMETER Kubos Core Altimeter SesnorInterface
 * @addtogroup KUBOS_CORE_ALTIMETER
 * @{
 */

/*
 * Enabling this sensor code requires certain configuration values to be present
 * in the application's configuration json. An example is given below:
 *
 *  {
 *      "sensors": {
 *          "bme280": {
 *              "spi_bus": "K_SPI1"
 *          }
 *      }
 *  }
 *
 * This would enable the sensor API and the bme280 sensor code and configure
 * it for the SPI bus K_SPI1.
 */

#ifndef ALTIMETER_H
#define ALTIMETER_H

#include "kubos-core/modules/sensors/sensors.h"

/**
 * Setup the altimeter interface and any related sensors
 * @return KSensorStatus SENSOR_OK on success, SENSOR_WRITE_ERROR or SENSOR_NOT_FOUND on error
 */
KSensorStatus k_initialize_altitude_sensor(void);

/**
 * Reads back pressure data from related sensor
 * @param press Pointer to pressure data in Pa (101325.0 - 0.0)
 * @return KSensorStatus SENSOR_OK on success, SENSOR_ERROR or SENSOR_READ_ERROR on error
 */
KSensorStatus k_get_pressure(float * press);

/**
 * Reads back altitude from related sensors
 * @param alt Pointer to altitude in meters (m)
 * @return KSensorStatus SENSOR_OK on success, SENSOR_ERROR or SENSOR_READ_ERROR on error
 */
KSensorStatus k_get_altitude(float * alt);

#endif

/* @} */
