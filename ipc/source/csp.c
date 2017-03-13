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

#include <csp/csp.h>

static bool csp_active = false;

bool kubos_csp_init(int csp_address)
{
    if (csp_active)
    {
        return true;
    }

    if (csp_buffer_init(20, 256) != CSP_ERR_NONE)
    {
        return false;
    }

    if (csp_init(csp_address) != CSP_ERR_NONE)
    {
        return false;
    }

    if (csp_route_start_task(500, 1) != CSP_ERR_NONE)
    {
        return false;
    }

#ifdef YOTTA_CFG_CSP_DEBUG
    csp_debug_set_level(CSP_ERROR, true);
    csp_debug_set_level(CSP_WARN, true);
    csp_debug_set_level(CSP_INFO, true);
    csp_debug_set_level(CSP_BUFFER, true);
    csp_debug_set_level(CSP_PACKET, true);
    csp_debug_set_level(CSP_PROTOCOL, true);
    csp_debug_set_level(CSP_LOCK, true);
#endif

    csp_active = true;

    return true;
}

void kubos_csp_terminate(void)
{
    csp_route_end_task();

    csp_terminate();

    csp_buffer_cleanup();

    csp_active = false;
}