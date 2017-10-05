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

DBusHandlerResult _ECPMessageHandler(DBusConnection * connection,
                                     DBusMessage * message, void * user_data);

ECPStatus ECP_Init(ECPContext * context, const char * name)
{
    ECPStatus err = ECP_OK;
    DBusError error;
    int       i = 0;

    /* Initialize context to known state */
    context->callbacks  = NULL;
    context->connection = NULL;

    do
    {
        dbus_error_init(&error);
        context->connection = dbus_bus_get(DBUS_BUS_SESSION, &error);
        if (NULL == context->connection)
        {
            err = ECP_GENERIC;
            break;
        }

        if (0 > dbus_bus_request_name(context->connection, name, 0, &error))
        {
            err = ECP_GENERIC;
            break;
        }

        if (!dbus_connection_add_filter(context->connection, _ECPMessageHandler,
                                        (void *) context, NULL))
        {
            err = ECP_GENERIC;
            break;
        }
    } while (0);

    dbus_error_free(&error);

    return (err);
}

ECPStatus ECP_Listen(ECPContext * context, const char * channel)
{
    ECPStatus err = ECP_OK;
    DBusError error;
    char      sig_match_str[100];

    sprintf(sig_match_str, "type='signal',interface='%s'", channel);

    do
    {
        dbus_error_init(&error);
        dbus_bus_add_match(context->connection, sig_match_str, &error);

        if (dbus_error_is_set(&error))
        {
            fprintf(stderr, "Name Error (%s)\n", error.message);
            dbus_error_free(&error);
            err = ECP_GENERIC;
            break;
        }

        dbus_connection_flush(context->connection);
    } while (0);

    return err;
}

ECPStatus ECP_Loop(ECPContext * context, unsigned int timeout)
{
    ECPStatus err = ECP_OK;

    dbus_connection_read_write_dispatch(context->connection, timeout);

    return err;
}

ECPStatus ECP_Destroy(ECPContext * context)
{
    ECPStatus err = ECP_OK;

    ECPMessageHandler * current = NULL;
    ECPMessageHandler * next    = NULL;

    current = context->callbacks;
    while (current != NULL)
    {
        next = current->next;
        free(current);
        current = next;
    }

    /** Need to figure out what d-bus wants us to clean up...
      * It looks like dbus_connection_close isn't needed since
      * we are using dbus_bus_get
      */

    return (err);
}

ECPStatus ECP_Broadcast(ECPContext * context, DBusMessage * message)
{
    ECPStatus     err    = ECP_OK;
    dbus_uint32_t serial = 0;

    if (!dbus_connection_send(context->connection, message, &serial))
    {
        err = ECP_GENERIC;
    }

    dbus_message_unref(message);

    return (err);
}

ECPStatus ECP_Handle_Message(ECPContext * context, DBusMessage * message)
{
    ECPMessageHandler * current    = NULL;
    const char * message_interface = dbus_message_get_interface(message);
    const char * message_member    = dbus_message_get_member(message);

    current = context->callbacks;
    while (current != NULL)
    {
        if ((0 == strcmp(message_interface, current->interface))
            && (0 == strcmp(message_member, current->member)))
        {
            current->parser(context, message, current);
            return ECP_OK;
        }
        current = current->next;
    }
    if (NULL == current)
    {
        return ECP_GENERIC;
    }

    return ECP_OK;
}

ECPStatus ECP_Call(ECPContext * context, DBusMessage * message)
{
    DBusMessage * reply = NULL;
    ECPStatus     err   = ECP_OK;
    DBusError     derr;

    dbus_error_init(&derr);

    if ((NULL != message) && (NULL != context->connection))
    {
        reply = dbus_connection_send_with_reply_and_block(
            context->connection, message, 1000, &derr);
        if (reply == NULL)
        {
            err = ECP_GENERIC;
        }
        else
        {
            dbus_message_unref(reply);
        }
        dbus_message_unref(message);
    }

    dbus_error_free(&derr);

    return err;
}

ECPStatus ECP_Add_Message_Handler(ECPContext *        context,
                                  ECPMessageHandler * new_handler)
{
    ECPMessageHandler * current = NULL;
    ECPStatus           err     = ECP_OK;

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

DBusHandlerResult _ECPMessageHandler(DBusConnection * connection,
                                     DBusMessage * message, void * user_data)
{
    ECPContext * context = NULL;
    if (NULL != user_data)
    {
        context = (ECPContext *) user_data;
        if (ECP_OK == ECP_Handle_Message(context, message))
        {
            return DBUS_HANDLER_RESULT_HANDLED;
        }
    }

    return DBUS_HANDLER_RESULT_NOT_YET_HANDLED;
}
