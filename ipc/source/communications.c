/*
 * Copyright (C) 2016 Kubos Corporation
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
// #ifdef YOTTA_CFG_TELEMETRY

#include "ipc/communications.h"

/**
 * Wrapper function for sending data via a csp connection
 * @param conn telemetry_conn containing a valid csp_conn_t *
 * @param data void pointer to data to be sent
 * @param length length of the data to be sent
 * @return bool true if successful, otherwise false
 */
static bool send_csp(telemetry_conn conn, void * data, uint16_t length);


bool server_setup(csp_socket_t ** socket, uint8_t port, uint8_t num_connections)
{
    if ((*socket = csp_socket(CSP_SO_NONE)) == NULL)
    {
        return false;
    }

    if (csp_bind(*socket, port) != CSP_ERR_NONE)
    {
        return false;
    }

    if (csp_listen(*socket, num_connections) != CSP_ERR_NONE)
    {
        return false;
    }

    return true;
}

bool server_accept(csp_socket_t ** socket, telemetry_conn * conn)
{
    csp_conn_t * csp_conn = NULL;
    if ((conn != NULL) && ((csp_conn = csp_accept(*socket, 1000)) != NULL))
    {
        conn->conn_handle = csp_conn;
        return true;
    }
    return false;
}

bool subscriber_connect(telemetry_conn * conn, uint8_t address, uint8_t port)
{
    csp_conn_t * csp_conn = NULL;
    if (conn == NULL)
    {
        return false;
    }

    csp_conn = csp_connect(CSP_PRIO_NORM, address, port, 1000, CSP_O_NONE);
    if (csp_conn != NULL)
    {
        conn->conn_handle = csp_conn;
        return true;
    }
    else
    {
        conn->conn_handle = NULL;
        return false;
    }
}

bool send_packet(telemetry_conn conn, telemetry_packet packet)
{
    return send_csp(conn, (void*)&packet, sizeof(packet));
}

bool send_request(telemetry_conn conn, telemetry_request request)
{
    return send_csp(conn, (void*)&request, sizeof(request));
}

static bool send_csp(telemetry_conn conn, void * data, uint16_t length)
{
    csp_packet_t * csp_packet = NULL;
    csp_conn_t * csp_conn = conn.conn_handle;
    if ((data != NULL) && (csp_conn != NULL))
    {
        csp_packet = csp_buffer_get(length);
        if (csp_packet != NULL)
        {
            memcpy(csp_packet->data, data, length);
            csp_packet->length = length;
            if (!csp_send(csp_conn, csp_packet, 1000))
            {
                csp_buffer_free(csp_packet);
                return false;
            }
            else
            {
                return true;
            }
        }
    }
    return false;
}

bool publisher_read_request(telemetry_conn conn, telemetry_request * request, uint8_t port)
{
    csp_packet_t * csp_packet = NULL;
    csp_conn_t * csp_conn = conn.conn_handle;
    if ((request != NULL) && (csp_conn != NULL))
    {
        if ((csp_packet = csp_read(csp_conn, 1000)) != NULL)
        {
            if (csp_conn_dport(csp_conn) == port)
            {
                memcpy(request, (telemetry_request*)csp_packet->data, sizeof(telemetry_request));
                csp_buffer_free(csp_packet);
                return true;
            }
            csp_service_handler(csp_conn, csp_packet);
        }
    }
    return false;
}

bool subscriber_read_packet(telemetry_conn conn, telemetry_packet * packet, uint8_t port)
{
    csp_packet_t * csp_packet = NULL;
    csp_conn_t * csp_conn = conn.conn_handle;
    if ((packet != NULL) && (csp_conn != NULL))
    {
        if ((csp_packet = csp_read(csp_conn, 1000)) != NULL)
        {
            if (csp_conn_sport(csp_conn) == port)
            {
                memcpy(packet, (telemetry_packet*)csp_packet->data, sizeof(telemetry_packet));
                csp_buffer_free(csp_packet);
                return true;
            }
            csp_service_handler(csp_conn, csp_packet);
        }
    }
    return false;
}

// #endif