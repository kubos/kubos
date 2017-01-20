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
#ifndef TELEMETRY_H
#define TELEMETRY_H

#include "ipc/pubsub.h"
#include "telemetry/config.h"
#include "telemetry/types.h"
#include <csp/arch/csp_thread.h>
#include <stdbool.h>

/**
 * Task used to receive incoming data from telemetry publishers.
 */
CSP_DEFINE_TASK(telemetry_rx_task);

/**
 * Performs basic telemetry connection and thread initialization. 
 * To be used in the main() prior to starting the scheduler.
 */
void telemetry_init();

void telemetry_cleanup();

/**
 * Subscribes to the telemetry system.
 * @param conn pointer to pubsub_conn which will be used to receive future telemetry data
 * @param sources bitmask of sources to subscribe to, a value of 0 will subscribe to all
 * @return bool true if successful, otherwise false
 */
bool telemetry_subscribe(pubsub_conn * conn, uint8_t sources);

void telemetry_unsubscribe(pubsub_conn * conn);

bool kprv_telemetry_subscribe(pubsub_conn * conn, uint8_t sources);

/**
 * Reads a telemetry packet from the telemetry server.
 * @param conn pubsub_connection to use for the request
 * @param packet pointer to telemetry_packet to store data in.
 * @return bool true if successful, otherwise false 
 */
bool telemetry_read(pubsub_conn conn, telemetry_packet * packet);

/**
 * Public facing telemetry input interface. Takes a telemetry_packet packet
 * and passes it through the telemetry system.
 * @param packet telemetry_packet to publish
 * @return bool true if successful, otherwise false
 */
bool telemetry_publish(telemetry_packet packet);

int telemetry_num_subscribers();

#endif
