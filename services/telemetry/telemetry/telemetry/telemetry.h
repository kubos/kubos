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
 * @defgroup Telemetry
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

/**
 * Performs basic telemetry connection and thread initialization. 
 * To be used in the main() prior to starting the scheduler.
 */
void telemetry_init(void);

void telemetry_client_init(void);

/**
 * Performs cleanup on telemetry resources & threads.
 */
void telemetry_cleanup(void);

/**
 * Connects to the telemetry system - thread safe version.
 * @return pubsub_conn* to pubsub_conn which will be used to receive future telemetry data.
 */
bool telemetry_connect(pubsub_conn * conn);

/**
 * Internal connect function - not thread safe.
 * @return pubsub_conn* to pubsub_conn which will be used to receive future telemetry data
 */
bool kprv_telemetry_connect(pubsub_conn * conn);

/**
 * Subscribes the pubsub_conn to the specified topic.
 * @param conn pointer to pubsub_conn
 * @param topic_id topic to subscribe to
 * @return bool true if successful, otherwise false
 */
bool telemetry_subscribe(const pubsub_conn * conn, uint16_t topic_id);

/**
 * Disconnects from the telemetry system.
 * @param conn pointer to pubsub_conn which is to be disconnected
 * @return bool true if successful, otherwise false
 */
bool telemetry_disconnect(pubsub_conn * conn);

/**
 * Unsubscribes a connection from a specific topic.
 * @param conn pointer to pubsub_conn which is to be unsubscribed
 * @param topic_id topic to remove subscription from
 * @return bool true if successful, otherwise false
 */
bool telemetry_unsubscribe(const pubsub_conn * conn, uint16_t topic_id);

/**
 * Reads a telemetry packet from the telemetry server.
 * @param conn pubsub_connection to use for the request
 * @param packet pointer to telemetry_packet to store data in.
 * @return bool true if successful, otherwise false 
 */
bool telemetry_read(const pubsub_conn * conn, telemetry_packet * packet);

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
bool telemetry_is_subscribed(const pubsub_conn * client_conn, uint16_t topic_id);

bool telemetry_parse_packet_msg(uint8_t * buffer, int buffer_size, telemetry_packet * packet);

int telemetry_encode_packet_msg(uint8_t * buffer, telemetry_packet * pkt);

bool telemetry_parse_msg_type(uint8_t * buffer, int buffer_size, telemetry_message_type * req);

int telemetry_encode_subscribe_msg(uint8_t * buffer, int * topic_id);
int telemetry_encode_unsubscribe_msg(uint8_t * buffer, int * topic_id);

#endif

/* @} */