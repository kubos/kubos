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
#include <eps-api/eps.h>

/**
 * D-Bus interface and path that all PowerManager signals and methods are
 * scoped under.
 */
#define POWER_MANAGER_INTERFACE "org.KubOS.PowerManager"
#define POWER_MANAGER_PATH "/org/KubOS/PowerManager"

/**
 * EnableLine method name
 */
#define POWER_MANAGER_ENABLE_LINE "EnableLine"

typedef tECP_Error (*enable_line_cb)(uint8_t line);

typedef struct
{
    tECP_MessageHandler super;
    enable_line_cb      cb;
} tECP_EnableLine_MessageHandler;

/**
 * Intermediate function used by ECP_Handle_Message
 * to parse out the DBusMessage into native data structures
 * and then hand off to the message specific callback
 */
tECP_Error on_enable_line_parser(tECP_Context * context, DBusMessage * message,
                                 struct _tECP_MessageHandler * handler);

/**
 * Creates and listener + registers callback for the
 * EnableLine method. This function should be used by the
 * process which is hosting the method
 */
tECP_Error on_enable_line(tECP_Context * context, enable_line_cb cb);

/**
 * Calls out to the EnableLine method
 */
tECP_Error enable_line(tECP_Context * context, uint8_t line);

/**
 * PowerManager signal name
 */
#define POWER_MANAGER_STATUS "PowerStatus"

typedef tECP_Error (*power_status_cb)(eps_power_status status);

typedef struct
{
    tECP_MessageHandler super;
    power_status_cb     cb;
} tECP_PowerStatus_MessageHandler;

/**
 * Parses out a PowerStatus signal into an eps_power_status struct.
 */
tECP_Error parse_power_status_message(eps_power_status * status,
                                      DBusMessage *      message);

/**
 * Takes a eps_power_status struct and creates a PowerStatus signal.
 */
tECP_Error format_power_status_message(eps_power_status status,
                                       DBusMessage **   message);

/**
 * Intermediate function used by ECP_Handle_Message
 * to parse out the DBusMessage into native data structures
 * and then hand off to the message specific callback
 */
tECP_Error on_power_status_parser(tECP_Context * context, DBusMessage * message,
                                  struct _tECP_MessageHandler * handler);

/**
 * Creates a listener + registers callback for the PowerStatus signal.
 * This function should be used by a process which is subscribed
 * to the PowerStatus signal.
 */
tECP_Error on_power_status(tECP_Context * context, power_status_cb cb);
