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

#include "evented-control/ecp.h"
#include <dbus/dbus.h>
#include <error.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/* This is the endpoint used by nanomsg for pub/sub broadcast messages.
** Eventually we'll want to read this from a configuration file, but
** hardcoding it is fine for the near term.
*/
char * broadcast_endpoint = "tcp://127.0.0.1:25901";

/**
 * Initialize an ECP Context
 *
*/

tECP_Error ECP_Init(tECP_Context * context, const char * name, tECP_Callback callback)
{
    tECP_Error err = ECP_E_NOERR;
    DBusError  error;
    int        i = 0;

    /* Initialize context to known state */
    context->talk       = -1;
    context->listen     = -1;
    context->talk_id    = -1;
    context->listen_id  = -1;
    context->callbacks  = NULL;
    context->connection = NULL;

    do
    {
        dbus_error_init(&error);
        context->connection = dbus_bus_get(DBUS_BUS_SESSION, &error);
        if (NULL == context->connection)
        {
            err = ECP_E_GENERIC;
            break;
        }

        if (0 > dbus_bus_request_name(context->connection, name, 0, &error))
        {
            err = ECP_E_GENERIC;
            break;
        }

        if (!dbus_connection_add_filter(context->connection, callback, NULL, NULL))
        {
            err = ECP_E_GENERIC;
            break;
        }
    } while (0);

    dbus_error_free(&error);

    return (err);
}

tECP_Error ECP_Listen(tECP_Context * context, const char * channel)
{
    tECP_Error err = ECP_E_NOERR;
    DBusError  error;
    char       sig_match_str[100];

    sprintf(sig_match_str, "type='signal',interface='%s'", channel);

    do
    {
        dbus_error_init(&error);
        dbus_bus_add_match(context->connection, sig_match_str, &error);

        if (dbus_error_is_set(&error))
        {
            fprintf(stderr, "Name Error (%s)\n", error.message);
            dbus_error_free(&error);
            err = ECP_E_GENERIC;
            break;
        }

        dbus_connection_flush(context->connection);
    } while (0);

    return err;
}

/**
 ** Block until a message is received or the timeout (in microseconds)
 expires
 */
tECP_Error ECP_Loop( tECP_Context * context, unsigned int timeout ) {
    tECP_Error err = ECP_E_NOERR;

    dbus_connection_read_write_dispatch(context->connection, timeout);

    return err;
}

/**
 ** Release resources allocated by ECP_Init()
 */
tECP_Error ECP_Destroy( tECP_Context * context ) {
  tECP_Error err = ECP_E_NOERR;

  /** Need to figure out what d-bus wants us to clean up...
    * It looks like dbus_connection_close isn't needed since
    * we are using dbus_bus_get
    */

  return( err );
}

/**
 ** Send a broadcast message on a pub-sub channel. Note: messages are
 ** broadcast immediately and don't wait for a call to ECP_Loop().
 */

#define BUFFER_SIZE ( sizeof( uint16_t ) + sizeof( tECP_Message ) )

tECP_Error ECP_Broadcast( tECP_Context * context, DBusMessage * message ) {
  tECP_Error err = ECP_E_NOERR;
  dbus_uint32_t serial = 0;

  if (!dbus_connection_send(context->connection, message, &serial))
  {
      err = ECP_E_GENERIC;
  }

  return( err );
}

tECP_Error ECP_Handle_Message(tECP_Context * context, DBusMessage * message)
{
    tECP_MessageHandler * current = NULL;
    const char * message_interface = dbus_message_get_interface(message);
    const char * message_member = dbus_message_get_member(message);

    current = context->callbacks;
    while (current != NULL)
    {
        if ((0 == strcmp(message_interface, current->interface)) &&
            (0 == strcmp(message_member, current->member)))
        {
            printf("Handling %s.%s\n", current->interface, current->member);
            current->parser(message, current->cb);
            return ECP_E_NOERR;
        }
        current = current->next;
    }
    if (NULL == current)
    {
        return ECP_E_GENERIC;
    }
}

tECP_Error ECP_Add_Message_Handler(tECP_Context * context, tECP_MessageHandler handler)
{
    tECP_MessageHandler * current = NULL;
    tECP_MessageHandler * new_handler = malloc(sizeof(tECP_MessageHandler));
    tECP_Error err = ECP_E_NOERR;

    memcpy(new_handler, &handler, sizeof(tECP_MessageHandler));

    if (NULL == context->callbacks)
    {
        context->callbacks = new_handler;
    }
    else
    {
        current = context->callbacks;
        while (NULL != current->next)
        {
            current = current->next;
        }
        current->next = new_handler;
    }

    return err;
}