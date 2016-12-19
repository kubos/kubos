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

#include "telemetry/telemetry.h"
#include <stdbool.h>

/**
 * Performs the neccesary setup for the telemetry server to begin
 * receiving subscriber connections.
 * @return bool true if successful, otherwise false
 */
bool server_setup();

/**
 * Attempts to accept a subscriber connection.
 * @param conn pointer to telemetry_conn where connection info will be stored 
 * @return bool true if successful, otherwise false
 */
bool server_accept(telemetry_conn * conn);

/**
 * Used by a telemetry subscriber (currently just in telemetry_subscribe)
 * to connect to the telemerty server.
 * @param conn pointer to telemetry_conn where connection info will be stored
 * @return bool true if successful, otherwise false
 */
bool subscriber_connect(telemetry_conn * conn);

/**
 * Attempts to receive a telemetry_request over the specified telemetry_conn
 * @param conn telemetry_conn to receive from
 * @param request pointer to telemetry_request to store data in
 * @return bool true if successful, otherwise false
 */ 
bool publisher_read_request(telemetry_conn conn, telemetry_request * request);

/**
 * Attempts to receive a telemetry_packet over the specified telemetry_conn
 * @param conn telemetry_conn to receive from
 * @param packet pointer telemetry_packet to store data in 
 * @return bool true if successful, otherwise false
 */ 
bool subscriber_read_packet(telemetry_conn conn, telemetry_packet * packet);

/**
 * Sends a telemetry_packet over the specified telemetry_conn
 * @param conn telemetry_conn to send packet over
 * @param packet telemetry_packet to send
 * @return bool true if successful, otherwise false
 */ 
bool send_packet(telemetry_conn conn, telemetry_packet packet);

/**
 * Sends a telemetry_request over the specified telemetry_conn
 * @param conn telemetry_conn to send request over
 * @param request telemetry_request to send
 * @return bool true if successful, otherwise false
 */ 
bool send_request(telemetry_conn conn, telemetry_request request);

#endif