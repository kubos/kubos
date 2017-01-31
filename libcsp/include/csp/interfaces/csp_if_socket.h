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
 * @defgroup SocketInterface
 * @addtogroup SocketInterface
 * @{
 */

#ifndef _CSP_IF_SOCKET_H
#define _CSP_IF_SOCKET_H

#ifdef __cplusplus
extern "C" {
#endif

#include <stdint.h>

#include <csp/csp.h>
#include <csp/csp_interface.h>

extern csp_iface_t csp_if_socket;

typedef enum {
    CSP_SOCKET_SERVER = 0,
    CSP_SOCKET_CLIENT
} csp_if_socket_modes;

int csp_socket_init(uint8_t mode, uint16_t port, char * addr);

#ifdef __cplusplus
} /* extern "C" */
#endif

#endif /* _CSP_IF_SOCKET_H */

/* @} */