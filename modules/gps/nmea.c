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
#include <errno.h>
#include <math.h>
#include <stdlib.h>
#include <string.h>

#include "nmea.h"
#include "gps.h"

static uint8_t parse_hex(char c)
{
    if (c < '0')
        return 0;

    if (c <= '9')
        return c - '0';

    if (c < 'A')
        return 0;

    if (c <= 'F')
        return (c - 'A') + 10;

    return 0;
}

static void parse_timestamp(char *ts, gps_fix_t *fix)
{
    char buf[4];
    strncpy(buf, ts, 2);
    buf[2] = '\0';
    fix->hour = (uint8_t) atoi(buf);

    strncpy(buf, ts + 2, 2);
    buf[2] = '\0';
    fix->minute = (uint8_t) atoi(buf);

    strncpy(buf, ts + 4, 2);
    buf[2] = '\0';
    fix->seconds = (uint8_t) atoi(buf);

    strncpy(buf, ts + 7, 3);
    buf[3] = '\0';
    fix->milliseconds = (uint16_t) atoi(buf);
}

static float parse_latlon(char *latlon)
{
    float coords = atof(latlon);
    int deg = coords / 100;
    float dec = (coords / 100.0f - deg) * 100;

    dec /= 60;
    dec += deg;

    return dec;
}

#define TIMESTAMP     1
#define GPRMC_FIXTYPE 2
#define GPRMC_LAT     3
#define GPRMC_LATDIR  4
#define GPRMC_LON     5
#define GPRMC_LONDIR  6
#define GPRMC_SPEED   7
#define GPRMC_ANGLE   8
#define GPRMC_DATE    9

#define GPGGA_LAT     2
#define GPGGA_LATDIR  3
#define GPGGA_LON     4
#define GPGGA_LONDIR  5
#define GPGGA_FIXQUAL 6
#define GPGGA_SATS    7
#define GPGGA_HDOP    8
#define GPGGA_ALT     9
#define GPGGA_GEOID   10

static int parse_gprmc(uint8_t field, char *value, gps_fix_t *fix)
{
    uint32_t date;

    switch (field) {
        case TIMESTAMP:
            parse_timestamp(value, fix);
            break;

        case GPRMC_FIXTYPE:
            if (*value != 'A') {
                return NMEA_ERR_INVALID_FIX;
            }
            break;

        case GPRMC_LAT:
            fix->latitude = parse_latlon(value);
            break;

        case GPRMC_LATDIR:
            if (*value == 'S') fix->latitude *= -1.0;
            break;

        case GPRMC_LON:
            fix->longitude = parse_latlon(value);
            break;

        case GPRMC_LONDIR:
            if (*value == 'W') fix->longitude *= -1.0;
            break;

        case GPRMC_SPEED:
            fix->speed = value ? atof(value): 0;
            break;

        case GPRMC_DATE:
            date = atoi(value);
            fix->day = date / 10000;
            fix->month = (date % 10000) / 100;
            fix->year = (date % 100);
            break;
    }

    return NMEA_OK;
}

static int parse_gpgga(uint8_t field, char *value, gps_fix_t *fix)
{
    switch (field) {
        case TIMESTAMP:
            parse_timestamp(value, fix);
            break;

        case GPGGA_LAT:
            fix->latitude = parse_latlon(value);
            break;

        case GPGGA_LATDIR:
            if (*value == 'S') fix->latitude *= -1.0;
            break;

        case GPGGA_LON:
            fix->longitude = parse_latlon(value);
            break;

        case GPGGA_LONDIR:
            if (*value == 'W') fix->longitude *= -1.0;
            break;

        case GPGGA_ALT:
            fix->altitude = value ? atof(value) : 0;
            break;
    }

    return NMEA_OK;
}

int nmea_parse(char *nmea, int len, gps_fix_t *fix)
{
    char str[128], *token;
    uint8_t i = 0, type;

    strncpy(str, nmea, len);
    str[len > 127 ? 127 : len] = '\0';

    // trim newlines
    while (str[len - 1] == '\r' || str[len - 1] == '\n') {
        str[len--] = '\0';
    }

    // validate checksum
    if (str[len - 3] == '*') {
        uint16_t sum = parse_hex(str[len - 2]) * 16;
        sum += parse_hex(str[len - 1]);

        // check checksum
        for (i = 1; i < len - 3; i++) {
            sum ^= str[i];
        }
        if (sum != 0) {
            // checksum mismatch
            return NMEA_ERR_CHECKSUM;
        }
    }

    str[len - 3] = '\0';
    len -= 3;

    for (token = strtok(str, ","), i = 0;
         token;
         token = strtok(NULL, ","), i++) {

        if (i == 0) {
            if (strncmp("$GPRMC", token, 6) == 0) {
                type = NMEA_GPRMC;
            } else if (strncmp("$GPGGA", token, 6) == 0) {
                type = NMEA_GPGGA;
            } else {
                return NMEA_ERR_UNKNOWN;
            }
            continue;
        }

        int result = NMEA_OK;
        switch (type) {
            case NMEA_GPRMC:
                result = parse_gprmc(i, token, fix);
                break;
            case NMEA_GPGGA:
                result = parse_gpgga(i, token, fix);
                break;
        }

        if (result != NMEA_OK) {
            return result;
        }
    }

    return NMEA_OK;
}
