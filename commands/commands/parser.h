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

#include <stdbool.h>

#define DEFAULT_COMMAND_STR_LENGTH 75

bool core_parse_args(int argc, char ** argv, char * command_name);

unsigned long get_hash(char *str);

int ping();

int build_info();

int exec_reboot();

typedef struct
{
    int ping;
    int build_info;
} Arguments;

