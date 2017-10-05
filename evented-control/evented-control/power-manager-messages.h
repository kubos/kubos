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
 * Fuction pointer for EnableLine callback
 */
typedef ECPStatus (*EnableLineCb)(uint8_t line);

/**
 * EnableLine message handler
 */
typedef struct
{
    /** Pointer back to generic message handler */
    ECPMessageHandler super;
    /** Pointer to enable line callback */
    EnableLineCb cb;
} ECPEnableLineMessageHandler;

/**
 * Intermediate function used by ECP_Handle_Message
 * to parse out the DBusMessage into native data structures
 * and then hand off to the message specific callback
 * @param[in] context
 * @param[in] message
 * @param[in] handler
 * @return ECPStatus
 */
ECPStatus on_enable_line_parser(ECPContext * context, DBusMessage * message,
                                struct _ECPMessageHandler * handler);

/**
 * Creates and listener + registers callback for the
 * EnableLine method. This function should be used by the
 * process which is hosting the method
 * @param[in] context
 * @param[in] cb
 * @return ECPStatus
 */
ECPStatus on_enable_line(ECPContext * context, EnableLineCb cb);

/**
 * Calls out to the EnableLine method
 * @param[in] context
 * @param[in] line
 * @return ECPStatus
 */
ECPStatus enable_line(ECPContext * context, uint8_t line);

/**
 * PowerManager signal name
 */
#define POWER_MANAGER_STATUS "PowerStatus"

/**
 * Function ponter type for PowerStatus callback
 */
typedef ECPStatus (*PowerStatusCb)(eps_power_status status);

/**
 * PowerStatus message handler
 */
typedef struct
{
    /** Pointer back to generic message handler */
    ECPMessageHandler super;
    /** Pointer to power status callback */
    PowerStatusCb cb;
} ECPPowerStatusMessageHandler;

/**
 * Parses out a PowerStatus signal into an eps_power_status struct.
 * @param[in] status
 * @param[in] message
 * @return ECPStatus
 */
ECPStatus parse_power_status_message(eps_power_status * status,
                                     DBusMessage *      message);

/**
 * Takes a eps_power_status struct and creates a PowerStatus signal.
 * @param[in] status
 * @param[in] message
 * @return ECPStatus
 */
ECPStatus format_power_status_message(eps_power_status status,
                                      DBusMessage **   message);

/**
 * Intermediate function used by ECP_Handle_Message
 * to parse out the DBusMessage into native data structures
 * and then hand off to the message specific callback
 * @param[in] context
 * @param[in] message
 * @param[in] handler
 * @return ECPStatus
 */
ECPStatus on_power_status_parser(ECPContext * context, DBusMessage * message,
                                 struct _ECPMessageHandler * handler);

/**
 * Creates a listener + registers callback for the PowerStatus signal.
 * This function should be used by a process which is subscribed
 * to the PowerStatus signal.
 * @param[in] context
 * @param[in] cb
 * @return ECPStatus
 */
ECPStatus on_power_status(ECPContext * context, PowerStatusCb cb);

/* @} */
