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

/* 00_basic.c
**
** This is a basic test of the API and a demo of how it's intended to be
** used.
*/

#include <stdio.h>
#include <stdlib.h>
#include "evented-control/ecp.h"
#include <evented-control/interfaces.h>

/* This is a callback setup with the ECP_Listen() API call. */
tECP_Error _sys_handler(tECP_Context * context, tECP_Message * message);

DBusHandlerResult message_handler(DBusConnection *connection, DBusMessage *message, void *user_data);

#define MY_NAME "org.KubOS.client"
#define POWER_STATUS_SIGNAL "org.KubOS.PowerManager.PowerStatus"

int main(int argc, char * argv[])
{
    /* Two data types you'll need to know about are tECP_Error and
    *tECP_Context.
    ** The former is (not surprisingly) an atomic integer type used to
    ** communicate success status of API calls. The latter contains all the
    ** state needed to communicate on the ECP message bus.
    */

    tECP_Error   err;
    tECP_Context context;

    /* Using MIT style do { ... } while( 0 ); construct in preference to gotos
    ** or nested if's.
    */
    do
    {

        /* First, we call the ECP_Init() function. This initializes the
        *connection
        ** with the message bus.
        */

        if (ECP_E_NOERR != (err = ECP_Init(&context, MY_NAME, message_handler)))
        {
            printf("00BASIC: Error calling ECP_Init(): %d\n", err);
            break;
        }

        printf("00BASIC: Successfully called ECP_Init()\n");

        if (ECP_E_NOERR != (err = ECP_Listen(&context, POWER_STATUS_SIGNAL)))
        {
            printf("Error calling ECP_Listen\n");
            break;
        }

        printf("Called ECP_Listen\n");

        for (int i = 0; i < 15; i++)
        {
            ECP_Loop(&context, 1000);
        }

       
        /* ECP_Destroy() cleans up after you're ready to stop interacting
        with
        ** the message bus.
        */
        if( ECP_E_NOERR != ( err = ECP_Destroy( & context ) ) ) {
          printf( "00BASIC: Error calling ECP_Destroy(): %d\n", err );
          break;
        }

        printf("00BASIC: Successfully called ECP_Destroy()\n");

    } while (0);

    return (err);
}

DBusHandlerResult message_handler(DBusConnection *connection, DBusMessage *message, void *user_data)
{
    const char *interface_name = dbus_message_get_interface(message);
    const char *member_name = dbus_message_get_member(message);

    printf("Got Message\n%s\n%s\n", interface_name, member_name);

    return DBUS_HANDLER_RESULT_NOT_YET_HANDLED;
}

tECP_Error _sys_handler(tECP_Context * context, tECP_Message * message)
{
    return (ECP_E_NOERR);
}
