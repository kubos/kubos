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
 * @defgroup PubSub
 * @addtogroup PubSub
 * @brief Internal PubSub API
 * @{
 */

#ifndef PUBSUB_H
#define PUBSUB_H

#include <stdbool.h>
#include <stdint.h>

/**
 * PubSub connection structure.
 */
typedef struct
{
    /*! Raw network connection handle - today this is a csp connection */
    int socket_handle;
    bool is_active;
} socket_conn;

bool kprv_socket_server_setup(uint16_t port, uint8_t num_connections);

bool kprv_socket_server_accept(socket_conn * conn);

bool kprv_socket_client_connect(socket_conn * conn, uint16_t port);

bool kprv_socket_send(socket_conn * conn, uint8_t * data_buffer, uint32_t data_length);

bool kprv_socket_recv(socket_conn * conn, uint8_t * data_buffer, uint32_t data_length);

#endif

/* @} */