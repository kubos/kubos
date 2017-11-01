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

#pragma once

#include <dbus/dbus.h>
#include "evented-control/ecp.h"

#define TEST_PUB_ONE_INTERFACE "org.KubOS.TestPublisherOne"
#define TEST_PUB_ONE_PATH "/org/KubOS/TestPublisherOne"
#define TEST_PUB_ONE_SIGNAL "TestSignalOne"

#define TEST_PUB_TWO_INTERFACE "org.KubOS.TestPublisherTwo"
#define TEST_PUB_TWO_PATH "/org/KubOS/TestPublisherTwo"
#define TEST_PUB_TWO_SIGNAL "TestSignalTwo"

typedef KECPStatus (*test_signal_cb)(int16_t num);

typedef struct
{
    ecp_message_handler super;
    test_signal_cb      cb;
} test_signal_message_handler;

KECPStatus on_test_signal_one(ecp_context * context, test_signal_cb cb);
KECPStatus on_test_signal_two(ecp_context * context, test_signal_cb cb);

KECPStatus format_test_signal_one_message(int16_t num, DBusMessage ** message);
KECPStatus format_test_signal_two_message(int16_t num, DBusMessage ** message);
