#ifndef TELEMETRY_H
#define TELEMETRY_H

#include <stdint.h>

/**
 * Telemetry packet routing information structure.
 */
typedef struct
{
    uint8_t source_id;
    uint8_t dest_flag;
    uint8_t data_type;
} telemetry_source;

/**
 * Telemetry data types.
 */
typedef enum {
    TELEMETRY_TYPE_INT = 0,
    TELEMETRY_TYPE_FLOAT
} telemetry_type;

/**
 * Telemetry union for data.
 */
typedef union
{
    int i;
    float f;
} telem_union;

/**
 * Basic telemetry packet structure - encapsulating routing information
 * and data.
 */
typedef struct
{
    telemetry_source source;
    telem_union data;
    uint16_t timestamp;
} telemetry_packet;

/**
 * Public facing telemetry input interface. Takes a telemetry_packet packet
 * and passes it through the telemetry system.
 */
void telemetry_submit(telemetry_packet packet);

#endif

