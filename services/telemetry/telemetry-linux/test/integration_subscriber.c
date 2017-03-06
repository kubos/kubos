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
static bool test_running = true;
static csp_thread_handle_t server_task_handle;
static telemetry_packet out_pkt = {
    .source.topic_id = 12,
    .source.subsystem_id = 11,
    .source.data_type = TELEMETRY_TYPE_INT,
    .data.i = 99
};

CSP_DEFINE_TASK(server_task)
{
    static csp_socket_t *sock;
    
    pubsub_conn conn = {
        .conn_handle = NULL
    };
    uint8_t message[256];
    uint16_t msg_size;

    sock = kprv_server_setup(TELEMETRY_EXTERNAL_PORT, TELEMETRY_SUBSCRIBERS_MAX_NUM);

    while (test_running)
    {
        while (!kprv_server_socket_accept(sock, &conn))
        {
            continue;
        }

        subscriber_list_item * sub = create_subscriber(conn);
        if (sub != NULL)
        {
            while (sub->active)
            {
                if (!client_rx_work(sub))
                    break;
                // Hardcoded publish for test purposes
                kprv_publish_packet(out_pkt);
            }
            destroy_subscriber(&sub);
        }
    }

    csp_close_socket(sock);

    kprv_subscriber_socket_close(&conn);

    // telemetry_server_cleanup();

    csp_thread_exit();
}

static int setup(void ** arg)
{
    test_running = true;
    
    kubos_csp_init(TEST_ADDRESS);

    csp_thread_create(server_task, "SERVER", 1024, NULL, 0, &server_task_handle);

    return 0;
}

static int teardown(void ** arg)
{
    printf("teardown\r\n");
    test_running = false;

    csp_sleep_ms(100);

    csp_thread_kill(server_task_handle);

    kubos_csp_terminate();

    return 0;
}


static void test_subscriber(void ** arg)
{
    int data = 121212;
    pubsub_conn conn;
    telemetry_packet in_packet;

    csp_sleep_ms(50);

    assert_true(telemetry_connect(&conn));
    assert_true(telemetry_subscribe(&conn, out_pkt.source.topic_id));

    csp_sleep_ms(1000);
    
    assert_true(telemetry_read(&conn, &in_packet));

    csp_sleep_ms(1000);

    assert_true(telemetry_disconnect(&conn));
}


int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test_setup_teardown(test_subscriber, setup, teardown),
    };

    return cmocka_run_group_tests(tests, NULL, NULL);
}