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
#ifdef YOTTA_CFG_SENSORS_GPS
#include <string.h>
#include <stdio.h>
#include <stdlib.h>
#include "kubos-core/unity/unity.h"
#include "kubos-core/k_test.h"
#include "kubos-core/modules/nmea.h"

static void test_BadChecksum(void)
{
    gps_fix_t fix;
    char *sentence = "$GPRMC,,,,,,,,,,,,B*FF";

    int result = nmea_parse(sentence, strlen(sentence), &fix);
    TEST_ASSERT_EQUAL_INT(result, NMEA_ERR_CHECKSUM);
}

static void test_Unknown(void)
{
    gps_fix_t fix;
    char *sentence = "$GPXYZ,A*21";

    int result = nmea_parse(sentence, strlen(sentence), &fix);
    TEST_ASSERT_EQUAL_INT(result, NMEA_ERR_UNKNOWN);
}

static void test_Gprmc(void)
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

    TEST_ASSERT_EQUAL_FLOAT(fix.latitude, 55.672033);
    TEST_ASSERT_EQUAL_FLOAT(fix.longitude, 12.52143);
    TEST_ASSERT_EQUAL_FLOAT(fix.speed, 1.06);

    TEST_ASSERT_EQUAL_INT(fix.day, 4);
    TEST_ASSERT_EQUAL_INT(fix.month, 11);
    TEST_ASSERT_EQUAL_INT(fix.year, 12);
}

static void test_GprmcInvalidFix(void)
{
    gps_fix_t fix;
    char *sentence =
        "$GPRMC,134730.361,V,,,,,,,,,,B*47";

    int result = nmea_parse(sentence, strlen(sentence), &fix);
    TEST_ASSERT_EQUAL_INT(result, NMEA_ERR_INVALID_FIX);
}

static void test_Gpgga(void)
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

    TEST_ASSERT_EQUAL_FLOAT(fix.latitude, 55.672086);
    TEST_ASSERT_EQUAL_FLOAT(fix.longitude, 12.521576);
    TEST_ASSERT_EQUAL_FLOAT(fix.altitude, 36.1);
}

K_TEST_MAIN() {
    UNITY_BEGIN();
    RUN_TEST(test_BadChecksum);
    RUN_TEST(test_Unknown);
    RUN_TEST(test_Gprmc);
    RUN_TEST(test_GprmcInvalidFix);
    RUN_TEST(test_Gpgga);
    return UNITY_END();
}

int main(void) {
    K_TEST_RUN_MAIN();
}
#else
int main(void) {
    return 0;
}
#endif