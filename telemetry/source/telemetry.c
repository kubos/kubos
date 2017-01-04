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
// #ifdef YOTTA_CFG_TELEMETRY

#include "telemetry/telemetry.h"
#include "telemetry/config.h"
#include <csp/arch/csp_queue.h>
#include <stdio.h>

/**
 * Iterates though all open telemetry connections and
 * publishes the packet IF the connection is interested/subscribed
 * @param packet telemetry_packet to publish
 */
static void telemetry_send(telemetry_packet packet);

/* Static array for holding persistent connections to telemetry subscribers */
static pubsub_conn telemetry_subs[TELEMETRY_NUM_SUBSCRIBERS];


/* Current number of active telemetry subscribers */
static uint8_t num_subs = 0;

/* Queue for incoming packets from publishers */
static csp_queue_handle_t packet_queue = NULL;

static csp_thread_handle_t telem_sub_handle;
static csp_thread_handle_t telem_rx_handle;

void telemetry_init()
{
    csp_buffer_init(10, 256);

    /* Init CSP with address MY_ADDRESS */
    csp_init(TELEMETRY_CSP_ADDRESS);

    /* Start router task with 500 word stack, OS task priority 1 */
    csp_route_start_task(500, 1);

    csp_thread_create(telemetry_get_subs, "TELEM_SUBS", 1000, NULL, 0, &telem_sub_handle);
    csp_thread_create(telemetry_rx_task, "TELEM_RX", 1000, NULL, 0, &telem_rx_handle);
}

CSP_DEFINE_TASK(telemetry_get_subs)
{
    /* Private csp_socket used by the telemetry server */
    csp_socket_t * socket = NULL;
    // socket = NULL;
    if (server_setup(&socket, TELEMETRY_CSP_PORT, TELEMETRY_NUM_SUBSCRIBERS))
    {
        while (num_subs < TELEMETRY_NUM_SUBSCRIBERS)
        {
            pubsub_conn conn;
            if (server_accept(&socket, &conn))
            {
                telemetry_request request;
                publisher_read(conn, (void*)&request, sizeof(telemetry_request), TELEMETRY_CSP_PORT);
                conn.sources = request.sources;
                telemetry_subs[num_subs++] = conn;
            }
        }
    }
    csp_thread_exit();
}

CSP_DEFINE_TASK(telemetry_rx_task)
{
    telemetry_packet packet;
    packet_queue = csp_queue_create(NUM_MESSAGE_QUEUE, sizeof(telemetry_packet));
    while(1)
    {
        if (csp_queue_dequeue(packet_queue, &packet, CSP_MAX_DELAY))
        {
            telemetry_send(packet);
        }
    }
}

static void telemetry_send(telemetry_packet packet)
{
    if(packet.source.data_type == TELEMETRY_TYPE_INT)
    {
        // printf("TELEM:%d:%d:%d\r\n", packet.source.source_id, packet.timestamp, packet.data.i);
    }
    if(packet.source.data_type == TELEMETRY_TYPE_FLOAT)
    {
        // printf("TELEM:%d:%d:%f\r\n", packet.source.source_id, packet.timestamp, packet.data.f);
    }

    uint8_t i = 0;
    for (i = 0; i < num_subs; i++)
    {
        // Currently if the sources flag is set to 0
        // the subscriber will get all data
        if ((telemetry_subs[i].sources == 0) || (packet.source.source_id & telemetry_subs[i].sources))
        {
            // send_packet(telemetry_subs[i], packet);
            send_csp(telemetry_subs[i], (void*)&packet, sizeof(telemetry_packet));
        }
    }
}

bool telemetry_publish(telemetry_packet packet)
{
    if ((packet_queue != NULL) && (csp_queue_enqueue(packet_queue, &packet, CSP_MAX_DELAY)))
    {
        return true;
    }
    return false;
}

bool telemetry_read(pubsub_conn conn, telemetry_packet * packet)
{
    int tries = 0;
    if (packet != NULL)
    {
        while (tries++ < TELEMETRY_SUBSCRIBER_READ_ATTEMPTS)
        {
            if (subscriber_read(conn, (void*)packet, sizeof(telemetry_packet), TELEMETRY_CSP_PORT))
                return true;
        }
    }
    return false;
}

bool telemetry_subscribe(pubsub_conn * conn, uint8_t sources)
{
    if ((conn != NULL) && subscriber_connect(conn, TELEMETRY_CSP_ADDRESS, TELEMETRY_CSP_PORT))
    {
        telemetry_request request = {
            .sources = sources
        };
        return send_csp(*conn, (void*)&request, sizeof(telemetry_request));
    }
    return false;
}

// #endif