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
 * @defgroup Telemetry General Telemetry API
 * @addtogroup Telemetry
 * @brief Telemetry Public telemetry interface
 * @{
 */

#ifndef TELEMETRY_H
#define TELEMETRY_H

#include "ipc/pubsub.h"
#include "telemetry/config.h"
#include "telemetry/types.h"
#include <csp/arch/csp_thread.h>
#include <stdbool.h>
#include <tinycbor/cbor.h>

/**
 * Performs basic telemetry connection and thread initialization. 
 * To be used in the main() prior to starting the scheduler.
 */
void telemetry_init(void);

/**
 * Performs basic telemetry client connection and thread initialization
 * @note This function is only used by telemetry under Kubos RT
 */
void telemetry_client_init(void);

/**
 * Performs cleanup on telemetry resources & threads.
 */
void telemetry_cleanup(void);

/**
 * Connects to the telemetry system - thread safe version.
 * @return socket_conn* to socket_conn which will be used to receive future telemetry data.
 */
bool telemetry_connect(socket_conn * conn);

/**
 * Internal connect function - not thread safe.
 * @return socket_conn* to socket_conn which will be used to receive future telemetry data
 */
bool kprv_telemetry_connect(socket_conn * conn);

/**
 * Subscribes the socket_conn to the specified topic.
 * @param conn pointer to socket_conn
 * @param topic_id topic to subscribe to
 * @return bool true if successful, otherwise false
 */
bool telemetry_subscribe(const socket_conn * conn, uint16_t topic_id);

/**
 * Disconnects from the telemetry system.
 * @param conn pointer to socket_conn which is to be disconnected
 * @return bool true if successful, otherwise false
 */
bool telemetry_disconnect(socket_conn * conn);

/**
 * Unsubscribes a connection from a specific topic.
 * @param conn pointer to socket_conn which is to be unsubscribed
 * @param topic_id topic to remove subscription from
 * @return bool true if successful, otherwise false
 */
bool telemetry_unsubscribe(const socket_conn * conn, uint16_t topic_id);

/**
 * Reads a telemetry packet from the telemetry server.
 * @param conn socket_connection to use for the request
 * @param packet pointer to telemetry_packet to store data in.
 * @return bool true if successful, otherwise false 
 */
bool telemetry_read(const socket_conn * conn, telemetry_packet * packet);

/**
 * Public facing telemetry input interface. Takes a telemetry_packet packet
 * and passes it through the telemetry system.
 * @param packet telemetry_packet to publish
 * @return bool true if successful, otherwise false
 */
bool telemetry_publish(telemetry_packet packet);

/**
 * @return int number of active telemetry subscribers
 */
int telemetry_num_subscribers(void);

/**
 * Checks if a telemetry client is subscribed to a topic
 * @param client_conn telemetry client
 * @param topic_id topic to check for
 * @return bool true if subscribed, otherwise false
 */
bool telemetry_is_subscribed(const socket_conn * client_conn, int topic_id);

#endif

/* @} */