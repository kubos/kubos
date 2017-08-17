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
 * Hamilton Slave UART Communication Demo Code
 *
 */

#include <ctype.h>
#include <errno.h>
#include <fcntl.h>
#include <math.h>
#include <pthread.h>
#include <signal.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <unistd.h>
#include <string.h>
#include <termios.h>
#include <unistd.h>

struct sigaction saio;

int running;
int debug;
int fd;

int set_interface_attribs(int speed)
{
    struct termios tty;

    if (tcgetattr(fd, &tty) < 0)
    {
        printf("** Error from tcgetattr: %s\n", strerror(errno));
        return -1;
    }

    cfsetospeed(&tty, (speed_t) speed);
    cfsetispeed(&tty, (speed_t) speed);

    tty.c_cflag |= (CLOCAL | CREAD); /* ignore modem controls */
    tty.c_cflag &= ~CSIZE;
    tty.c_cflag |= CS8; /* 8-bit characters */
    tty.c_cflag &= ~PARENB; /* no parity bit */
    tty.c_cflag &= ~CSTOPB; /* only need 1 stop bit */
    tty.c_cflag &= ~CRTSCTS; /* no hardware flowcontrol */

    tty.c_iflag
        &= ~(IGNBRK | BRKINT | ICRNL | INLCR | PARMRK | INPCK | ISTRIP | IXON);
    tty.c_lflag &= ~ICANON|ECHO; /* non-canonical input */
    tty.c_oflag = 0; /* raw output */

    tty.c_cc[VMIN]  = 18; //I know that we're looking for an 18 byte message "Test message nnn"
    tty.c_cc[VTIME] = 0;

    tcflush(fd, TCIFLUSH);
    if (tcsetattr(fd, TCSANOW, &tty) != 0)
    {
        printf("** Error from tcsetattr: %s\n", strerror(errno));
        return -1;
    }
    return 0;
}

void set_mincount(int fd, int mcount)
{
    struct termios tty;

    if (tcgetattr(fd, &tty) < 0)
    {
        printf("** Error tcgetattr: %s\n", strerror(errno));
        return;
    }

    tty.c_cc[VMIN]  = mcount ? 1 : 0;
    tty.c_cc[VTIME] = 5; /* half second timer */

    if (tcsetattr(fd, TCSANOW, &tty) < 0)
        printf("** Error tcsetattr: %s\n", strerror(errno));
}

int uart_init(void)
{
    char * portname = "/dev/ttyS1";
    int    wlen;

    fd = open(portname, O_RDWR | O_NOCTTY | O_SYNC);
    if (fd < 0)
    {
        printf("** Error opening %s: %s\n", portname, strerror(errno));
        return -1;
    }

    /* Set file characteristics for interrupt routine */
    fcntl(fd, F_SETFL, FNDELAY);
    fcntl(fd, F_SETOWN, getpid());
    fcntl(fd, F_SETFL,  O_ASYNC);

    /* Baudrate 230400, 8 bits, no parity, 1 stop bit */
    //set_interface_attribs(B230400);
    set_interface_attribs(B115200);
    // set_mincount(fd, 0);                /* set to pure timed read */

    return 0;
}

int uart_close(void)
{
    close(fd);
}

static char    uart_buf[256];

int uart_read(void)
{

    int rdlen;
    int result = 0;

    memset(uart_buf, 0, sizeof(uart_buf));

    rdlen = read(fd, uart_buf, sizeof(uart_buf) - 1);

    if (rdlen < 0)
    {
        printf("** Error from read: %d: %s\n", rdlen, strerror(errno));

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

    saio.sa_handler = sigio_handler;
    saio.sa_flags = 0;
    saio.sa_restorer = NULL;

    /* The UART bus receiving data will trigger an I/O interrupt */
    sigaction(SIGIO, &saio, NULL);

    // Open connection to slave
    uart_init();

    while (running)
    {

        //Need while loop to keep app running while waiting for messages from master
        sleep(1);
    }

    uart_close();

    return 0;
}
