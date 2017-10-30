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

#define TEST_SERVER_INTERFACE "org.KubOS.Server"
#define TEST_SERVER_PATH "/org/KubOS/Server"

#define TEST_SERVER_METHOD "TestMethod"

typedef KECPStatus (*test_method_cb)(int16_t num);

typedef struct
{
    ecp_message_handler super;
    test_method_cb      cb;
} ECPTestMethod_MessageHandler;

KECPStatus on_test_method(ecp_context * context, test_method_cb cb);
KECPStatus call_test_method(ecp_context * context, uint8_t value);
