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
 * Functions for publishing/subscribing/parsing the TestSignal signal exposed
 * by the Test Publisher.
 *
 * org.KubOS.TestPublisher.TestSignal
 */

#include "messages.h"
#include <dbus/dbus.h>
#include <stdint.h>
#include <stdlib.h>
#include "evented-control/ecp.h"

ECPStatus format_test_signal_one_message(int16_t num, DBusMessage ** message)
{
    DBusMessageIter iter;

    *message = dbus_message_new_signal(
        TEST_PUB_ONE_PATH, TEST_PUB_ONE_INTERFACE, TEST_PUB_ONE_SIGNAL);
    dbus_message_append_args(*message, DBUS_TYPE_INT16, &num,
                             DBUS_TYPE_INVALID);

    return ECP_OK;
}

ECPStatus format_test_signal_two_message(int16_t num, DBusMessage ** message)
{
    DBusMessageIter iter;

    *message = dbus_message_new_signal(
        TEST_PUB_TWO_PATH, TEST_PUB_TWO_INTERFACE, TEST_PUB_TWO_SIGNAL);
    dbus_message_append_args(*message, DBUS_TYPE_INT16, &num,
                             DBUS_TYPE_INVALID);

    return ECP_OK;
}

ECPStatus parse_test_signal_message(int16_t * num, DBusMessage * message)
{
    DBusMessageIter iter;
    DBusError       derror;

    dbus_error_init(&derror);

    if (!dbus_message_get_args(message, &derror, DBUS_TYPE_INT16, num,
                               DBUS_TYPE_INVALID))
    {
        printf("Had issuing parsing args\n%s\n", derror.message);
        return ECP_GENERIC;
    }

    return ECP_OK;
}

ECPStatus on_test_signal_parser(ECPContext * context, DBusMessage * message,
                                struct _ECPMessageHandler * handler)
{
    int16_t                       num;
    ECPTestSignalMessageHandler * status_handler
        = (ECPTestSignalMessageHandler *) handler;

    if (ECP_OK == parse_test_signal_message(&num, message))
    {
        status_handler->cb(num);
    }
}

ECPStatus on_test_signal_one(ECPContext * context, test_signal_cb cb)
{
    ECPTestSignalMessageHandler * handler = malloc(sizeof(*handler));
    handler->super.next                   = NULL;
    handler->super.interface              = TEST_PUB_ONE_INTERFACE;
    handler->super.member                 = TEST_PUB_ONE_SIGNAL;
    handler->super.parser                 = &on_test_signal_parser;
    handler->cb                           = cb;

    ECP_Add_Message_Handler(context, &handler->super);

    return ECP_Listen(context, TEST_PUB_ONE_INTERFACE);
}

ECPStatus on_test_signal_two(ECPContext * context, test_signal_cb cb)
{
    ECPTestSignalMessageHandler * handler = malloc(sizeof(*handler));
    handler->super.next                   = NULL;
    handler->super.interface              = TEST_PUB_TWO_INTERFACE;
    handler->super.member                 = TEST_PUB_TWO_SIGNAL;
    handler->super.parser                 = &on_test_signal_parser;
    handler->cb                           = cb;

    ECP_Add_Message_Handler(context, &handler->super);

    return ECP_Listen(context, TEST_PUB_TWO_INTERFACE);
}
