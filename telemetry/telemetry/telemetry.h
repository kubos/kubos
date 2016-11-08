#ifndef TELEMETRY_H
#define TELEMETRY_H

#include <stdint.h>

/**
 * Telemetry packet routing information structure
 */
typedef struct
{
    uint8_t source_id;
    uint8_t dest_flag;
} telemetry_source;

/**
 * Basic telemetry packet structure - encapsulating routing information
 * and data.
 */
typedef struct
{
    telemetry_source source;
    uint16_t timestamp;
    uint16_t data;
} telemetry_packet;

/**
 * Public facing telemetry input interface. Takes a telemetry_packet packet
 * and passes it through the telemetry system.
 */
void telemetry_submit(telemetry_packet packet);

#endif