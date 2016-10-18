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

#include "kubos-core/modules/sensors/temperature.h"

#ifdef YOTTA_CFG_SENSORS_HTU21D
#include "kubos-core/modules/sensors/htu21d.h"
#endif

KSensorStatus k_initialize_temperature_sensor(void)
{
    KSensorStatus ret;

#ifdef YOTTA_CFG_SENSORS_HTU21D
    ret = htu21d_setup();
#else
    ret = SENSOR_NOT_FOUND;
#endif

    return ret;
}

KSensorStatus k_get_temperature(float * temp)
{
    KSensorStatus ret;

#ifdef YOTTA_CFG_SENSORS_HTU21D
    ret = htu21d_read_temperature(temp);
#else
    ret = SENSOR_NOT_FOUND;
#endif

    return ret;
}

KSensorStatus k_get_humidity(float * hum)
{
    KSensorStatus ret;

#ifdef YOTTA_CFG_SENSORS_HTU21D
    ret = htu21d_read_humidity(hum);
#else
    ret = SENSOR_NOT_FOUND;
#endif

    return ret;
}

#endif
