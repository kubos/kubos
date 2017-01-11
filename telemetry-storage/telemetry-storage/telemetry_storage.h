/*
 * Copyright (C) 2016 Kubos Corporation
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

#include <telemetry/telemetry.h>

#define FILE_EXTENSION_CSV ".csv"
#define FILE_EXTENSION_HEX ".hex"


typedef enum 
{
    FORMAT_TYPE_CSV = 0,        /* (0) CSV */
    FORMAT_TYPE_HEX             /* (1) HEX */
} output_data_format;


/**
 * @brief store a telemetry packet in a particular format specified by
 *        the configuration for logging.
 * @param packet the telemetry packet to store.
 */
void telemetry_store(telemetry_packet packet);


/**
 * @brief print telemetry packet data.
 * @param packet a telemetry packet with data to print.
 */
void print_to_console(telemetry_packet data);


#endif
