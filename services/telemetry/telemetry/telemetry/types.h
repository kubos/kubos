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
 * @defgroup Types
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
    int topic_id;
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

typedef enum
{
    MESSAGE_TYPE_PACKET = 0,
    MESSAGE_TYPE_SUBSCRIBE,
    MESSAGE_TYPE_UNSUBSCRIBE,
    MESSAGE_TYPE_DISCONNECT
} telemetry_message_type;

typedef enum
{
    RESPONSE_OK = 0,
    RESPONSE_ERR
} telemetry_response_type;

/* Structure for storing a list of telemetry sources */
typedef struct topic_list_item
{
    uint16_t topic_id;
    struct topic_list_item * next;
} topic_list_item;

/* Structure for storing telemetry subscribers in a list */
typedef struct subscriber_list_item
{
    bool active;
    uint16_t id;
    socket_conn conn;
    csp_queue_handle_t packet_queue;
    topic_list_item * topics;
    csp_thread_handle_t rx_thread;
    struct subscriber_list_item * next;
} subscriber_list_item;


#endif

/* @} */