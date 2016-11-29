#ifndef AGGREGATOR_H
#define AGGREGATOR_H

#include <csp/arch/csp_thread.h>
#include <telemetry/telemetry.h>

/**
 * Thread for aggregating telemetry data. Currently this just calls the
 * user-defined function user_aggregator in a loop.
 */
CSP_DEFINE_TASK(aggregator);

/**
 * Macro for creating the aggregator thread
 */
#define INIT_AGGREGATOR_THREAD                                                      \
{                                                                                   \
    csp_thread_handle_t agg_handle;                                                 \
    csp_thread_create(aggregator, "AGGREGATOR", 1024, NULL, 0, &agg_handle);        \
}

/**
 * Function stub for user-defined telemetry aggregator. This function
 * will be called repeatedly in a loop by the aggregator thread.
 *
 */
void user_aggregator();

/**
 * Convenience wrapper function for telemetry submission
 */ 
void aggregator_submit(telemetry_source, uint16_t data);

#endif