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
#include <telemetry-linux/msg.h>
#include <telemetry-linux/server.h>
#include <telemetry/config.h>
#include <telemetry/telemetry.h>

#include <csp/arch/csp_queue.h>
#include <csp/arch/csp_semaphore.h>
#include <ipc/socket.h>
#include <kubos-core/utlist.h>
#include <stdio.h>
#include <stdlib.h>
#include <tinycbor/cbor.h>

/* Initial element in list of telemetry subscribers */
static subscriber_list_item * subscribers = NULL;

/* Base id for subscribers */
static uint16_t sub_id = 0;

/**
 * Used to compare topic_ids when searching a topic list
 */
static int topic_cmp(const topic_list_item * a, const topic_list_item * b)
{
    return (a->topic_id != b->topic_id);
}

subscriber_list_item * kprv_subscriber_init(socket_conn conn)
{
    subscriber_list_item * sub = NULL;
    if ((sub = calloc(1, sizeof(subscriber_list_item))) != NULL)
    {
        sub->topics = NULL;
        memcpy(&(sub->conn), &conn, sizeof(socket_conn));
        sub->packet_queue = csp_queue_create(MESSAGE_QUEUE_SIZE, sizeof(telemetry_packet));
        sub->active = true;
        sub->id = sub_id++;
        sub->next = NULL;
        sub->rx_thread = 0;
    }
    return sub;
}

bool kprv_subscriber_add(subscriber_list_item * sub)
{
    if (sub == NULL)
    {
        return false;
    }

    LL_APPEND(subscribers, sub);

    return true;
}

void kprv_subscriber_destroy(subscriber_list_item ** sub)
{
    if ((sub != NULL) && (*sub != NULL))
    {
        csp_thread_kill((*sub)->rx_thread);

        kprv_subscriber_remove_all_topics(*sub);

        kprv_socket_close(&((*sub)->conn));

        csp_queue_remove((*sub)->packet_queue);

        free(*sub);
        *sub = NULL;
    }
}

bool kprv_subscriber_add_topic(subscriber_list_item * sub, uint16_t topic_id)
{
    bool ret = false;
    if (sub != NULL)
    {
        topic_list_item * new_topic = NULL;
        if ((new_topic = calloc(1, sizeof(topic_list_item))) != NULL)
        {
            new_topic->topic_id = topic_id;
            LL_APPEND(sub->topics, new_topic);
            ret = true;
        }
    }
    return ret;
}

bool kprv_subscriber_remove_topic(subscriber_list_item * sub, uint16_t topic_id)
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

void kprv_subscriber_remove_all_topics(subscriber_list_item * sub)
{
    if (sub->topics != NULL)
    {
        topic_list_item *temp_topic, *next_topic;
        LL_FOREACH_SAFE(sub->topics, temp_topic, next_topic)
        {
            LL_DELETE(sub->topics, temp_topic);
            free(temp_topic);
        }
    }
}

bool kprv_subscriber_has_topic(const subscriber_list_item * sub, uint16_t topic_id)
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
            ret = true;
        }
    }
    return ret;
}

void kprv_delete_all_subscribers()
{
    if (subscribers != NULL)
    {
        subscriber_list_item *cur, *next;
        LL_FOREACH_SAFE(subscribers, cur, next)
        {
            LL_DELETE(subscribers, cur);
            kprv_subscriber_destroy(&cur);
        }
    }
}

bool kprv_publish_packet(telemetry_packet packet)
{
    bool ret = true;
    subscriber_list_item *current, *next;
    LL_FOREACH_SAFE(subscribers, current, next)
    {
        if (kprv_subscriber_has_topic(current, packet.source.topic_id))
        {
            if (!kprv_socket_send(&(current->conn), (void *)&packet, sizeof(telemetry_packet)))
            {
                printf("Failed to publish to %d\r\n", current->id);
                ret = false;
                break;
            }
        }
    }
    return ret;
}

bool telemetry_process_message(subscriber_list_item * sub, const void * buffer, int buffer_size)
{
    bool ret = false;
    telemetry_message_type req;
    telemetry_packet packet;
    uint16_t topic_id;

    if ((sub == NULL) || (buffer == NULL))
    {
        return false;
    }

    if (telemetry_parse_msg_type(buffer, buffer_size, &req))
    {
        switch (req)
        {
            case MESSAGE_TYPE_PACKET:
                if (telemetry_parse_packet_msg(buffer, buffer_size, &packet))
                {
                    ret = kprv_publish_packet(packet);
                }
                break;
            case MESSAGE_TYPE_SUBSCRIBE:
                if (telemetry_parse_subscribe_msg(buffer, buffer_size, &topic_id))
                {
                    ret = kprv_subscriber_add_topic(sub, topic_id);
                }
                break;
            case MESSAGE_TYPE_UNSUBSCRIBE:
                if (telemetry_parse_unsubscribe_msg(buffer, buffer_size, &topic_id))
                {
                    ret = kprv_subscriber_remove_topic(sub, topic_id);
                }
                break;
            case MESSAGE_TYPE_DISCONNECT:
                sub->active = false;
                ret = true;
                break;
            default:
                break;
        }
    }
    return ret;
}

CSP_DEFINE_TASK(client_handler)
{
    subscriber_list_item * sub = NULL;
    if (param == NULL)
    {
        return CSP_TASK_RETURN;
    }

    sub = (subscriber_list_item *)param;

    while (sub->active == true)
    {
        client_rx_work(sub);
    }

    kprv_subscriber_destroy(&sub);

    csp_thread_exit();
}

bool client_rx_work(subscriber_list_item * sub)
{
    uint8_t msg[TELEMETRY_BUFFER_SIZE];
    uint32_t msg_size;
    bool ret = false;

    if (sub != NULL)
    {
        if (kprv_socket_recv(&(sub->conn), (void *)msg, TELEMETRY_BUFFER_SIZE, &msg_size))
        {
            ret = telemetry_process_message(sub, (void *)msg, msg_size);
        }
    }

    return ret;
}

void telemetry_server_cleanup(void)
{
    kprv_delete_all_subscribers();
}