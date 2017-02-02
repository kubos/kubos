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
typedef struct topic_list_item
{
    uint16_t topic_id;
    struct topic_list_item * next;
} topic_list_item;

/* Structure for storing telemetry subscribers in a list */
typedef struct subscriber_list_item
{
    uint16_t connection_id;
    pubsub_conn client_conn;
    pubsub_conn server_conn;
    topic_list_item * topics;
    struct subscriber_list_item * next;
} subscriber_list_item;

bool kprv_has_topic(subscriber_list_item * sub, uint16_t topic_id);

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
        csp_close(temp_sub->server_conn.conn_handle);
        csp_close(temp_sub->client_conn.conn_handle);

        if (temp_sub->topics != NULL)
        {
            topic_list_item * temp_topic, * next_topic;
            LL_FOREACH_SAFE(temp_sub->topics, temp_topic, next_topic)
            {
                LL_DELETE(temp_sub->topics, temp_topic);
                free(temp_topic);
            }
        }

        free(temp_sub);
    }

    csp_mutex_remove(&subscribing_lock);
    csp_queue_remove(packet_queue);
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
        printf("TELEM:%d:%d:%d\r\n", packet.source.topic_id, packet.timestamp, packet.data.i);
    }
    if(packet.source.data_type == TELEMETRY_TYPE_FLOAT)
    {
        printf("TELEM:%d:%d:%f\r\n", packet.source.topic_id, packet.timestamp, packet.data.f);
    }

    subscriber_list_item * current, * next;
    LL_FOREACH_SAFE(subscribers, current, next)
    {
        pubsub_conn subscriber = current->server_conn;
        if (kprv_has_topic(current, packet.source.topic_id))
        {
            kprv_send_csp(&subscriber, (void*)&packet, sizeof(telemetry_packet));
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

bool telemetry_read(pubsub_conn * conn, telemetry_packet * packet)
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

subscriber_list_item * telemetry_add_subscriber(pubsub_conn server_conn, pubsub_conn client_conn)
{
    subscriber_list_item * new_sub = NULL;
    if ((new_sub = malloc(sizeof(subscriber_list_item))) != NULL)
    {
        memcpy(&(new_sub->server_conn), &server_conn, sizeof(pubsub_conn));
        memcpy(&(new_sub->client_conn), &client_conn, sizeof(pubsub_conn));
        new_sub->topics = NULL;
        LL_APPEND(subscribers, new_sub);
    }
    return new_sub;
}

pubsub_conn * kprv_telemetry_connect()
{
    pubsub_conn * conn = NULL;
    pubsub_conn client_conn;
    bool ret = false;
    if (kprv_subscriber_connect(&client_conn, TELEMETRY_CSP_ADDRESS, TELEMETRY_CSP_PORT))
    {
        char msg;

        ret = kprv_send_csp(&client_conn, (void*)&msg, sizeof(msg));    
    }
    if (ret)
    {
        pubsub_conn server_conn;
        if (kprv_server_accept(socket, &server_conn))
        {
            char msg;
            ret = kprv_publisher_read(&server_conn, (void*)&msg, sizeof(msg), TELEMETRY_CSP_PORT);
            if (ret)
            {
                subscriber_list_item * sub = telemetry_add_subscriber(server_conn, client_conn);
                conn = &(sub->client_conn);
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
            csp_close(client_conn.conn_handle);
            client_conn.conn_handle = NULL;
            ret = false;
        }
    }
    return conn;
}

pubsub_conn * telemetry_connect()
{
    pubsub_conn * client_conn = NULL;
    csp_mutex_lock(&subscribing_lock, CSP_INFINITY);
    client_conn = kprv_telemetry_connect(client_conn);
    csp_mutex_unlock(&subscribing_lock);
    return client_conn;
}

bool telemetry_disconnect(pubsub_conn * client_conn)
{
    bool ret = false;
    csp_mutex_lock(&unsubscribing_lock, CSP_INFINITY);
    if ((client_conn != NULL) && (csp_close(client_conn->conn_handle) == CSP_ERR_NONE))
    {
        subscriber_list_item * current, * next;
        LL_FOREACH_SAFE(subscribers, current, next)
        {
            pubsub_conn server_conn = current->server_conn;
            if (csp_conn_check_alive(server_conn.conn_handle) != CSP_ERR_NONE)
            {
                if (csp_close(server_conn.conn_handle) == CSP_ERR_NONE)
                {
                    LL_DELETE(subscribers, current);
                    if (current->topics != NULL)
                    {
                        topic_list_item * temp_topic, * next_topic;
                        LL_FOREACH_SAFE(current->topics, temp_topic, next_topic)
                        {
                            LL_DELETE(current->topics, temp_topic);
                            free(temp_topic);
                        }
                    }
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

subscriber_list_item * kprv_get_subscriber(pubsub_conn * client_conn)
{
    subscriber_list_item * current, * next;
    LL_FOREACH_SAFE(subscribers, current, next)
    {
        if (client_conn->conn_handle == current->client_conn.conn_handle)
            return current;
    }
    return NULL;
}

bool kprv_add_topic(subscriber_list_item * sub, uint16_t topic_id)
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

int topic_cmp(topic_list_item * a, topic_list_item * b)
{
    return (a->topic_id != b->topic_id);
}

bool kprv_has_topic(subscriber_list_item * sub, uint16_t topic_id)
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

bool kprv_remove_topic(subscriber_list_item * sub, uint16_t topic_id)
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

bool telemetry_is_subscribed(pubsub_conn * client_conn, uint16_t topic_id)
{
    bool ret = false;
    if (client_conn != NULL)
    {
        subscriber_list_item * sub = kprv_get_subscriber(client_conn);
        if (sub != NULL)
        {
            ret = kprv_has_topic(sub, topic_id);
        }
    }
    return ret;
}

bool telemetry_subscribe(pubsub_conn * client_conn, uint16_t topic_id)
{
    bool ret = false;
    if (client_conn != NULL)
    {
        subscriber_list_item * sub = kprv_get_subscriber(client_conn);
        if (sub != NULL)
        {
            ret = kprv_add_topic(sub, topic_id);
        }
    }
    return ret;
}

bool telemetry_unsubscribe(pubsub_conn * client_conn, uint16_t topic_id)
{
    bool ret = false;
    if (client_conn != NULL)
    {
        subscriber_list_item * sub = kprv_get_subscriber(client_conn);
        if (sub != NULL)
        {
            ret = kprv_remove_topic(sub, topic_id);
        }
    }
    return ret;
}

int telemetry_num_subscribers()
{
    subscriber_list_item * temp;
    int count;
    LL_COUNT(subscribers, temp, count);
    return count;
}