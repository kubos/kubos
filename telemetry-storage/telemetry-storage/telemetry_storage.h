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
#ifndef TELEMETRY_STORAGE_H
#define TELEMETRY_STORAGE_H

#include <stdbool.h>
#include <telemetry/telemetry.h>
#include <kubos-core/modules/klog.h>
#include "telemetry-storage/config.h"

#define FILE_EXTENSION_CSV ".csv"
#define FILE_EXTENSION_HEX ".hex"
#define FILE_EXTENSION_NONE ""


typedef enum 
{
    FORMAT_TYPE_CSV = 0,        /* (0) CSV */
    FORMAT_TYPE_HEX             /* (1) HEX */
} output_data_format;


/**
 * Task used to subscribe to, receive, and store all telemetry packets.
 */
CSP_DEFINE_TASK(telemetry_store_rx);

/**
 * Subscribes to all telemetry sources and stores them as specified
 * in the configuration. 
 * To be used in the main() prior to starting the scheduler and after
 * the telemetry system has been initialized.
 */
void telemetry_storage_init();


/**
 * @brief store a telemetry packet in a particular format specified by
 *        the configuration.
 * @param packet the telemetry packet to store.
 * @retval true if successful, otherwise false
 */
bool telemetry_store(telemetry_packet packet);


#endif
