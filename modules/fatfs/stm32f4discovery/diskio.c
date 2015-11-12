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
 *
 * FatFs driver for STM32F4
 */
#include <stdio.h>
#include <stdlib.h>

#include "diskio.h"
#include "hwtimer.h"
#include "periph_conf.h"
#include "stm32f4_util.h"

#ifndef FATFS_SPI
#define FATFS_SPI SPI_0
#endif

#ifndef FATFS_CS_PORT
#define FATFS_CS_PORT PORT_B
#endif

#ifndef FATFS_CS_PIN
#define FATFS_CS_PIN  6
#endif

#define FATFS_CS_GPIO GPIO(FATFS_CS_PORT, FATFS_CS_PIN)
#define CS_LOW  gpio_clear(FATFS_CS_GPIO)
#define CS_HIGH gpio_set(FATFS_CS_GPIO)

#define DELAY_MS(ms) hwtimer_spin(HWTIMER_TICKS((ms) * 1000L))
#define ELAPSED_MS(start) (HWTIMER_TICKS_TO_US(hwtimer_now() - start) / 1000L)

#define ENABLE_DEBUG 0
#define ENABLE_TRACE 0
#include "debug.h"

#if ENABLE_TRACE
#define TRACE(...) DEBUG(__VA_ARGS__)
#else
#define TRACE(...)
#endif

//static SPI_TypeDef *sd_spi;
static char sd_command[6];
static BYTE sd_card_type;

static int wait_ready(UINT wt) {
    BYTE d;
    long start = hwtimer_now();

    do {
        spi_transfer_byte(FATFS_SPI, 0xFF, (char *) &d);
        /* Wait for card goes ready or timeout */
    } while (d != 0xFF && ELAPSED_MS(start) < wt);

    if (d == 0xFF) {
        DEBUG("wait_ready: OK\n");
        return 1;
    } else {
        DEBUG("wait_ready: timeout\n");
        return 0;
    }
}

static void deselect(void) {
    CS_HIGH;

    /* Dummy clock (force DO hi-z for multiple slave SPI) */
    spi_transfer_byte(FATFS_SPI, 0xFF, NULL);
}

static int select(void) {
    CS_LOW;
    /* Dummy clock (force DO enabled) */
    spi_transfer_byte(FATFS_SPI, 0xFF, NULL);

    if (wait_ready(500)) {
        DEBUG("select: OK\n");
        return 1;
    }

    DEBUG("select: no\n");
    deselect();
    return 0;
}

#if ENABLE_TRACE
static void TRACE_SECTOR(BYTE *sector) {
  int i, j;
  printf("      ");
  for (i = 0; i < 16; i++) {
      printf("%02X ", i);
  }
  printf("\n      ------------------------------------------------\n");

  for (i = 0; i < 32; i++) {
      printf("[%03X] ", i * 16);
      for (j = 0; j < 16; j++) {
          int b = j + (i * 16);
          printf("%02X ", sector[b]);
      }
      printf("\n");
  }
}
#else
#define TRACE_SECTOR(s)
#endif

#define RCV_MAX_MS 500

static int rcvr_datablock(BYTE *buff, UINT btr) {
    BYTE token;
    long start = hwtimer_now();

    DEBUG("rcvr_datablock %u\n", btr);

    // Wait for DataStart token in timeout of 200ms 
    do {
        spi_transfer_byte(FATFS_SPI, 0xFF, (char *) &token);
        DEBUG("rcv token:%02X\n", token);
    } while ((token == 0xFF) && ELAPSED_MS(start) < RCV_MAX_MS);

    if (token != 0xFE) {
        DEBUG("bad token: 0x%02X != 0xFE\n", token);
        // Function fails if invalid DataStart token or timeout 
        return 0;
    }

    // Store trailing data to the buffer
    spi_transfer_bytes(FATFS_SPI, NULL, (char *) buff, btr);
    TRACE_SECTOR(buff);

    spi_transfer_byte(FATFS_SPI, 0xFF, (char *) &token);
    DEBUG("crc1 token=0x%02X\n", token);

    spi_transfer_byte(FATFS_SPI, 0xFF, (char *) &token); // Discard CRC
    DEBUG("crc2 token=0x%02X\n", token);

    return 1;
}

static BYTE send_cmd(BYTE cmd, DWORD arg) {
    BYTE n, res;

    if (cmd & 0x80) { /* Send a CMD55 prior to ACMD<n> */
        cmd &= 0x7F;
        res = send_cmd(55, 0);
        if (res > 1) return res;
    }

    /* Select the card and wait for ready except to stop multiple block read */
    if (cmd != SDC_STOP_TRANSMISSION) {
        deselect();
        if (!select()) {
            DEBUG("Failed to stop transmission\n");
            return 0xFF;
        }
    }

    /* Send command packet */
    sd_command[0] = 0x40 | cmd;
    sd_command[1] = (char)(arg >> 24);
    sd_command[2] = (char)(arg >> 16);
    sd_command[3] = (char)(arg >> 8);
    sd_command[4] = (char)arg;
    sd_command[5] = 0x01;
    if (cmd == 0) sd_command[5] = 0x95;
    if (cmd == 8) sd_command[5] = 0x87;

    spi_transfer_bytes(FATFS_SPI, sd_command, NULL, 6);

    /* Receive command resp */
    if (cmd == SDC_STOP_TRANSMISSION) {
        /* Discard following one byte when CMD12 */
        spi_transfer_byte(FATFS_SPI, 0xFF, NULL);
    }

    n = 10; /* Wait for response (10 bytes max) */
    do {
        spi_transfer_byte(FATFS_SPI, 0xFF, (char *) &res);
    } while ((res & 0x80) && --n);

    DEBUG("cmd %d arg %lu, res: %d\n", cmd, arg, res);
    return res; /* Return received response */
}

static void init_spi(void) {
    gpio_init(GPIO(PORT_B, 6), GPIO_DIR_OUT, GPIO_PULLUP);

    spi_init_master(FATFS_SPI, SPI_CONF_SECOND_FALLING, SPI_SPEED_400KHZ);
    gpio_set(GPIO(PORT_B, 6));
}

/*-----------------------------------------------------------------------*/
/* Inidialize a Drive                                                    */
/*-----------------------------------------------------------------------*/
DSTATUS disk_initialize (BYTE pdrv /* Physical drive nmuber (0..) */)
{
/*    switch (FATFS_SPI) {
#if SPI_0_EN
        case SPI_0:
            sd_spi = SPI0_DEV;
            break;
#endif
#if SPI_1_EN
#endif
#if SPI_2_EN
#endif
    }
*/

    hwtimer_init();
    //spi_init_master(FATFS_SPI, conf, SPI_SPEED_400KHZ);
    init_spi();

    CS_LOW;

    // send 10 dummy bytes to wake up SDC
    for (uint8_t i = 0; i < 10; i++) {
        spi_transfer_byte(FATFS_SPI, 0xFF, NULL);
    }

    long start = hwtimer_now(); // timeout = 1sec
    BYTE n, cmd, ty, ocr[4];
    ty = 0;
    if (send_cmd(SDC_GO_IDLE_STATE, 0) == 1) {				/* Put the card SPI/Idle state */
        if (send_cmd(8, 0x1AA) == 1) {	/* SDv2? */
            for (n = 0; n < 4; n++) {
                spi_transfer_byte(FATFS_SPI, 0xFF, (char *)&ocr[n]);	/* Get 32 bit return value of R7 resp */
            }
            if (ocr[2] == 0x01 && ocr[3] == 0xAA) {				/* Is the card supports vcc of 2.7-3.6V? */
                while (ELAPSED_MS(start) < 1000 && send_cmd(0x80 + 41, 1UL << 30)) ;	/* Wait for end of initialization with ACMD41(HCS) */
                if (ELAPSED_MS(start) < 1000 && send_cmd(58, 0) == 0) {		/* Check CCS bit in the OCR */
                    for (n = 0; n < 4; n++) {
                        spi_transfer_byte(FATFS_SPI, 0xFF, (char *) &ocr[n]);
                    }
                    ty = (ocr[0] & 0x40) ? CT_SD2 | CT_BLOCK : CT_SD2;	/* Card id SDv2 */
                }
            }
        } else {	/* Not SDv2 card */
            if (send_cmd(0x80 + 41, 0) <= 1) 	{	/* SDv1 or MMC? */
                ty = CT_SD1; cmd = 0x80 + 41;	/* SDv1 (ACMD41(0)) */
            } else {
                ty = CT_MMC; cmd = 1;	/* MMCv3 (CMD1(0)) */
            }
            while (ELAPSED_MS(start) < 1000 && send_cmd(cmd, 0));			/* Wait for end of initialization */
            if (ELAPSED_MS(start) < 1000 || send_cmd(16, 512) != 0) {	/* Set block length: 512 */
                ty = 0;
            }
        }
    }
    sd_card_type = ty; /* Card type */

    deselect();
    return 0;
}



/*-----------------------------------------------------------------------*/
/* Get Disk Status                                                       */
/*-----------------------------------------------------------------------*/

DSTATUS disk_status (
        BYTE pdrv		/* Physical drive nmuber (0..) */
        )
{
    // TODO add support for CD pin
    return 0;
}



/*-----------------------------------------------------------------------*/
/* Read Sector(s)                                                        */
/*-----------------------------------------------------------------------*/


DRESULT disk_read (
        BYTE pdrv,		/* Physical drive nmuber (0..) */
        BYTE *buff,		/* Data buffer to store read data */
        DWORD sector,	/* Sector address (LBA) */
        UINT count		/* Number of sectors to read (1..128) */
        )
{
    printf("disk_read sector=%lu, count=%u\n", sector, count);

    if (!(sd_card_type & CT_BLOCK)) {
        sector *= 512;	/* LBA ot BA conversion (byte addressing cards) */
    }

    if (count == 1) {	/* Single sector read */
        if ((send_cmd(SDC_READ_SINGLE_BLOCK, sector) == 0)	/* READ_SINGLE_BLOCK */
                && rcvr_datablock(buff, 512))
            count = 0;
    } else {				/* Multiple sector read */
        if (send_cmd(SDC_READ_MULTIPLE_BLOCK, sector) == 0) {	/* READ_MULTIPLE_BLOCK */
            do {
                if (!rcvr_datablock(buff, 512)) {
                    DEBUG("failed to read sector %d\n", count);
                    break;
                }
                buff += 512;
            } while (--count);
            DEBUG("stop transmission\n");
            send_cmd(SDC_STOP_TRANSMISSION, 0);				/* STOP_TRANSMISSION */
        }
    }
    deselect();

    return count ? RES_ERROR : RES_OK;	/* Return result */
}



/*-----------------------------------------------------------------------*/
/* Write Sector(s)                                                       */
/*-----------------------------------------------------------------------*/

#if _USE_WRITE
/* Send multiple byte */
static void xmit_spi_multi (
        const BYTE *buff,	/* Pointer to the data */
        UINT btx			/* Number of bytes to send (even number) */
        )
{
    /* Write multiple bytes */
    spi_transfer_bytes(FATFS_SPI, (char *)buff, NULL, btx);
}

static int xmit_datablock (	/* 1:OK, 0:Failed */
        const BYTE *buff,	/* Ponter to 512 byte data to be sent */
        BYTE token			/* Token */
        )
{
    BYTE resp;

    if (!wait_ready(500)) {
        return 0;		/* Wait for card ready */
    }

    spi_transfer_byte(FATFS_SPI, token, NULL);					/* Send token */
    if (token != 0xFD) {				/* Send data if token is other than StopTran */
        xmit_spi_multi(buff, 512);		/* Data */
        spi_transfer_byte(FATFS_SPI, 0xFF, NULL);
        spi_transfer_byte(FATFS_SPI, 0xFF, NULL);	/* Dummy CRC */

        spi_transfer_byte(FATFS_SPI, 0xFF, (char *)&resp);				/* Receive data resp */
        if ((resp & 0x1F) != 0x05)		/* Function fails if the data packet was not accepted */
            return 0;
    }
    return 1;
}

DRESULT disk_write (
        BYTE pdrv,			/* Physical drive nmuber (0..) */
        const BYTE* buff,	/* Data to be written */
        DWORD sector,		/* Sector address (LBA) */
        UINT count			/* Number of sectors to write (1..128) */
        )
{
    if (!(sd_card_type & CT_BLOCK)) {
        sector *= 512; // convert LBA to physical address
    }

    if (count == 1) {	/* Single sector write */
        if ((send_cmd(SDC_WRITE_BLOCK, sector) == 0)	/* WRITE_BLOCK */
                && xmit_datablock(buff, 0xFE))
            count = 0;
    } else {				/* Multiple sector write */
        if (sd_card_type & CT_SDC) send_cmd(0x80+23, count);	/* Predefine number of sectors */
        if (send_cmd(SDC_WRITE_MULTIPLE_BLOCK, sector) == 0) {	/* WRITE_MULTIPLE_BLOCK */
            do {
                if (!xmit_datablock(buff, 0xFC)) {
                    break;
                }
                buff += 512;
            } while (--count);
            if (!xmit_datablock(0, 0xFD)) {	/* STOP_TRAN token */
                count = 1;
            }
        }
    }
    deselect();

    return count ? RES_ERROR : RES_OK;	/* Return result */
}
#endif


/*-----------------------------------------------------------------------*/
/* Miscellaneous Functions                                               */
/*-----------------------------------------------------------------------*/

#if _USE_IOCTL
DRESULT disk_ioctl (
        BYTE pdrv,
        BYTE cmd,		/* Control code */
        void *buff		/* Buffer to send/receive control data */
        )
{
    DRESULT res = RES_ERROR;
    BYTE n, csd[16];
    DWORD *dp, st, ed, csize;

    switch (cmd) {
        case CTRL_SYNC :		/* Wait for end of internal write process of the drive */
            if (select()) {
                res = RES_OK;
                deselect();
            }
            break;

        case GET_SECTOR_COUNT :	/* Get drive capacity in unit of sector (DWORD) */
            if ((send_cmd(SDC_SEND_CSD, 0) == 0) && rcvr_datablock(csd, 16)) {
                if ((csd[0] >> 6) == 1) {	/* SDC ver 2.00 */
                    csize = csd[9] + ((WORD)csd[8] << 8) + ((DWORD)(csd[7] & 63) << 16) + 1;
                    *(DWORD*)buff = csize << 10;
                } else {					/* SDC ver 1.XX or MMC ver 3 */
                    n = (csd[5] & 15) + ((csd[10] & 128) >> 7) + ((csd[9] & 3) << 1) + 2;
                    csize = (csd[8] >> 6) + ((WORD)csd[7] << 2) + ((WORD)(csd[6] & 3) << 10) + 1;
                    *(DWORD*)buff = csize << (n - 9);
                }
                res = RES_OK;
            }
            break;

        case GET_BLOCK_SIZE :	/* Get erase block size in unit of sector (DWORD) */
            if (sd_card_type & CT_SD2) {	/* SDC ver 2.00 */
                if (send_cmd(0x80+13, 0) == 0) {	/* Read SD status */
                    spi_transfer_byte(FATFS_SPI, 0xFF, NULL);
                    if (rcvr_datablock(csd, 16)) {				/* Read partial block */
                        for (n = 64 - 16; n; n--) {
                            spi_transfer_byte(FATFS_SPI, 0xFF, NULL);	/* Purge trailing data */
                        }
                        *(DWORD*)buff = 16UL << (csd[10] >> 4);
                        res = RES_OK;
                    }
                }
            } else {					/* SDC ver 1.XX or MMC */
                if ((send_cmd(SDC_SEND_CSD, 0) == 0) && rcvr_datablock(csd, 16)) {	/* Read CSD */
                    if (sd_card_type & CT_SD1) {	/* SDC ver 1.XX */
                        *(DWORD*)buff = (((csd[10] & 63) << 1) + ((WORD)(csd[11] & 128) >> 7) + 1) << ((csd[13] >> 6) - 1);
                    } else {					/* MMC */
                        *(DWORD*)buff = ((WORD)((csd[10] & 124) >> 2) + 1) * (((csd[11] & 3) << 3) + ((csd[11] & 224) >> 5) + 1);
                    }
                    res = RES_OK;
                }
            }
            break;

        case CTRL_ERASE_SECTOR :	/* Erase a block of sectors (used when _USE_ERASE == 1) */
            if (!(sd_card_type & CT_SDC)) break;				/* Check if the card is SDC */
            if (disk_ioctl(pdrv, MMC_GET_CSD, csd)) break;	/* Get CSD */
            if (!(csd[0] >> 6) && !(csd[10] & 0x40)) break;	/* Check if sector erase can be applied to the card */
            dp = buff; st = dp[0]; ed = dp[1];				/* Load sector block */
            if (!(sd_card_type & CT_BLOCK)) {
                st *= 512; ed *= 512;
            }
            if (send_cmd(32, st) == 0 && send_cmd(33, ed) == 0 && send_cmd(38, 0) == 0 && wait_ready(30000))	/* Erase sector block */
                res = RES_OK;	/* FatFs does not check result of this command */
            break;

        case MMC_GET_CSD :    // Receive CSD as a data block (16 bytes)
            if (send_cmd(SDC_SEND_CSD, 0) == 0    /// READ_CSD
                    && rcvr_datablock(csd, 16))
                res = RES_OK;
            break;

        default:
            res = RES_PARERR;
    }

    deselect();

    return res;
}
#endif

DWORD get_fattime (void)
{
    /* Returns current time packed into a DWORD variable */
    return	  ((DWORD)(2013 - 1980) << 25)	/* Year 2013 */
        | ((DWORD)7 << 21)				/* Month 7 */
        | ((DWORD)28 << 16)				/* Mday 28 */
        | ((DWORD)0 << 11)				/* Hour 0 */
        | ((DWORD)0 << 5)				/* Min 0 */
        | ((DWORD)0 >> 1);				/* Sec 0 */
}
