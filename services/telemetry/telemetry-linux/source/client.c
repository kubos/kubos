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

void telemetry_client_shutdown(void)
{
    csp_route_end_task();
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

bool telemetry_connect(pubsub_conn * conn)
{
    bool ret = false;
    if (kprv_subscriber_socket_connect(conn, TELEMETRY_CSP_ADDRESS, TELEMETRY_EXTERNAL_PORT))
    {
        ret = true;
    }
    return ret;
}

bool telemetry_disconnect(pubsub_conn * client_conn)
{
    bool ret = false;
    // csp_mutex_lock(&unsubscribing_lock, CSP_INFINITY);
    if (client_conn != NULL)
    {
        printf("Send disconnect\r\n");
        uint8_t buffer[256] = {0};
        int msg_size = telemetry_encode_disconnect_msg(buffer);
        if (msg_size > 0)
        {
            ret = kprv_send_csp(client_conn, buffer, msg_size);
            printf("Sent dis %d\r\n", msg_size);
        }
        kprv_subscriber_socket_close(client_conn);
    }
    // csp_mutex_unlock(&unsubscribing_lock);
    return ret;
}

bool telemetry_subscribe(const pubsub_conn * client_conn, int topic_id)
{
    bool ret = false;
    if (client_conn != NULL)
    {
        uint8_t buffer[256] = {0};
        int msg_size = telemetry_encode_subscribe_msg(buffer, &topic_id);
        if (msg_size > 0)
        {
            ret = kprv_send_csp(client_conn, buffer, msg_size);
        }
    }
    return ret;
}

bool telemetry_unsubscribe(const pubsub_conn * client_conn, int topic_id)
{
    bool ret = false;
    if (client_conn != NULL)
    {
        uint8_t buffer[256] = {0};
        int msg_size = telemetry_encode_unsubscribe_msg(buffer, &topic_id);

        if (msg_size > 0)
        {
            ret = kprv_send_csp(client_conn, buffer, msg_size);
        }
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
    bool ret = false;
    if ((ret = telemetry_connect(&conn)) == true)
    {
        uint8_t buffer[256] = {0};
        int msg_size = telemetry_encode_packet_msg(buffer, &pkt);

        if (msg_size > 0)
        {
            ret = kprv_send_csp(&conn, buffer, msg_size);
            printf("Published %d\r\n", msg_size);
        }
        // telemetry_disconnect(&conn);
    }

    return ret;
}