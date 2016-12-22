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
#ifdef YOTTA_CFG_TELEMETRY

#include "communications.h"
#include "telemetry/config.h"

/* Private csp_socket used by the telemetry server */
static csp_socket_t * socket;

/**
 * Wrapper function for sending data via a csp connection
 * @param conn telemetry_conn containing a valid csp_conn_t *
 * @param data void pointer to data to be sent
 * @param length length of the data to be sent
 * @return bool true if successful, otherwise false
 */
static bool send_csp(telemetry_conn conn, void * data, uint16_t length);

void telemetry_init()
{
    csp_buffer_init(10, 256);

    /* Init CSP with address MY_ADDRESS */
    csp_init(TELEMETRY_CSP_ADDRESS);

    /* Start router task with 500 word stack, OS task priority 1 */
    csp_route_start_task(500, 1);

    TELEMETRY_THREADS;
}

bool server_setup()
{
    if ((socket = csp_socket(CSP_SO_NONE)) == NULL)
    {
        return false;
    }

    if (csp_bind(socket, TELEMETRY_CSP_PORT) != CSP_ERR_NONE)
    {
        return false;
    }

    if (csp_listen(socket, TELEMETRY_NUM_SUBSCRIBERS) != CSP_ERR_NONE)
    {
        return false;
    }

    return true;
}

bool server_accept(telemetry_conn * conn)
{
    csp_conn_t * csp_conn = NULL;
    if ((conn != NULL) && ((csp_conn = csp_accept(socket, 1000)) != NULL))
    {
        conn->conn_handle = csp_conn;
        return true;
    }
    return false;
}

bool subscriber_connect(telemetry_conn * conn)
{
    csp_conn_t * csp_conn = NULL;
    if (conn == NULL)
    {
        return false;
    }

    csp_conn = csp_connect(CSP_PRIO_NORM, TELEMETRY_CSP_ADDRESS, TELEMETRY_CSP_PORT, 1000, CSP_O_NONE);
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
        csp_packet = csp_buffer_get(20);
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

bool publisher_read_request(telemetry_conn conn, telemetry_request * request)
{
    csp_packet_t * csp_packet = NULL;
    csp_conn_t * csp_conn = conn.conn_handle;
    if ((request != NULL) && (csp_conn != NULL))
    {
        if ((csp_packet = csp_read(csp_conn, 1000)) != NULL)
        {
            switch(csp_conn_dport(csp_conn))
            {
                case TELEMETRY_CSP_PORT:
                    memcpy(request, (telemetry_request*)csp_packet->data, sizeof(telemetry_request));
                    csp_buffer_free(csp_packet);
                    return true;
                default:
                    csp_service_handler(csp_conn, csp_packet);
            }
        }
    }
    return false;
}

bool subscriber_read_packet(telemetry_conn conn, telemetry_packet * packet)
{
    csp_packet_t * csp_packet = NULL;
    csp_conn_t * csp_conn = conn.conn_handle;
    if ((packet != NULL) && (csp_conn != NULL))
    {
        if ((csp_packet = csp_read(csp_conn, 1000)) != NULL)
        {
            switch(csp_conn_sport(csp_conn))
            {
                case TELEMETRY_CSP_PORT:
                    memcpy(packet, (telemetry_packet*)csp_packet->data, sizeof(telemetry_packet));
                    csp_buffer_free(csp_packet);
                    return true;
                default:
                    csp_service_handler(csp_conn, csp_packet);
                    break;
            }
        }
    }
    return false;
}

#endif
