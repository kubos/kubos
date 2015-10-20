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
#include <math.h>
#include <stdbool.h>
#include <stdio.h>
#include <string.h>
#include <stdint.h>
#include <stdlib.h>

#include "aprs.h"

#define TYPE_LAT 0
#define TYPE_LON 1

int aprs_format_latlon_dm(char *dest, float dec_degrees, uint8_t type)
{
    uint16_t degrees = abs((int) dec_degrees);
    float minutes = fabsf(((int)dec_degrees) - dec_degrees) * 60;
    char suffix;

    switch (type) {
        case TYPE_LAT:
            suffix = dec_degrees >= 0 ? 'N' : 'S';
            return sprintf(dest, "%02d%05.2f%c", degrees, minutes, suffix);
        case TYPE_LON:
        default:
            suffix = dec_degrees >= 0 ? 'E' : 'W';
            return sprintf(dest, "%03d%05.2f%c", degrees, minutes, suffix);
    }
}

int aprs_position_format(char* dest, aprs_position_t *position)
{
    char latitude_str[16], longitude_str[16];
    if (!position || !dest) {
        return -1;
    }

    aprs_format_latlon_dm(latitude_str, position->latitude, TYPE_LAT);
    aprs_format_latlon_dm(longitude_str, position->longitude, TYPE_LON);

    return sprintf(dest,
        "/%02d%02d%02dh%s/%sO%03d/%03d/A=%06d",
        position->hour, position->minute, position->second,
        latitude_str, longitude_str,
        position->course, position->speed, position->altitude);
}

gnrc_pktsnip_t *aprs_position_build(aprs_position_t *position)
{
    gnrc_pktsnip_t *pkt = gnrc_pktbuf_add(NULL, NULL, APRS_POSITION_LEN,
                                          GNRC_NETTYPE_UNDEF);

    aprs_position_format((char *) pkt->data, position);
    return pkt;
}

void aprs_telemetry_init(aprs_telemetry_t *telemetry)
{
    int i;
    if (!telemetry) {
        return;
    }

    telemetry->packet_id = 0;
    telemetry->digital = 0;

    for (i = 0; i < 5; i++) {
        telemetry->analog_desc[i].name = NULL;
        telemetry->analog_desc[i].unit = NULL;
        telemetry->analog[i] = 0;
    }

    for (i = 0; i < 8; i++) {
        telemetry->digital_desc[i].name = NULL;
        telemetry->digital_desc[i].unit = NULL;
    }
}

void aprs_telemetry_add_analog(aprs_telemetry_t *telemetry, int field,
                               char *name, char *unit, uint8_t init_value)
{
    if (!telemetry || field > 4 || field < 0) {
        return;
    }

    telemetry->analog_desc[field].name = name;
    telemetry->analog_desc[field].unit = unit;
    telemetry->analog[field] = init_value;
}

void aprs_telemetry_add_digital(aprs_telemetry_t *telemetry, int bit,
                                char *name, char *unit, uint8_t bit_value)
{
    if (!telemetry || bit > 7 || bit < 0) {
        return;
    }

    telemetry->digital_desc[bit].name = name;
    telemetry->digital_desc[bit].unit = unit;
    telemetry->digital |= (bit_value << bit);
}

int aprs_telemetry_format(char* dest, aprs_telemetry_t *telemetry)
{
    uint8_t b;
    char d_bits[9];
    char *p = d_bits;

    d_bits[8] = '\0';

    for (b = 0x80; b > 0; b >>= 1) {
        *p++ = ((telemetry->digital & b) == b) ? '1' : '0';
    }

    return sprintf(dest,
        "T#%03d,%03d,%03d,%03d,%03d,%03d,%s",
        telemetry->packet_id, telemetry->analog[0], telemetry->analog[1],
        telemetry->analog[2], telemetry->analog[3], telemetry->analog[4],
        d_bits);

}

gnrc_pktsnip_t *aprs_telemetry_build(aprs_telemetry_t *telemetry)
{
    gnrc_pktsnip_t *pkt = gnrc_pktbuf_add(NULL, NULL, APRS_POSITION_LEN,
                                          GNRC_NETTYPE_UNDEF);

    aprs_telemetry_format((char *) pkt->data, telemetry);
    return pkt;
}

#define APRS_DESC_NAME 0
#define APRS_DESC_UNIT 1

int aprs_append_desc(char *dest, char *param, uint8_t maxlen, uint8_t comma)
{
    int size = 0;
    if (comma) {
        maxlen--;
        strcat(dest, ",");
        dest++;
        size++;
    }

    if (param) {
        int param_len = strlen(param);
        size += param_len > maxlen ? maxlen : param_len;
        strncat(dest, param, maxlen);
    }

    return size;
}

static int _format_desc(char *dest, char *callsign, uint8_t desc_type,
                        aprs_telemetry_t *telemetry)
{
    int size = sprintf(dest, ":%-9s:%s.", callsign,
                       desc_type == APRS_DESC_NAME ? "PARM" : "UNIT");

    uint8_t i = 0, comma = false;
    uint8_t analog_maxlens[] = APRS_T_ANALOG_MAXLENS;
    uint8_t digital_maxlens[] = APRS_T_DIGITAL_MAXLENS;

    for (; i < 13; i++) {
        aprs_telemetry_desc_t *desc;
        uint8_t maxlen;
        char *param;

        if (i < 5) {
            desc = &telemetry->analog_desc[i];
            maxlen = analog_maxlens[i];
        } else {
            desc = &telemetry->digital_desc[i-5];
            maxlen = digital_maxlens[i-5];
        }

        param = desc_type == APRS_DESC_NAME ?  desc->name : desc->unit;
        if (!param) {
            return size;
        }

        size += aprs_append_desc(dest, param, maxlen + 1, comma);
        if (!comma) {
            comma = true;
        }
    }

    return size;
}

int aprs_telemetry_params_format(char *dest, char *callsign,
                                 aprs_telemetry_t *telemetry)
{
    return _format_desc(dest, callsign, APRS_DESC_NAME, telemetry);
}

int aprs_telemetry_units_format(char *dest, char *callsign,
                                aprs_telemetry_t *telemetry)
{
    return _format_desc(dest, callsign, APRS_DESC_UNIT, telemetry);
}

