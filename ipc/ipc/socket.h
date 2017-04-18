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

/**
 * @defgroup Socket TCP Socket Interface
 * @addtogroup Socket
 * @brief IPC Socket API
 * @{
 */

#pragma once

#include <arpa/inet.h>
#include <stdbool.h>
#include <stdint.h>
#include <sys/socket.h>

/**
 * PubSub connection structure.
 */
typedef struct
{
    /*! Status flag */
    bool is_active;
    /*! Raw network connection handle - today this is a csp connection */
    int socket_handle;
    /*! Socket address info structure */
    struct sockaddr_in socket_addr;
} socket_conn;

/**
 * Performs the low level init and setup of the server side tcp socket
 * @param [out] conn pointer to socket_conn where connection info will be stored
 * @param [in] port port to listen for connections on
 * @param [in] num_connections max number of connections to listen for
 * @return bool true if successful, otherwise false
 */
bool kprv_socket_server_setup(socket_conn * conn, uint16_t port, uint8_t num_connections);

/**
 * Attempts to accept a new socket connection - currently blocking.
 * @param [in] server_conn pointer to socket_conn with listening socket info
 * @param [out] client_conn pointer to socket_conn where accepted connection info is stored
 * @return bool true if successful, otherwise false
 */
bool kprv_socket_server_accept(const socket_conn * server_conn, socket_conn * client_conn);

/**
 * Attempts to open a socket connection
 * @param [out] conn pointer to socket_conn where connection info will be stored
 * @param [in] port port to connect on
 * @return bool true if successful, otherwise false
 */
bool kprv_socket_client_connect(socket_conn * conn, uint16_t port);

/**
 * Performs low level shutdown and closure of connection
 * @param [in,out] conn pointer to socket_conn
 * @return bool true if successful, otherwise false
 */
bool kprv_socket_close(socket_conn * conn);

/**
 * Performs socket send
 * @param [in] conn pointer to socket_conn
 * @param [in] data_buffer data to send
 * @param [in] data_length length of data to send
 * @return bool true if successful, otherwise false
 */
bool kprv_socket_send(const socket_conn * conn, const uint8_t * data_buffer, uint32_t data_length);

/**
 * Performs socket receive
 * @param [in] conn pointer to socket_conn
 * @param [out] data_buffer buffer to write received data to
 * @param [in] data_length max size of data buffer
 * @param [out] length_read number of bytes actually received
 * @return bool true if successful, otherwise false
 */
bool kprv_socket_recv(const socket_conn * conn, uint8_t * data_buffer, uint32_t data_length, uint32_t * length_read);

/* @} */