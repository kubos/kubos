/*
 * KubOS RT
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
#ifndef DISK_H
#define DISK_H

#include <stdio.h>

/* Expanded error codes in addition to fatfs return codes */
typedef enum {
    END_OF_FILE = 20,   /* f_eof returned a non-zero value indicating EOF */
    NOT_A_DIGIT,        /* (21) isdigit returned 0 indicating not a digit */
    F_WRITE_ERROR       /* (22) f_write returned -1 indicating an error */
} EXPANDED_RESULTS;


/**
 * @brief open, appends, and writes or if the filename does not exist
 *        creates a new file and writes a string.
 * @param file_path a pointer to the filename to write to.
 * @param data_buffer a pointer to the string to write.
 * @param data_len the length of the string.
 * @retval a table of values which (0 being 'okay') is found at 
 *         http://elm-chan.org/fsw/ff/en/rc.html
 */
uint16_t disk_save_string(const char *file_path, char *data_buffer, uint16_t data_len);


/**
 * @brief open a file and load an unsigned 16 bit integer.
 * @param file_path a pointer to the filename to open.
 * @param value the pointer to store the retrieved value.
 * @param entry the file line to read from the top of the file.
 * @retval a table of values which (0 being 'okay') is found at 
 *         http://elm-chan.org/fsw/ff/en/rc.html and the expanded 
 *         results.
 */
uint16_t disk_load_uint16(const char *file_path, uint16_t * value, uint16_t line);


/**
 * @brief open and appends and writes or if the filename does not exist
 *        creates a new file and writes an unsigned 16 bit integer.
 * @param file_path a pointer to the filename to open.
 * @param value the value to write.
 * @retval a table of values which (0 being 'okay') is found at 
 *         http://elm-chan.org/fsw/ff/en/rc.html and the expanded 
 *         results.
 */
uint16_t disk_save_uint16(const char *file_path, uint16_t value);


#endif
