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
 *
 * NMEA parser ported to C from Adafruit_GPS:
 *     https://github.com/adafruit/Adafruit-GPS-Library
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

int parse_nmea(char *nmea, int length, gps_fix_t *fix)
{
    uint8_t i;
    int32_t degree;
    long minutes;
    char degreebuff[10];

    if (!nmea || !fix || length <= 0) {
        return -EINVAL;
    }

    nmea[length] = '\0';
    // do checksum check
    // first look if we even have one
    if (nmea[length - 4] == '*') {
        uint16_t sum = parse_hex(nmea[length - 3]) * 16;
        sum += parse_hex(nmea[length - 2]);

        // check checksum
        for (i = 1; i < length - 4; i++) {
            sum ^= nmea[i];
        }
        if (sum != 0) {
            // checksum mismatch
            return NMEA_ERR_CHECKSUM;
        }
    }

    // look for a few common sentences
    if (strstr(nmea, "$GPGGA")) {
        // found GGA
        char *p = nmea;
        float timef;
        uint32_t time;

        // get time
        p = strchr(p, ',')+1;
        timef = atof(p);
        time = timef;

        fix->hour = time / 10000;
        fix->minute = (time % 10000) / 100;
        fix->seconds = (time % 100);
        fix->milliseconds = fmod(timef, 1.0) * 1000;

        // parse out latitude
        p = strchr(p, ',') + 1;
        if (',' != *p)
        {
            strncpy(degreebuff, p, 2);
            p += 2;
            degreebuff[2] = '\0';
            degree = atol(degreebuff) * 10000000;
            strncpy(degreebuff, p, 2); // minutes
            p += 3; // skip decimal point
            strncpy(degreebuff + 2, p, 4);
            degreebuff[6] = '\0';
            minutes = 50 * atol(degreebuff) / 3;
            //latitude_fixed = degree + minutes;
            float latitude = degree / 100000 + minutes * 0.000006F;
            fix->latitude = (latitude-100*(int)(latitude/100))/60.0;
            fix->latitude += (int)(latitude/100);
        }

        p = strchr(p, ',') + 1;
        if (',' != *p)
        {
            if (p[0] == 'S') fix->latitude *= -1.0;
            /*if (p[0] == 'N') lat = 'N';
            else if (p[0] == 'S') lat = 'S';
            else if (p[0] == ',') lat = 0;
            else return false;*/
        }

        // parse out longitude
        p = strchr(p, ',') + 1;
        if (',' != *p)
        {
            strncpy(degreebuff, p, 3);
            p += 3;
            degreebuff[3] = '\0';
            degree = atol(degreebuff) * 10000000;
            strncpy(degreebuff, p, 2); // minutes
            p += 3; // skip decimal point
            strncpy(degreebuff + 2, p, 4);
            degreebuff[6] = '\0';
            minutes = 50 * atol(degreebuff) / 3;
            //longitude_fixed = degree + minutes;
            float longitude = degree / 100000 + minutes * 0.000006F;
            fix->longitude = (longitude-100*(int)(longitude/100))/60.0;
            fix->longitude += (int)(longitude/100);
        }

        p = strchr(p, ',')+1;
        if (',' != *p)
        {
            if (p[0] == 'W') fix->longitude *= -1.0;
            /*if (p[0] == 'W') lon = 'W';
            else if (p[0] == 'E') lon = 'E';
            else if (p[0] == ',') lon = 0;
            else return false;*/
        }

        p = strchr(p, ',')+1;
        if (',' != *p)
        {
            //fix->fixquality = atoi(p);
        }

        p = strchr(p, ',')+1;
        if (',' != *p)
        {
            //satellites = atoi(p);
        }

        p = strchr(p, ',')+1;
        if (',' != *p)
        {
            //HDOP = atof(p);
        }

        p = strchr(p, ',')+1;
        if (',' != *p)
        {
            fix->altitude = atof(p);
        }

        p = strchr(p, ',')+1;
        p = strchr(p, ',')+1;
        if (',' != *p)
        {
            //geoidheight = atof(p);
        }
        return NMEA_OK;
    }

    if (strstr(nmea, "$GPRMC")) {
        // found RMC
        char *p = nmea;

        // get time
        p = strchr(p, ',')+1;
        float timef = atof(p);
        uint32_t time = timef;
        fix->hour = time / 10000;
        fix->minute = (time % 10000) / 100;
        fix->seconds = (time % 100);
        fix->milliseconds = fmod(timef, 1.0) * 1000;

        p = strchr(p, ',')+1;
        if (p[0] == 'A') {
            //fix = true;
        } else if (p[0] == 'V') {
            //fix = false;
        } else {
            return NMEA_ERR_INVALID_FIX;
        }

        // parse out latitude
        p = strchr(p, ',') + 1;
        if (',' != *p)
        {
            strncpy(degreebuff, p, 2);
            p += 2;
            degreebuff[2] = '\0';
            long degree = atol(degreebuff) * 10000000;
            strncpy(degreebuff, p, 2); // minutes
            p += 3; // skip decimal point
            strncpy(degreebuff + 2, p, 4);
            degreebuff[6] = '\0';
            long minutes = 50 * atol(degreebuff) / 3;
            //latitude_fixed = degree + minutes;
            float latitude = degree / 100000 + minutes * 0.000006F;
            fix->latitude = (latitude-100*(int)(latitude/100))/60.0;
            fix->latitude += (int)(latitude/100);
        }

        p = strchr(p, ',') + 1;
        if (*p != ',')
        {
            if (*p != 'S' && *p != 'N') {
                return NMEA_ERR_INVALID_DIR;
            }

            if (*p == 'S') {
                fix->latitude *= -1.0;
            }
        }

        // parse out longitude
        p = strchr(p, ',') + 1;
        if (',' != *p)
        {
            strncpy(degreebuff, p, 3);
            p += 3;
            degreebuff[3] = '\0';
            degree = atol(degreebuff) * 10000000;
            strncpy(degreebuff, p, 2); // minutes
            p += 3; // skip decimal point
            strncpy(degreebuff + 2, p, 4);
            degreebuff[6] = '\0';
            minutes = 50 * atol(degreebuff) / 3;
            //longitude_fixed = degree + minutes;
            float longitude = degree / 100000 + minutes * 0.000006F;
            fix->longitude = (longitude-100*(int)(longitude/100))/60.0;
            fix->longitude += (int)(longitude/100);
        }

        p = strchr(p, ',')+1;
        if (',' != *p)
        {
            if (p[0] == 'W') fix->longitude *= -1.0;
            /*if (p[0] == 'W') lon = 'W';
            else if (p[0] == 'E') lon = 'E';
            else if (p[0] == ',') lon = 0;
            else return false;*/
        }
        // speed
        p = strchr(p, ',')+1;
        if (',' != *p)
        {
            fix->speed = atof(p);
        }

        // angle
        p = strchr(p, ',')+1;
        if (',' != *p)
        {
            //fix->angle = atof(p);
        }

        p = strchr(p, ',')+1;
        if (',' != *p)
        {
            uint32_t fulldate = atof(p);
            fix->day = fulldate / 10000;
            fix->month = (fulldate % 10000) / 100;
            fix->year = (fulldate % 100);
        }
        // we dont parse the remaining, yet!
        return NMEA_OK;
    }

    return NMEA_UNKNOWN;
}


