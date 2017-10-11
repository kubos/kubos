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
    KECPStatus      err = ECP_ERROR;
    DBusMessageIter iter;

    if (NULL != (*message = dbus_message_new_signal(POWER_MANAGER_PATH,
                                                    POWER_MANAGER_INTERFACE,
                                                    POWER_MANAGER_STATUS)))
    {
        if (dbus_message_append_args(*message, DBUS_TYPE_INT16,
                                     &(status.line_one), DBUS_TYPE_INT16,
                                     &(status.line_two), DBUS_TYPE_INVALID))
        {
            err = ECP_OK;
        }
    }

    return err;
}

KECPStatus parse_power_status_message(eps_power_state * status,
                                      DBusMessage *     message)
{
    KECPStatus      err = ECP_ERROR;
    DBusMessageIter iter;
    DBusError       derror;
    uint16_t        line_one, line_two;

    dbus_error_init(&derror);

    if ((NULL != status) && (NULL != message))
    {
        if (!dbus_message_get_args(message, &derror, DBUS_TYPE_INT16,
                                   &(status->line_one), DBUS_TYPE_INT16,
                                   &(status->line_two), DBUS_TYPE_INVALID))
        {
            fprintf(stderr, "Had issuing parsing args\n%s\n", derror.message);
        }
        else
        {
            err = ECP_OK;
        }
    }
    dbus_error_free(&derror);

    return err;
}

KECPStatus on_power_status_parser(const ecp_context *           context,
                                  DBusMessage *                 message,
                                  struct _ecp_message_handler * handler)
{
    KECPStatus                     err = ECP_ERROR;
    eps_power_state                status;
    power_status_message_handler * status_handler
        = (power_status_message_handler *) handler;

    if ((NULL != context) && (NULL != message) && (NULL != handler))
    {
        if (ECP_OK == (err = parse_power_status_message(&status, message)))
        {
            err = status_handler->cb(status);
        }
    }
    return err;
}

KECPStatus on_power_status(ecp_context * context, power_status_cb cb)
{
    power_status_message_handler * handler;
    KECPStatus                     err = ECP_ERROR;

    if (NULL != context)
    {
        handler = malloc(sizeof(*handler));
        if (handler != NULL)
        {
            handler->super.next      = NULL;
            handler->super.interface = POWER_MANAGER_INTERFACE;
            handler->super.member    = POWER_MANAGER_STATUS;
            handler->super.parser    = &on_power_status_parser;
            handler->cb              = cb;

            if (ECP_OK
                == (err = ecp_add_message_handler(context, &handler->super)))
            {
                err = ecp_listen(context, POWER_MANAGER_INTERFACE);
            }
        }
    }

    return err;
}
