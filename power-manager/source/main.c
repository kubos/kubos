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

ECPStatus enable_line_handler(uint8_t line);

int main()
{
    ECPStatus        err = ECP_OK;
    int              i;
    int              initialized = 0;
    eps_power_status status;
    ECPContext       context;

    do
    {
        if (ECP_OK != (err = ECP_Init(&context, POWER_MANAGER_INTERFACE)))
        {
            printf("Error %d calling ECP_Init()\n", err);
            break;
        }

        if (ECP_OK != on_enable_line(&context, &enable_line_handler))
        {
            printf("Error registering enable line callback\n");
            break;
        }

        /* Now loop for (at most) 15 seconds, looking for a message */
        for (i = 0; (i < 15) && (err == ECP_OK); i++)
        {
            printf("Sending power status\n");
            DBusMessage * message;
            eps_get_power_status(&status);
            format_power_status_message(status, &message);
            ECP_Broadcast(&context, message);
            err = ECP_Loop(&context, 1000);
        }

        if (err != ECP_OK)
        {
            printf("Error %d calling ECP_Loop()\n", err);
        }
    } while (0);

    if (ECP_OK != (err = ECP_Destroy(&context)))
    {
        printf("Error %d calling ECP_Destroy()\n", err);
    }

    return err;
}

ECPStatus enable_line_handler(uint8_t line)
{
    printf("Enable line..\n");
    eps_enable_power_line(line);
}
