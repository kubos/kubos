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
#include <telemetry/telemetry.h>


bool telemetry_csp_init(void)
{
    csp_buffer_init(20, 256);

    /* Init CSP with address MY_ADDRESS */
    csp_init(TELEMETRY_CSP_ADDRESS);

    /* Start router task with 500 word stack, OS task priority 1 */
    csp_route_start_task(500, 1);

    // packet_queue = csp_queue_create(MESSAGE_QUEUE_SIZE, sizeof(telemetry_packet));

    // csp_mutex_create(&subscribing_lock);
    // csp_mutex_create(&unsubscribing_lock);

    // csp_debug_set_level(CSP_ERROR, true);
    // csp_debug_set_level(CSP_WARN, true);
    // csp_debug_set_level(CSP_INFO, true);
    // csp_debug_set_level(CSP_BUFFER, true);
    // csp_debug_set_level(CSP_PACKET, true);
    // csp_debug_set_level(CSP_PROTOCOL, true);
    // csp_debug_set_level(CSP_LOCK, true);
}

telemetry_csp_terminate(void)
{
    csp_route_end_task();

    csp_terminate();

    csp_buffer_cleanup();
}