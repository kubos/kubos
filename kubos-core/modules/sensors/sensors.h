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

#ifndef SENSORS_H
#define SENSORS_H

typedef enum
{
    SENSOR_OK = 0,
    SENSOR_ERROR,
    SENSOR_READ_ERROR,
    SENSOR_WRITE_ERROR,
    SENSOR_NOT_FOUND,
    SENSOR_NOT_CALIBRATED
} KSensorStatus;

typedef struct
{
    double x;
    double y;
    double z;
} k_sensor_vector_t;

typedef struct
{
    double w;
    double x;
    double y;
    double z;
} k_position_vector_t;

/**
 * initialize KubOS sensor interface
 */
KSensorStatus k_initialize_sensors(void);

/**
 * @return float temperature in celsius (-40.0 to 125.0)
 */
float k_get_temperature(void);

/**
 * @return float relative humidity in percentage (0.0 - 100.0)
 */
float k_get_humidity(void);

/**
 * @return float pressure reading in hPa (300.0 - 1100.0)
 */
float k_get_pressure(void);

/**
 * @return absolute altitude in meters
 */
float k_get_altitude(void);

/**
 * @return gyroscope reading in a 3D vector
 */
k_sensor_vector_t k_get_gyro(void);

/**
 * @return compass reading in a 3D vector
 */
k_sensor_vector_t k_get_compass(void);

/**
 * @return accelerometer reading in a 3D vector
 */
k_sensor_vector_t k_get_acceleration(void);

/**
 * @return absolute position calcuation in a quaternion vector
 */
k_position_vector_t k_get_absolute_position(void);

/**
 * @return status of sensor interface
 */
KSensorStatus k_sensor_status(void);

#endif
