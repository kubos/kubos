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

#include "telemetry/telemetry.h"
#include <cmocka.h>
#include <tinycbor/cbor.h>

static void test_client_connect(void ** arg)
{
    socket_conn conn;

    will_return(__wrap_kprv_socket_client_connect, true);

    assert_true(telemetry_connect(&conn));
    assert_true(conn.socket_handle > 0);
    assert_true(conn.is_active);
}

static void test_client_disconnect(void ** arg)
{
    socket_conn conn;

    will_return(__wrap_kprv_socket_client_connect, true);

    telemetry_connect(&conn);

    expect_not_value(__wrap_kprv_socket_send, conn->is_active, false);
    expect_not_value(__wrap_kprv_socket_send, data, NULL);
    will_return(__wrap_kprv_socket_send, true);

    assert_true(telemetry_disconnect(&conn));
    assert_false(conn.is_active);
}

static void test_client_subscribe(void ** arg)
{
    socket_conn conn;

    will_return(__wrap_kprv_socket_client_connect, true);
    kprv_socket_client_connect(&conn, 0);

    expect_not_value(__wrap_kprv_socket_send, conn->is_active, false);
    expect_not_value(__wrap_kprv_socket_send, data, NULL);
    will_return(__wrap_kprv_socket_send, true);

    assert_true(telemetry_subscribe(&conn, 0));
}

static void test_client_unsubscribe(void ** arg)
{
    socket_conn conn;

    will_return(__wrap_kprv_socket_client_connect, true);
    kprv_socket_client_connect(&conn, 0);

    expect_not_value(__wrap_kprv_socket_send, conn->is_active, false);
    expect_not_value(__wrap_kprv_socket_send, data, NULL);
    will_return(__wrap_kprv_socket_send, true);

    assert_true(telemetry_unsubscribe(&conn, 0));
}

static void test_client_read(void ** arg)
{
    socket_conn conn;
    telemetry_packet packet;

    will_return(__wrap_kprv_socket_client_connect, true);
    kprv_socket_client_connect(&conn, 0);

    expect_value(__wrap_kprv_socket_recv, conn->is_active, true);
    expect_not_value(__wrap_kprv_socket_recv, buffer, NULL);
    will_return(__wrap_kprv_socket_recv, "");
    will_return(__wrap_kprv_socket_recv, true);

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