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
#include <csp/drivers/socket.h>
#include <csp/interfaces/csp_if_socket.h>

#include <inttypes.h>
#include <stdint.h>

#include <arpa/inet.h>
#include <sys/socket.h>

#define LOCAL_ADDRESS "127.0.0.1"
#define SERVER_MAX_CONNECTIONS 5

/**
 * Low level init of server socket connection. Includes waiting on client connection
 * @param socket_iface socket interface to store socket handle in
 * @param port port to accept connection on
 * @return int CSP_ERR_NONE if success, otherwise CSP_ERR_DRIVER
 */
static int socket_server_init(csp_socket_handle_t * socket_iface, uint16_t port);

/**
 * Low level init of client server connection. Includes connecting to server socket
 * @param socket_iface socket interface to store socket handle in
 * @param part port to connect to
 * @param addr address to connect to
 * @return int CSP_ERR_NONE if success, otherwise CSP_ERR_DRIVER
 */
static int socket_client_init(csp_socket_handle_t * socket_iface, uint16_t port);

static int socket_server_init(csp_socket_handle_t * socket_iface, uint16_t port) {
	int socket_handle;
	static int server_socket;
	static struct sockaddr_in server;
	static bool server_init = false;

	if (socket_iface == NULL) {
		return CSP_ERR_DRIVER;
	}

	if (server_init == false) {
		server_socket = socket(AF_INET, SOCK_STREAM, IPPROTO_SCTP);
		if (server_socket == -1) {
			csp_log_error("Failed to init socket\n");
			return CSP_ERR_DRIVER;
		}
		if (setsockopt(server_socket, SOL_SOCKET, SO_REUSEADDR, &(int){ 1 }, sizeof(int)) < 0) {
			csp_log_error("setsockopt(SO_REUSEADDR) failed");
			return CSP_ERR_DRIVER;
		}
		// For now we will only accept local socket connections
		server.sin_addr.s_addr = inet_addr(LOCAL_ADDRESS);
		server.sin_family = AF_INET;
		server.sin_port = htons(port);
		if (bind(server_socket, (struct sockaddr *)&server, sizeof(server)) < 0) {
			csp_log_error("Failed to bind\n");
			return CSP_ERR_DRIVER;
		}

		if (listen(server_socket, SERVER_MAX_CONNECTIONS) < 0) {
			csp_log_error("Failed to listen\n");
			return CSP_ERR_DRIVER;
		}
		server_init = true;
	}

	socket_handle = accept(server_socket, NULL, NULL);
	if (socket_handle < 0) {
		csp_log_error("Accept failed %d\n", socket_handle);
		return CSP_ERR_DRIVER;
	}
	socket_iface->socket_handle = socket_handle;
	socket_iface->is_active = true;
	return CSP_ERR_NONE;
}

static int socket_client_init(csp_socket_handle_t * socket_iface, uint16_t port) {
	int socket_handle;
	struct sockaddr_in server;

	if (socket_iface == NULL)
		return CSP_ERR_DRIVER;

	//Create socket
	socket_handle = socket(AF_INET, SOCK_STREAM, IPPROTO_SCTP);
	if (socket_handle == -1) {
		csp_log_error("Could not create socket");
		return CSP_ERR_DRIVER;
	}

	server.sin_addr.s_addr = inet_addr(LOCAL_ADDRESS);
	server.sin_family = AF_INET;
	server.sin_port = htons(port);

	//Connect to remote server
	if (connect(socket_handle, (struct sockaddr *)&server, sizeof(server)) != 0) {
		csp_log_error("Connect failed. Error");
		return CSP_ERR_DRIVER;
	}
	socket_iface->socket_handle = socket_handle;
	socket_iface->is_active = true;
	return CSP_ERR_NONE;
}

int socket_init(csp_socket_handle_t * socket_iface, uint8_t mode, uint16_t port) {
	if (socket_iface == NULL) {
		return CSP_ERR_DRIVER;
	}

	if (mode == CSP_SOCKET_SERVER) {
		return socket_server_init(socket_iface, port);
	} else if (mode == CSP_SOCKET_CLIENT) {
		return socket_client_init(socket_iface, port);
	}
	return CSP_ERR_DRIVER;
}

int socket_close(csp_socket_handle_t * socket_driver) {
	if (socket_driver == NULL) {
		return CSP_ERR_DRIVER;
	}

	if (!socket_driver->is_active) {
		return CSP_ERR_NONE;
	}

	socket_driver->is_active = false;

	if (shutdown(socket_driver->socket_handle, SHUT_RDWR) != 0) {
		return CSP_ERR_DRIVER;
	}

	if (close(socket_driver->socket_handle) != 0) {
		return CSP_ERR_DRIVER;
	}

	return CSP_ERR_NONE;
}

int socket_status(const csp_socket_handle_t * socket_iface) {
	int error;
	int retval;
	socklen_t len;

	retval = getsockopt(socket_iface->socket_handle, SOL_SOCKET, SO_ERROR, &error, &len);
	if ((retval != 0) || (error != 0)) {
		csp_log_error("Socket status ret %d error %d", retval, error);
		return CSP_ERR_DRIVER;
	}
	return CSP_ERR_NONE;
}