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

DBusHandlerResult _ecp_message_handler(DBusConnection * connection,
                                       DBusMessage * message, void * user_data);

KECPStatus ecp_init(ecp_context * context, const char * name)
{
    KECPStatus err = ECP_ERROR;
    DBusError  error;

    /* Initialize context to known state */
    context->callbacks  = NULL;
    context->connection = NULL;

    if ((NULL != context) && (NULL != name))
    {
        do
        {
            dbus_error_init(&error);
            context->connection = dbus_bus_get(DBUS_BUS_SESSION, &error);
            if (NULL == context->connection)
            {
                fprintf(stderr, "Error connecting to bus - %s\n",
                        error.message);
            }

            if (0 > dbus_bus_request_name(context->connection, name, 0, &error))
            {
                fprintf(stderr, "Error requesting name - %s\n", error.message);
            }

            if (!dbus_connection_add_filter(context->connection,
                                            _ecp_message_handler,
                                            (void *) context, NULL))
            {
                fprintf(stderr, "Error adding filter\n");
            }
            err = ECP_OK;
        } while (0);
        dbus_error_free(&error);
    }

    return (err);
}

KECPStatus ecp_listen(ecp_context * context, const char * channel)
{
    KECPStatus err = ECP_ERROR;
    DBusError  derr;
    char       sig_match_str[100];

    if ((NULL != context) && (NULL != channel))
    {
        sprintf(sig_match_str, "type='signal',interface='%s'", channel);
        dbus_error_init(&derr);
        dbus_bus_add_match(context->connection, sig_match_str, &derr);
        if (dbus_error_is_set(&derr))
        {
            fprintf(stderr, "Error adding listener match - %s\n", derr.message);
        }
        else
        {
            err = ECP_OK;
        }

        dbus_error_free(&derr);
        dbus_connection_flush(context->connection);
    }
    return err;
}

KECPStatus ecp_loop(const ecp_context * context, unsigned int timeout)
{
    KECPStatus err = ECP_ERROR;

    if (dbus_connection_read_write_dispatch(context->connection, timeout))
    {
        err = ECP_OK;
    }

    return err;
}

KECPStatus ecp_destroy(ecp_context * context)
{
    KECPStatus err = ECP_ERROR;

    ecp_message_handler * current = NULL;
    ecp_message_handler * next    = NULL;

    if (NULL != context)
    {
        current = context->callbacks;
        while (current != NULL)
        {
            next = current->next;
            free(current);
            current = next;
        }
        err = ECP_OK;
    }
    /** Need to figure out what d-bus wants us to clean up...
     * It looks like dbus_connection_close isn't needed since
     * we are using dbus_bus_get
     */

    return err;
}

KECPStatus ecp_handle_message(const ecp_context * context, DBusMessage * message)
{
    KECPStatus            err     = ECP_ERROR;
    ecp_message_handler * current = NULL;
    const char *          message_interface;
    const char *          message_member;

    if ((NULL != context) && (NULL != message))
    {
        message_interface = dbus_message_get_interface(message);
        message_member    = dbus_message_get_member(message);
        current           = context->callbacks;
        while (current != NULL)
        {
            if ((0 == strcmp(message_interface, current->interface))
                && (0 == strcmp(message_member, current->member)))
            {
                current->parser(context, message, current);
                err = ECP_OK;
                break;
            }
            current = current->next;
        }
    }

    return err;
}

KECPStatus ecp_send_with_reply(const ecp_context * context,
                               DBusMessage * message, uint32_t timeout)
{
    DBusMessage * reply = NULL;
    KECPStatus    err   = ECP_ERROR;
    DBusError     derr;

    dbus_error_init(&derr);

    if ((NULL != message) && (NULL != context) && (NULL != context->connection))
    {
        reply = dbus_connection_send_with_reply_and_block(
            context->connection, message, timeout, &derr);
        if (NULL != reply)
        {
            err = ECP_OK;
            dbus_message_unref(reply);
        }
        else
        {
            fprintf(stderr, "Error send_with_reply %s\n", derr.message);
        }
        dbus_message_unref(message);
    }

    dbus_error_free(&derr);

    return err;
}

KECPStatus ecp_send(const ecp_context * context, DBusMessage * message)
{
    KECPStatus err = ECP_ERROR;

    if ((NULL != context) && (NULL != context->connection) && (NULL != message))
    {
        if (dbus_connection_send(context->connection, message, NULL))
        {
            err = ECP_OK;
        }
        dbus_message_unref(message);
    }
    return err;
}

KECPStatus ecp_add_message_handler(ecp_context *         context,
                                   ecp_message_handler * new_handler)
{
    ecp_message_handler * current = NULL;
    KECPStatus            err     = ECP_ERROR;

    if ((NULL != context) && (NULL != new_handler))
    {
        if (NULL == context->callbacks)
        {
            context->callbacks = new_handler;
            err                = ECP_OK;
        }
        else
        {
            current = context->callbacks;
            while (NULL != current->next)
            {
                current = current->next;
            }
            current->next = new_handler;
            err           = ECP_OK;
        }
    }

    return err;
}

DBusHandlerResult _ecp_message_handler(DBusConnection * connection,
                                       DBusMessage * message, void * user_data)
{
    ecp_context * context = NULL;
    if ((NULL != connection) && (NULL != message) && (NULL != user_data))
    {
        context = (ecp_context *) user_data;
        if (ECP_OK == ecp_handle_message(context, message))
        {
            return DBUS_HANDLER_RESULT_HANDLED;
        }
    }

    return DBUS_HANDLER_RESULT_NOT_YET_HANDLED;
}
