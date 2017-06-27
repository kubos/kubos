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
 * @defgroup Telemetry-Server
 * @addtogroup Telemetry-Server
 * @brief Telemetry Server Private Interface
 * @{
 */

#pragma once

#include <ipc/socket.h>
#include <stdbool.h>
#include <stdint.h>
#include <telemetry/telemetry.h>

/**
 * Inits a new subscriber structure
 * @param[in] conn socket_connection used by subscriber
 * @return subscriber_list_item * valid pointer if successful, otherwise NULL
 */
subscriber_list_item * kprv_subscriber_init(socket_conn conn);

/**
 * Adds existing subscriber item to global list
 * @param[in] sub subscriber to add to list
 * @return bool true if successful
 */
bool kprv_subscriber_add(subscriber_list_item * sub);

/**
 * Closes and destroys subscriber structure
 * @param[in,out] sub pointer to subscriber pointer to destroy, will be set to NULL
 */
void kprv_subscriber_destroy(subscriber_list_item ** sub);

/**
 * Adds a topic id to a subscribers list of topics
 * @param[in,out] sub subscriber list item
 * @param[in] topic_id topic id to add
 * @return bool true if successful, false otherwise
 */
bool kprv_subscriber_add_topic(subscriber_list_item * sub, uint16_t topic_id);

/**
 * Removes a topic id from a subscribers list of topics
 * @param[in,out] sub subscriber list item
 * @param[in] topic_id topic id to remove
 * @return bool true if successful, false otherwise
 */
bool kprv_subscriber_remove_topic(subscriber_list_item * sub, uint16_t topic_id);

/**
 * Removes all topics from a subscriber
 * @param[in,out] sub subscriber to remove all topics from
 */
void kprv_subscriber_remove_all_topics(subscriber_list_item * sub);

/**
 * Checks if a subscriber is subscribed to a topic
 * @param[in] sub subscriber_list_item to check for topic
 * @param[in] topic_id topic id to check for
 * @return bool true if topic found, otherwise false
 */
bool kprv_subscriber_has_topic(const subscriber_list_item * sub, uint16_t topic_id);

/**
 * Iterates and removes global list of subscribers
 */
void kprv_delete_all_subscribers();

/**
 * Processes new message from subscriber
 * @param[in,out] sub subscriber to process message with
 * @param[in] buffer buffer to read message from
 * @param[in] buffer_size size of buffer
 * @return bool true if successful, otherwise false
 */
bool telemetry_process_message(subscriber_list_item * sub, const void * buffer, int buffer_size);

/**
 * Performs cleanup of telemetry server stuff
 */
void telemetry_server_cleanup(void);

/**
 * Task for handling communication with client connections
 */
CSP_DEFINE_TASK(client_handler);

/**
 * Performs work of receiving and processing packets from subscribers
 * @param[in] sub subscriber to process data from
 * @return bool true if successful, otherwise false
 */
bool client_rx_work(subscriber_list_item * sub);

/**
 * Attempts to publish telemetry_packet to subscribers
 * @param[in] packet telemetry_packet to publish
 * @return bool true if successful, otherwise false
 */
bool kprv_publish_packet(telemetry_packet packet);

/* @} */