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

#include <kubos-core/modules/klog.h>

#ifdef YOTTA_CFG_LOG_PART_SIZE
#define LOG_PART_SIZE YOTTA_CFG_LOG_PART_SIZE
#else
#define LOG_PART_SIZE 4096 //This is totally random
#endif

#ifdef YOTTA_CFG_LOG_MAX_PARTS
#define LOG_MAX_PARTS YOTTA_CFG_LOG_MAX_PARTS
#else
#define LOG_MAX_PARTS 128 //This is totally random too
#endif

/* Global log handle */
klog_handle log_handle;
