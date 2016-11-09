#ifndef DESTINATIONS_H
#define DESTINATIONS_H

/**
 * This file contains CSP ports and telemetry flags for routing to telemetry destinations.
 * This file will be custom per the configuration of the current project
 */

/* Address used for the current CSP instance */
#define TELEMETRY_CSP_ADDRESS 1

/* Destination flags used in the telemetry_source structure */
#define TELEMETRY_BEACON_FLAG 0x1
#define TELEMETRY_HEALTH_FLAG 0x10

/* CSP Ports used to send telemetry_packet information to destinations */
#define TELEMETRY_BEACON_PORT 10
#define TELEMETRY_HEALTH_PORT 11

#endif