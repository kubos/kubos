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
 * @defgroup Types Telemetry Types
 * @addtogroup Types
 * @brief Public telemetry types
 * @{
 */

#ifndef TYPES_H
#define TYPES_H

#include <stdint.h>
#include <csp/csp.h>
#include <csp/arch/csp_queue.h>
#include <csp/arch/csp_thread.h>
#include <ipc/socket.h>

/**
 * Telemetry union for storing data.
 */
typedef union
{
    /*! Storage for integer data */
    int i;
    /*! Storage for float data */
    float f;
} telemetry_union;

/**
 * Telemetry data types - each enum corresponds to a
 * a member of telemetry_union.
 */
typedef enum
{
    /*! Indicates stored integer data */
    TELEMETRY_TYPE_INT = 0,
    /*! Indicates stored float data */
    TELEMETRY_TYPE_FLOAT
} telemetry_data_type;

/**
 * Telemetry source structure.
 */
typedef struct
{
    /*! Source identifier - used for subscribing */
    uint16_t topic_id;
    /*! Data type identifier */    
    telemetry_data_type data_type;
    /*! Subsystem identifier */
    int subsystem_id;
} telemetry_source;

/**
 * Basic telemetry packet structure - encapsulating routing information
 * and data.
 */
typedef struct
{
    /*! Telemetry source structure */
    telemetry_source source;
    /*! Data payload */
    telemetry_union data;
    /*! Timestamp indicating packet creation time */
    int timestamp;
} telemetry_packet;

/**
 * Telemetry message types. Used for serializing/deserializing messages
 */
typedef enum
{
    /*! Message containing data */
    MESSAGE_TYPE_PACKET = 0,
    /*! Message containing subscribe request */
    MESSAGE_TYPE_SUBSCRIBE,
    /*! Message containing unsubscribe request */
    MESSAGE_TYPE_UNSUBSCRIBE,
    /*! Message containing disconnect request */
    MESSAGE_TYPE_DISCONNECT
} telemetry_message_type;

/**
 * Telemetry response status
 */
typedef enum
{
    /*! Indicates valid response */
    RESPONSE_OK = 0,
    /*! Indicates erroneous respnose */
    RESPONSE_ERR
} telemetry_response_type;

/**
 * Structure for storing a list of telemetry sources 
 */
typedef struct topic_list_item
{
    /*! Topic id */
    uint16_t topic_id;
    /*! Next topic list item */
    struct topic_list_item * next;
} topic_list_item;

/**
 * Structure for storing telemetry subscribers in a list
 */
typedef struct subscriber_list_item
{
    /*! Subscriber active flag */
    bool active;
    /*! Subscriber id */
    uint16_t id;
    /*! Handle for tcp socket connection */
    socket_conn conn;
    /*! Internal packet queue */
    csp_queue_handle_t packet_queue;
    /*! Pointer to list of subscribed topics */
    topic_list_item * topics;
    /*! Handle for subscriber's message receive thread */
    csp_thread_handle_t rx_thread;
    /*! Next subscriber in list */
    struct subscriber_list_item * next;
} subscriber_list_item;


#endif

/* @} */