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

/* Initial element in list of telemetry subscribers */
static subscriber_list_item * subscribers = NULL;

CSP_DEFINE_TASK(client_rx_task);

subscriber_list_item * create_subscriber(pubsub_conn conn)
{
    subscriber_list_item * sub = NULL;
    if ((sub = malloc(sizeof(subscriber_list_item))) != NULL)
    {
        sub->topics = NULL;
        memcpy(&(sub->conn), &conn, sizeof(pubsub_conn));
        sub->packet_queue = csp_queue_create(MESSAGE_QUEUE_SIZE, sizeof(telemetry_packet));
        sub->active = true;
        LL_APPEND(subscribers, sub);
    }
    return sub;
}

void destroy_subscriber(subscriber_list_item ** sub)
{
    csp_close((*sub)->conn.conn_handle);
    (*sub)->conn.conn_handle = NULL;

    free(*sub);
    *sub = NULL;
}

int topic_cmp(const topic_list_item * a, const topic_list_item * b)
{
    return (a->topic_id != b->topic_id);
}

bool kprv_add_topic(subscriber_list_item * sub, int topic_id)
{
    bool ret = false;
    if (sub != NULL)
    {
        topic_list_item * new_topic = NULL;
        if ((new_topic = malloc(sizeof(topic_list_item))) != NULL)
        {
            new_topic->topic_id = topic_id;
            LL_APPEND(sub->topics, new_topic);
            ret = true;
        }
    }
    return ret;
}

bool kprv_remove_topic(subscriber_list_item * sub, int topic_id)
{
    bool ret = false;
    if (sub != NULL)
    {
        topic_list_item topic = {
            .topic_id = topic_id
        };
        topic_list_item * temp;
        LL_SEARCH(sub->topics, temp, &topic, topic_cmp);
        if (temp != NULL)
        {
            LL_DELETE(sub->topics, temp);
            free(temp);
            ret = true;
        }
    }
    return ret;
}

bool kprv_has_topic(const subscriber_list_item * sub, uint16_t topic_id)
{
    bool ret = false;
    if (sub != NULL)
    {
        topic_list_item topic = {
            .topic_id = topic_id
        };
        topic_list_item * temp;
        LL_SEARCH(sub->topics, temp, &topic, topic_cmp);
        if (temp != NULL)
            ret = true;
    }
    return ret;
}


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

bool telemetry_publish_packet(subscriber_list_item * sub, telemetry_packet packet)
{
    if (!csp_queue_enqueue(sub->packet_queue, (void*)&packet, 1000))
        return false;
    return true;
}

bool telemetry_get_packet(subscriber_list_item * sub, telemetry_packet * packet)
{
    if (!csp_queue_dequeue(sub->packet_queue, (void*)packet, 1000))
        return false;
    return true;
}

int telemetry_get_num_packets(subscriber_list_item * sub)
{
    return csp_queue_size(sub->packet_queue);
}

bool kprv_publish_packet(telemetry_packet packet)
{
    bool ret = true;
    subscriber_list_item * current, * next;
    LL_FOREACH_SAFE(subscribers, current, next)
    {
        if (kprv_has_topic(current, packet.source.topic_id))
        {
            if (!telemetry_publish_packet(current, packet))
            {
                ret = false;
                break;
            }
        }
    }
    return ret;
}

bool telemetry_process_message(subscriber_list_item * sub, void * buffer, int buffer_size)
{
    bool ret = false;
    telemetry_message_type req;
    telemetry_packet packet;
    int topic_id;

    if (telemetry_parse_msg_type(buffer, buffer_size, &req))
    {
        switch (req) {
            case MESSAGE_TYPE_PACKET:
                if (telemetry_parse_packet_msg(buffer, buffer_size, &packet))
                {
                    printf("Got packet %d %d\r\n", packet.source.topic_id, packet.data.i);
                    ret = kprv_publish_packet(packet);
                }
                break;
            case MESSAGE_TYPE_SUBSCRIBE:
                if (telemetry_parse_subscribe_msg(buffer, buffer_size, &topic_id))
                {
                    printf("Got subscribe %d\r\n", topic_id);
                    ret = kprv_add_topic(sub, topic_id);
                }
                break;
            case MESSAGE_TYPE_UNSUBSCRIBE:
                if (telemetry_parse_unsubscribe_msg(buffer, buffer_size, &topic_id))
                {
                    ret = kprv_remove_topic(sub, topic_id);
                }
                break;
            case MESSAGE_TYPE_DISCONNECT:
                printf("Got disconnect\r\n");
                sub->active = false;
                ret = true;
                break;
            default:
                printf("Got other msg type\r\n");
                break;
        }
    }
    return ret;
}

CSP_DEFINE_TASK(client_rx_task)
{
    pubsub_conn conn;
    subscriber_list_item * sub = NULL;
    if (param == NULL)
    {
        printf("No conn found\r\n");
        return CSP_TASK_RETURN;
    }
    conn = *((pubsub_conn*)param);
    sub = create_subscriber(conn);

    uint8_t message[256];
    uint16_t msg_size;

    while (sub->active == true)
    {
        while (telemetry_get_num_packets(sub) > 0)
        {
            telemetry_packet packet;
            telemetry_get_packet(sub, &packet);
        }
        if (!kprv_cbor_read(&conn, (void*)message, 256, TELEMETRY_EXTERNAL_PORT, &msg_size))
            continue;

        telemetry_process_message(sub, (void*)message, msg_size);
    }

    destroy_subscriber(&sub);

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