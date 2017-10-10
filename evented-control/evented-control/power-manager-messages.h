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
 * @defgroup Power Power Manager Messages
 * @addtogroup Power
 * @{
 */

#pragma once

#include <dbus/dbus.h>
#include <eps-api/eps.h>

/**
 * D-Bus interface for PowerManager object
 */
#define POWER_MANAGER_INTERFACE "org.KubOS.PowerManager"

/**
 * D-Bus path for PowerManager object
 */
#define POWER_MANAGER_PATH "/org/KubOS/PowerManager"

/**
 * EnableLine method name
 */
#define POWER_MANAGER_ENABLE_LINE "EnableLine"

/**
 * PowerManager signal name
 */
#define POWER_MANAGER_STATUS "PowerStatus"

/**
 * Fuction pointer for EnableLine callback
 */
typedef KECPStatus (*enable_line_cb)(uint8_t line);

/**
 * Function ponter type for PowerStatus callback
 */
typedef KECPStatus (*power_status_cb)(eps_power_state status);

/**
 * EnableLine message handler
 */
typedef struct
{
    /** Pointer back to generic message handler */
    ecp_message_handler super;
    /** Pointer to enable line callback */
    enable_line_cb cb;
} enable_line_message_handler;

/**
 * PowerStatus message handler
 */
typedef struct
{
    /** Pointer back to generic message handler */
    ecp_message_handler super;
    /** Pointer to power status callback */
    power_status_cb cb;
} power_status_message_handler;

/**
 * Intermediate function used by ecp_handle_message
 * to parse out the DBusMessage into native data structures
 * and then hand off to the message specific callback
 * @param[in] context
 * @param[in] message
 * @param[in] handler
 * @return KECPStatus
 */
KECPStatus on_enable_line_parser(const ecp_context *           context,
                                 DBusMessage *                 message,
                                 struct _ecp_message_handler * handler);

/**
 * Creates and listener + registers callback for the
 * EnableLine method. This function should be used by the
 * process which is hosting the method
 * @param[in] context
 * @param[in] cb
 * @return KECPStatus
 */
KECPStatus on_enable_line(ecp_context * context, enable_line_cb cb);

/**
 * Calls out to the EnableLine method
 * @param[in] context
 * @param[in] line
 * @return KECPStatus
 */
KECPStatus enable_line(ecp_context * context, uint8_t line);

/**
 * Parses out a PowerStatus signal into an eps_power_state struct.
 * @param[in] status
 * @param[in] message
 * @return KECPStatus
 */
KECPStatus parse_power_status_message(eps_power_state * status,
                                      DBusMessage *     message);

/**
 * Takes a eps_power_state struct and creates a PowerStatus signal.
 * @param[in] status
 * @param[in] message
 * @return KECPStatus
 */
KECPStatus format_power_status_message(eps_power_state status,
                                       DBusMessage **  message);

/**
 * Intermediate function used by ecp_handle_message
 * to parse out the DBusMessage into native data structures
 * and then hand off to the message specific callback
 * @param[in] context
 * @param[in] message
 * @param[in] handler
 * @return KECPStatus
 */
KECPStatus on_power_status_parser(const ecp_context *           context,
                                  DBusMessage *                 message,
                                  struct _ecp_message_handler * handler);

/**
 * Creates a listener + registers callback for the PowerStatus signal.
 * This function should be used by a process which is subscribed
 * to the PowerStatus signal.
 * @param[in] context
 * @param[in] cb
 * @return KECPStatus
 */
KECPStatus on_power_status(ecp_context * context, power_status_cb cb);

/* @} */
