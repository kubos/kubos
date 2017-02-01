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
 *
 */
int socket_init(csp_socket_handle_t * socket_iface, uint8_t mode, uint16_t port, char * addr);

/**
 *
 */
int socket_status(const csp_socket_handle_t * socket_iface);

#endif /* SOCKET_H_ */

/* @} */