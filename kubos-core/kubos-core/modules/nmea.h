/*
 * KubOS Core Flight Services
 * Copyright (C) 2015 Kubos Corporation
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
  * @defgroup KUBOS_CORE_NMEA Kubos Core NMEA Parsing Interface
  * @addtogroup KUBOS_CORE_NMEA
  * @{
  */

#ifndef NMEA_H
#define NMEA_H

#define NMEA_OK               0
#define NMEA_ERR_CHECKSUM    -1
#define NMEA_ERR_INVALID_FIX -2
#define NMEA_ERR_INVALID_DIR -3
#define NMEA_ERR_UNKNOWN     -4

#define NMEA_UNKNOWN 0
#define NMEA_GPRMC   1
#define NMEA_GPGGA   2

#include "kubos-core/modules/gps.h"

int nmea_parse(char *nmea, int length, gps_fix_t *fix);

#endif // NMEA_H

/* @} */
