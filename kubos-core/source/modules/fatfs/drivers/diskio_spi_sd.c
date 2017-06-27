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
 * The document SD Specifications Part 1 Physical Layer Simplified Specification v5.00
 * is heavily referenced in this source. Any numerical sections indicated in comments
 * are assumed to be from that document, unless otherwise stated.
 *
 * A copy of the specification for reference - https://www.sdcard.org/downloads/pls/pdf/part1_500.pdf.
 */

/**
{
    "fs": {
        "fatfs": {
            "driver": "spi": {
                "dev": "K_SPI1",
                "cs": "P26"
            }
        }
    }
}
**/

#ifdef YOTTA_CFG_FS_FATFS_DRIVER_SPI

#include <stdio.h>
#include "kubos-core/modules/fatfs/diskio.h"
#include "kubos-hal/spi.h"
#include "kubos-hal/gpio.h"
#include "FreeRTOS.h"
#include "task.h"

#define SPI_DEV YOTTA_CFG_FS_FATFS_DRIVER_SPI_DEV
#define SPI_CS YOTTA_CFG_FS_FATFS_DRIVER_SPI_CS

#define SD_BLOCK_SIZE 512

/**
 * SD Card type defines (for internal usage)
 */
#define TYPE_NONE 0x00
/* MMC ver 3 */
#define TYPE_MMC 0x01
/* SD ver 1 */
#define TYPE_SD1 0x02
/* SD ver 2 */
#define TYPE_SD2 0x04
/* SD */
#define TYPE_SDC (TYPE_SD1 | TYPE_SD2)

/**
 * See Section 7.3.1.3 for a detailed command list.
 *
 * We use 0x80 to mark bit 8 high as a way to indicate 
 * "application commands", as these need slightly different
 * treatment when transmitting.
 */
typedef enum
{
    CMD0_GO_IDLE_STATE = 0,
    CMD1_SEND_OP_COND = 1,
    CMD5_NO_SEND_OP_COND = 5,
    CMD8_SEND_IF_COND = 8,
    CMD9_SEND_CSD = 9,
    CMD10_SEND_CID = 10,
    CMD12_STOP_TRANSMISSION = 12,
    CMD13_SEND_STATUS = 13,
    CMD16_SET_BLOCKLEN = 16,
    CMD17_READ_SINGLE_BLOCK = 17,
    CMD18_READ_MULTIPLE_BLOCK = 18,
    CMD23_SET_BLOCK_COUNT = 23,
    CMD24_WRITE_BLOCK = 24,
    CMD25_WRITE_MULTIPLE_BLOCK = 25,
    CMD32_ERASE_WR_BLK_START = 32,
    CMD33_ERASE_WR_BLK_END = 33,
    CMD38_ERASE = 38,
    CMD55_APP_CMD = 55,
    CMD56_GEN_CMD = 56,
    CMD58_READ_OCR = 58,
    ACMD13_SD_STATUS = (0x80+13),
    ACMD23_SET_WR_BLK_ERASE_COUNT = (0x80+23),
    ACMD41_SEND_OP_COND_SDC = (0x80+41)
} spi_cmd;

/**
 * @param buffer Buffer to store received data in
 * @param block_length Length of buffer to receive
 * @return uint8_t 0 if receive failed, otherwise 1 
 */
static uint8_t receive_datablock(uint8_t * buffer, uint16_t block_length);

/**
 * @param cmd command to send to sd card
 * @param arg command argument to send
 * @return uint8_t first byte of sd card's response
 */
static uint8_t send_cmd(spi_cmd cmd, uint32_t arg);

static uint8_t dummy = 0xFF;
static int sd_card_type = TYPE_NONE;

static void cs_high(void)
{
    k_gpio_write(SPI_CS, 1);
}

static void cs_low(void)
{
    k_gpio_write(SPI_CS, 0);
}

static DRESULT init_spi(void)
{
    /**
     * Chan (http://elm-chan.org/docs/mmc/mmc_e.html) suggests we run in SPI Mode 0
     * when interfacing with the SD Card. This appears to work well so here we are.
     */
    KSPIConf conf = (KSPIConf) {
        .role = K_SPI_MASTER,
        .direction = K_SPI_DIRECTION_2LINES,
        .data_size = K_SPI_DATASIZE_8BIT,
        .clock_phase = K_SPI_CPOL_LOW,
        .clock_polarity = K_SPI_CPHA_1EDGE,
        .first_bit = K_SPI_FIRSTBIT_MSB,
        .speed = 400000
    };

    k_spi_init(SPI_DEV, &conf);

    /* Do not enable chip select prior to initializing spi */
    /* Initialize chip select pin */
    k_gpio_init(SPI_CS, K_GPIO_OUTPUT, K_GPIO_PULL_UP);
    cs_high();    

    return RES_OK;
}

static int wait_ready(int wait)
{
    uint8_t resp = 0;
    int count = 0;
    do
    {
        k_spi_write_read(SPI_DEV, &dummy, &resp, 1);
    } 
    while((resp != 0xFF) && (count++ < wait));

    if (resp == 0xFF)
    {
        return 1;
    }
    else
    {
        return 0;
    }
}

 
static void spi_deselect(void)
{
    /**
     * Section 3.7.1, Table 3.1 - In SPI mode asserting positive on chip select indicates inactive. 
     */
    cs_high();
}


static int spi_select(void)
{
    /**
     * Section 3.7.1, Table 3.1 - In SPI Mode asserting negative on chip select indicates active.
     */
    cs_low();

    if (wait_ready(500))
    {
        return 1;
    }

    spi_deselect();
    return 0;
}

/**
 * Sections 4.3.3 and 7.2.3 detail the SD data reading process.
 */
static uint8_t receive_datablock(uint8_t * buffer, uint16_t block_length)
{
    uint8_t crc[2];
    uint8_t token;
    uint8_t count = 0;

    /**
     * Section 7.2.3 - SD Card will respond to a valid read command
     * with a response followed by a data token.
     */
    do 
    {
        k_spi_read(SPI_DEV, &token, 1);
    } 
    while ((token == 0xFF) && (count++ < 254));

    /**
     * Section 7.3.3.2 - For single block read and multiple block read the data token is:
     *
     *      7 6 5 4 3 2 1 0
     *     +---------------+
     *     |1|1|1|1|1|1|1|0|
     *     +---------------+
     * 
     * Which is 0xFE in hex. Any other token is considered an error.
     */ 
    if (token != 0xFE)
    {
        return 0;
    }

    /**
     * Section 7.2.3 - Data block will be sent after the data token.
     * Data block is followed up by a 16-bit CRC.
     */
    k_spi_read(SPI_DEV, buffer, block_length);

    /* CRC is not currently verified but it must be read for the
     * slave to complete the operation
     */
    k_spi_write_read(SPI_DEV, &dummy, &crc[0], 1);
    k_spi_write_read(SPI_DEV, &dummy, &crc[1], 1);
    return 1;
}

static uint8_t transmit_datablock(const uint8_t * buffer, uint8_t token)
{
    uint8_t response;

    /* Do not send data until card is out of busy state */
    if (!wait_ready(500))
    {
        return 0;
    }

    /**
     * Section 7.2.4 - Each transmitted datablock will be preceded by a data token
     * Section 7.3.3.2 - 0xFD represents the Stop Transmission token.
     * If we send this we should not send a data block
     */
    k_spi_write(SPI_DEV, &token, 1);
    if (token != 0xFD)
    {
        k_spi_write(SPI_DEV, (uint8_t *) buffer, SD_BLOCK_SIZE);
        k_spi_write(SPI_DEV, &dummy, 1);
        k_spi_write(SPI_DEV, &dummy, 1);
        k_spi_read(SPI_DEV, &response, 1);
        /**
         * Section 7.3.3.1 - Each block written will be acknowledged by a data token.
         * A token indicating the data was accepted will be:
         *
         *     4 3 2 1 0
         *   +-----------+
         *   | 0|0|1|0|1 |
         *   +-----------+
         *
         * In hex this ends up 0x05.
         * Note - Bits 7-5 are ignored in this check.
         */
        if ((response & 0x1F) != 0x05)
        {
            return 0;
        }
    }
    /**
     * We do not currently check for programming errors via CM13 (Section 7.2.4)
     * But we do wait for the SD to be done writing/ready (echoing back 0xFF).
     */
    wait_ready(1000);
    return 1;
}


static uint8_t send_cmd(spi_cmd cmd, uint32_t arg)
{
    uint8_t command[6];
    uint8_t response = 0;
    int n;

    /**
     * Section 4.3.9 - Prior to sending an Application Command, CMD55 must be sent
     * to prepare the sd card.
     */
    if (cmd & 0x80)
    {
        /* Clear bit 8, our special marker for application comands */
        cmd &= 0x7F;
        response = send_cmd(CMD55_APP_CMD, 0);
        if (response > 1) 
        {
            return response;
        }
    }
    
    if (cmd != CMD12_STOP_TRANSMISSION)
    {
        spi_deselect();
        if (!spi_select()) 
        {
            return 0xFF;
        }
    }
    
    /**
     * Section 7.3.1.1
     * The first byte of the command contains the start bit, transmission bit
     * and command index, it is layed out like so:
     *    7 6 5 4 3 2 1 0
     *  +-----------------+
     *  | 0|1|x|x|x|x|x|x |
     *  +-----------------+
     *  The command is OR'd with 0x40 to set the tranmission bit.
     */
    command[0] = (0x40 | cmd);
    /**
     * The (long) command argument must be broken up into four bytes.
     */
    command[1] = (uint8_t)(arg >> 24);
    command[2] = (uint8_t)(arg >> 16);
    command[3] = (uint8_t)(arg >> 8);
    command[4] = (uint8_t)(arg >> 0);
    /**
     * Last byte is CRC + end bit (1). A default CRC value of 0 is used
     * for all commands except those that require it (CMD0, CMD8).
     *
     * Section 7.2.2 - The SPI interface is initialized in the CRC off mode
     * by default. The reset command (CMD0) requires CRC because it is received
     * in SD mode, and CMD8 always has CRC verification on.
     */
    command[5] = 0x01;
    if (cmd == CMD0_GO_IDLE_STATE)  command[5] = 0x95;
    if (cmd == CMD8_SEND_IF_COND)   command[5] = 0x87;
    k_spi_write(SPI_DEV, command, 6);    
    
    vTaskDelay(10);
    n = 10;
    do
    {
        k_spi_read(SPI_DEV, &response, 1);    
    }
    while((response & 0x80) && --n);

    return response;
}


/**
 * Sections 4.2.3 and 7.2.1 detail the card initialization
 * and indentification process.
 *
 * @param pdrv Physical drive number. Currenly only one drive is supported so this is ignored.
 */
DSTATUS disk_initialize (BYTE pdrv)
{
    DSTATUS status = STA_NOINIT;
    uint8_t buf;
    int count = 0;
    uint8_t ty = 0;
    uint8_t i = 0;
    uint8_t resp = 0;
    uint8_t ocr[5];

    if (init_spi() != RES_OK)
    {
        return status;
    }    

    /**
     * Section 7.2.1 - The SD Card will enter SPI mode if CS is asserted (negative)
     * and CMD0 is received.
     */
    cs_low();

    // Send 10 bytes to give the SD 80 clock cycles to wake up
    for (uint8_t i = 0; i < 10; i++)
    {
        k_spi_write_read(SPI_DEV, &dummy, &buf, 1);
    }

    /**
     * Section 7.3.1.3 - CMD0 Resets the SD Memory Card, causing the Card to enter idle state.
     * Also see Section 6.4.1 - Power Up Sequence.
     */
    if (send_cmd(CMD0_GO_IDLE_STATE, 0) == 1)
    {
        /**
         * Section 4.3.13 -  CMD8 initialize SD v2.00 Cards currently in idle state.
         * Cards which do not support this command (pre v2.00) will not respond 
         * and will remain in idle state.
         * Check pattern 0x1AA should be echoed back if accepted.
         */
        if (send_cmd(CMD8_SEND_IF_COND, 0x1AA) == 1)
        {
            /* Figure 4-2 - A valid response to CMD8 indicates V2.00 or later */
            k_spi_read(SPI_DEV, ocr, 4);
            /* Look for echoed check pattern */
            if ((ocr[2] == 0x01) && (ocr[3] == 0xAA))
            {
                count = 0;
                while (count++ < 1000)
                {
                    /**
                     * Section 6.4.1 Power Up Sequence
                     * ACMD41 activates the initialization process.
                     * Repeatedly sending allows us to check the busy bit of the SD card.
                     * Busy bit will be unset once the initialization process is complete.
                     * Section 4.3.2.1 - The busy bit is bit 31.
                     */
                    if (send_cmd(ACMD41_SEND_OP_COND_SDC, 1UL << 30) == 0)
                    {
                        break;
                    }
                }
                
                
                if (count < 1000)
                {
                    /**
                     * Section 7.2.1 - After sending CMD8, CMD58 may be sent to check the Card Capacity Status.
                     */
                    if (send_cmd(CMD58_READ_OCR, 0) == 0)
                    {
                        k_spi_read(SPI_DEV, ocr, 4);
                        /**
                        * It appears that the response from CMD58 is the OCR register in (desc byte) order
                        * Therefore byte 0 represents bits 23-31. The 0x40 mask will give us the 7th bit, or 
                        * OCR bit 30, which is the Card Capacity Status (CCS) bit. 
                        * 5.1 - High Capacity SD cards set CCS to 1, Standard Capacity SD cards set CCS to 0.
                        */
                        sd_card_type = (ocr[0] & 0x40) ? TYPE_SDC : TYPE_SD2;
                    }
                }

                /**
                 * Section 4.7.4 - CMD16 sets the block length (in bytes) to be used for read/write/lock commands.
                 * 512 is the default block length, and that is our default, but we'll still set it.
                 */
                if (count >= 1000 || (send_cmd(CMD16_SET_BLOCKLEN, SD_BLOCK_SIZE) != 0))
                {
                    sd_card_type = TYPE_NONE;
                }
            }
        }
        else
        {
            /**
             * Figure 4-2 - No/invalid response to CMD8 indicates either
             * - V2.00 with voltage mismatch
             * - v1.X 
             * - Not valid SD Card
             */
            uint8_t cmd;
            if (send_cmd(ACMD41_SEND_OP_COND_SDC, 0) <= 1)
            {	
                /* SDv1 or MMC? */
                sd_card_type = TYPE_SD1;
                cmd = ACMD41_SEND_OP_COND_SDC;	/* SDv1 (ACMD41(0)) */
            } 
            else
            {
                sd_card_type = TYPE_MMC;
                cmd = CMD1_SEND_OP_COND;	/* MMCv3 (CMD1(0)) */
            }
            
            count = 0;
            while (count++ < 1000)
            {
                if (send_cmd(cmd, 0) == 0)
                {
                    break;
                }
            }

            /**
             * Section 4.7.4 - CMD16 sets the block length (in bytes) to be used for read/write/lock commands.
             * 512 is the default block length, and that is our default, but we'll still set it.
             */
            if ((count >= 1000) || (send_cmd(CMD16_SET_BLOCKLEN, SD_BLOCK_SIZE) != 0)) 
            {
                sd_card_type = TYPE_NONE;
            }
        }
    }
    
    if (sd_card_type == TYPE_NONE)
    {
        status |= STA_NODISK;
    }
    else
    {
        status = STA_OK;
    }

    spi_deselect();

    return status;
}

DSTATUS disk_status (BYTE pdrv)
{
    DSTATUS status = STA_OK;
    if (sd_card_type == TYPE_NONE)
    {
        status = STA_NODISK + STA_NOINIT;
    }
    return status;
}

/**
 * Sections 4.3.3 and 7.2.3 detail the SD Data Read process
 */
DRESULT disk_read (BYTE pdrv, BYTE * buff, DWORD sector, UINT count)
{
    uint32_t block_addr = sector;
    DRESULT result = RES_ERROR;

    if (!(sd_card_type & TYPE_SDC))
    {
        block_addr = sector * SD_BLOCK_SIZE;
    }

    if (count == 1)
    {
        /**
         * Section 7.2.3 - CMD17 Initiates reading a single block
         */
        if (send_cmd(CMD17_READ_SINGLE_BLOCK, block_addr) == 0)
        {
            if (receive_datablock(buff, SD_BLOCK_SIZE))
            {
                result = RES_OK;
            }
        }
    }
    else
    {
        /**
         * Section 7.2.3 - CMD18 Initiates reading multiple blocks
         */
        if (send_cmd(CMD18_READ_MULTIPLE_BLOCK, block_addr) == 0)
        {
            do
            {
                if (!receive_datablock(buff, SD_BLOCK_SIZE))
                {
                    break;
                }
                buff += SD_BLOCK_SIZE;
            } while (--count);
            /**
             * Section 4.3.3 - Data will be continuously transferred until
             * a CMD12 is sent.
             */
            if ((count == 0) && (send_cmd(CMD12_STOP_TRANSMISSION, 0)))
            {
                result = RES_OK;
            }
        }
    }

    spi_deselect();
    return result;
}

/**
 * Sections 4.3.4 and 7.2.4 detail the SD data writing process
 */
DRESULT disk_write (BYTE pdrv, const BYTE* buff, DWORD sector, UINT count)
{
    uint32_t block_addr = sector;
    DRESULT result = RES_OK;
    
    if (!(sd_card_type & TYPE_SDC))
    {
        block_addr = sector * SD_BLOCK_SIZE;
    }
    
    if (count == 1)
    {
        /**
         * Section 7.2.4 - CMD24 Initiates writing a single block 
         */
        if (send_cmd(CMD24_WRITE_BLOCK, block_addr) == 0)
        {
            /**
             * Section 7.3.3.2 - The block write start token is defined as:
             *     7 6 5 4 3 2 1 0
             *   +-----------------+
             *   | 1|1|1|1|1|1|1|0 |
             *   +-----------------+
             * Which translates to 0xFE
             */
            if (!transmit_datablock(buff, 0xFE))
            {
                result = RES_ERROR;
            }
        }
    }
    else
    {
        /**
         * Section 4.3.4 - Setting a number of write blocks to be pre-erased (CMD23)
         * will make the multi block write operation faster
         *
         * Section 7.2.4 - CMD25 Initiates writing multiple blocks
         */
        if ((send_cmd(ACMD23_SET_WR_BLK_ERASE_COUNT, count) == 0) && 
            (send_cmd(CMD25_WRITE_MULTIPLE_BLOCK, block_addr) == 0))
        {
            do
            {
                /**
                 * Section 7.3.3.2 - The multi block write start token is defined as:
                 *     7 6 5 4 3 2 1 0
                 *   +-----------------+
                 *   | 1|1|1|1|1|1|0|0 |
                 *   +-----------------+
                 * Which translates to 0xFC
                 */
                if (!transmit_datablock(buff, 0xFC))
                {
                    result = RES_ERROR;
                    break;
                }
                buff += SD_BLOCK_SIZE;

            } while (--count);
            /**
             * Section 7.2.4 - In a multiple block transmission, the stop transmission
             * will be done by sending the 'Stop Tran' token.
             *
             * Section 7.3.3.2 - The multi block write stop tran token is defined as:
             *     7 6 5 4 3 2 1 0
             *   +-----------------+
             *   | 1|1|1|1|1|1|0|1 |
             *   +-----------------+
             * Which translates to 0xFD
             */
            if (!transmit_datablock(0, 0xFD))
            {
                result = RES_ERROR;
            }
        }
    }

    spi_deselect();
    return result;
}

/**
 * http://elm-chan.org/fsw/ff/en/dioctl.html
 * The disk_ioctl function controls device specific features and miscellaneous 
 * functions other than generic read/write.
 */
DRESULT disk_ioctl (BYTE pdrv, BYTE cmd, void* buff)
{
    DRESULT result = RES_ERROR;

    if (sd_card_type == TYPE_NONE)
    {
        return RES_NOTRDY;
    }

    switch (cmd)
    {
        case CTRL_SYNC:
            /**
             * http://elm-chan.org/fsw/ff/en/dioctl.html
             * Make sure that the drive has finished pending writes.
             * The spi_select function will return 1 if the SD is able 
             * to echo back 0xFF, which indicates it is done with work.
             */
            if (spi_select())
            {
                spi_deselect();
                result = RES_OK;
            }
            break;
        
        default:
            result = RES_PARERR;
            break;
    }
    spi_deselect();

    return result;
}

#endif
