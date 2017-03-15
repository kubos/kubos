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
#include <csp/arch/csp_thread.h>
#include <csp/csp.h>
#include <ipc/socket.h>
#include <tinycbor/cbor.h>

#define TEST_INT_PORT 10
#define TEST_EXT_PORT 20
#define TEST_NUM_CON 5
#define TEST_ADDRESS 1
#define TEST_SOCKET_PORT 8181

static uint8_t send_msg[] = "test123test";
static uint8_t recv_msg[12];

CSP_DEFINE_TASK(client_task)
{
    socket_conn conn;
    int tries = 0;

    while ((tries++ < 5) && !kprv_socket_client_connect(&conn, TEST_SOCKET_PORT))
    {
        csp_sleep_ms(10);
    }

    assert_true(conn.is_active);
    assert_true(conn.socket_handle > 0);

    assert_true(kprv_socket_send(&conn, send_msg, 12));

    assert_true(kprv_socket_close(&conn));
    assert_false(conn.is_active);

    assert_false(kprv_socket_send(&conn, send_msg, 12));

    csp_thread_exit();
}

static void test_socket_server(void ** arg)
{
    csp_thread_handle_t client_task_handle;
    socket_conn server_conn, client_conn;
    uint32_t bytes_recv = 0;

    csp_thread_create(client_task, "CLIENT", 1024, NULL, 0, &client_task_handle);

    assert_true(kprv_socket_server_setup(&server_conn, TEST_SOCKET_PORT, 20));

    assert_true(kprv_socket_server_accept(&server_conn, &client_conn));
    assert_true(client_conn.socket_handle > 0);
    assert_true(client_conn.is_active);

    assert_true(kprv_socket_recv(&client_conn, recv_msg, 12, &bytes_recv));

    assert_int_equal(bytes_recv, 12);

    assert_string_equal(send_msg, recv_msg);

    assert_true(kprv_socket_close(&client_conn));
    assert_false(client_conn.is_active);

    assert_false(kprv_socket_recv(&client_conn, recv_msg, 12, &bytes_recv));

    csp_thread_kill(client_task_handle);
}

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(test_socket_server),
    };

    return cmocka_run_group_tests(tests, NULL, NULL);
}