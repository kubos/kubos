/*
 * Copyright (C) 2017 Kubos Corporation
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

#include <cmocka.h>
#include "telemetry/telemetry.h"

static void test_telemetry_subscribe_null_conn(void ** arg)
{
    assert_false(kprv_telemetry_subscribe(NULL, 0));
}

static void test_telemetry_subscribe(void ** arg)
{
    pubsub_conn conn;

    expect_not_value(__wrap_kprv_send_csp, conn.conn_handle, NULL);
    expect_not_value(__wrap_kprv_send_csp, data, NULL);
    will_return(__wrap_kprv_send_csp, true);

    expect_value(__wrap_kprv_subscriber_connect, conn, &conn);
    will_return(__wrap_kprv_subscriber_connect, "");
    will_return(__wrap_kprv_subscriber_connect, true);

    assert_true(kprv_telemetry_subscribe(&conn, 0));
}

static void test_telemetry_read_conn_null_handle(void ** arg)
{
    pubsub_conn conn;
    conn.conn_handle = NULL;
    telemetry_packet packet;

    expect_value_count(__wrap_kprv_subscriber_read, conn.conn_handle, NULL, TELEMETRY_SUBSCRIBER_READ_ATTEMPTS);
    expect_value_count(__wrap_kprv_subscriber_read, buffer, &packet, TELEMETRY_SUBSCRIBER_READ_ATTEMPTS);

    will_return_count(__wrap_kprv_subscriber_read, false, TELEMETRY_SUBSCRIBER_READ_ATTEMPTS);

    assert_false(telemetry_read(conn, &packet));
}

static void test_telemetry_read_null_packet(void ** arg)
{
    pubsub_conn conn;
    telemetry_packet packet;

    assert_false(telemetry_read(conn, NULL));
}

static void test_telemetry_read(void ** arg)
{
    pubsub_conn conn;
    telemetry_packet packet;

    expect_value(__wrap_kprv_subscriber_connect, conn, &conn);
    will_return(__wrap_kprv_subscriber_connect, "");
    will_return(__wrap_kprv_subscriber_connect, true);

    expect_not_value(__wrap_kprv_send_csp, conn.conn_handle, NULL);
    expect_not_value(__wrap_kprv_send_csp, data, NULL);
    will_return(__wrap_kprv_send_csp, true);
    
    kprv_telemetry_subscribe(&conn, 0);

    expect_value(__wrap_kprv_subscriber_read, conn.conn_handle, conn.conn_handle);
    expect_value(__wrap_kprv_subscriber_read, buffer, &packet);
    will_return(__wrap_kprv_subscriber_read, true);

    assert_true(telemetry_read(conn, &packet));
}

static void test_telemetry_publish_no_setup(void ** arg)
{
    telemetry_packet packet;
    assert_false(telemetry_publish(packet));
}

/* Removed the telemetry_publish success unit test 
   because it literally just puts something into a queue.
   This case is probably better exercised in an integration test.
*/

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(test_telemetry_subscribe_null_conn),
        cmocka_unit_test(test_telemetry_subscribe),
        cmocka_unit_test(test_telemetry_read_conn_null_handle),
        cmocka_unit_test(test_telemetry_read_null_packet),
        cmocka_unit_test(test_telemetry_read),
        cmocka_unit_test(test_telemetry_publish_no_setup),
    };

    return cmocka_run_group_tests(tests, NULL, NULL);
}