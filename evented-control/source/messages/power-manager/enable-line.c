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
#include "evented-control/ecp.h"
#include "evented-control/messages.h"

tECP_Error on_enable_line_parser(tECP_Context * context,
                                 DBusMessage * message, void * handler)
{
    DBusMessage * reply = NULL;

    reply = dbus_message_new_method_return(message);
    dbus_connection_send(context->connection, reply, NULL);
    dbus_message_unref(reply);
}

tECP_Error on_enable_line(tECP_Context * context, void * cb)
{
    tECP_MessageHandler enable_line_handler
        = {.interface = POWER_MANAGER_INTERFACE,
           .member    = POWER_MANAGER_ENABLE_LINE,
           .parser    = &on_enable_line_parser,
           .cb        = (void *) cb };

    return ECP_Add_Message_Handler(context, enable_line_handler);
}

tECP_Error enable_line(tECP_Context * context, uint8_t line)
{
    return ECP_Call(context, POWER_MANAGER_INTERFACE, POWER_MANAGER_PATH, POWER_MANAGER_ENABLE_LINE);
}