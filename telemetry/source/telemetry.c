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
#include "telemetry/config.h"
#include <csp/arch/csp_queue.h>
#include <csp/arch/csp_semaphore.h>
#include <kubos-core/utlist.h>
#include <stdio.h>
#include <stdlib.h>

/* Structure for storing a list of telemetry sources */
typedef struct source_list_item
{
    telemetry_source source;
    struct source_list_item * next;
} source_list_item;

/* Structure for storing telemetry subscribers in a list */
typedef struct subscriber_list_item
{
    pubsub_conn conn;
    source_list_item * subscriptions;
    struct subscriber_list_item * next;
} subscriber_list_item;


/**
 * Iterates though all open telemetry connections and
 * publishes the packet IF the connection is interested/subscribed
 * @param packet telemetry_packet to publish
 */
static void telemetry_send(telemetry_packet packet);

/* Queue for incoming packets from publishers */
static csp_queue_handle_t packet_queue = NULL;

/* Handle for telemetry packet receiving thread */
static csp_thread_handle_t telem_rx_handle;

/* Mutex to lock subscribing process */
static csp_mutex_t subscribing_lock;

/* Mutex to lock unsubscribing process */
static csp_mutex_t unsubscribing_lock;

/* Bool flag used to indicate telemetry up/down, used to start cleanup process */
static bool telemetry_running = true;

/* Initial element in list of telemetry subscribers */
static subscriber_list_item * subscribers = NULL;

/* Private CSP socket used for telemetry connections */
static csp_socket_t * socket = NULL;

void telemetry_init()
{
    csp_buffer_init(20, 256);

    /* Init CSP with address MY_ADDRESS */
    csp_init(TELEMETRY_CSP_ADDRESS);

    /* Start router task with 500 word stack, OS task priority 1 */
    csp_route_start_task(500, 1);

    packet_queue = csp_queue_create(MESSAGE_QUEUE_SIZE, sizeof(telemetry_packet));

    csp_mutex_create(&subscribing_lock);
    csp_mutex_create(&unsubscribing_lock);

#ifdef DEBUG
    csp_debug_toggle_level(CSP_ERROR);
    csp_debug_toggle_level(CSP_WARN);
    csp_debug_toggle_level(CSP_INFO);
    csp_debug_toggle_level(CSP_BUFFER);
    csp_debug_toggle_level(CSP_PACKET);
    csp_debug_toggle_level(CSP_PROTOCOL);
    csp_debug_toggle_level(CSP_LOCK);
#endif

    csp_thread_create(telemetry_rx_task, "TELEM_RX", TELEMETRY_RX_THREAD_STACK_SIZE, NULL, TELEMETRY_RX_THREAD_PRIORITY, &telem_rx_handle);

    socket = kprv_server_setup(TELEMETRY_CSP_PORT, TELEMETRY_SUBSCRIBERS_MAX_NUM);
}

void telemetry_cleanup()
{
    subscriber_list_item * temp_sub, * next_sub;

    telemetry_running = false;
    csp_thread_kill(telem_rx_handle);

    csp_route_end_task();

    LL_FOREACH_SAFE(subscribers, temp_sub, next_sub)
    {
        LL_DELETE(subscribers, temp_sub);
        csp_close(temp_sub->conn.conn_handle);
        free(temp_sub);
    }

    csp_mutex_remove(&subscribing_lock);
    csp_queue_remove(packet_queue);
}

bool telemetry_add_subscriber(pubsub_conn conn)
{
    bool ret = false;
    subscriber_list_item * new_sub = NULL;
    if ((new_sub = malloc(sizeof(subscriber_list_item))) != NULL)
    {
        memcpy(&(new_sub->conn), &conn, sizeof(pubsub_conn));
        LL_APPEND(subscribers, new_sub);
        ret = true;
    }
    return ret;
}

CSP_DEFINE_TASK(telemetry_rx_task)
{
    telemetry_packet packet;
    while(telemetry_running)
    {
        if (csp_queue_dequeue(packet_queue, &packet, CSP_MAX_DELAY))
        {
            telemetry_send(packet);
        }
    }
    csp_thread_exit();
}

static void telemetry_send(telemetry_packet packet)
{
    // These print statements should be converted to debug logging
    // Once we have a logging system in place :)
    if(packet.source.data_type == TELEMETRY_TYPE_INT)
    {
        printf("TELEM:%d:%d:%d\r\n", packet.source.source_id, packet.timestamp, packet.data.i);
    }
    if(packet.source.data_type == TELEMETRY_TYPE_FLOAT)
    {
        printf("TELEM:%d:%d:%f\r\n", packet.source.source_id, packet.timestamp, packet.data.f);
    }

    subscriber_list_item * current, * next;
    LL_FOREACH_SAFE(subscribers, current, next)
    {
        pubsub_conn subscriber = current->conn;
        if ((subscriber.sources == 0) || (packet.source.source_id & subscriber.sources))
        {
            kprv_send_csp(subscriber, (void*)&packet, sizeof(telemetry_packet));
        }
    }
}

bool telemetry_publish(telemetry_packet packet)
{
    if ((packet_queue != NULL) && (csp_queue_enqueue(packet_queue, &packet, CSP_MAX_DELAY)))
    {
        return true;
    }
    return false;
}

bool telemetry_read(pubsub_conn conn, telemetry_packet * packet)
{
    int tries = 0;
    if (packet != NULL)
    {
        while (tries++ < TELEMETRY_SUBSCRIBER_READ_ATTEMPTS)
        {
            if (kprv_subscriber_read(conn, (void*)packet, sizeof(telemetry_packet), TELEMETRY_CSP_PORT))
                return true;
        }
    }
    return false;
}

bool kprv_telemetry_connect(pubsub_conn * client_conn)
{
    bool ret = false;
    if ((client_conn != NULL) && kprv_subscriber_connect(client_conn, TELEMETRY_CSP_ADDRESS, TELEMETRY_CSP_PORT))
    {
        telemetry_request request = {
            
        };

        ret = kprv_send_csp(*client_conn, (void*)&request, sizeof(telemetry_request));
    }
    if (ret)
    {
        pubsub_conn server_conn;
        if (kprv_server_accept(socket, &server_conn))
        {
            telemetry_request request;
            ret = kprv_publisher_read(server_conn, (void*)&request, sizeof(telemetry_request), TELEMETRY_CSP_PORT);
            if (ret)
            {
                server_conn.sources = request.sources;
                ret = telemetry_add_subscriber(server_conn);
            }
        }
        else
        {
            /* 
                It is possible for CSP to run out of connections in the
                middle of the subscription process. In this case the subscriber
                will get a connection but the server will not get a corresponding one.
                If the server never sends the subscribing_done_signal then
                we know it failed to get a connection. In this case
                we should cleanup and return an error.
            */
            csp_close(client_conn->conn_handle);
            client_conn->conn_handle = NULL;
            ret = false;
        }
    }
    return ret;
}

bool telemetry_connect(pubsub_conn * client_conn)
{
    bool ret = false;
    csp_mutex_lock(&subscribing_lock, CSP_INFINITY);
    ret = kprv_telemetry_connect(client_conn);
    csp_mutex_unlock(&subscribing_lock);
    return ret;
}

bool telemetry_disconnect(pubsub_conn * conn)
{
    bool ret = false;
    csp_mutex_lock(&unsubscribing_lock, CSP_INFINITY);
    if ((conn != NULL) && (csp_close(conn->conn_handle) == CSP_ERR_NONE))
    {
        subscriber_list_item * current, * next;
        LL_FOREACH_SAFE(subscribers, current, next)
        {
            pubsub_conn subscriber = current->conn;
            if (csp_conn_check_alive(subscriber.conn_handle) != CSP_ERR_NONE)
            {
                if (csp_close(subscriber.conn_handle) == CSP_ERR_NONE)
                {
                    LL_DELETE(subscribers, current);
                    free(current);
                    ret = true;
                }
                break;
            }
        }
    }
    csp_mutex_unlock(&unsubscribing_lock);
    return ret;
}

bool telemetry_subscribe(pubsub_conn * client_conn, uint16_t topic_id)
{
    return false;
}

bool telemetry_unsubscribe(pubsub_conn * client_conn, uint16_t topic_id)
{
    return false;
}

int telemetry_num_subscribers()
{
    subscriber_list_item * temp;
    int count;
    LL_COUNT(subscribers, temp, count);
    return count;
}