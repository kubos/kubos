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

typedef struct telemetry_subscriber
{
    pubsub_conn conn;
    struct telemetry_subscriber * next;
} telemetry_subscriber;

/**
 * Iterates though all open telemetry connections and
 * publishes the packet IF the connection is interested/subscribed
 * @param packet telemetry_packet to publish
 */
static void telemetry_send(telemetry_packet packet);

/* Static array for holding persistent connections to telemetry subscribers */
static pubsub_conn subscriber;

/* Current number of active telemetry subscribers */
static uint8_t num_subs = 0;

/* Queue for incoming packets from publishers */
static csp_queue_handle_t packet_queue = NULL;

static csp_thread_handle_t telem_sub_handle;
static csp_thread_handle_t telem_rx_handle;

static csp_mutex_t subscribing_lock;
static csp_bin_sem_handle_t subscribing_done_signal;
static csp_bin_sem_handle_t telemetry_ready_signal;

static bool telemetry_running = true;
static bool subscribing_done = false;
static telemetry_subscriber * subscriber_list_head = NULL;

void telemetry_init()
{
    csp_buffer_init(20, 256);

    /* Init CSP with address MY_ADDRESS */
    csp_init(TELEMETRY_CSP_ADDRESS);

    /* Start router task with 500 word stack, OS task priority 1 */
    csp_route_start_task(500, 1);

    packet_queue = csp_queue_create(MESSAGE_QUEUE_SIZE, sizeof(telemetry_packet));

    csp_mutex_create(&subscribing_lock);
    csp_bin_sem_create(&subscribing_done_signal);
    csp_bin_sem_wait(&subscribing_done_signal, 0);

    csp_bin_sem_create(&telemetry_ready_signal);
    csp_bin_sem_wait(&telemetry_ready_signal, 0);

#ifdef DEBUG
    csp_debug_toggle_level(CSP_ERROR);
    csp_debug_toggle_level(CSP_WARN);
    csp_debug_toggle_level(CSP_INFO);
    csp_debug_toggle_level(CSP_BUFFER);
    csp_debug_toggle_level(CSP_PACKET);
    csp_debug_toggle_level(CSP_PROTOCOL);
    csp_debug_toggle_level(CSP_LOCK);
#endif

    csp_thread_create(telemetry_get_subs, "TELEM_SUBS", 1000, NULL, 2, &telem_sub_handle);
    csp_thread_create(telemetry_rx_task, "TELEM_RX", 1000, NULL, 2, &telem_rx_handle);
}

void telemetry_cleanup()
{
    telemetry_subscriber * temp_sub, * next_sub;

    telemetry_running = false;
    csp_thread_kill(telem_sub_handle);
    csp_thread_kill(telem_rx_handle);

    csp_route_end_task();

    LL_FOREACH_SAFE(subscriber_list_head, temp_sub, next_sub)
    {
        LL_DELETE(subscriber_list_head, temp_sub);
        csp_close(temp_sub->conn.conn_handle);
        free(temp_sub);
    }

    csp_mutex_remove(&subscribing_lock);
    csp_bin_sem_remove(&subscribing_done_signal);
    csp_queue_remove(packet_queue);
}

void telemetry_add_subscriber(pubsub_conn conn)
{
    telemetry_subscriber * new_sub = NULL;
    if ((new_sub = malloc(sizeof(telemetry_subscriber))) != NULL)
    {
        memcpy(&(new_sub->conn), &conn, sizeof(pubsub_conn));
        LL_APPEND(subscriber_list_head, new_sub);
    }
}

CSP_DEFINE_TASK(telemetry_get_subs)
{
    /* Private csp_socket used by the telemetry server */
    csp_socket_t * socket = NULL;
    // printf("get_subs server setup\r\n");
    if ((socket = kprv_server_setup(TELEMETRY_CSP_PORT, TELEMETRY_SUBSCRIBERS_MAX_NUM)) != NULL)
    {
        while (telemetry_running)
        {
            pubsub_conn conn;
            // printf("get_subs server_accept\r\n");
            // csp_bin_sem_post(&subscribing_done_signal);
            csp_bin_sem_post(&telemetry_ready_signal);
            if (kprv_server_accept(socket, &conn))
            {
                subscribing_done = false;
                // csp_sleep_ms(500);
                telemetry_request request;
                // printf("get_subs pub_read\r\n");
                kprv_publisher_read(conn, (void*)&request, sizeof(telemetry_request), TELEMETRY_CSP_PORT);
                conn.sources = request.sources;
                // printf("get_subs add_sub\r\n");
                telemetry_add_subscriber(conn);
                // csp_bin_sem_post(&subscribing_done_signal);
                // printf("get_subs post done\r\n");
                csp_bin_sem_post(&subscribing_done_signal);
                subscribing_done = true;
            }
        }
    }
    csp_thread_exit();
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

    telemetry_subscriber * current, * next;
    LL_FOREACH_SAFE(subscriber_list_head, current, next)
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

bool kprv_telemetry_subscribe(pubsub_conn * conn, uint8_t sources)
{
    bool ret = false;
    if ((conn != NULL) && kprv_subscriber_connect(conn, TELEMETRY_CSP_ADDRESS, TELEMETRY_CSP_PORT))
    {
        telemetry_request request = {
            .sources = sources
        };

        ret = kprv_send_csp(*conn, (void*)&request, sizeof(telemetry_request));
    }
    return ret;
}

bool telemetry_subscribe(pubsub_conn * conn, uint8_t sources)
{
    bool ret = false;
    csp_mutex_lock(&subscribing_lock, CSP_INFINITY);
    ret = kprv_telemetry_subscribe(conn, sources);
    if (ret)
    {
        /* 
           It is possible for CSP to run out of connections in the
           middle of the subscription process. In this case the subscriber
           will get a connection but the server will not get a corresponding one.
           If the server never sends the subscribing_done_signal then
           we know it failed to get a connection. In this case
           we should cleanup and return an error.
        */
        if (csp_bin_sem_wait(&subscribing_done_signal, 100) != CSP_SEMAPHORE_OK)
        {
            csp_close(conn->conn_handle);
            conn->conn_handle = NULL;
            ret = false;
        }
    } 
    csp_mutex_unlock(&subscribing_lock);
    return ret;
}

void telemetry_unsubscribe(pubsub_conn * conn)
{
    if (conn != NULL)
    {
        csp_close(conn->conn_handle);

        telemetry_subscriber * current, * next;
        LL_FOREACH_SAFE(subscriber_list_head, current, next)
        {
            pubsub_conn subscriber = current->conn;
            if (csp_conn_check(subscriber.conn_handle) != CSP_ERR_NONE)
            {
                LL_DELETE(subscriber_list_head, current);
                csp_close(subscriber.conn_handle);
                free(current);
            }
        }
    }
}

int telemetry_num_subscribers()
{
    telemetry_subscriber * temp;
    int count;
    LL_COUNT(subscriber_list_head, temp, count);
    return count;
}