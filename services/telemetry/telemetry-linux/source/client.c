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

void telemetry_client_init(void)
{
    csp_buffer_init(20, 256);

    /* Init CSP with client address */
    csp_init(TELEMETRY_CSP_CLIENT_ADDRESS);

    csp_route_start_task(500, 1);

    csp_debug_set_level(CSP_ERROR, true);
    csp_debug_set_level(CSP_WARN, true);
    csp_debug_set_level(CSP_INFO, true);
    csp_debug_set_level(CSP_BUFFER, true);
    csp_debug_set_level(CSP_PACKET, true);
    csp_debug_set_level(CSP_PROTOCOL, true);
    csp_debug_set_level(CSP_LOCK, true);
}

// bool telemetry_is_subscribed(const pubsub_conn * client_conn, uint16_t topic_id)
// {
//     bool ret = false;
//     if (client_conn != NULL)
//     {
//         subscriber_list_item * sub = kprv_get_subscriber(client_conn);
//         if (sub != NULL)
//         {
//             ret = kprv_has_topic(sub, topic_id);
//         }
//     }
//     return ret;
// }

bool telemetry_subscribe(const pubsub_conn * client_conn, uint16_t topic_id)
{
    bool ret = false;
    if (client_conn != NULL)
    {
        telemetry_message message = {
            .type = MESSAGE_TYPE_SUBSCRIBE,
            // .topic_id = topic_id
        };
        ret = kprv_send_csp(client_conn, &message, sizeof(telemetry_message));
        // subscriber_list_item * sub = kprv_get_subscriber(client_conn);
        // if (sub != NULL)
        // {
        //     ret = kprv_add_topic(sub, topic_id);
        // }
    }
    return ret;
}

bool telemetry_unsubscribe(const pubsub_conn * client_conn, uint16_t topic_id)
{
    bool ret = false;
    if (client_conn != NULL)
    {
        telemetry_message message = {
            .type = MESSAGE_TYPE_UNSUBSCRIBE,
            // .topic_id = topic_id
        };
        ret = kprv_send_csp(client_conn, &message, sizeof(telemetry_message));
        // subscriber_list_item * sub = kprv_get_subscriber(client_conn);
        // if (sub != NULL)
        // {
        //     ret = kprv_remove_topic(sub, topic_id);
        // }
    }
    return ret;
}



bool telemetry_disconnect(pubsub_conn * client_conn)
{
    bool ret = false;
    // csp_mutex_lock(&unsubscribing_lock, CSP_INFINITY);
    if (client_conn != NULL)
    {
        telemetry_message msg = {
            .type = MESSAGE_TYPE_DISCONNECT
        };
        kprv_send_csp(client_conn, &msg, sizeof(telemetry_message));
        csp_close(client_conn->conn_handle);
        ret = true;
    }
    // csp_mutex_unlock(&unsubscribing_lock);
    return ret;
}

bool telemetry_connect(pubsub_conn * conn)
{
    // pubsub_conn * client_conn = NULL;
    // csp_mutex_lock(&subscribing_lock, CSP_INFINITY);
    return kprv_telemetry_connect(conn);
    // csp_mutex_unlock(&subscribing_lock);
    // return client_conn;
}

bool kprv_telemetry_connect(pubsub_conn * conn)
{
    bool ret = false;
    printf("telemetry_connect\r\n");
    if (kprv_subscriber_socket_connect(conn, TELEMETRY_CSP_ADDRESS, TELEMETRY_EXTERNAL_PORT))
    {
        printf("subscriber_connected\r\n");
        ret = true;
    } else {
        printf("subscriber_connect failed\r\n");
    }
    return ret;
}

bool telemetry_read(const pubsub_conn * conn, telemetry_packet * packet)
{
    int tries = 0;
    if (packet != NULL)
    {
        while (tries++ < TELEMETRY_SUBSCRIBER_READ_ATTEMPTS)
        {
            if (kprv_subscriber_read(conn, (void*)packet, sizeof(telemetry_packet), TELEMETRY_INTERNAL_PORT))
                return true;
        }
    }
    return false;
}

bool telemetry_publish(telemetry_packet pkt)
{
    pubsub_conn conn;

    telemetry_connect(&conn);

    telemetry_message msg = {
        .type = MESSAGE_TYPE_PACKET,
        .payload_size = sizeof(telemetry_packet),
        .payload = &pkt
    };

    kprv_send_csp(&conn, &msg, sizeof(telemetry_message));

    telemetry_disconnect(&conn);


    return true;
}