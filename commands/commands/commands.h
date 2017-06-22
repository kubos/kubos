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

#ifdef YOTTA_CFG_COMMANDS_DEFAULT_COMMAND_STR_LEN
#define DEFAULT_COMMAND_STR_LEN YOTTA_CFG_COMMANDS_DEFAULT_COMMAND_STR_LEN
#else
#define DEFAULT_COMMAND_STR_LEN 75
#endif

#ifdef YOTTA_CFG_COMMANDS_SUPERVISOR_MAX_REQUEST_RETRIES
#define SUPERVISOR_MAX_REQUEST_RETRIES YOTTA_CFG_COMMANDS_SUPERVISOR_MAX_REQUEST_RETRIES
#else
#define SUPERVISOR_MAX_REQUEST_RETRIES 3
#endif

int reboot(); //Power cylce the iOBC

int reset(); //Supervisor reset

int emergency_reset(); //emergency reset

int get_and_run_command(char * command_name);

bool core_parse_args(int argc, char ** argv, char * cmd_string);
