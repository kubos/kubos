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

#include "telemetry-linux/telemetry.h"
#include <cmocka.h>
#include <csp/arch/csp_thread.h>
#include <csp/csp.h>
#include <csp/drivers/socket.h>
#include <csp/interfaces/csp_if_socket.h>
#include <ipc/csp.h>
#include <ipc/pubsub.h>
#include <telemetry/telemetry.h>
#include <tinycbor/cbor.h>

#define TEST_INT_PORT 10
#define TEST_EXT_PORT 20
#define TEST_NUM_CON 5
#define TEST_ADDRESS 1
#define TEST_SOCKET_PORT 8888

CSP_DEFINE_TASK(client_task)
{
    telemetry_packet pkt;
    pkt.source.topic_id = 12;
    pkt.source.subsystem_id = 11;
    pkt.source.data_type = TELEMETRY_TYPE_INT;
    pkt.timestamp = 101010;
    int data = 121212;

    printf("Begin client task\r\n");

    csp_sleep_ms(100);

    pkt.data.i = data;
    assert_true(telemetry_publish(pkt));

    csp_thread_exit();
}

static void test_server(void ** arg)
{
    csp_thread_handle_t client_task_handle;
    socket_conn server_conn;
    socket_conn conn;
    uint8_t message[256];
    uint32_t msg_size;

    csp_thread_create(client_task, "CLIENT", 1024, NULL, 0, &client_task_handle);

    assert_true(kprv_socket_server_setup(&server_conn, TELEMETRY_SOCKET_PORT, TELEMETRY_SUBSCRIBERS_MAX_NUM));
    assert_true(kprv_socket_server_accept(&server_conn, &conn));
    assert_true(conn.socket_handle > 0);
    assert_true(conn.is_active);

    subscriber_list_item * sub = create_subscriber(conn);
    assert_non_null(sub);

    assert_true(kprv_socket_recv(&(sub->conn), message, 256, &msg_size));

    assert_true(telemetry_process_message(sub, (void *)message, msg_size));

    destroy_subscriber(&sub);

    telemetry_server_cleanup();

    csp_thread_kill(client_task_handle);
}

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(test_server),
    };

    return cmocka_run_group_tests(tests, NULL, NULL);
}