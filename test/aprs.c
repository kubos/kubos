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
#include "kubos-core/unity/unity.h"
#include <string.h>

#include "kubos-core/modules/aprs.h"

static void test_Position(void)
{
    char buffer[64];
    aprs_position_t pos = {
        .hour = 22, .minute = 7, .second = 35,
        .latitude = 12, .longitude = 34,
        .course = 0, .speed = 12, .altitude = 184
    };

    int result = aprs_position_format(buffer, &pos);

    TEST_ASSERT_EQUAL_INT(APRS_POSITION_LEN, result);
    TEST_ASSERT_EQUAL_STRING("/220735h1200.00N/03400.00EO000/012/A=000184", buffer);
}

static void test_TelemetryValues(void)
{
    aprs_telemetry_t t;
    aprs_telemetry_init(&t);

    aprs_telemetry_add_analog(&t, 0, "A1", "U1", 128);
    TEST_ASSERT_EQUAL_STRING(t.analog_desc[0].name, "A1");
    TEST_ASSERT_EQUAL_STRING(t.analog_desc[0].unit, "U1");
    TEST_ASSERT_EQUAL_INT(t.analog[0], 128);

    aprs_telemetry_add_digital(&t, 0, "D2", "U2", 1);
    TEST_ASSERT_EQUAL_STRING(t.digital_desc[0].name, "D2");
    TEST_ASSERT_EQUAL_STRING(t.digital_desc[0].unit, "U2");
    TEST_ASSERT_EQUAL_INT(t.digital, 1);

    aprs_telemetry_add_digital(&t, 1, "D3", "U3", 1);
    TEST_ASSERT_EQUAL_STRING(t.digital_desc[1].name, "D3");
    TEST_ASSERT_EQUAL_STRING(t.digital_desc[1].unit, "U3");
    TEST_ASSERT_EQUAL_INT(t.digital, 3);
}

static void test_TelemetryFormat(void)
{
    aprs_telemetry_t t = {
        .packet_id = 123,
        .analog = { 10, 50, 70, 200, 250 },
        .digital = 0x81
    };
    char buffer[64];
    int result;

    result = aprs_telemetry_format(buffer, &t);
    TEST_ASSERT_EQUAL_INT(result, APRS_TELEMETRY_LEN);
    TEST_ASSERT_EQUAL_STRING(buffer, "T#123,010,050,070,200,250,10000001");
}

static void test_TelemetryParams(void)
{
    aprs_telemetry_t t = {
        .packet_id = 123,
        .analog_desc = {
            { .name = "A1", .unit = "U1" }, { .name = "A2", .unit = "U2" },
            { .name = "A3", .unit = "U3" }, { .name = "A4", .unit = "U4" },
            { .name = "A56789", .unit = "U5" }
        },
        .analog = { 10, 50, 70, 200, 250 },
        .digital_desc = {
            { .name = "B1", .unit = "U6" }, { .name = "B2", .unit = "U7" },
            { .name = "B3", .unit = "U8" }, { .name = "B4", .unit = "U9" },
            { .name = "B5", .unit = "U10" }, { .name = "B6", .unit = "U11" },
            { .name = "B7", .unit = "U12" }, { .name = "B8", .unit = "U13" }
        },
        .digital = 0x81
    };

    char buffer[64];
    int result;
    char *params_expected = ":CALL-1   :PARM.A1,A2,A3,A4,A567,B1,B2,B3,B4,B5,B6,B7,B8";
    char *units_expected = ":CALL-1   :UNIT.U1,U2,U3,U4,U5,U6,U7,U8,U9,U10,U1,U1,U1";

    result = aprs_telemetry_params_format(buffer, "CALL-1", &t);
    TEST_ASSERT_EQUAL_STRING(buffer, params_expected);
    TEST_ASSERT_EQUAL_INT(result, (int) strlen(params_expected));

    result = aprs_telemetry_units_format(buffer, "CALL-1", &t);
    TEST_ASSERT_EQUAL_STRING(buffer, units_expected);
    TEST_ASSERT_EQUAL_INT(result, (int) strlen(units_expected));
}

int main(void)
{
    UNITY_BEGIN();
    RUN_TEST(test_Position);
    RUN_TEST(test_TelemetryValues);
    RUN_TEST(test_TelemetryFormat);
    RUN_TEST(test_TelemetryParams);
    return UNITY_END();
}
