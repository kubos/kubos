#ifdef YOTTA_CFG_TELEMETRY

#include "telemetry/telemetry.h"
#include "telemetry/config.h"
#include <stdio.h>

static void telemetry_publish(telemetry_packet packet);

static telemetry_conn telemetry_subs[TELEMETRY_NUM_DESTINATIONS];
static uint8_t num_subs = 0;

/**
 * Public facing telemetry input interface. Takes a telemetry_packet packet
 * and passes it through the telemetry system.
 *
 * Currently just prints out the data passed in and passes it along to the publish interface. Start simple right?
 */
void telemetry_submit(telemetry_packet packet)
{
    if(packet.source.data_type == TELEMETRY_TYPE_INT) {
        printf("TELEM:%d:%d:%d\r\n", packet.source.source_id, packet.timestamp, packet.data.i);
    }
    if(packet.source.data_type == TELEMETRY_TYPE_FLOAT) {
        printf("TELEM:%d:%d:%f\r\n", packet.source.source_id, packet.timestamp, packet.data.f);
    }
    telemetry_publish(packet);
}

CSP_DEFINE_TASK(telemetry_get_subs)
{
    uint8_t running = 1;
    
    if (!conn_server_setup())
    {
        printf("failed to setup connection server\n");
    }
    else
    {
        printf("listening for subs\r\n");
        while (num_subs < TELEMETRY_NUM_DESTINATIONS)
        {
            telemetry_conn conn;
            if (conn_server_accept(&conn))
            {
                telemetry_request_t request = conn_server_read_request(conn);
                conn.sources = request.sources;
                telemetry_subs[num_subs++] = conn;
            }
        }
        printf("all subs gathered\r\n");
    }
}

static void telemetry_publish(telemetry_packet packet)
{
    printf("publish dis\r\n");
    uint8_t i = 0;
    for (i = 0; i < num_subs; i++)
    {
        printf("packet source %d telem sources %d\n", packet.source.source_id, telemetry_subs[i].sources);
        if (packet.source.source_id & telemetry_subs[i].sources)
        {
            printf("publish to sub %d\r\n", i);
            conn_send_packet(telemetry_subs[i], packet);
        }
    }
}

telemetry_packet telemetry_read(telemetry_conn conn, telemetry_request_t request)
{
    telemetry_packet telem_packet;

    while (1)
    {
        if (conn_client_read_packet(conn, &telem_packet))
            return telem_packet;
    }
}

telemetry_conn telemetry_subscribe(uint8_t source_flag)
{
    telemetry_conn conn;
    conn = conn_connect(source_flag);
    return conn;
}


#endif
