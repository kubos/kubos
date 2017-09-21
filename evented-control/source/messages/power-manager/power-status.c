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
 * Functions for publishing/subscribing/parsing the PowerStatus signal exposed
 * by the Power Manager.
 *
 * org.KubOS.PowerManager.PowerStatus
 */
#include <dbus/dbus.h>
#include <stdlib.h>
#include "evented-control/ecp.h"
#include "evented-control/messages.h"

tECP_Error format_power_status_message(eps_power_status status,
                                       DBusMessage **   message)
{
    DBusMessageIter iter;

    *message = dbus_message_new_signal(
        POWER_MANAGER_PATH, POWER_MANAGER_INTERFACE, POWER_MANAGER_STATUS);
    dbus_message_append_args(*message, DBUS_TYPE_INT16, &(status.line_one),
                             DBUS_TYPE_INT16, &(status.line_two),
                             DBUS_TYPE_INVALID);

    return ECP_NOERR;
}

tECP_Error parse_power_status_message(eps_power_status * status,
                                      DBusMessage *      message)
{
    DBusMessageIter iter;
    DBusError       derror;
    uint16_t        line_one, line_two;

    dbus_error_init(&derror);

    if (!dbus_message_get_args(message, &derror, DBUS_TYPE_INT16,
                               &(status->line_one), DBUS_TYPE_INT16,
                               &(status->line_two), DBUS_TYPE_INVALID))
    {
        printf("Had issuing parsing args\n%s\n", derror.message);
        return ECP_GENERIC;
    }

    return ECP_NOERR;
}

tECP_Error on_power_status_parser(tECP_Context * context, DBusMessage * message,
                                  struct _tECP_MessageHandler * handler)
{
    eps_power_status                  status;
    tECP_PowerStatus_MessageHandler * status_handler
        = (tECP_PowerStatus_MessageHandler *) handler;
    if (ECP_NOERR == parse_power_status_message(&status, message))
    {
        status_handler->cb(status);
    }
}

tECP_Error on_power_status(tECP_Context * context, power_status_cb cb)
{
    tECP_PowerStatus_MessageHandler * handler = malloc(sizeof(*handler));
    handler->super.next                       = NULL;
    handler->super.interface                  = POWER_MANAGER_INTERFACE;
    handler->super.member                     = POWER_MANAGER_STATUS;
    handler->super.parser                     = &on_power_status_parser;
    handler->cb                               = cb;

    ECP_Add_Message_Handler(context, &handler->super);

    return ECP_Listen(context, POWER_MANAGER_INTERFACE);
}
