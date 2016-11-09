#ifdef YOTTA_CFG_TELEMETRY

#include "telemetry/telemetry.h"
#include <csp/csp.h>

static void telemetry_publish(telemetry_packet packet);
static void telemetry_publish_send(uint8_t port, telemetry_packet packet);

/**
 * Public facing telemetry input interface. Takes a telemetry_packet packet
 * and passes it through the telemetry system.
 *
 * Currently just prints out the data passed in and passes it along to the publish interface. Start simple right?
 */
void telemetry_submit(telemetry_packet packet)
{
    printf("TELEM:%d:%d:%d\r\n", packet.source.source_id, packet.timestamp, packet.data);
    telemetry_publish(packet);
}

/**
 * This function represents the telemetry output/publish interface. The destination just needs to
 * setup a csp connection on a port that telemetry connects to.
 */
static void telemetry_publish_send(uint8_t port, telemetry_packet packet)
{
    csp_packet_t * csp_packet;
	csp_conn_t * conn;
    csp_packet = csp_buffer_get(10);
    if (csp_packet == NULL) {
        /* Could not get buffer element */
        printf("Failed to get buffer element\n");
        return;
    }

    /* Connect to host HOST, port PORT with regular UDP-like protocol and 1000 ms timeout */
    conn = csp_connect(CSP_PRIO_NORM, TELEMETRY_CSP_ADDRESS, port, 1000, CSP_O_NONE);
    if (conn == NULL) {
        /* Connect failed */
        printf("Connection failed\n");
        /* Remember to free packet buffer */
        csp_buffer_free(csp_packet);
        return;
    }

    memcpy(csp_packet->packet, &packet, sizeof(telemetry_packet));

    /* Set packet length */
    csp_packet->length = sizeof(telemetry_packet);

    // /* Send packet */
    if (!csp_send(conn, csp_packet, 1000)) {
        /* Send failed */
        printf("Send failed\n");
        csp_buffer_free(csp_packet);
    }
    
    csp_close(conn);
}

/**
 * This function holds the master list of destinations interested in 
 * telemetry data. The list here will be different for each project/binary..
 */
static void telemetry_publish(telemetry_packet packet)
{
    /** autogen list of ports which telem should publish to */
    telemetry_publish_send(TELEMETRY_BEACON_PORT, packet);
    telemetry_publish_send(TELEMETRY_HEALTH_PORT, packet);
}

#endif