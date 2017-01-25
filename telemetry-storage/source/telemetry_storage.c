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
#include <stdlib.h>
#include <stdio.h>

#include "telemetry-storage/telemetry_storage.h"
#include "telemetry-storage/config.h"
#include "telemetry-storage/disk.h"


CSP_DEFINE_TASK(telemetry_store_rx)
{
    telemetry_packet packet;
    telemetry_conn connection;

    /* Subscribe to telemetry publishers as specified in the configuration */
    while (!telemetry_subscribe(&connection, STORAGE_SUBSCRIPTIONS))
    {
        /* Retry subscribing at the interval specified in the configuration*/
        csp_sleep_ms(STORAGE_SUBSCRIBE_RETRY_INTERVAL);
    }

    while (1)
    {
        if (telemetry_read(connection, &packet))
        {
            /* Store telemetry packets from the telemetry system */
            telemetry_store(packet);
        }
    }
}


void telemetry_storage_init()
{
    TELEMETRY_STORE_THREAD;
}


/**
 * @brief creates a filename that corresponds to the telemetry packet source_id and 
 *        the csp packet address.
 * @param filename_buf_ptr a pointer to the char[] to write to.
 * @param source_id the telemetry packet source_id from packet.source.source_id.
 * @param address the csp packet address from packet->id.src. 
 * @param file_extension. 
 * @retval The length of the filename written.
 */
static uint16_t create_filename(char *filename_buf_ptr, uint8_t source_id, unsigned int address, const char *file_extension)
{
    int len;

    if (filename_buf_ptr == NULL || file_extension == NULL) 
    {
        return 0;
    }

    len = snprintf(filename_buf_ptr, FILE_NAME_BUFFER_SIZE, "%u%u%s", source_id, address, file_extension);

    if(len < 0 || len >= FILE_NAME_BUFFER_SIZE) 
    {
        printf("Filename char limit exceeded. Have %d, need %d + \\0\n", FILE_NAME_BUFFER_SIZE, len);
        return 0;
    }
    return len;
}


/**
 * @brief creates a formatted log entry from the telemetry packet.
 * @param data_buf_ptr a pointer to the char[] to write to.
 * @param packet a telemetry packet to create a log entry from.
 * @retval The length of the log entry written.
 */
static uint16_t format_log_entry_csv(char *data_buf_ptr, telemetry_packet packet) 
{
    int len = 0;

    if (data_buf_ptr == NULL) 
    {
        return 0;
    }

    if(packet.source.data_type == TELEMETRY_TYPE_INT) 
    {
        len = snprintf(data_buf_ptr, DATA_BUFFER_SIZE, "%u,%d\r\n", packet.timestamp, packet.data.i);
        if(len < 0 || len >= DATA_BUFFER_SIZE) 
        {
            printf("Data char limit exceeded for int packet. Have %d, need %d + \\0\n", DATA_BUFFER_SIZE, len);
            return 0;
        }
    }

    if(packet.source.data_type == TELEMETRY_TYPE_FLOAT) 
    {
        len = snprintf(data_buf_ptr, DATA_BUFFER_SIZE, "%u,%f\r\n", packet.timestamp, packet.data.f);
        if(len < 0 || len >= DATA_BUFFER_SIZE) 
        {
            printf("Data char limit exceeded for float packet. Have %d, need %d + \\0\n", DATA_BUFFER_SIZE, len);
            return 0;
        }
    }
    return len;
}


/**
 * @brief print telemetry packet data.
 * @param packet a telemetry packet with data to print.
 */
static void print_to_console(telemetry_packet packet)
{
    if(packet.source.data_type == TELEMETRY_TYPE_INT) 
    {
        printf("%d\r\n", packet.data.i);
    }

    if(packet.source.data_type == TELEMETRY_TYPE_FLOAT) 
    {
        printf("%f\r\n", packet.data.f);
    }
}


bool telemetry_store(telemetry_packet packet)
{
    static char filename_buffer[FILE_NAME_BUFFER_SIZE];
    static char *filename_buf_ptr;
    static char data_buffer[DATA_BUFFER_SIZE];
    static char *data_buf_ptr;

    uint16_t data_len;
    uint16_t filename_len;
    uint16_t sd_stat;

    filename_buf_ptr = filename_buffer;
    data_buf_ptr = data_buffer;
    
    if(DATA_OUTPUT_FORMAT == FORMAT_TYPE_CSV)
    { 
        filename_len = create_filename(filename_buf_ptr, packet.source.source_id, packet.source.subsystem_id, FILE_EXTENSION_CSV);
        data_len = format_log_entry_csv(data_buf_ptr, packet);
    
        /* Save log entry */
        if (filename_len > 0 && data_len > 0)
        {
            sd_stat = disk_save_string(filename_buf_ptr, data_buf_ptr, data_len);
            if (sd_stat != 0)
            {
                printf("Error saving telemetry log entry. Code %u \r\n", sd_stat);
                return false;
            }
            //printf("Log Entry = %s\n", data_buf_ptr);
            //printf("Filename = %s\n", filename_buf_ptr);
            //printf("The data length %u\r\n", data_len);
            return true;
        }
        else 
        {
            printf("Error, decoding log entry or filename are blank \r\n");
            return false;
        }
    }
    else if(DATA_OUPUT_FORMAT == FORMAT_TYPE_NONE)
    {
        /* Placeholder for no file extension */
    }
    else if(DATA_OUTPUT_FORMAT == FORMAT_TYPE_HEX)
    { 
        /* Placeholder for hexidecimal format */
    }
    else
    {
        printf("Telemetry storage format type not found\r\n");
    }
}
