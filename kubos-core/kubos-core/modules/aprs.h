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
  * @defgroup KUBOS_CORE_APRS Kubos Core APRS Parsing Interface
  * @addtogroup KUBOS_CORE_APRS
  * @{
  */

#ifndef APRS_H
#define APRS_H

#include <stdint.h>

#include "kubos-core/modules/ax25.h"

#define APRS_DIGI_APRS  ax25_addr_init("APRS")
#define APRS_DIGI_TCPIP ax25_addr_init("TCPIP*")
#define APRS_NOCALL     ax25_addr_init("N0CALL")

#define APRSIS_ADDRS(src_) { \
    APRS_DIGI_APRS, \
    ax25_addr_init(src_), \
    APRS_DIGI_APRS, \
    APRS_DIGI_TCPIP, \
}

#define APRSIS_ADDRS_LEN 4

// APRS position string
// POSITION  = '/{time}h{location}O{course:03.0f}/{speed:03.0f}/A={alt:06.0f}'
// /HHMMSShDDMM.MMN/DDDMM.MMNOCCC/SSS/A=AAAAAA

#define APRS_POSITION_LEN 43

typedef struct aprs_position_s {
    uint8_t hour;
    uint8_t minute;
    uint8_t second;
    float latitude; // --90 -> 90 degrees
    float longitude; // -180 -> 180 degrees
    int16_t course; // degrees 1-360 clockwise from due north
    int16_t speed; // in knots
    int32_t altitude; // in feet
} aprs_position_t;

int aprs_position_format(char* dest, aprs_position_t *position);
k_buffer_t *aprs_position_build(aprs_position_t *position);

// APRS telemetry string
// TELEMETRY = 'T#{id:03d},{r1:03d},{r2:03d},{r3:03d},{r4:03d},{r5:03d},{d:08b}'
// T#PPP,RRR,RRR,RRR,RRR,RRR,DDDDDDDD

#define APRS_TELEMETRY_LEN 34

#define APRS_T_ANALOG_MAXLENS  { 7, 6, 5, 5, 4 }
#define APRS_T_DIGITAL_MAXLENS { 5, 4, 3, 3, 3, 2, 2, 2}

typedef struct aprs_telemetry_desc_s {
    char *name;
    char *unit;
} aprs_telemetry_desc_t;

typedef struct aprs_telemetry_s {
    uint16_t packet_id;

    aprs_telemetry_desc_t analog_desc[5];
    uint8_t analog[5];

    aprs_telemetry_desc_t digital_desc[8];
    uint8_t digital;
} aprs_telemetry_t;

void aprs_telemetry_init(aprs_telemetry_t *telemetry);
void aprs_telemetry_add_analog(aprs_telemetry_t *telemetry, int field,
                               char *name, char *unit, uint8_t init_value);
void aprs_telemetry_add_digital(aprs_telemetry_t *telemetry, int bit,
                                char *name, char *unit, uint8_t bit_value);

int aprs_telemetry_format(char* dest, aprs_telemetry_t *telemetry);
k_buffer_t *aprs_telemetry_build(aprs_telemetry_t *telemetry);

// APRS Telemetry Parameter Name message
// :CCCCCCCCC:PARM.F1,F2,F3,F4,F5,B1,B2,B3,B4,B5,B6,B7,B8

int aprs_telemetry_params_format(char *dest, char *callsign,
                                 aprs_telemetry_t *telemetry);
int aprs_telemetry_units_format(char *dest, char *callsign,
                                aprs_telemetry_t *telemetry);
#endif

/* @} */
