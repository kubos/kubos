#ifdef YOTTA_CFG_TELEMETRY_AGGREGATOR

#include "telemetry-aggregator/aggregator.h"
#include <csp/csp.h>

CSP_DEFINE_TASK(aggregator)
{
    while(1)
    {
        user_aggregator();
        csp_sleep_ms(YOTTA_CFG_TELEMETRY_AGGREGATOR_INTERVAL);
    }
}


void aggregator_submit(telemetry_source source, uint16_t data)
{
    telemetry_submit((telemetry_packet){
        .data = data,
        .timestamp = csp_get_ms(),
        .source = source
    });
}

#endif