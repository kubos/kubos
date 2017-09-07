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
#include <stdio.h>
#include "evented-control/ecp.h"
#include "eps-api/eps.h"

tECP_Error handle_eps_commands(tECP_Context * context, tECP_Message * message);

int main()
{
    tECP_Context context;
    tECP_Error   err = ECP_E_NOERR;
    tECP_Message msg;
    int          i;
    int          initialized = 0;

    do
    {
        if (ECP_E_NOERR != (err = ECP_Init(&context)))
        {
            printf("Error %d calling ECP_Init()\n", err);
            break;
        }

        initialized = 1;

        if (ECP_E_NOERR != (err = ECP_Listen(&context, ECP_C_EPS, handle_eps_commands)))
        {
            printf("Error %d calling ECP_Listen()\n", err);
            break;
        }

        /* Now loop for (at most) 15 seconds, looking for a message */
        for (i = 0; (i < 15) && (err == ECP_E_NOERR); i++)
        {
            err = ECP_Loop(&context, 10000 * 1000);
        }

        if (err != ECP_E_NOERR)
        {
            printf("Error %d calling ECP_Loop()\n", err);
            break;
        }
    } while (0);

    if (1 == initialized)
    {
        if (ECP_E_NOERR != (err = ECP_Destroy(&context)))
        {
            printf("Error %d calling ECP_Destroy()\n", err);
        }
    }

    if (ECP_E_NOERR == err)
    {
        return (0);
    }
    else
    {
        return (2);
    }
}

tECP_Error handle_eps_commands(tECP_Context * context, tECP_Message * message)
{
    tECP_Error   err = ECP_E_NOERR;
    tECP_Message msg;
    tECP_Message_EPS_Line eps_line;

    {
        switch (message->id)
        {
            case ECP_M_EPS_ON:
                printf("Turning EPS power line on\n");
                eps_line = message->content.line;

                eps_enable_power_line(eps_line.line);

                msg.id = ECP_M_EPS_ON_ACK;
                err = ECP_Broadcast(context, ECP_C_EPS, &msg);
                break;
        }
    }

    return (err);
}