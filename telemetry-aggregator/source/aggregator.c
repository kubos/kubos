/*
 * Copyright (C) 2017 Kubos Corporation
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

#ifdef YOTTA_CFG_TELEMETRY_AGGREGATOR

#include "telemetry-aggregator/aggregator.h"
#include "telemetry-aggregator/config.h"

#include <csp/csp.h>

CSP_DEFINE_TASK(aggregator)
{
    while(1)
    {
        user_aggregator();
        csp_sleep_ms(TELEMETRY_AGGREGATOR_INTERVAL);
    }
}


void aggregator_submit(telemetry_source source, uint16_t data)
{
    telemetry_publish((telemetry_packet){
        .data = data,
        .timestamp = csp_get_ms(),
        .source = source
    });
}

#endif