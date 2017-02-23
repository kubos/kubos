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
#include <tinycbor/cbor.h>
#include "telemetry/telemetry.h"

static void test_client_connect(void ** arg)
{
    pubsub_conn conn;

    conn.conn_handle = NULL;

    will_return(__wrap_kprv_subscriber_socket_connect, "");
    will_return(__wrap_kprv_subscriber_socket_connect, true);

    assert_true(telemetry_connect(&conn));
    assert_non_null(conn.conn_handle);
}

static void test_client_disconnect(void ** arg)
{
    pubsub_conn conn;
    conn.conn_handle = NULL;

    will_return(__wrap_kprv_subscriber_socket_connect, "");
    will_return(__wrap_kprv_subscriber_socket_connect, true);

    telemetry_connect(&conn);

    expect_not_value(__wrap_kprv_send_csp, conn->conn_handle, NULL);
    expect_not_value(__wrap_kprv_send_csp, data, NULL);
    will_return(__wrap_kprv_send_csp, true);

    expect_not_value(__wrap_csp_close, conn, NULL);
    will_return(__wrap_csp_close, CSP_ERR_NONE);

    assert_true(telemetry_disconnect(&conn));
    assert_null(conn.conn_handle); 
}

static void test_client_subscribe(void ** arg)
{
    pubsub_conn conn;
    
    will_return(__wrap_kprv_subscriber_socket_connect, "");
    will_return(__wrap_kprv_subscriber_socket_connect, true);
    kprv_subscriber_socket_connect(&conn, 0, 0);

    expect_not_value(__wrap_kprv_send_csp, conn->conn_handle, NULL);
    expect_not_value(__wrap_kprv_send_csp, data, NULL);
    will_return(__wrap_kprv_send_csp, true);

    assert_true(telemetry_subscribe(&conn, 0));
}

static void test_client_unsubscribe(void ** arg)
{
    pubsub_conn conn;
    
    will_return(__wrap_kprv_subscriber_socket_connect, "");
    will_return(__wrap_kprv_subscriber_socket_connect, true);
    kprv_subscriber_socket_connect(&conn, 0, 0);

    expect_not_value(__wrap_kprv_send_csp, conn->conn_handle, NULL);
    expect_not_value(__wrap_kprv_send_csp, data, NULL);
    will_return(__wrap_kprv_send_csp, true);

    assert_true(telemetry_unsubscribe(&conn, 0));
}

static void test_client_read(void ** arg)
{
    pubsub_conn conn;
    telemetry_packet packet;

    will_return(__wrap_kprv_subscriber_socket_connect, "");
    will_return(__wrap_kprv_subscriber_socket_connect, true);
    kprv_subscriber_socket_connect(&conn, 0, 0);

    expect_not_value(__wrap_kprv_subscriber_read, conn->conn_handle, NULL);
    expect_not_value(__wrap_kprv_subscriber_read, buffer, NULL);
    will_return(__wrap_kprv_subscriber_read, true);

    assert_true(telemetry_read(&conn, &packet));
}

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(test_client_connect),
        cmocka_unit_test(test_client_disconnect),
        cmocka_unit_test(test_client_subscribe),
        cmocka_unit_test(test_client_unsubscribe),
        cmocka_unit_test(test_client_read),
    };

    return cmocka_run_group_tests(tests, NULL, NULL);
}