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

#include "ipc/pubsub.h"
#include "ipc/config.h"

#include <csp/drivers/socket.h>
#include <csp/interfaces/csp_if_socket.h>


csp_socket_t * kprv_server_setup(uint8_t port, uint8_t num_connections)
{
    csp_socket_t * socket = NULL;

    if ((socket = csp_socket(CSP_SO_NONE)) == NULL)
    {
        return NULL;
    }

    if (csp_bind(socket, port) != CSP_ERR_NONE)
    {
        return NULL;
    }

    if (csp_listen(socket, num_connections) != CSP_ERR_NONE)
    {
        return NULL;
    }

    return socket;
}

bool kprv_server_accept(csp_socket_t * socket, pubsub_conn * conn)
{
    csp_conn_t * csp_conn = NULL;
    if ((socket == NULL) || (conn == NULL))
    {
        return false;
    }
    if ((csp_conn = csp_accept(socket, 1000)) != NULL)
    {
        conn->conn_handle = csp_conn;
        return true;
    }
    return false;
}

bool kprv_server_socket_accept(csp_socket_t * socket, pubsub_conn * conn)
{
    csp_conn_t * csp_conn = NULL;
    if ((socket == NULL) || (conn == NULL))
    {
        return false;
    }

    if (socket_init(&(conn->socket_driver), CSP_SOCKET_SERVER, IPC_SOCKET_PORT) != CSP_ERR_NONE)
    {
        return false;
    }
    if (csp_socket_init(&(conn->csp_socket_if), &(conn->socket_driver)) != CSP_ERR_NONE)
    {
        return false;
    }
    csp_route_set(CSP_DEFAULT_ROUTE, &(conn->csp_socket_if), CSP_NODE_MAC);
    if ((csp_conn = csp_accept(socket, 1000)) != NULL)
    {
        conn->conn_handle = csp_conn;
        return true;
    }

    return false;
}

bool kprv_subscriber_connect(pubsub_conn * conn, uint8_t address, uint8_t port)
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

bool kprv_subscriber_socket_connect(pubsub_conn * conn, uint8_t address, uint8_t port)
{
    csp_conn_t * csp_conn = NULL;
    if (conn == NULL)
    {
        return false;
    }

    if (socket_init(&(conn->socket_driver), CSP_SOCKET_CLIENT, IPC_SOCKET_PORT) != CSP_ERR_NONE)
    {
        return false;
    }

    if (csp_socket_init(&(conn->csp_socket_if), &(conn->socket_driver)) != CSP_ERR_NONE)
    {
        return false;
    }

    csp_route_set(address, &(conn->csp_socket_if), CSP_NODE_MAC);

    csp_conn = csp_connect(CSP_PRIO_NORM, address, port, 1000, CSP_O_NONE);
    if (csp_conn != NULL)
    {
        conn->conn_handle = csp_conn;
        return true;
    }

    conn->conn_handle = NULL;
    return false;
}

void kprv_subscriber_socket_close(pubsub_conn * conn)
{
    if (conn != NULL)
    {
        csp_close(conn->conn_handle);
        conn->conn_handle = NULL;
        csp_socket_close(&(conn->csp_socket_if), &(conn->socket_driver)); 
    }
}

bool kprv_send_csp(const pubsub_conn * conn, const void * data, uint16_t length)
{
    csp_packet_t * csp_packet = NULL;
    csp_conn_t * csp_conn = NULL;
    if ((conn != NULL) && (conn->conn_handle != NULL) && (data != NULL) && (length > 0))
    {
        csp_conn = conn->conn_handle;
        if ((csp_packet = csp_buffer_get(length)) != NULL)
        {
            memcpy(csp_packet->data, data, length);
            csp_packet->length = length;
            if (!csp_send(csp_conn, csp_packet, IPC_SEND_TIMEOUT))
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

bool kprv_publisher_read(const pubsub_conn * conn, void * buffer, int buffer_size, uint8_t port)
{
    csp_packet_t * csp_packet = NULL;
    csp_conn_t * csp_conn = NULL;
    if ((conn != NULL) && (conn->conn_handle != NULL) && (buffer != NULL))
    {
        csp_conn = conn->conn_handle;
        if ((csp_packet = csp_read(csp_conn, IPC_READ_TIMEOUT)) != NULL)
        {
            if (csp_conn_dport(csp_conn) == port)
            {
                memcpy(buffer, (void*)csp_packet->data, buffer_size);
                csp_buffer_free(csp_packet);
                return true;
            }
            csp_service_handler(csp_conn, csp_packet);
        }
    }
    return false;
}

bool kprv_subscriber_read(const pubsub_conn * conn, void * buffer, int buffer_size, uint8_t port)
{
    csp_packet_t * csp_packet = NULL;
    csp_conn_t * csp_conn = NULL;
    if ((conn != NULL) && (conn->conn_handle != NULL) && (buffer != NULL))
    {
        csp_conn = conn->conn_handle;
        if ((csp_packet = csp_read(csp_conn, IPC_READ_TIMEOUT)) != NULL)
        {
            if (csp_conn_sport(csp_conn) == port)
            {
                memcpy(buffer, (void*)csp_packet->data, buffer_size);
                csp_buffer_free(csp_packet);
                return true;
            }
            csp_service_handler(csp_conn, csp_packet);
        }
    }
    return false;
}