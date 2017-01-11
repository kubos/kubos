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
#include <ctype.h>
#include <string.h>

#include "telemetry-storage/disk.h"

#include <kubos-core/modules/fatfs/ff.h>
#include <kubos-core/modules/fatfs/diskio.h>
#include <kubos-core/modules/fs/fs.h>


/** 
 * @brief Open a file for write and append.
 * @param fp a pointer to the file object structure.
 * @param path to the file to open.
 * @return ret a table of values which (0 being 'okay') is found at
 *         http://elm-chan.org/fsw/ff/en/rc.html  
 */
static uint16_t open_append(FIL *Fil, const char *path)
{
    uint16_t ret;
    ret = f_open(Fil, path, FA_WRITE | FA_OPEN_ALWAYS);
    if (ret == FR_OK)
    {
        ret = f_lseek(Fil, f_size(Fil));
        if (ret != FR_OK)
        {
            f_close(Fil);
        }
    }
    return ret;
}


/** 
 * @brief Register a work area of a volume and open the file to write
 *        and append.
 * @param FatFs a pointer to the file system object.
 * @param fp a pointer to the file object structure.
 * @param path to the file to open.
 * @return ret a table of values which (0 being 'okay') is found at
 *         http://elm-chan.org/fsw/ff/en/rc.html  
 */
static uint16_t open_file_write(FATFS *FatFs, FIL *Fil, const char *path)
{
    uint16_t ret;
    if ((ret = f_mount(FatFs, "", 1)) == FR_OK)
    {
        ret = open_append(Fil, path);
    }
    return ret;
}


/** 
 * @brief Register a work area of a volume and open the file to read.
 * @param FatFs a pointer to the file system object.
 * @param fp a pointer to the file object structure.
 * @param path to the file to open.
 * @return ret a table of values which (0 being 'okay') is found at
 *         http://elm-chan.org/fsw/ff/en/rc.html  
 */
static uint16_t open_file_read(FATFS *FatFs, FIL *Fil, const char *path)
{
    uint16_t ret;
    if ((ret = f_mount(FatFs, "", 1)) == FR_OK)
    {
        ret = f_open(Fil, path, FA_READ | FA_OPEN_EXISTING);
    }
    return ret;
}


/**
 * TODO improve this function and implement more efficient line counting.
 * @brief Checks for an available string to read from the calibration 
 *        profile file and, if it is available, reads the string.
 * @param Fil,  pointer to the file object structure
 * @param value a pointer to the value to write back to.
 * @param line the line number to read from.
 * @return ret, FR_OK (0), END_OF_FILE (20), or NOT_A_DIGIT (21).
 */
uint16_t read_value(FIL *Fil, uint16_t *value, uint16_t line)
{
    uint16_t ret = FR_OK;
    uint16_t temp = 0;
    int c;
    int i;
    char buffer[128];

    /* Move to the line specified */
    for (i = 0; i < line; i++)
    {
        /* Make sure there's something to read */
        if(f_eof(Fil))
        {
            return END_OF_FILE;
        }
        f_gets(buffer, sizeof buffer, Fil);
    }

    /* Make sure there's something to read */
    if(f_eof(Fil))
    {
        return END_OF_FILE;
    }

    f_gets(buffer, sizeof buffer, Fil);

    if(!isdigit(buffer[0]))
    {
        return NOT_A_DIGIT;
    }

    /* convert read string to uint */
    for (c = 0; isdigit(buffer[c]); c++)
    {
        temp = temp * 10 + buffer[c] - '0';
    }

    *value = temp;

    return ret;
}


/** 
 * @brief Write a unsigned 16-bit integer to file.
 * @param Fil, pointer to the file object structure.
 * @param value, the uint16_t value to write.
 * @return ret returns 0 for success and F_WRITE_ERROR (22) for failure.  
 */
uint16_t write_value(FIL *Fil, uint16_t value)
{
    if ((f_printf(Fil, "%d\n", value)) != -1)
    {
        return  0;
    }
    return F_WRITE_ERROR;
}


uint16_t disk_save_string(const char *file_path, char *data_buffer, uint16_t data_len)
{
    static FATFS FatFs;
    static FIL Fil;
    UINT bw;
    uint16_t sd_stat = FR_OK;
    
    sd_stat = open_file_write(&FatFs, &Fil, file_path);

    /* Retry once */
    if (sd_stat != FR_OK)
    {
        f_close(&Fil);
        f_mount(NULL, "", 0);
        sd_stat = open_file_write(&FatFs, &Fil, file_path);
    }

    if (sd_stat == FR_OK)
    {
        sd_stat = f_write(&Fil, data_buffer, data_len, &bw);
        f_close(&Fil);
    }
    return sd_stat;
}


uint16_t disk_load_uint16(const char *file_path, uint16_t *value, uint16_t line)
{
    static FATFS FatFs;
    static FIL Fil;
    uint16_t sd_stat = FR_OK;

    sd_stat = open_file_read(&FatFs, &Fil, file_path);

    if (sd_stat == FR_OK)
    {
        sd_stat = read_value(&Fil, value, line);
        f_close(&Fil);
    }
    return sd_stat;
}


uint16_t disk_save_uint16(const char *file_path, uint16_t value)
{
    static FATFS FatFs;
    static FIL Fil;
    uint16_t sd_stat = FR_OK;

    sd_stat = open_file_write(&FatFs, &Fil, file_path);

    /* Retry once */
    if (sd_stat != FR_OK)
    {
        f_close(&Fil);
        f_mount(NULL, "", 0);
        sd_stat = open_file_write(&FatFs, &Fil, file_path);
    }

    if (sd_stat == FR_OK)
    {
        sd_stat = write_value(&Fil, value);
        f_close(&Fil);
    }
    return sd_stat;
}

