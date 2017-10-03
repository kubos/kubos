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
 * Functions for calling/handling/parsing the TestMethod exposed
 * by the Test Server.
 *
 * org.KubOS.TestPublisher.TestSignal
 */

#include "messages.h"
#include <dbus/dbus.h>
#include <stdint.h>
#include <stdlib.h>
#include "evented-control/ecp.h"

ECPStatus on_test_method_parser(ECPContext * context, DBusMessage * message,
                                 struct _ECPMessageHandler * handler)
{
    DBusMessage *                    reply = NULL;
    tECP_TestMethod_MessageHandler * method_handler
        = (tECP_TestMethod_MessageHandler *) handler;
    int16_t value = 0;

    dbus_message_get_args(message, NULL, DBUS_TYPE_INT16, &value);

    method_handler->cb(value);

    reply = dbus_message_new_method_return(message);
    dbus_message_append_args(reply, DBUS_TYPE_INVALID);
    dbus_connection_send(context->connection, reply, NULL);
    dbus_message_unref(reply);
}

ECPStatus on_test_method(ECPContext * context, test_method_cb cb)
{
    tECP_TestMethod_MessageHandler * test_method_handler
        = malloc(sizeof(*test_method_handler));
    test_method_handler->super.interface = TEST_SERVER_INTERFACE;
    test_method_handler->super.member    = TEST_SERVER_METHOD;
    test_method_handler->super.parser    = &on_test_method_parser;
    test_method_handler->super.next      = NULL;
    test_method_handler->cb              = cb;

    return ECP_Add_Message_Handler(context, &test_method_handler->super);
}

ECPStatus call_test_method(ECPContext * context, uint8_t value)
{
    DBusMessage * message = NULL;

    message = dbus_message_new_method_call(
        TEST_SERVER_INTERFACE, TEST_SERVER_PATH, TEST_SERVER_INTERFACE,
        TEST_SERVER_METHOD);

    dbus_message_append_args(message, DBUS_TYPE_INT16, &value,
                             DBUS_TYPE_INVALID);

    return ECP_Call(context, message);
}
