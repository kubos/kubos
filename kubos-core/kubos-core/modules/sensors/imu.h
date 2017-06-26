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
 * @defgroup KUBOS_CORE_IMU Kubos Core IMU Sensor Interface
 * @addtogroup KUBOS_CORE_IMU
 * @{
 */

/**
 * Enabling this sensor code requires certain configuration values to be present
 * in the application's configuration json. An example is given below:
 *
 *  {
 *      "sensors": {
 *          "bno055": {
 *              "i2c_bus": "K_I2C1"
 *          }
 *      }
 *  }
 *
 * This would enable the sensor API and the bno055 sensor code and configure
 * it for the I2C bus K_I2C1.
 */
#ifndef IMU_H
#define IMU_H

#include "kubos-core/modules/sensors/sensors.h"

/**
 * 3D IMU vector
 */
typedef struct
{
    double x;
    double y;
    double z;
} k_sensor_vector_t;

/**
 * Quaternion absolute position vector
 */
typedef struct
{
    double w;
    double x;
    double y;
    double z;
} k_position_vector_t;

/**
 * Setup the IMU interface and any related sensors
 * @return KSensorStatus, SENSOR_OK on success, SENSOR_WRITE_ERROR,
 * SENSOR_NOT_FOUND or SENSOR_NOT_CALIBRATED on error
 */
KSensorStatus k_initialize_imu_sensor(void);

/**
 * Reads gyroscope data from the related sensor
 * @param gyro pointer to 3D IMU vector struct
 * @return KSensorStatus, SENSOR_OK on success, SENSOR_NOT_FOUND,
 * SENSOR_READ_ERROR, SENSOR_WRITE_ERROR on error
 */
KSensorStatus k_get_gyro(k_sensor_vector_t * gyro);

/**
 * Reads magnetometer data from the related sensor
 * @param mag pointer to 3D IMU vector struct
 * @return KSensorStatus, SENSOR_OK on success, SENSOR_NOT_FOUND,
 * SENSOR_READ_ERROR, SENSOR_WRITE_ERROR on error
 */
KSensorStatus k_get_compass(k_sensor_vector_t * mag);

/**
 * Reads accelerometer data from the related sensor
 * @param accel pointer to 3D IMU vector struct
 * @return KSensorStatus, SENSOR_OK on success, SENSOR_NOT_FOUND,
 * SENSOR_READ_ERROR, SENSOR_WRITE_ERROR on error
 */
KSensorStatus k_get_acceleration(k_sensor_vector_t * accel);

/**
 * Computes absolute position in a quaternion vector using sensor fusion
 * @param pos pointer to quaternion position struct
 * @return KSensorStatus, SENSOR_OK on success, SENSOR_NOT_FOUND,
 * SENSOR_READ_ERROR, SENSOR_WRITE_ERROR on error
 */
KSensorStatus k_get_absolute_position(k_position_vector_t * pos);

#endif

/* @} */
