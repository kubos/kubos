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

#include <tinycbor/cbor.h>

#include <csp/interfaces/csp_if_socket.h>
#include <csp/drivers/socket.h>

/* Queue for incoming packets from publishers */
// static csp_queue_handle_t packet_queue = NULL;

/* Handle for telemetry packet receiving thread */
static csp_thread_handle_t telem_rx_handle;

/* Private CSP socket used for telemetry connections */
static csp_socket_t * socket = NULL;

/* Structure for storing a list of telemetry sources */
typedef struct topic_list_item
{
    uint16_t topic_id;
    struct topic_list_item * next;
} topic_list_item;

/* Structure for storing telemetry subscribers in a list */
typedef struct subscriber_list_item
{
    pubsub_conn conn;
    topic_list_item * topics;
    csp_queue_handle_t packet_queue;
    struct subscriber_list_item * next;
} subscriber_list_item;

/* Initial element in list of telemetry subscribers */
static subscriber_list_item * subscribers = NULL;

CSP_DEFINE_TASK(client_rx_task);


static void kprv_add_subscriber(pubsub_conn conn)
{
    subscriber_list_item * new_sub = NULL;
    if ((new_sub = malloc(sizeof(subscriber_list_item))) != NULL)
    {
        memcpy(&(new_sub->conn), &conn, sizeof(pubsub_conn));
        new_sub->topics = NULL;
        LL_APPEND(subscribers, new_sub);
    }
}

bool kprv_cbor_read(const pubsub_conn * conn, void * buffer, int max_buffer_size, uint8_t port, uint16_t * size_received)
{
    csp_packet_t * csp_packet = NULL;
    csp_conn_t * csp_conn = NULL;
    if ((conn != NULL) && (conn->conn_handle != NULL) && (buffer != NULL) && (size_received != NULL))
    {
        csp_conn = conn->conn_handle;
        if ((csp_packet = csp_read(csp_conn, 1000)) != NULL)
        {
            if (csp_conn_dport(csp_conn) == port)
            {
                memcpy(buffer, &csp_packet->data, csp_packet->length);
                *size_received = csp_packet->length;
                csp_buffer_free(csp_packet);
                return true;
            }
            csp_service_handler(csp_conn, csp_packet);
        }
    }
    return false;
}

void telemetry_process_message(void * buffer, int buffer_size)
{
    telemetry_message_type req;
    telemetry_packet packet;
    int topic_id;

    if (telemetry_parse_msg_type(buffer, buffer_size, &req))
    {
        switch (req) {
            case MESSAGE_TYPE_PACKET:
                if (telemetry_parse_packet_msg(buffer, buffer_size, &packet))
                {
                    printf("Got telemetry packet\r\n");
                    printf("Topic %d Data %d\r\n", packet.source.topic_id, packet.data.i);
                }
                break;
            case MESSAGE_TYPE_SUBSCRIBE:
                if (telemetry_parse_subscribe_msg(buffer, buffer_size, &topic_id))
                {
                    printf("Got subscribe request for %d\r\n", topic_id);
                }
                break;
            case MESSAGE_TYPE_UNSUBSCRIBE:
                if (telemetry_parse_unsubscribe_msg(buffer, buffer_size, &topic_id))
                {
                    printf("Got UNsubscribe request for %d\r\n", topic_id);
                }
                break;
            case MESSAGE_TYPE_DISCONNECT:
                printf("Got disconnect request\r\n");
                break;
            default:
                printf("Got other msg type\r\n");
                break;
        }
    }
}

CSP_DEFINE_TASK(client_rx_task)
{
    pubsub_conn conn;

    if (param == NULL)
    {
        printf("No conn found\r\n");
        return CSP_TASK_RETURN;
    }
    conn = *((pubsub_conn*)param);

    uint8_t message[256];
    uint16_t msg_size;

    while (1)
    {
        if (!kprv_cbor_read(&conn, (void*)message, 256, TELEMETRY_EXTERNAL_PORT, &msg_size))
            continue;

        telemetry_process_message((void*)message, msg_size);
    }

    csp_close(conn.conn_handle);

    return CSP_TASK_RETURN;
}


CSP_DEFINE_TASK(telemetry_rx_task)
{
    printf("begin socket comms\r\n");
    static csp_socket_t *sock;
    csp_packet_t *packet;

    /* Create socket and listen for incoming connections */
    sock = csp_socket(CSP_SO_NONE);
    csp_bind(sock, TELEMETRY_EXTERNAL_PORT);
    csp_listen(sock, 10);

    // telemetry_message_type request;
    // telemetry_response_type response;

    // // pubsub_conn conn;
    // telemetry_packet pkt;
    // uint16_t topic_id;

    csp_thread_handle_t rx_thread_handle;
    pubsub_conn conn;
     /* Super loop */
    while (1) {
        
        if (!kprv_server_accept(sock, &conn))
        {
            continue;
        }

        printf("Got csp socket - spawning thread\r\n");
        
        
        csp_thread_create(client_rx_task, NULL, 1000, &conn, 0, &rx_thread_handle);
        csp_sleep_ms(1000);
        // free(conn);
    }
}

void telemetry_server_init(void)
{
    csp_buffer_init(20, 256);

    /* Init CSP with address MY_ADDRESS */
    csp_init(TELEMETRY_CSP_ADDRESS);

    /* Start router task with 500 word stack, OS task priority 1 */
    csp_route_start_task(500, 1);

    // packet_queue = csp_queue_create(MESSAGE_QUEUE_SIZE, sizeof(telemetry_packet));

    // csp_mutex_create(&subscribing_lock);
    // csp_mutex_create(&unsubscribing_lock);

    // csp_debug_set_level(CSP_ERROR, true);
    // csp_debug_set_level(CSP_WARN, true);
    // csp_debug_set_level(CSP_INFO, true);
    // csp_debug_set_level(CSP_BUFFER, true);
    // csp_debug_set_level(CSP_PACKET, true);
    // csp_debug_set_level(CSP_PROTOCOL, true);
    // csp_debug_set_level(CSP_LOCK, true);

    csp_thread_create(telemetry_rx_task, "TELEM_RX", TELEMETRY_RX_THREAD_STACK_SIZE, NULL, TELEMETRY_RX_THREAD_PRIORITY, &telem_rx_handle);

    socket = kprv_server_setup(TELEMETRY_INTERNAL_PORT, TELEMETRY_SUBSCRIBERS_MAX_NUM);
}