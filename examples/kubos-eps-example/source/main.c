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

#include <eps-api/eps.h>
#include <evented-control/ecp.h>
#include <evented-control/messages.h>
#include <stdio.h>
#include <stdlib.h>

tECP_Error status_handler(eps_power_status status);

#define MY_NAME "org.KubOS.client"

int main(int argc, char * argv[])
{
    tECP_Error   err;
    tECP_Context context;

    do
    {

        if (ECP_NOERR != (err = ECP_Init(&context, MY_NAME)))
        {
            printf("Error calling ECP_Init(): %d\n", err);
            break;
        }
        printf("Successfully called ECP_Init()\n");

        if (ECP_NOERR != (err = on_power_status(&context, &status_handler)))
        {
            printf("Error calling on_power_status\n");
            break;
        }

        if (ECP_NOERR != (err = enable_line(&context, 1)))
        {
            printf("Error calling enable line\n");
            break;
        }

        printf("Successfully enabled line 1\n");

        for (int i = 0; i < 10; i++)
        {
            ECP_Loop(&context, 100);
        }
    } while (0);

    if (ECP_NOERR != (err = ECP_Destroy(&context)))
    {
        printf("Error calling ECP_Destroy(): %d\n", err);
    }

    return (err);
}

tECP_Error status_handler(eps_power_status status)
{
    printf("Got status %d:%d\n", status.line_one, status.line_two);
}
