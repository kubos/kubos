/*
 * Copyright (C) 2016 Kubos Corporation
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
#ifndef COMMUNICATIONS_H
#define COMMUNICATIONS_H

#include <csp/csp.h>
#include <stdint.h>
#include <stdbool.h>

/**
 * PubSub connection structure.
 */
typedef struct
{
    /* Bitmask of sources this connection is subscribed to */
    uint8_t sources;
    /* Raw network connection handle - today this is a csp connection */
    csp_conn_t * conn_handle;
} pubsub_conn;

/**
 * Performs the neccesary setup for the telemetry server to begin
 * receiving subscriber connections.
 * @return bool true if successful, otherwise false
 */
bool server_setup(csp_socket_t ** socket, uint8_t port, uint8_t num_connections);

/**
 * Attempts to accept a subscriber connection.
 * @param conn pointer to pubsub_conn where connection info will be stored 
 * @return bool true if successful, otherwise false
 */
bool server_accept(csp_socket_t ** socket, pubsub_conn * conn);

/**
 * Used by a telemetry subscriber (currently just in telemetry_subscribe)
 * to connect to the telemerty server.
 * @param conn pointer to pubsub_conn where connection info will be stored
 * @return bool true if successful, otherwise false
 */
bool subscriber_connect(pubsub_conn * conn, uint8_t address, uint8_t port);

/**
 * Attempts to receive a telemetry_request over the specified pubsub_conn
 * @param conn pubsub_conn to receive from
 * @param request pointer to telemetry_request to store data in
 * @return bool true if successful, otherwise false
 */ 
bool publisher_read(pubsub_conn conn, void * buffer, int buffer_size, uint8_t port);

/**
 * Attempts to receive a telemetry_packet over the specified pubsub_conn
 * @param conn pubsub_conn to receive from
 * @param packet pointer telemetry_packet to store data in 
 * @return bool true if successful, otherwise false
 */ 
bool subscriber_read(pubsub_conn conn, void * buffer, int buffer_size, uint8_t port);

/**
 * Wrapper function for sending data via a csp connection
 * @param conn pubsub_conn containing a valid csp_conn_t *
 * @param data void pointer to data to be sent
 * @param length length of the data to be sent
 * @return bool true if successful, otherwise false
 */
bool send_csp(pubsub_conn conn, void * data, uint16_t length);

#endif