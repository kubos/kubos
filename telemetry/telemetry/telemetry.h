#ifndef TELEMETRY_H
#define TELEMETRY_H

#include <stdint.h>
#include <csp/arch/csp_thread.h>
#include <csp/csp.h>

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
} telemetry_data_type;

/**
 * Telemetry union for data.
 */
typedef union
{
    int i;
    float f;
} telemetry_union;

/**
 * Basic telemetry packet structure - encapsulating routing information
 * and data.
 */
typedef struct
{
    telemetry_source source;
    telemetry_union data;
    uint16_t timestamp;
} telemetry_packet;

typedef struct
{
    uint8_t sources;
    csp_conn_t * conn_handle;
} telemetry_conn;

typedef struct
{
    uint8_t sources;
} telemetry_request_t;


void telemetry_conn_init();

telemetry_conn telemetry_subscribe(uint8_t source_flag);

telemetry_packet telemetry_read(telemetry_conn conn, telemetry_request_t request);

/**
 * Public facing telemetry input interface. Takes a telemetry_packet packet
 * and passes it through the telemetry system.
 */
void telemetry_submit(telemetry_packet packet);

CSP_DEFINE_TASK(telemetry_get_subs);

#define TELEMETRY_THREADS   csp_thread_handle_t telem_sub_handle; \
                            csp_thread_create(telemetry_get_subs, "TELEM_SUBS", 1000, NULL, 0, &telem_sub_handle);



telemetry_conn conn_connect(uint8_t sources);

void conn_send(telemetry_conn conn, telemetry_packet packet);

void conn_read(telemetry_conn conn, telemetry_request_t request);

telemetry_request_t conn_server_read_request(telemetry_conn conn);
uint8_t conn_client_read_packet(telemetry_conn conn, telemetry_packet * packet);

uint8_t conn_send_packet(telemetry_conn conn, telemetry_packet packet);

#endif
