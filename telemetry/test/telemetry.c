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

static void test_telemetry_subscribe_null_conn(void)
{
    TEST_ASSERT_EQUAL_INT(telemetry_subscribe(NULL, 0), false);
}

static void test_telemetry_subscribe_conn_null_handle(void)
{
    pubsub_conn conn;
    conn.conn_handle = NULL;
    TEST_ASSERT_EQUAL_INT(telemetry_subscribe(&conn, 0), false);
}

static void test_telemetry_subscribe(void)
{
    pubsub_conn conn;
    TEST_ASSERT_EQUAL_INT(telemetry_subscribe(&conn, 0), true);
}

static void test_telemetry_read_conn_null_handle(void)
{
    pubsub_conn conn;
    conn.conn_handle = NULL;
    telemetry_packet packet;

    TEST_ASSERT_EQUAL_INT(telemetry_read(conn, &packet), false);
}

static void test_telemetry_read_null_packet(void)
{
    pubsub_conn conn;
    telemetry_packet packet;
    TEST_ASSERT_EQUAL_INT(telemetry_read(conn, NULL), false);
}

static void test_telemetry_read(void)
{
    pubsub_conn conn;
    telemetry_packet packet;
    // set to non-null addr
    conn.conn_handle = 0xFFF;
    TEST_ASSERT_EQUAL_INT(telemetry_read(conn, &packet), true);
}

static void test_telemetry_publish_no_setup(void)
{
    telemetry_packet packet;
    TEST_ASSERT_EQUAL_INT(telemetry_publish(packet), false);
}

static void test_telemetry_publish(void)
{
    telemetry_packet packet;
    init_telemetry_queue();
    TEST_ASSERT_EQUAL_INT(telemetry_publish(packet), true);
    free_telemetry_queue();
}


K_TEST_MAIN()
{
    UNITY_BEGIN();

    RUN_TEST(test_telemetry_publish_no_setup);
    
    RUN_TEST(test_telemetry_subscribe_null_conn);
    RUN_TEST(test_telemetry_subscribe_conn_null_handle);
    RUN_TEST(test_telemetry_subscribe);

    RUN_TEST(test_telemetry_read_conn_null_handle);
    RUN_TEST(test_telemetry_read_null_packet);
    RUN_TEST(test_telemetry_read);

    RUN_TEST(test_telemetry_publish);

    return UNITY_END();
}

int main(void)
{
    K_TEST_RUN_MAIN();
}