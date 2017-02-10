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
#include <telemetry/telemetry.h>
#include <telemetry/config.h>

#include <csp/arch/csp_queue.h>
#include <csp/arch/csp_semaphore.h>
#include <kubos-core/utlist.h>
#include <stdio.h>
#include <stdlib.h>

#include <csp/interfaces/csp_if_socket.h>
#include <csp/drivers/socket.h>

/* Queue for incoming packets from publishers */
static csp_queue_handle_t packet_queue = NULL;

/* Handle for telemetry packet receiving thread */
static csp_thread_handle_t telem_rx_handle;

/* Private CSP socket used for telemetry connections */
static csp_socket_t * socket = NULL;

CSP_DEFINE_TASK(telemetry_rx_task) {
    printf("begin socket comms\r\n");
    static csp_socket_t *sock;
    csp_conn_t *conn;
    csp_packet_t *packet;

    /* Create socket and listen for incoming connections */
    sock = csp_socket(CSP_SO_NONE);
    csp_bind(sock, TELEMETRY_EXTERNAL_PORT);
    csp_listen(sock, 5);

    telemetry_message msg;
    telemetry_packet pkt;

     /* Super loop */
    while (1) {
        /* Process incoming packet */
        conn = csp_accept(sock, 1000);
        if (conn) {
            packet = csp_read(conn, 100);
            if (packet)
            {
                if (csp_conn_dport(conn) == TELEMETRY_EXTERNAL_PORT)
                {
                    memcpy(&msg, &packet->data, sizeof(telemetry_message));
                    
                    printf("Received message of type %d\r\n", msg.type);
                    printf("Received message of len %d\r\n", msg.payload_size);
                    csp_buffer_free(packet);
                } 
                else 
                {
                    csp_service_handler(conn, packet);
                }

                
            }
            csp_close(conn);
        }
    }
}

void telemetry_server_init(void)
{
    csp_buffer_init(20, 256);

    /* Init CSP with address MY_ADDRESS */
    csp_init(TELEMETRY_CSP_ADDRESS);

    /* Start router task with 500 word stack, OS task priority 1 */
    csp_route_start_task(500, 1);

    packet_queue = csp_queue_create(MESSAGE_QUEUE_SIZE, sizeof(telemetry_packet));

    // csp_mutex_create(&subscribing_lock);
    // csp_mutex_create(&unsubscribing_lock);

    csp_debug_set_level(CSP_ERROR, true);
    csp_debug_set_level(CSP_WARN, true);
    csp_debug_set_level(CSP_INFO, true);
    csp_debug_set_level(CSP_BUFFER, true);
    csp_debug_set_level(CSP_PACKET, true);
    csp_debug_set_level(CSP_PROTOCOL, true);
    csp_debug_set_level(CSP_LOCK, true);

    csp_thread_create(telemetry_rx_task, "TELEM_RX", TELEMETRY_RX_THREAD_STACK_SIZE, NULL, TELEMETRY_RX_THREAD_PRIORITY, &telem_rx_handle);

    socket = kprv_server_setup(TELEMETRY_INTERNAL_PORT, TELEMETRY_SUBSCRIBERS_MAX_NUM);
}