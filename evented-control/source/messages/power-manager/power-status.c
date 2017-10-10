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
#include <stdio.h>
#include <stdlib.h>
#include "evented-control/ecp.h"
#include "evented-control/messages.h"

KECPStatus format_power_status_message(eps_power_state status,
                                       DBusMessage **  message)
{
    DBusMessageIter iter;

    *message = dbus_message_new_signal(
        POWER_MANAGER_PATH, POWER_MANAGER_INTERFACE, POWER_MANAGER_STATUS);
    dbus_message_append_args(*message, DBUS_TYPE_INT16, &(status.line_one),
                             DBUS_TYPE_INT16, &(status.line_two),
                             DBUS_TYPE_INVALID);

    return ECP_OK;
}

KECPStatus parse_power_status_message(eps_power_state * status,
                                      DBusMessage *     message)
{
    KECPStatus      err = ECP_OK;
    DBusMessageIter iter;
    DBusError       derror;
    uint16_t        line_one, line_two;

    dbus_error_init(&derror);

    if (!dbus_message_get_args(message, &derror, DBUS_TYPE_INT16,
                               &(status->line_one), DBUS_TYPE_INT16,
                               &(status->line_two), DBUS_TYPE_INVALID))
    {
        printf("Had issuing parsing args\n%s\n", derror.message);
        err = ECP_GENERIC;
    }

    dbus_error_free(&derror);

    return err;
}

KECPStatus on_power_status_parser(const ecp_context *           context,
                                  DBusMessage *                 message,
                                  struct _ecp_message_handler * handler)
{
    eps_power_state                status;
    power_status_message_handler * status_handler
        = (power_status_message_handler *) handler;
    if (ECP_OK == parse_power_status_message(&status, message))
    {
        status_handler->cb(status);
        return ECP_GENERIC;
    }
    return ECP_OK;
}

KECPStatus on_power_status(ecp_context * context, power_status_cb cb)
{
    power_status_message_handler * handler = malloc(sizeof(*handler));
    handler->super.next                    = NULL;
    handler->super.interface               = POWER_MANAGER_INTERFACE;
    handler->super.member                  = POWER_MANAGER_STATUS;
    handler->super.parser                  = &on_power_status_parser;
    handler->cb                            = cb;

    ecp_add_message_handler(context, &handler->super);

    return ecp_listen(context, POWER_MANAGER_INTERFACE);
}
