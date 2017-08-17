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
 * Hamilton Master UART Communication Demo Code
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

    /* Line processing flags */
    tty.c_lflag = ICANON;            /* Canonical input */

    /* Character processing flags */
    tty.c_cflag |= (CLOCAL | CREAD); /* Ignore modem controls */
    tty.c_cflag &= ~CSIZE;           /* Clear current char size mask */
    tty.c_cflag |= CS8;              /* 8-bit characters */
    tty.c_cflag &= ~PARENB;          /* No parity bit */
    tty.c_cflag &= ~CSTOPB;          /* Only need 1 stop bit */
    tty.c_cflag &= ~CRTSCTS;         /* No hardware flowcontrol */

    /* Input flags */
    tty.c_iflag
        &= ~(IGNBRK | BRKINT | ICRNL | INLCR | PARMRK | INPCK | ISTRIP | IXON);

    /* Output flags */
    tty.c_oflag = 0;                /* Raw output */

    tty.c_cc[VMIN]  = 1;            /* One input byte is enough to return from read () */
    tty.c_cc[VTIME] = 0;            /* No inter-character timer */

    tcflush(fd, TCIFLUSH);          /* Clear anything that's currently in the UART buffers */

    /* Update the terminal settings */
    if (tcsetattr(fd, TCSANOW, &tty) != 0)
    {
        printf("** Error from tcsetattr: %s\n", strerror(errno));
        return -1;
    }

    printf("Set attributes\n");

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
    char * portname = "/dev/ttyS3";
    int    wlen;

    fd = open(portname, O_RDWR | O_NOCTTY | O_NDELAY);
    if (fd < 0)
    {
        printf("** Error opening %s: %s\n", portname, strerror(errno));
        return -1;
    }

    /* Set file characteristics for interrupt routine */
    fcntl(fd, F_SETFL, FNDELAY);
    fcntl(fd, F_SETOWN, getpid());
    //fcntl(fd, F_SETFL,  O_ASYNC );

    /* Baudrate 230400, 8 bits, no parity, 1 stop bit */
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

    rdlen = read(fd, uart_buf, sizeof(uart_buf) - 1);

    if (rdlen < 0)
    {
        printf("** Error from read: %d: %s\n", rdlen, strerror(errno));

        return -1;
    }

    printf("Received: %s\n", uart_buf);

    return 0;
}


/*void sigio_handler(int sig)
{
    printf("Received data from UART\n");
    uart_read();
}*/

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
    char counter = 0;

    running = 1;

    /*
     * Application options:
     * -------------------
     * 1. Transfer file
     * 2. Run endpoint command
     * 3. Request telemetry
     * 4. ???
     */
    while ((opt = getopt(argc, argv, "f:c:t")) != -1)
    {
        switch (opt)
        {
            case 'c':
                //Run specific endpoint command
                //Get command from next argument
                //Validate?
                //Send it
                break;
            case 'd':
                debug = 1;
                break;
            default:
                printf("Unknown option: %c", opt);
        }
    }

    signal(SIGINT, sigint_handler);

/*
    saio.sa_handler = sigio_handler;
    saio.sa_flags = 0;
    saio.sa_restorer = NULL;

    sigaction(SIGIO, &saio, NULL);*/

    // Open connection to slave
    uart_init();

    while (running)
    {

        char testmsg[] = "Test Message nnn\n";

        printf("Writing message %d\n", counter);

        sprintf(testmsg, "Test Message %03d\n", counter++);

        int ret = write(fd, testmsg, sizeof(testmsg));

        printf("Wrote %d bytes\n", ret);

        //read(fd, response);

        sleep(5);
    }

    uart_close();

    return 0;
}
