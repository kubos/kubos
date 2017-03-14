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
#include <csp/csp.h>
#include <csp/drivers/socket.h>
#include <csp/interfaces/csp_if_socket.h>
#include <ipc/csp.h>
#include <ipc/pubsub.h>
#include <tinycbor/cbor.h>

#define TEST_INT_PORT 10
#define TEST_EXT_PORT 20
#define TEST_NUM_CON 5
#define TEST_ADDRESS 1
#define TEST_SOCKET_PORT 8888

static char send_msg[] = "test123test";

CSP_DEFINE_TASK(server_task)
{
    csp_socket_t * ext_socket = NULL;
    char server_recv_msg[12];
    pubsub_conn conn = {
        .conn_handle = NULL
    };

    ext_socket = kprv_server_setup(TEST_EXT_PORT, 20);

    kprv_server_socket_accept(ext_socket, &conn);

    kprv_publisher_read(&conn, server_recv_msg, 12, TEST_EXT_PORT);

    kprv_send_csp(&conn, send_msg, 12);

    csp_close_socket(ext_socket);

    kprv_subscriber_socket_close(&conn);

    csp_thread_exit();
}

static void test_client(void ** arg)
{
    pubsub_conn conn;
    csp_thread_handle_t server_task_handle;
    char client_recv_msg[12];
    int tries = 0;
    bool status = false;

    kubos_csp_init(TEST_ADDRESS);

    csp_thread_create(server_task, "server", 1024, NULL, 0, &server_task_handle);

    csp_sleep_ms(100);

    while ((tries++ < 5) && !status)
    {
        status = kprv_subscriber_socket_connect(&conn, TEST_ADDRESS, TEST_EXT_PORT);
    }
    assert_true(status);

    assert_non_null(conn.conn_handle);

    assert_true(kprv_send_csp(&conn, send_msg, 12));

    csp_sleep_ms(100);

    tries = 0;
    status = false;
    while ((tries++ < 5) && !status)
    {
        status = kprv_subscriber_read(&conn, client_recv_msg, 12, TEST_EXT_PORT);
    }
    assert_true(status);

    assert_string_equal(send_msg, client_recv_msg);

    kprv_subscriber_socket_close(&conn);

    csp_thread_kill(server_task_handle);

    kubos_csp_terminate();
}

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(test_client),
    };

    return cmocka_run_group_tests(tests, NULL, NULL);
}