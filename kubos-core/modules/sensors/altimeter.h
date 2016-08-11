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

#ifndef ALTIMETER_H
#define ALTIMETER_H

#include "kubos-core/modules/sensors/sensors.h"

/**
 * initialize KubOS altitude sensor
 */
KSensorStatus k_initialize_altitude_sensor(void);

/**
 * @return float pressure reading in hPa (300.0 - 1100.0)
 */
KSensorStatus k_get_pressure(float * press);

/**
 * @return absolute altitude in meters
 */
KSensorStatus k_get_altitude(float * alt);

#endif
