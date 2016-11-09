#ifndef SOURCES_H
#define SOURCES_H

#include "telemetry/telemetry.h"
#include "telemetry/destinations.h"

/**
 * This file has telemetry data source/routing structures created for use
 * by client code. This file will be custom based on the configuration of the project.
 */

/* The number of unique telemetry_source(s) points each destination will be receiving */ 
#define TELEMETRY_NUM_BEACON 3
#define TELEMETRY_NUM_HEALTH 2

/* Structures representing telemetry_source(s) currently configured */ 
static telemetry_source pos_x_source = { .source_id = 0, 
    .data_type = TELEMETRY_TYPE_FLOAT, .dest_flag = TELEMETRY_BEACON_FLAG };
static telemetry_source pos_y_source = { .source_id = 1, 
    .data_type = TELEMETRY_TYPE_FLOAT, .dest_flag = TELEMETRY_BEACON_FLAG };
static telemetry_source temp_source = { .source_id = 2,
    .data_type = TELEMETRY_TYPE_FLOAT, .dest_flag = TELEMETRY_HEALTH_FLAG };
static telemetry_source gps_source = { .source_id = 3,
    .data_type = TELEMETRY_TYPE_FLOAT, .dest_flag = TELEMETRY_BEACON_FLAG | TELEMETRY_HEALTH_FLAG };

#endif

