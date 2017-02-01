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

#include <csp/csp.h>
#include <csp/csp_interface.h>
#include <csp/csp_error.h>
#include <csp/interfaces/csp_if_socket.h>
#include <csp/drivers/socket.h>
#include <csp/arch/csp_thread.h>

#include <inttypes.h>
#include <stdint.h>

#include <sys/socket.h>
#include <arpa/inet.h>

#define BUF_SIZE 250

/**
 *
 */
static int csp_socket_tx(csp_iface_t *ifc, csp_packet_t *packet, uint32_t timeout);


/**
 *
 */
CSP_DEFINE_TASK(csp_socket_rx);

static int csp_socket_tx(csp_iface_t *ifc, csp_packet_t *packet, uint32_t timeout) {
    csp_log_info("csp_socket_tx\r\n");
    if ((ifc == NULL) || (ifc->driver == NULL)) {
        csp_log_error("Null pointer for interface or driver\r\n");
        return CSP_ERR_DRIVER;
    }

    csp_socket_handle_t * socket_driver = ifc->driver;

    csp_log_info("now we write\r\n");
    /* Write packet to socket */
    int result = write(socket_driver->socket_handle, &packet->length, packet->length + sizeof(uint32_t) + sizeof(uint16_t)); 
    if ( result < 0) {
        csp_log_error("Socket write error: %u %s\r\n", result, strerror(result));
    }
    csp_buffer_free(packet);
    return CSP_ERR_NONE;
}

CSP_DEFINE_TASK(csp_socket_rx) {
    csp_iface_t socket_interface;
    csp_socket_handle_t * socket_driver;
    csp_packet_t * packet = csp_buffer_get(BUF_SIZE);

    if (param == NULL) {
        csp_log_error("No socket param found\r\n");
        return CSP_TASK_RETURN;
    }
    socket_interface = *((csp_iface_t*)param);

    if (socket_interface.driver == NULL) {
        csp_log_error("No socket driver found\r\n");
        return CSP_TASK_RETURN;
    }

    socket_driver = socket_interface.driver;
        
    while(recv(socket_driver->socket_handle, &packet->length, BUF_SIZE, 0) > 0) {
        csp_new_packet(packet, &socket_interface, NULL);

        packet = csp_buffer_get(BUF_SIZE);
        if (packet == NULL) {
            break;
        }
    }

    return CSP_TASK_RETURN;
}

int csp_socket_init(csp_iface_t * socket_iface, csp_socket_handle_t * socket_driver) {
    if ((socket_iface == NULL) || (socket_driver == NULL))
        return CSP_ERR_DRIVER;

    socket_iface->driver = socket_driver;
    socket_iface->nexthop = csp_socket_tx;
    socket_iface->name = "socket";
    socket_iface->mtu = BUF_SIZE;

    /* Start RX thread */
	static csp_thread_handle_t handle_rx;
	int ret = csp_thread_create(csp_socket_rx, "SOCKET_RX", 1000, socket_iface, 0, &handle_rx);
	csp_log_info("Task start %d\r\n", ret);

    /* Register interface */
    csp_iflist_add(socket_iface);

    return CSP_ERR_NONE;
}
