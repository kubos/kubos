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

#define POWER_MANAGER_INTERFACE     "org.KubOS.PowerManager"
#define POWER_MANAGER_PATH          "/org/KubOS/PowerManager"
#define POWER_MANAGER_STATUS        "PowerStatus"

typedef tECP_Error (*power_status_cb)(eps_power_status status);

tECP_Error check_message(DBusMessage * message, const char * interface, const char * member);
tECP_Error parse_power_status_message(eps_power_status * status, DBusMessage * message);
tECP_Error format_power_status_message(eps_power_status status, DBusMessage ** message);
tECP_Error on_power_status(tECP_Context * context, power_status_cb cb);
