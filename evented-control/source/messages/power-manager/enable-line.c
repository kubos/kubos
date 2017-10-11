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
/**
 * Functions for calling/handling/parsing the EnableLine method exposed
 * by the Power Manager.
 *
 * org.KubOS.PowerManager.EnableLine
 */

#include <dbus/dbus.h>
#include <stdio.h>
#include <stdlib.h>
#include "evented-control/ecp.h"
#include "evented-control/messages.h"

KECPStatus on_enable_line_parser(const ecp_context *           context,
                                 DBusMessage *                 message,
                                 struct _ecp_message_handler * handler)
{
    KECPStatus                    err = ECP_ERROR;
    DBusError                     derr;
    DBusMessage *                 reply = NULL;
    uint8_t                       line  = -1;
    enable_line_message_handler * line_handler
        = (enable_line_message_handler *) handler;

    if ((NULL != context) && (NULL != handler) && (NULL != message))
    {
        dbus_error_init(&derr);
        if (dbus_message_get_args(message, &derr, DBUS_TYPE_BYTE, &line,
                                  DBUS_TYPE_INVALID))
        {
            if (ECP_OK == (err = line_handler->cb(line)))
            {
                if (NULL != (reply = dbus_message_new_method_return(message)))
                {
                    if (dbus_connection_send(context->connection, reply, NULL))
                    {
                        err = ECP_OK;
                    }
                    dbus_message_unref(reply);
                }
            }
        }
        else
        {
            fprintf(stderr, "Error getting message args %s\n", derr.message);
        }
    }
    return err;
}

KECPStatus on_enable_line(ecp_context * context, enable_line_cb cb)
{
    KECPStatus                    err = ECP_ERROR;
    enable_line_message_handler * enable_line_handler;

    if (NULL != context)
    {
        if (NULL != (enable_line_handler = malloc(sizeof(*enable_line_handler))))
        {
            enable_line_handler->super.interface = POWER_MANAGER_INTERFACE;
            enable_line_handler->super.member    = POWER_MANAGER_ENABLE_LINE;
            enable_line_handler->super.parser    = &on_enable_line_parser;
            enable_line_handler->super.next      = NULL;
            enable_line_handler->cb              = cb;

            err = ecp_add_message_handler(context, &enable_line_handler->super);
        }
    }
    return err;
}

KECPStatus enable_line(ecp_context * context, uint8_t line)
{
    DBusMessage * message = NULL;
    KECPStatus    err     = ECP_ERROR;

    if (NULL != context)
    {
        if (NULL != (message = dbus_message_new_method_call(
                         POWER_MANAGER_INTERFACE, POWER_MANAGER_PATH,
                         POWER_MANAGER_INTERFACE, POWER_MANAGER_ENABLE_LINE)))
        {
            if (dbus_message_append_args(message, DBUS_TYPE_BYTE, &line,
                                         DBUS_TYPE_INVALID))
            {
                err = ecp_send_with_reply(context, message,
                                          DEFAULT_SEND_TIMEOUT);
            }
        }
    }
    return err;
}
