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
 * Generic message parsing/handling functions not tied to any
 * specific subsystem
 */

#include <dbus/dbus.h>
#include <string.h>
#include "evented-control/ecp.h"

KECPStatus check_message(DBusMessage * message, const char * interface,
                         const char * member)
{
    KECPStatus   err = ECP_ERROR;
    const char * msg_interface;
    const char * msg_member;

    if ((NULL != message) && (NULL != interface) && (NULL != member))
    {
        msg_interface = dbus_message_get_interface(message);
        msg_member    = dbus_message_get_member(message);

        if ((0 == strcmp(msg_interface, interface))
            && (0 == strcmp(msg_member, member)))
        {
            err = ECP_OK;
        }
    }
    return err;
}
