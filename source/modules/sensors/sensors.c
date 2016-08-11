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

#include "kubos-core/modules/sensors/sensors.h"

#ifdef YOTTA_CFG_SENSORS_BME280
#include "kubos-core/modules/sensors/bme280.h"
#endif
#ifdef YOTTA_CFG_SENSORS_HTU21D
#include "kubos-core/modules/sensors/htu21d.h"
#endif
#ifdef YOTTA_CFG_SENSORS_BNO055
#include "kubos-core/modules/sensors/bno055.h"
#endif

/* globals */
static KSensorStatus _sensor_status;

KSensorStatus k_initialize_sensors(void)
{
    KSensorStatus ret;
#ifdef YOTTA_CFG_SENSORS_BME280
    ret = bme280_setup();
#endif
#ifdef YOTTA_CFG_SENSORS_HTU21D
    htu21d_setup();
#endif
#ifdef YOTTA_CFG_SENSORS_BNO055
    ret = bno055_setup(OPERATION_MODE_NDOF);
#endif

    _sensor_status = ret;
    return ret;
}

float k_get_temperature(void)
{
    KSensorStatus ret;
    float temp;

#ifdef YOTTA_CFG_SENSORS_HTU21D
    ret = htu21d_read_temperature(&temp);
#else
    ret = SENSOR_NOT_FOUND;
    temp = -1;
#endif

    _sensor_status = ret;
    return temp;
}

float k_get_humidity(void)
{
    KSensorStatus ret;
    float hum;

#ifdef YOTTA_CFG_SENSORS_HTU21D
    ret = htu21d_read_humidity(&hum);
#else
    ret = SENSOR_NOT_FOUND;
    hum = -1;
#endif

    _sensor_status = ret;
    return hum;
}

float k_get_pressure(void)
{
    KSensorStatus ret;
    float press;

#ifdef YOTTA_CFG_SENSORS_BME280
    ret = SENSOR_OK;
    press = bme280_read_pressure();
#else
    ret = SENSOR_NOT_FOUND;
    press = -1;
#endif

    _sensor_status = ret;
    return press;
}

float k_get_altitude(void)
{
    KSensorStatus ret;
    float alt;

#ifdef YOTTA_CFG_SENSORS_BME280
    ret = SENSOR_OK;
    alt = bme280_read_altitude(1013.25);
#else
    ret = SENSOR_NOT_FOUND;
    alt = -1;
#endif

    _sensor_status = ret;
    return alt;
}

k_sensor_vector_t k_get_gyro(void)
{
    KSensorStatus ret;
    k_sensor_vector_t gyro;

#ifdef YOTTA_CFG_SENSORS_BNO055
    ret = bno055_get_data_vector(VECTOR_GYROSCOPE, (bno055_vector_data_t*) &gyro);
#else
    ret = SENSOR_NOT_FOUND;
    gyro.x = -1;
    gyro.y = -1;
    gyro.z = -1;
#endif

    _sensor_status = ret;
    return gyro;
}

k_sensor_vector_t k_get_compass(void)
{
    KSensorStatus ret;
    k_sensor_vector_t mag;

#ifdef YOTTA_CFG_SENSORS_BNO055
    ret = bno055_get_data_vector(VECTOR_MAGNETOMETER, (bno055_vector_data_t*) &mag);
#else
    ret = SENSOR_NOT_FOUND;
    mag.x = -1;
    mag.y = -1;
    mag.z = -1;
#endif

    _sensor_status = ret;
    return mag;
}

k_sensor_vector_t k_get_acceleration(void)
{
    KSensorStatus ret;
    k_sensor_vector_t accel;
#ifdef YOTTA_CFG_SENSORS_BNO055
    ret = bno055_get_data_vector(VECTOR_ACCELEROMETER, (bno055_vector_data_t*) &accel);
#else
    ret = SENSOR_NOT_FOUND;
    accel.x = -1;
    accel.y = -1;
    accel.z = -1;
#endif

    _sensor_status = ret;
    return accel;
}

k_position_vector_t k_get_absolute_position(void)
{
    KSensorStatus ret;
    k_position_vector_t pos;
#ifdef YOTTA_CFG_SENSORS_BNO055
    ret = bno055_get_position((bno055_quat_data_t*) &pos);
#else
    ret = SENSOR_NOT_FOUND;
    pos.w = -1;
    pos.x = -1;
    pos.y = -1;
    pos.z = -1;
    return pos;
#endif

    _sensor_status = ret;
    return pos;
}

KSensorStatus k_sensor_status(void)
{
    /* retreive global status */
    return _sensor_status;
}

#endif
