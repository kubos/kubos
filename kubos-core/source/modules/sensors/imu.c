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

#ifdef YOTTA_CFG_SENSORS

#include "kubos-core/modules/sensors/imu.h"

#ifdef YOTTA_CFG_SENSORS_BNO055
#include "kubos-core/modules/sensors/bno055.h"
#endif

KSensorStatus k_initialize_imu_sensor(void)
{
    KSensorStatus ret;
#ifdef YOTTA_CFG_SENSORS_BNO055
    ret = bno055_setup(OPERATION_MODE_NDOF);
#else
    ret = SENSOR_NOT_FOUND;
#endif

    return ret;
}

KSensorStatus k_get_gyro(k_sensor_vector_t * gyro)
{
    KSensorStatus ret;

#ifdef YOTTA_CFG_SENSORS_BNO055
    ret = bno055_get_data_vector(VECTOR_GYROSCOPE, (bno055_vector_data_t *) gyro);
#else
    ret = SENSOR_NOT_FOUND;
#endif

    return ret;
}

KSensorStatus k_get_compass(k_sensor_vector_t * mag)
{
    KSensorStatus ret;

#ifdef YOTTA_CFG_SENSORS_BNO055
    ret = bno055_get_data_vector(VECTOR_MAGNETOMETER, (bno055_vector_data_t *) mag);
#else
    ret = SENSOR_NOT_FOUND;
#endif

    return ret;
}

KSensorStatus k_get_acceleration(k_sensor_vector_t * accel)
{
    KSensorStatus ret;

#ifdef YOTTA_CFG_SENSORS_BNO055
    ret = bno055_get_data_vector(VECTOR_ACCELEROMETER, (bno055_vector_data_t *) accel);
#else
    ret = SENSOR_NOT_FOUND;
#endif

    return ret;
}

KSensorStatus k_get_absolute_position(k_position_vector_t * pos)
{
    KSensorStatus ret;

#ifdef YOTTA_CFG_SENSORS_BNO055
    ret = bno055_get_position((bno055_quat_data_t *) pos);
#else
    ret = SENSOR_NOT_FOUND;
#endif

    return ret;
}

#endif
