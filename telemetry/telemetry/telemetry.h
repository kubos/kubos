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
#ifndef TELEMETRY_H
#define TELEMETRY_H

#include "telemetry/types.h"
#include <csp/arch/csp_thread.h>

/**
 * Task used to create, accept and store connections from subscribers.
 */
CSP_DEFINE_TASK(telemetry_get_subs);

/**
 * Macro to be used in main() for creating neccesary telemetry threads.
 */
#define TELEMETRY_THREADS   csp_thread_handle_t telem_sub_handle; \
                            csp_thread_create(telemetry_get_subs, "TELEM_SUBS", 1000, NULL, 0, &telem_sub_handle);

/**
 * Performs basic telemetry connection and thread initialization. 
 * To be used in the main() prior to starting the scheduler.
 */
void telemetry_init();

/**
 * Subscribes to the telemetry system.
 * @param conn pointer to telemetry_conn which will be used to receive future telemetry data
 * @param sources bitmask of sources to subscribe to
 * @return bool true if successful, otherwise false
 */
bool telemetry_subscribe(telemetry_conn * conn, uint8_t sources);

/**
 * Reads a telemetry packet from the telemetry server.
 * @param conn telemetry_connection to use for the request
 * @param packet pointer to telemetry_packet to store data in.
 * @return bool true if successful, otherwise false 
 */
bool telemetry_read(telemetry_conn conn, telemetry_packet * packet);

/**
 * Public facing telemetry input interface. Takes a telemetry_packet packet
 * and passes it through the telemetry system.
 * @param packet telemetry_packet to publish
 * @return bool true if successful, otherwise false
 */
bool telemetry_publish(telemetry_packet packet);

#endif
