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
 * @defgroup SocketDriver
 * @addtogroup SocketDriver
 * @{
 */

#ifndef SOCKET_H_
#define SOCKET_H_

#include <csp/interfaces/csp_if_socket.h>

/**
 * Initializes and connects a unix socket, returns socket handle
 * @param socket_iface socket interface to store handle in
 * @param mode CSP_SOCKET_CLIENT or CSP_SOCKET_SERVER, which type of connection is created
 * @param port socket interface port number
 * @return int CSP_ERR_NONE if successful, otherwise CSP_ERR_DRIVER
 */
int socket_init(csp_socket_handle_t * socket_iface, uint8_t mode, uint16_t port);

int socket_close(csp_socket_handle_t * socket_driver);

/**
 * Attempts to check open/closed status of socket
 * @param socket_iface socket interface containing socket handle
 * @return int CSP_ERR_NONE if socket open, otherwise CSP_ERR_DRIVER
 */
int socket_status(const csp_socket_handle_t * socket_iface);

bool cbor_parse_csp_packet(csp_packet_t * packet, void * buffer, int buffer_size);

int cbor_encode_csp_packet(csp_packet_t * packet, uint8_t * buffer);

#endif /* SOCKET_H_ */

/* @} */