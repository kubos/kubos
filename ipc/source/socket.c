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

#include "ipc/config.h"
#include "ipc/pubsub_socket.h"

#include <arpa/inet.h>
#include <stddef.h>
#include <sys/socket.h>
#include <unistd.h>

#define LOCAL_ADDRESS "127.0.0.1"

bool kprv_socket_server_setup(socket_conn * conn, uint16_t port, uint8_t num_connections)
{
    if (conn == NULL)
    {
        return false;
    }

    if ((conn->socket_handle = socket(AF_INET, SOCK_STREAM, IPPROTO_SCTP)) == -1)
    {
        return false;
    }

    conn->socket_addr.sin_addr.s_addr = inet_addr(LOCAL_ADDRESS);
    conn->socket_addr.sin_family = AF_INET;
    conn->socket_addr.sin_port = htons(port);

    if (bind(conn->socket_handle, (struct sockaddr *)&(conn->socket_addr), sizeof(struct sockaddr_in)) < 0)
    {
        return false;
    }

    if (listen(conn->socket_handle, num_connections) < 0)
    {
        return false;
    }

    conn->is_active = true;

    return true;
}

bool kprv_socket_server_accept(const socket_conn * server_conn, socket_conn * client_conn)
{
    if ((server_conn == NULL) || (client_conn == NULL))
    {
        return false;
    }

    int socket_handle = accept(server_conn->socket_handle, NULL, NULL);
    if (socket_handle < 0)
    {
        client_conn->is_active = false;
        return false;
    }

    client_conn->socket_handle = socket_handle;

    client_conn->is_active = true;

    return true;
}

bool kprv_socket_client_connect(socket_conn * conn, uint16_t port)
{
    int socket_handle;
    struct sockaddr_in server;

    if (conn == NULL)
    {
        return false;
    }

    socket_handle = socket(AF_INET, SOCK_STREAM, IPPROTO_SCTP);
    if (socket_handle == -1)
    {
        conn->is_active = false;
        return false;
    }

    server.sin_addr.s_addr = inet_addr(LOCAL_ADDRESS);
    server.sin_family = AF_INET;
    server.sin_port = htons(port);

    //Connect to remote server
    if (connect(socket_handle, (struct sockaddr *)&server, sizeof(server)) != 0)
    {
        conn->is_active = false;
        return false;
    }

    conn->socket_handle = socket_handle;
    conn->is_active = true;

    return true;
}

bool kprv_socket_send(const socket_conn * conn, const uint8_t * data_buffer, uint32_t data_length)
{
    if ((conn == NULL) || (data_buffer == NULL) || (conn->is_active == false))
    {
        return false;
    }

    int result = send(conn->socket_handle, data_buffer, data_length, MSG_NOSIGNAL);
    if (result < 0)
    {
        return false;
    }

    return true;
}

bool kprv_socket_recv(const socket_conn * conn, uint8_t * data_buffer, uint32_t data_length, uint32_t * length_read)
{
    if ((conn == NULL) || (length_read == NULL) || (data_buffer == NULL) || (conn->is_active == false))
    {
        return false;
    }

    int recv_size = recv(conn->socket_handle, (void *)data_buffer, data_length, 0);
    if (recv_size < 0) {
        return false;
    }

    *length_read = recv_size;

    return true;
}

bool kprv_socket_close(socket_conn * conn)
{
    if (conn == NULL)
    {
        return false;
    }

    conn->is_active = false;

    if (shutdown(conn->socket_handle, SHUT_RDWR) != 0)
    {
        return false;
    }

    if (close(conn->socket_handle) != 0)
    {
        return false;
    }

    return true;
}
