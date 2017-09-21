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
#include "eps-api/eps.h"
#include <stddef.h>
#include <stdio.h>
#include <string.h>

eps_err eps_enable_power_line(uint16_t line)
{
    // Place holder implementation to enable integration
    printf("EPS enable line %d\n", line);
    return EPS_OK;
}

eps_err eps_get_power_status(eps_power_status * status)
{
    // Place holder implementation to enable integration
    if (status != NULL)
    {
        status->line_one = 1;
        status->line_two = 0;
        return EPS_OK;
    }
    return EPS_ERROR;
}
