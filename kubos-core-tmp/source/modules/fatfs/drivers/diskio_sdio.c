/*
 * KubOS Core Flight Services
 * Copyright (C) 2015 Kubos Corporation
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

/**
{
    "fs": {
        "fatfs": {
            "driver": "sdio"
        }
    }
}
**/

#ifdef YOTTA_CFG_FS_FATFS_DRIVER_SDIO

#include "kubos-core/modules/fatfs/diskio.h"
#include "kubos-hal/sdio.h"

/* Status of SDCARD */
static volatile DSTATUS Stat = STA_NOINIT;

#define SD_BLOCK_SIZE 512


DSTATUS disk_initialize (BYTE pdrv)
{
    if (k_sdio_init() != SDIO_OK)
    {
        return STA_NOINIT;
    }

    return RES_OK;
}

DSTATUS disk_status (BYTE pdrv)
{
    Stat = STA_NOINIT;
    if (k_sdio_card_status() == SDIO_OK)
    {
        Stat &= ~STA_NOINIT;
    }
    else
    {
        Stat |= STA_NOINIT;
    }

    return Stat;
}

DRESULT disk_read (BYTE pdrv, BYTE * buff, DWORD sector, UINT count)
{
    uint64_t block_addr = sector * SD_BLOCK_SIZE;
    if (k_sdio_read_blocks((uint32_t*)buff, block_addr, SD_BLOCK_SIZE, count) != SDIO_OK)
    {
        return RES_ERROR;
    }
    return RES_OK;
}

DRESULT disk_write (BYTE pdrv, const BYTE* buff, DWORD sector, UINT count)
{
    uint64_t block_addr = sector * SD_BLOCK_SIZE;
    if (k_sdio_write_blocks((uint32_t*)buff, block_addr, SD_BLOCK_SIZE, count) != SDIO_OK)
    {
        return RES_ERROR;
    }
    return RES_OK;
}

DRESULT disk_ioctl (BYTE pdrv, BYTE cmd, void* buff)
{
    DRESULT res = RES_ERROR;
    k_sdio_card_info_t card_info;

    switch (cmd) {
        /* Make sure that no pending write process */
        case CTRL_SYNC :
            res = RES_OK;
            break;

        /* Size in bytes for single sector */
        case GET_SECTOR_SIZE:
            *(WORD *)buff = SD_BLOCK_SIZE;
            res = RES_OK;
            break;

        /* Get number of sectors on the disk (DWORD) */
        case GET_SECTOR_COUNT :
            card_info = k_sdio_card_info();
            *(DWORD *)buff = card_info.capacity / SD_BLOCK_SIZE;
            res = RES_OK;
            break;

        /* Get erase block size in unit of sector (DWORD) */
        case GET_BLOCK_SIZE :
            *(DWORD*)buff = SD_BLOCK_SIZE;
            break;

        default:
            res = RES_PARERR;
    }

    return res;
}

DWORD get_fattime (void)
{
    return      ((DWORD)(2013 - 1980) << 25)    /* Year 2013 */
        | ((DWORD)7 << 21)                /* Month 7 */
        | ((DWORD)28 << 16)                /* Mday 28 */
        | ((DWORD)0 << 11)                /* Hour 0 */
        | ((DWORD)0 << 5)                /* Min 0 */
        | ((DWORD)0 >> 1);                /* Sec 0 */
}

#endif
