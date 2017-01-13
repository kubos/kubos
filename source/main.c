/*
 * KubOS Linux
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

#include <errno.h>
#include <stdlib.h>
#include <stdio.h>

#include <csp/csp.h>
#include <csp/arch/csp_thread.h>

/* Example Interface */
#define MY_ADDRESS 1
#define MY_PORT    10

CSP_DEFINE_TASK(csp_server) {

    csp_conn_t *conn;
    csp_packet_t *packet;

    /* Create socket without any socket options */
    csp_socket_t *sock = csp_socket(CSP_SO_NONE);

    /* Bind all ports to socket */
    csp_bind(sock, CSP_ANY);

    /* Create 10 connections backlog queue */
    csp_listen(sock, 10);

    /* Process incoming connections */
    while (1) {

        /* Wait for connection, 100 ms timeout */
        if ((conn = csp_accept(sock, 100)) == NULL)
            continue;

        /* Read packets. Timeout is 100 ms */
        while ((packet = csp_read(conn, 100)) != NULL) {
            switch (csp_conn_dport(conn)) {
                case MY_PORT:
                    /* Process packet here */
                	printf("Packet received on MY_PORT: %s\r\n", (char *) packet->data);
                    csp_buffer_free(packet);
                    break;

                default:
                    /* Let the service handler reply pings, buffer use, etc. */
                    csp_service_handler(conn, packet);
                    break;
            }
        }

        /* Close current connection, and handle next */
        csp_close(conn);

    }

    return CSP_TASK_RETURN;
}

CSP_DEFINE_TASK(csp_client) {

    csp_packet_t * packet;
    csp_conn_t * conn;
    int result = 0;

    while (1) {

        /**
         * Try ping
         */
        csp_sleep_ms(200);

        result = csp_ping(MY_ADDRESS, 100, 100, CSP_O_NONE);

        printf("Ping result %d [ms]\n", result);

        csp_sleep_ms(1000);

        /**
          * Try data packet to server
          */

        /* Get packet buffer for data */
        packet = csp_buffer_get(100);
        if (packet == NULL) {
            /* Could not get buffer element */
        	printf("Failed to get buffer element\n");
            return CSP_TASK_RETURN;
        }

        /* Connect to host HOST, port PORT with regular UDP-like protocol and 1000 ms timeout */
        conn = csp_connect(CSP_PRIO_NORM, MY_ADDRESS, MY_PORT, 100, CSP_O_NONE);
        if (conn == NULL) {
            /* Connect failed */
        	printf("Connection failed\n");
            /* Remember to free packet buffer */
            csp_buffer_free(packet);
            return CSP_TASK_RETURN;
        }

        /* Copy dummy data to packet */
        char *msg = "Hello World";
        strcpy((char *) packet->data, msg);

        /* Set packet length */
        packet->length = strlen(msg);

        /* Send packet */
        if (!csp_send(conn, packet, 100)) {
            /* Send failed */
        	printf("Send failed\n");
            csp_buffer_free(packet);
        }

        /* Close connection */
        csp_close(conn);
    }

    return CSP_TASK_RETURN;
}

int main(void)
{

	/* Initialize CSP
     * Not interfacing to any external devices, so we don't need to register
     * a route
     */
	printf("Initializing CSP\n");

    csp_buffer_init(5, 256);
    csp_init(MY_ADDRESS);
    csp_route_start_task(500, 1);

    /* Initialize example threads */
    printf("Starting example tasks\n");

    csp_thread_handle_t handle_server;
    csp_thread_handle_t handle_client;

    csp_thread_create(csp_server, "CSPSRV", 1000, NULL, 0, &handle_server);
    csp_thread_create(csp_client, "CSPCLI", 1000, NULL, 0, &handle_client);

    while (1)
    {
    	csp_sleep_ms(100000);
    }

    return 0;
}
