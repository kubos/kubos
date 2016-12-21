/*
 * KubOS HAL
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

#include <unity/unity.h>
#include <unity/k_test.h>
#include "telemetry/telemetry.h"

static void test_subscriber_read_packet_no_init(void)
{
    telemetry_packet packet;
    TEST_ASSERT_EQUAL_INT(telemetry_publish(packet), false);
}

static void test_subscriber_read_packet_init(void)
{
    telemetry_packet packet;
    // telemetry_setup();
    TEST_ASSERT_EQUAL_INT(telemetry_publish(packet), true);
    // telemetry_teardown();
}

static void test_telemetry_read_no_setup(void)
{
    telemetry_packet packet;
    telemetry_conn conn;

    TEST_ASSERT_EQUAL_INT(telemetry_read(conn, &packet), false);
}

static void test_telemetry_read_null_packet(void)
{
    telemetry_packet packet;
    telemetry_conn conn;

    TEST_ASSERT_EQUAL_INT(telemetry_read(conn, NULL), false);
}

static void test_telemetry_read_packet(void)
{
    telemetry_packet packet;
    telemetry_conn conn;

    TEST_ASSERT_EQUAL_INT(telemetry_read(conn, &packet), false);
}


K_TEST_MAIN()
{
    UNITY_BEGIN();
    RUN_TEST(test_subscriber_read_packet_no_init);
    RUN_TEST(test_subscriber_read_packet_init);
    RUN_TEST(test_telemetry_read_no_setup);
    RUN_TEST(test_telemetry_read_null_packet);
    RUN_TEST(test_telemetry_read_packet);
    return UNITY_END();
}

int main(void)
{
    K_TEST_RUN_MAIN();
}