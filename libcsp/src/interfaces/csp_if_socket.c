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
#include <csp/arch/csp_thread.h>

#include <inttypes.h>
#include <stdint.h>

#include <sys/socket.h>
#include <arpa/inet.h>

#define BUF_SIZE 250

/* Currently handling a single socket connection per csp instance */
static int csp_socket_handle = 0;
static int server_socket, c;
static struct sockaddr_in server, client;

int csp_socket_tx(csp_iface_t *ifc, csp_packet_t *packet, uint32_t timeout) {
    /* Write packet to socket */
    int result = write(csp_socket_handle, &packet->length, packet->length + sizeof(uint32_t) + sizeof(uint16_t)); 
    if ( result < 0)
    {
        csp_log_error("Socket write error: %u %s\r\n", result, strerror(result));
    }
    csp_buffer_free(packet);
    return CSP_ERR_NONE;
}

CSP_DEFINE_TASK(csp_socket_rx) {
    csp_packet_t * packet = csp_buffer_get(BUF_SIZE);
        
    while(recv(csp_socket_handle, &packet->length, BUF_SIZE, 0) > 0) {
        csp_new_packet(packet, &csp_if_socket, NULL);

        packet = csp_buffer_get(BUF_SIZE);
        if (packet == NULL) {
            break;
        }
    }

    return CSP_TASK_RETURN;
}

int csp_socket_init(uint8_t mode, uint16_t port, char * addr) {
    /* Init actual socket */
    if (mode == CSP_SOCKET_SERVER) {
        server_socket = socket(AF_INET, SOCK_STREAM, 0);
        if (server_socket == -1) {
            csp_log_error("Failed to init socket\n");
            return CSP_ERR_DRIVER;
        }
        server.sin_addr.s_addr = inet_addr("127.0.0.1");
        server.sin_family = AF_INET;
        server.sin_port = htons(port);
        if (bind(server_socket, (struct sockaddr*)&server, sizeof(server)) < 0) {
            csp_log_error("Failed to bind\n");
            return CSP_ERR_DRIVER;
        }

        listen(server_socket, 3);
        c = sizeof(struct sockaddr_in);

        csp_log_info("Wait to accept\n");
        csp_socket_handle = accept(server_socket, (struct sockaddr *)&client, (socklen_t*)&c);
        if (csp_socket_handle < 0) {
            csp_log_error("Accept failed\n");
            return CSP_ERR_DRIVER;
        }
        csp_log_info("Accepted!\n");
    }
    else if (mode == CSP_SOCKET_CLIENT) {
        //Create socket
        csp_socket_handle = socket(AF_INET, SOCK_STREAM, 0);
        if (csp_socket_handle == -1) {
            csp_log_error("Could not create socket");
            return CSP_ERR_DRIVER;
        }
        csp_log_info("Socket created");
        
        server.sin_addr.s_addr = inet_addr(addr);
        server.sin_family = AF_INET;
        server.sin_port = htons(port);
    
        //Connect to remote server
        if (connect(csp_socket_handle, (struct sockaddr *)&server , sizeof(server)) < 0) {
            csp_log_error("connect failed. Error");
            return CSP_ERR_DRIVER;
        }
        
        csp_log_info("Connected\n");
    }

    /* Start RX thread */
	static csp_thread_handle_t handle_rx;
	int ret = csp_thread_create(csp_socket_rx, "SOCKET_RX", 1000, NULL, 0, &handle_rx);
	csp_log_info("Task start %d\r\n", ret);

    /* Register interface */
    csp_iflist_add(&csp_if_socket);

    return CSP_ERR_NONE;
}

/** Interface definition **/
csp_iface_t csp_if_socket = {
    .name = "socket",
    .nexthop = csp_socket_tx,
    .mtu = 250
};