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

#include <evented-control/ecp.h>
#include <evented-control/interfaces.h>
#include <eps-api/eps.h>
#include <stdio.h>
#include <stdlib.h>

DBusHandlerResult message_handler(DBusConnection * connection,
                                  DBusMessage * message, void * user_data);

tECP_Error status_handler(eps_power_status status);

#define MY_NAME "org.KubOS.client"

static tECP_Context context;

int main(int argc, char * argv[])
{
    tECP_Error   err;

    do
    {

        if (ECP_E_NOERR != (err = ECP_Init(&context, MY_NAME, message_handler)))
        {
            printf("00BASIC: Error calling ECP_Init(): %d\n", err);
            break;
        }
        printf("00BASIC: Successfully called ECP_Init()\n");

        if (ECP_E_NOERR != (err = on_power_status(&context, &status_handler)))
        {
            printf("Error calling on_power_status\n");
            break;
        }

        for (int i = 0; i < 15; i++)
        {
            ECP_Loop(&context, 1000);
        }

        if (ECP_E_NOERR != (err = ECP_Destroy(&context)))
        {
            printf("00BASIC: Error calling ECP_Destroy(): %d\n", err);
            break;
        }

        printf("00BASIC: Successfully called ECP_Destroy()\n");

    } while (0);

    return (err);
}

DBusHandlerResult message_handler(DBusConnection * connection,
                                  DBusMessage * message, void * user_data)
{
    if (ECP_E_NOERR == ECP_Handle_Message(&context, message))
    {
        return DBUS_HANDLER_RESULT_HANDLED;
    }

    return DBUS_HANDLER_RESULT_NOT_YET_HANDLED;
}

tECP_Error status_handler(eps_power_status status)
{
    printf("Got status %d:%d\n", status.line_one, status.line_two);
}