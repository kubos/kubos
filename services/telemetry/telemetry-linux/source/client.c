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
#include <telemetry/config.h>
#include <telemetry/telemetry.h>

#include <csp/arch/csp_queue.h>
#include <csp/arch/csp_semaphore.h>
#include <csp/csp.h>

#include <kubos-core/utlist.h>
#include <stdio.h>
#include <stdlib.h>

#include <ipc/socket.h>

#include <tinycbor/cbor.h>

bool telemetry_connect(socket_conn * conn)
{
    bool ret = false;
    if (kprv_socket_client_connect(conn, TELEMETRY_SOCKET_PORT))
    {
        ret = true;
    }
    return ret;
}

bool telemetry_disconnect(socket_conn * client_conn)
{
    bool ret = false;
    if (client_conn != NULL)
    {
        uint8_t buffer[TELEMETRY_BUFFER_SIZE] = { 0 };
        int msg_size = telemetry_encode_disconnect_msg(buffer);
        if (msg_size > 0)
        {
            ret = kprv_socket_send(client_conn, buffer, msg_size);
        }
        kprv_socket_close(client_conn);
    }
    return ret;
}

bool telemetry_subscribe(const socket_conn * client_conn, uint16_t topic_id)
{
    bool ret = false;
    if (client_conn != NULL)
    {
        uint8_t buffer[TELEMETRY_BUFFER_SIZE] = { 0 };
        int msg_size = telemetry_encode_subscribe_msg(buffer, &topic_id);
        if (msg_size > 0)
        {
            ret = kprv_socket_send(client_conn, buffer, msg_size);
        }
    }
    return ret;
}

bool telemetry_unsubscribe(const socket_conn * client_conn, uint16_t topic_id)
{
    bool ret = false;
    if (client_conn != NULL)
    {
        uint8_t buffer[TELEMETRY_BUFFER_SIZE] = { 0 };
        int msg_size = telemetry_encode_unsubscribe_msg(buffer, &topic_id);

        if (msg_size > 0)
        {
            ret = kprv_socket_send(client_conn, buffer, msg_size);
        }
    }
    return ret;
}

bool telemetry_read(const socket_conn * conn, telemetry_packet * packet)
{
    int tries = 0;
    uint32_t msg_size;
    if ((packet != NULL) && (packet != NULL))
    {
        while (tries++ < TELEMETRY_SUBSCRIBER_READ_ATTEMPTS)
        {
            if (kprv_socket_recv(conn, (void *)packet, sizeof(telemetry_packet), &msg_size))
            {
                return true;
            }
        }
    }
    return false;
}

bool telemetry_publish(telemetry_packet pkt)
{
    socket_conn conn;
    bool ret = false;
    if (telemetry_connect(&conn) == true)
    {
        uint8_t buffer[TELEMETRY_BUFFER_SIZE] = { 0 };
        int msg_size = telemetry_encode_packet_msg(buffer, &pkt);

        if (msg_size > 0)
        {
            ret = kprv_socket_send(&conn, buffer, msg_size);
        }
        telemetry_disconnect(&conn);
    }

    return ret;
}