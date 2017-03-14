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

#include <csp/arch/csp_thread.h>
#include <csp/csp.h>
#include <csp/csp_error.h>
#include <csp/csp_interface.h>
#include <csp/drivers/socket.h>
#include <csp/interfaces/csp_if_socket.h>
#include <inttypes.h>
#include <stdint.h>
#include <stdlib.h>

#include <arpa/inet.h>
#include <sys/socket.h>

#include <tinycbor/cbor.h>

/**
 * Callback function used by CSP to transmit packets
 * @param ifc csp socket interface
 * @param packet packet to send
 * @param timeout currently not used
 * @return int currently always returns CSP_ERR_NONE
 */

/**
 * Task spawned for each new csp_if_socket for handling receiving data
 */
CSP_DEFINE_TASK(csp_socket_rx);

int csp_socket_tx(struct csp_iface_s * ifc, csp_packet_t * packet, uint32_t timeout) {
	if ((ifc == NULL) || (ifc->driver == NULL)) {
		csp_log_error("Null pointer for interface or driver\r\n");
		return CSP_ERR_DRIVER;
	}

	csp_log_info("csp_socket_tx go\r\n");

	csp_socket_handle_t * socket_driver = ifc->driver;

	csp_log_info("Is active? %d", socket_driver->is_active);

	uint8_t write_buffer[SOCKET_BUFFER_SIZE];
	int write_size = cbor_encode_csp_packet(packet, write_buffer);
	if (write_size > 0) {
		csp_log_info("about to write csp packet %d - %d\r\n", write_size, packet->length);
		int result = send(socket_driver->socket_handle, write_buffer, write_size, MSG_NOSIGNAL);
		csp_log_info("csp_socket_tx write %d\r\n", result);
		if (result < 0) {
			csp_log_error("Socket write error: %u %s\r\n", result, strerror(result));
			return CSP_ERR_DRIVER;
		}
		csp_buffer_free(packet);
	} else {
		csp_log_error("encode csp packet failed\r\n");
	}

	return CSP_ERR_NONE;
}

CSP_DEFINE_TASK(csp_socket_rx) {
	csp_iface_t socket_interface;
	csp_socket_handle_t * socket_driver;
	csp_packet_t * packet = csp_buffer_get(SOCKET_BUFFER_SIZE);

	if (param == NULL) {
		csp_log_error("No socket param found\r\n");
		csp_thread_exit();
	}

	memcpy(&socket_interface, (csp_iface_t *)param, sizeof(csp_iface_t));

	if (socket_interface.driver == NULL) {
		csp_log_error("No socket driver found\r\n");
		csp_thread_exit();
	}

	socket_driver = socket_interface.driver;

	char buffer[SOCKET_BUFFER_SIZE];
	while (socket_driver->is_active) {
		memset(buffer, '\0', SOCKET_BUFFER_SIZE);
		int recv_size = recv(socket_driver->socket_handle, (void *)buffer, SOCKET_BUFFER_SIZE, 0);
		if (recv_size > 0) {
			if (cbor_parse_csp_packet(packet, buffer, recv_size)) {
				csp_new_packet(packet, &socket_interface, NULL);
				packet = csp_buffer_get(SOCKET_BUFFER_SIZE);
				if (packet == NULL) {
					csp_log_error("Out of packet buffers\r\n");
					break;
				}
			} else {
				csp_log_error("Could not parse out csp packet");
			}
		}
	}

	csp_buffer_free(packet);

	csp_thread_exit();
}

int csp_socket_init(csp_iface_t * socket_iface, csp_socket_handle_t * socket_driver) {
	if ((socket_iface == NULL) || (socket_driver == NULL)) {
		return CSP_ERR_DRIVER;
    }

	socket_iface->driver = socket_driver;
	socket_iface->nexthop = csp_socket_tx;
	socket_iface->name = "socket";
	socket_iface->mtu = SOCKET_BUFFER_SIZE;

	/* Start RX thread */
	if (csp_thread_create(csp_socket_rx, "SOCKET_RX", 1000, socket_iface, 0, &(socket_driver->rx_thread_handle)) != 0) {
		return CSP_ERR_DRIVER;
	}

	/* Register interface */
	csp_iflist_add(socket_iface);

	return CSP_ERR_NONE;
}

int csp_socket_close(csp_iface_t * socket_iface, csp_socket_handle_t * socket_driver) {
	if ((socket_iface == NULL) || (socket_driver == NULL)) {
		return CSP_ERR_DRIVER;
	}

	socket_close(socket_driver);

	csp_thread_kill((socket_driver->rx_thread_handle));

	return CSP_ERR_NONE;
}
