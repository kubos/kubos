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

#ifndef IMU_H
#define IMU_H

#include "kubos-core/modules/sensors/sensors.h"

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
 * initialize KubOS imu sensor
 */
KSensorStatus k_initialize_imu_sensor(void);

/**
 * @return gyroscope reading in a 3D vector
 */
KSensorStatus k_get_gyro(k_sensor_vector_t * gyro);

/**
 * @return compass reading in a 3D vector
 */
KSensorStatus k_get_compass(k_sensor_vector_t * mag);

/**
 * @return accelerometer reading in a 3D vector
 */
KSensorStatus k_get_acceleration(k_sensor_vector_t * accel);

/**
 * @return absolute position calcuation in a quaternion vector
 */
KSensorStatus k_get_absolute_position(k_position_vector_t * pos);

#endif
