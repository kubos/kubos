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
#ifndef TELEMETRY_STORAGE_CONFIG_H
#define TELEMETRY_STORAGE_CONFIG_H


/* If there is a configuration present... */
#ifdef YOTTA_CFG_TELEMETRY_STORAGE
/* Maximum size/length of the filename buffer */
#define FILE_NAME_BUFFER_SIZE YOTTA_CFG_TELEMETRY_STORAGE_FILE_NAME_BUFFER_SIZE

/* Maximum size/length of the storage buffer */
#define DATA_BUFFER_SIZE YOTTA_CFG_TELEMETRY_STORAGE_DATA_BUFFER_SIZE

/* Output format (CSV (0), HEX (1), etc) */
#define DATA_OUTPUT_FORMAT YOTTA_CFG_TELEMETRY_STORAGE_DATA_OUTPUT_FORMAT

#endif


/* If there is not a configuration present use defaults... */
#ifndef YOTTA_CFG_TELEMETRY_STORAGE

/* Set at FatFs LFN max length */
#define FILE_NAME_BUFFER_SIZE 255

#define DATA_BUFFER_SIZE 128

/* CSV default */
#define DATA_OUTPUT_FORMAT FORMAT_TYPE_CSV

#endif


#endif
