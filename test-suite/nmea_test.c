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
#include <embUnit.h>
#include <string.h>
#include <stdio.h>

#include "nmea.h"

#include "tests.h"

static void nmea_bad_checksum(void)
{
    gps_fix_t fix;
    char *sentence = "$GPRMC,,,,,,,,,,,,B*FF";

    int result = nmea_parse(sentence, strlen(sentence), &fix);
    TEST_ASSERT_EQUAL_INT(result, NMEA_ERR_CHECKSUM);
}

static void nmea_unknown(void)
{
    gps_fix_t fix;
    char *sentence = "$GPXYZ,A*21";

    int result = nmea_parse(sentence, strlen(sentence), &fix);
    TEST_ASSERT_EQUAL_INT(result, NMEA_ERR_UNKNOWN);
}

static void nmea_gprmc(void)
{
    gps_fix_t fix;
    char *sentence =
        "$GPRMC,134730.361,A,5540.3220,N,01231.2858,E,1.06,86.57,041112,,,A*55";

    int result = nmea_parse(sentence, strlen(sentence), &fix);
    TEST_ASSERT_EQUAL_INT(result, NMEA_OK);
    TEST_ASSERT_EQUAL_INT(fix.hour, 13);
    TEST_ASSERT_EQUAL_INT(fix.minute, 47);
    TEST_ASSERT_EQUAL_INT(fix.seconds, 30);
    TEST_ASSERT_EQUAL_INT(fix.milliseconds, 361);

    ASSERT_FUZZY_EQUAL_FLOAT(fix.latitude, 55.672033, 5);
    ASSERT_FUZZY_EQUAL_FLOAT(fix.longitude, 12.52143, 5);
    ASSERT_FUZZY_EQUAL_FLOAT(fix.speed, 1.06, 2);

    TEST_ASSERT_EQUAL_INT(fix.day, 4);
    TEST_ASSERT_EQUAL_INT(fix.month, 11);
    TEST_ASSERT_EQUAL_INT(fix.year, 12);
}

static void nmea_gprmc_invalid_fix(void)
{
    gps_fix_t fix;
    char *sentence =
        "$GPRMC,134730.361,V,,,,,,,,,,B*47";

    int result = nmea_parse(sentence, strlen(sentence), &fix);
    TEST_ASSERT_EQUAL_INT(result, NMEA_ERR_INVALID_FIX);
}

static void nmea_gpgga(void)
{
    gps_fix_t fix;
    char *sentence =
        "$GPGGA,134731.361,5540.3252,N,01231.2946,E,1,10,0.8,36.1,M,41.5,M,,0000*6C";

    int result = nmea_parse(sentence, strlen(sentence), &fix);
    TEST_ASSERT_EQUAL_INT(result, NMEA_OK);
    TEST_ASSERT_EQUAL_INT(fix.hour, 13);
    TEST_ASSERT_EQUAL_INT(fix.minute, 47);
    TEST_ASSERT_EQUAL_INT(fix.seconds, 31);
    TEST_ASSERT_EQUAL_INT(fix.milliseconds, 361);

    ASSERT_FUZZY_EQUAL_FLOAT(fix.latitude, 55.672086, 5);
    ASSERT_FUZZY_EQUAL_FLOAT(fix.longitude, 12.521576, 5);
    ASSERT_FUZZY_EQUAL_FLOAT(fix.altitude, 36.1, 1);
}

TestRef nmea_suite(void)
{
    EMB_UNIT_TESTFIXTURES(fixtures) {
        new_TestFixture(nmea_bad_checksum),
        new_TestFixture(nmea_unknown),
        new_TestFixture(nmea_gprmc),
        new_TestFixture(nmea_gprmc_invalid_fix),
        new_TestFixture(nmea_gpgga),
    };

    EMB_UNIT_TESTCALLER(nmea_tests, NULL, NULL, fixtures);
    return (TestRef) &nmea_tests;
}
