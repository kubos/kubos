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

#include <ipc/pubsub.h>
#include <ipc/csp.h>
#include <csp/csp.h>
#include <csp/drivers/socket.h>
#include <csp/interfaces/csp_if_socket.h>
#include <tinycbor/cbor.h>
#include <telemetry/telemetry.h>
#include <cmocka.h>

#define TEST_INT_PORT 10
#define TEST_EXT_PORT 20
#define TEST_NUM_CON 5
#define TEST_ADDRESS 1
#define TEST_SOCKET_PORT 8888

static char send_msg[] = "test123test";
static char recv_msg[12];

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
    telemetry_publish(pkt);

    return CSP_TASK_RETURN;
}


static void test_server(void ** arg)
{
    static csp_socket_t *sock;
    csp_packet_t *packet;
    csp_thread_handle_t client_task_handle;
    pubsub_conn conn = {
        .conn_handle = NULL
    };
    uint8_t message[256];
    uint16_t msg_size;

    kubos_csp_init(TEST_ADDRESS);

    csp_thread_create(client_task, "CLIENT", 1024, NULL, 0, &client_task_handle);

    sock = kprv_server_setup(TELEMETRY_EXTERNAL_PORT, TELEMETRY_SUBSCRIBERS_MAX_NUM);
    assert_non_null(sock);

    assert_true(kprv_server_socket_accept(sock, &conn));
    assert_non_null(conn.conn_handle);

    subscriber_list_item * sub = create_subscriber(conn);
    assert_non_null(sub);

    assert_true(kprv_cbor_read(&(sub->conn), (void*)message, 256, TELEMETRY_EXTERNAL_PORT, &msg_size));

    assert_true(telemetry_process_message(sub, (void*)message, msg_size));

    telemetry_server_cleanup();

    csp_thread_kill(client_task_handle);

    kubos_csp_terminate();
}


int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(test_server),
    };

    return cmocka_run_group_tests(tests, NULL, NULL);
}