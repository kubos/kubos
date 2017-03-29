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


#include <stdint.h>
#include <stdio.h>

#include <commands/errors.h>
#include <commands/supervisor.h>


int supervisor_init()
{
    printf("Stubbed out supervisor init\n");
    return NO_ERR;
}

int supervisor_get_version(supervisor_version_configuration_t* versionReply)
{
    printf("Stubbed out supervisor get_version\n");
    return NO_ERR;
}


int supervisor_power_cycle_iobc(supervisor_generic_reply_t* reply)
{
    printf("Stubbed out iOBC powercycle\n");
    return NO_ERR;
}
