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
#include <stdlib.h>
#include "evented-control/ecp.h"
#include "evented-control/messages.h"

ECPStatus on_enable_line_parser(ECPContext * context, DBusMessage * message,
                                struct _ECPMessageHandler * handler)
{
    DBusMessage *                 reply = NULL;
    uint8_t                       line  = -1;
    ECPEnableLineMessageHandler * line_handler
        = (ECPEnableLineMessageHandler *) handler;

    dbus_message_get_args(message, NULL, DBUS_TYPE_BYTE, &line,
                          DBUS_TYPE_INVALID);

    line_handler->cb(line);

    reply = dbus_message_new_method_return(message);
    dbus_connection_send(context->connection, reply, NULL);
    dbus_message_unref(reply);
    return ECP_OK;
}

ECPStatus on_enable_line(ECPContext * context, EnableLineCb cb)
{
    ECPEnableLineMessageHandler * enable_line_handler
        = malloc(sizeof(*enable_line_handler));
    enable_line_handler->super.interface = POWER_MANAGER_INTERFACE;
    enable_line_handler->super.member    = POWER_MANAGER_ENABLE_LINE;
    enable_line_handler->super.parser    = &on_enable_line_parser;
    enable_line_handler->super.next      = NULL;
    enable_line_handler->cb              = cb;

    return ECP_Add_Message_Handler(context, &enable_line_handler->super);
}

ECPStatus enable_line(ECPContext * context, uint8_t line)
{
    DBusMessage * message = NULL;
    ECPStatus     err     = ECP_OK;

    message = dbus_message_new_method_call(
        POWER_MANAGER_INTERFACE, POWER_MANAGER_PATH, POWER_MANAGER_INTERFACE,
        POWER_MANAGER_ENABLE_LINE);

    dbus_message_append_args(message, DBUS_TYPE_BYTE, &line, DBUS_TYPE_INVALID);

    return ECP_Call(context, message);
}
