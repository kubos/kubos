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
#ifndef TYPES_H
#define TYPES_H

#include <stdint.h>
#include <csp/csp.h>

/**
 * Telemetry union for storing data.
 */
typedef union
{
    int i;
    float f;
} telemetry_union;

/**
 * Telemetry data types - each enum corresponds to a
 * a member of telemetry_union.
 */
typedef enum {
    TELEMETRY_TYPE_INT = 0,
    TELEMETRY_TYPE_FLOAT
} telemetry_data_type;

/**
 * Telemetry source structure.
 */
typedef struct
{
    /* Source identifier - used for subscribing */
    uint8_t source_id;
    /* Data type identifier */    
    telemetry_data_type data_type;
} telemetry_source;

/**
 * Basic telemetry packet structure - encapsulating routing information
 * and data.
 */
typedef struct
{
    telemetry_source source;
    telemetry_union data;
    uint16_t timestamp;
} telemetry_packet;

/**
 * Telemetry connection structure.
 */
typedef struct
{
    /* Bitmask of sources this connection is subscribed to */
    uint8_t sources;
    /* Actual connection handle - today this is a csp connection */
    csp_conn_t * conn_handle;
} telemetry_conn;

/**
 * Telemetry request structure - currently used for subscribing to sources.
 */
typedef struct
{
    uint8_t sources;
} telemetry_request;

#endif