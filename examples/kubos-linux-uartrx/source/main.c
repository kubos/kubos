/*
 * KubOS Linux
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
 *
 * UART Communication Demo Code - Receiver
 *
 */

#include <errno.h>
#include <fcntl.h>
#include <signal.h>
#include <stdio.h>
#include <string.h>
#include <termios.h>
#include <unistd.h>

struct sigaction saio;

int running;
int fd;

int uart_init(void)
{
    char *         device = "/dev/ttyS1";
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

    return 0;
}

int uart_close(void)
{
    close(fd);
}

static char uart_buf[256];

int uart_read(void)
{

    int rdlen;

    /* Make sure the buffer is cleared from the last time we read */
    memset(uart_buf, 0, sizeof(uart_buf));

    /* Read as much data as is available */
    rdlen = read(fd, uart_buf, sizeof(uart_buf) - 1);

    if (rdlen < 0)
    {
        perror("Error from read");

        return -1;
    }

    printf("Received(%d): %s\n", rdlen, uart_buf);

    return 0;
}

void sigio_handler(int sig)
{
    printf("Received data from UART\n");
    uart_read();
}

void sigint_handler(int sig)
{
    running = 0;
    sleep(2);
    signal(SIGINT, SIG_DFL);
    kill(getpid(), SIGINT);
}

int main(int argc, char * argv[])
{

    int opt;

    running = 1;

    /* Ctrl+C will trigger a signal to end the program */
    signal(SIGINT, sigint_handler);

    saio.sa_handler  = sigio_handler;
    saio.sa_flags    = 0;
    saio.sa_restorer = NULL;

    /* The UART bus receiving data will trigger an I/O interrupt */
    sigaction(SIGIO, &saio, NULL);

    /* Open connection to transmitter */
    uart_init();

    while (running)
    {

        /* Need while loop to keep app running while waiting for messages from
         * master */
        sleep(1);
    }

    /* Cleanup */
    uart_close();

    return 0;
}
