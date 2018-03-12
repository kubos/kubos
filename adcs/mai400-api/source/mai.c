/*
 * Copyright (C) 2018 Kubos Corporation
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

#include "mai400-api/mai400-api.h"
#include "mai400-api/checksum.h"
#include <errno.h>
#include <fcntl.h>
#include <signal.h>
#include <stdio.h>
#include <string.h>
#include <termios.h>
#include <unistd.h>

struct sigaction saio; //UART receive signal interrupt
int fd_write; //TODO: Move to UART HAL???
int fd_read; //TODO: Move to UART HAL???

//TODO: init config
//Merge init functions?
KADCSStatus k_adcs_init_tx()
{
    //Open UART bus file
    //Set connection settings
    //Be sad that we don't have a UART HAL yet
    char *         device = "/dev/ttyS3";
    struct termios uart;
    speed_t        speed = B115200;

    /*
     * Open UART (terminal)
     * O_WRONLY - Write only (TODO: W+R?)
     * O_NOCTTY - This terminal should not be the controlling terminal of the
     * process
     *            (That should still be the debug UART)
     * O_DSYNC  - Writes should block until all data has been successfully
     *            written out of the buffer to the underlying hardware
     */
    fd_write = open(device, O_WRONLY | O_NOCTTY | O_DSYNC);
    if (fd_write < 0)
    {
        perror("Error opening device");
        return -1;
    }

    /*
     * Get the current terminal settings. There are a bunch, and we only want
     * to update a few, so we'll preserve everything else that's currently set.
     */
    if (tcgetattr(fd_write, &uart) < 0)
    {
        perror("Error from tcgetattr");
        return -1;
    }

    /* Set output baudrate */
    cfsetospeed(&uart, speed);

    /* Character processing flags */
    uart.c_cflag |= (CLOCAL | CREAD);   /* Ignore modem controls */
    uart.c_cflag &= ~CSIZE;             /* Clear current char size mask */
    uart.c_cflag |= CS8;                /* 8-bit characters */
    uart.c_cflag &= ~PARENB;            /* No parity bit */
    uart.c_cflag &= ~CSTOPB;            /* 1 stop bit */
    uart.c_cflag &= ~CRTSCTS;           /* No hardware flowcontrol */

    /* Output flags */
    uart.c_oflag = 0; /* Raw output */

    /* Clear anything that's currently in the UART buffers */
    tcflush(fd_write, TCIOFLUSH);

    /* Update the terminal settings */
    if (tcsetattr(fd_write, TCSANOW, &uart) != 0)
    {
        perror("Error from tcsetattr");
        return -1;
    }

    return ADCS_OK;
}

KADCSStatus k_adcs_init_rx()
{
    //Set up UART receiver
    saio.sa_handler  = sigio_handler;
    saio.sa_flags    = 0;
    saio.sa_restorer = NULL;

    /* The UART bus receiving data will trigger an I/O interrupt */
    sigaction(SIGIO, &saio, NULL);

    char *         device = "/dev/ttyS3";
    speed_t        speed  = B115200;
    struct termios tty;

    /*
     * Open UART (terminal)
     * O_RDONLY - Read only
     * O_NOCTTY - This terminal should not be the controlling terminal of the
     *            process (that should still be the debug UART)
     * O_NDELAY - Open file in non-blocking mode
     * O_ASYNC  - Enable SIGIO generation when data is received
     */
    fd = open(device, O_RDONLY | O_NOCTTY | O_NDELAY | O_ASYNC);
    if (fd < 0)
    {
        perror("** Error opening device");
        return -1;
    }

    /* Set this process as the PID that should receive the SIGIO signals on
     * this file */
    fcntl(fd, F_SETOWN, getpid());

    /*
     * Get the current terminal settings. There are a bunch, and we only want
     * to update a few, so we'll preserve everything else that's currently
     * set.
     */
    if (tcgetattr(fd, &tty) < 0)
    {
        perror("Error from tcgetattr");
        return -1;
    }

    /* Set input baudrate */
    cfsetispeed(&tty, speed);

    /* Character processing flags */
    tty.c_cflag |= (CLOCAL | CREAD);    /* Ignore modem controls */
    tty.c_cflag &= ~CSIZE;              /* Clear current char size mask */
    tty.c_cflag |= CS8;                 /* 8-bit characters */
    tty.c_cflag &= ~PARENB;             /* No parity bit */
    tty.c_cflag &= ~CSTOPB;             /* 1 stop bit */
    tty.c_cflag &= ~CRTSCTS;            /* No hardware flowcontrol */

    /* Input flags */
    tty.c_iflag &= ~(IGNBRK |           /* Don't ignore BREAK conditions */
                     BRKINT |           /* Read BREAKs as null bytes */
                     ICRNL |            /* Do not translate carriage returns */
                     INLCR |            /* Do not translate NL to CR */
                     PARMRK |           /* Do not mark errors */
                     INPCK |            /* Turn off parity checking */
                     ISTRIP |           /* Do not strip off eighth bit */
                     IXON);             /* Turn off flow control */

    tty.c_lflag &= ~ICANON | ECHO;      /* Non-canonical (raw) input */

    tty.c_cc[VMIN] = 18; /* We know that we're looking for an 18 byte message
                            "Test message nnn" */
    tty.c_cc[VTIME] = 2; /* But give a 2/10th second timeout in case something
                            goes wrong mid-read */

    /* Clear anything that's currently in the UART buffers */
    tcflush(fd, TCIOFLUSH);

    /* Update the terminal settings */
    if (tcsetattr(fd, TCSANOW, &tty) != 0)
    {
        perror("Error from tcsetattr");
        return -1;
    }

    return ADCS_OK;
}

void k_adcs_terminate()
{
    close(fd_write);
    close(fd_read);
}

KADCSStatus k_adcs_reset()
{
    KADCSStatus status;

    adcs_set_mode_msg reset_msg = { 0 };

    reset_msg.hdr.sync = SYNC;
    reset_msg.hdr.data_len = 0;
    reset_msg.hdr.id = REQUEST_RESET;
    reset_msg.hdr.addr = 0;

    status = kprv_adcs_send_message(&reset_msg);
    if (status != ADCS_OK)
    {
        fprintf(stderr, "Failed to request reset of ADCS: %d\n", status);
        return ADCS_ERROR;
    }

    adcs_set_mode_msg confirm_msg = { 0 };

    confirm_msg.hdr.sync = SYNC;
    confirm_msg.hdr.data_len = 0;
    confirm_msg.hdr.id = RESET_CONFIRM;
    confirm_msg.hdr.addr = 0;

    status = kprv_adcs_send_message(&confirm_msg);
    if (status != ADCS_OK)
    {
        fprintf(stderr, "Failed to confirm reset of ADCS: %d\n", status);
        return ADCS_ERROR;
    }

    return ADCS_OK;
}

KADCSStatus k_adcs_set_mode(ACSMode mode, int32_t sec_vec, int32_t pri_axis, int32_t sec_axis, int32_t qbi_cmd4)
{
    adcs_set_mode_msg msg = { 0 };

    msg.hdr.sync = SYNC;
    msg.hdr.data_len = sizeof(adcs_set_mode_msg - FRAME_SZ);
    msg.hdr.id = SET_ACS_MODE;
    msg.hdr.addr = 0;

    msg.sec_vec = sec_vec;
    msg.pri_axis = pri_axis;
    msg.sec_axis = sec_axis;
    msg.qbi_cmd4 = qbi_cmd4;

    return kprv_adcs_send_message(&msg);
}

KADCSStatus k_adcs_get_info()
{
    adcs_get_info_msg msg = { 0 };

    msg.hdr.sync = SYNC;
    msg.hdr.data_len = 0;
    msg.hdr.id = GET_INFO;
    msg.hdr.addr = 0;

    return kprv_adcs_send_message(&msg);
}

KADCSStatus kprv_adcs_send_message(const uint8_t * msg)
{
    adcs_msg_header * hdr = (adcs_msg_header *) msg;
    uint16_t data_len = hdr.data_len;
    uint8_t * data = msg + HDR_SZ;

    uint16_t crc = crc_xmodem(data, data_len);

    uint16_t * data_crc = data + data_len;

    *data_crc = crc;

    if(write(fd_write, msg, data_len + HDR_SZ + 2) < 0)
    {
        perror("Error writing to MAI-400");
        return ADCS_ERROR;
    }

    return ADCS_OK;
}

static char uart_buf[256];
void kprv_adcs_receive_message(const uint8_t * msg)
{
    int rdlen;

    /* Make sure the buffer is cleared from the last time we read */
    memset(uart_buf, 0, sizeof(uart_buf));

    /* Read as much data as is available */
    rdlen = read(fd_read, uart_buf, sizeof(uart_buf) - 1);

    if (rdlen < 0)
    {
        perror("Error from read");

        return -1;
    }

    printf("Received(%d): %s\n", rdlen, uart_buf);

    adcs_msg_header * hdr = (adcs_msg_header *) uart_buf;
    uint16_t data_len = hdr.data_len;
    uint8_t * data = uart_buf + HDR_SZ;

    //Calc CRC
    uint16_t crc = crc_xmodem(data, data_len);

    uint16_t * data_crc = data + data_len;

    if (*data_crc != crc)
    {
        //TODO: Or other error handling stuff...
    }

    //TODO: Good message receive logic. Where does this get pushed to?
    //Maybe check message ID?
}

void sigio_handler(int sig)
{
    printf("Received data from UART\n");
    kprv_adcs_receive_message();
}
