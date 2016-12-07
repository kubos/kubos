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
#ifdef YOTTA_CFG_TELEMETRY

#include "telemetry/telemetry.h"
#include "telemetry/config.h"
#include "communications.h"
#include <stdio.h>

/**
 * Iterates though all open telemetry connections and
 * publishes the packet IF the connection is interested/subscribed
 * @param packet telemetry_packet to publish
 */
static void telemetry_send(telemetry_packet packet);

/* Static array for holding persistent connections to telemetry subscribers */
static telemetry_conn telemetry_subs[TELEMETRY_NUM_SUBSCRIBERS];

/* Current number of active telemetry subscribers */
static uint8_t num_subs = 0;


CSP_DEFINE_TASK(telemetry_get_subs)
{
    if (server_setup())
    {
        while (num_subs < TELEMETRY_NUM_SUBSCRIBERS)
        {
            telemetry_conn conn;
            if (server_accept(&conn))
            {
                telemetry_request request;
                publisher_read_request(conn, &request);
                conn.sources = request.sources;
                telemetry_subs[num_subs++] = conn;
            }
        }
    }
}

static void telemetry_send(telemetry_packet packet)
{
    uint8_t i = 0;
    for (i = 0; i < num_subs; i++)
    {
        if ((telemetry_subs[i].sources == 0) || (packet.source.source_id & telemetry_subs[i].sources))
        {
            send_packet(telemetry_subs[i], packet);
        }
    }
}

bool telemetry_publish(telemetry_packet packet)
{
    if(packet.source.data_type == TELEMETRY_TYPE_INT) {
        printf("TELEM:%d:%d:%d\r\n", packet.source.source_id, packet.timestamp, packet.data.i);
    }
    if(packet.source.data_type == TELEMETRY_TYPE_FLOAT) {
        printf("TELEM:%d:%d:%f\r\n", packet.source.source_id, packet.timestamp, packet.data.f);
    }
    telemetry_send(packet);
    return true;
}

bool telemetry_read(telemetry_conn conn, telemetry_packet * packet)
{
    int tries = 0;
    while (tries++ < 10)
    {
        if (subscriber_read_packet(conn, packet))
            return true;
    }
    return false;
}

bool telemetry_subscribe(telemetry_conn * conn, uint8_t sources)
{
    if (subscriber_connect(conn))
    {
        telemetry_request request = {
            .sources = sources
        };
        return send_request(*conn, request);
    }
    return false;
}

#endif