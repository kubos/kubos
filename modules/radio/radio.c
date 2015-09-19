#include <errno.h>
#include <termios.h>
#include <unistd.h>
#include <string.h>
#include "kernel.h"

#include <sys/types.h>
#include <sys/stat.h>
#include <fcntl.h>

#include "libkiss.h"


int set_interface_attribs (int fd, int speed, int parity)
{
        struct termios tty;
        memset (&tty, 0, sizeof tty);
        if (tcgetattr (fd, &tty) != 0)
        {
                printf ("error %d from tcgetattr", errno);
                return -1;
        }

        cfsetospeed (&tty, speed);
        cfsetispeed (&tty, speed);

        tty.c_cflag = (tty.c_cflag & ~CSIZE) | CS8;     // 8-bit chars
        // disable IGNBRK for mismatched speed tests; otherwise receive break
        // as \000 chars
        tty.c_iflag &= ~IGNBRK;         // disable break processing
        tty.c_lflag = 0;                // no signaling chars, no echo,
                                        // no canonical processing
        tty.c_oflag = 0;                // no remapping, no delays
        tty.c_cc[VMIN]  = 0;            // read doesn't block
        tty.c_cc[VTIME] = 5;            // 0.5 seconds read timeout

        tty.c_iflag &= ~(IXON | IXOFF | IXANY); // shut off xon/xoff ctrl

        tty.c_cflag |= (CLOCAL | CREAD);// ignore modem controls,
                                        // enable reading
        tty.c_cflag &= ~(PARENB | PARODD);      // shut off parity
        tty.c_cflag |= parity;
        tty.c_cflag &= ~CSTOPB;
        tty.c_cflag &= ~CRTSCTS;

        if (tcsetattr (fd, TCSANOW, &tty) != 0)
        {
                printf ("error %d from tcsetattr", errno);
                return -1;
        }
        return 0;
}

void set_blocking (int fd, int should_block)
{
        struct termios tty;
        memset (&tty, 0, sizeof tty);
        if (tcgetattr (fd, &tty) != 0)
        {
                printf ("error %d from tggetattr", errno);
                return;
        }

        tty.c_cc[VMIN]  = should_block ? 1 : 0;
        tty.c_cc[VTIME] = 5;            // 0.5 seconds read timeout

        if (tcsetattr (fd, TCSANOW, &tty) != 0)
                printf ("error %d setting term attributes", errno);
}



int testRadio(void)
{
	int result;
	unsigned char buf[255];
	unsigned int packetlen;

	char *portname = "/tmp/kisstnc";

	int fd = open (portname, O_RDWR | O_NOCTTY | O_SYNC);

	if (fd < 0)
	{
	        printf ("error %d opening %s: %s", errno, portname, strerror (errno));
	        return -1;
	}

    set_interface_attribs (fd, B1200, 0);  // set speed to 115,200 bps, 8n1 (no parity) **1200**
	set_blocking (fd, 0);                // set no blocking

	result=libkiss_buildpacket(LIBKISS_FUNC_DATA, 0, "The quick brown fox jumped over the lazy dogs.", 46, buf, sizeof(buf), &packetlen);

	printf("Build Packet success? %s\n", result == 0 ? "Yes" : "No");

	write (fd, buf, packetlen);           // send 7 character greeting

	usleep ((sizeof(buf) + 25) * 100);             // sleep enough to transmit the 7 plus
	                                     // receive 25:  approx 100 uS per char transmit
	char buf2 [100];
	int n = read (fd, buf2, sizeof buf2);  // read up to 100 characters if ready to read

	printf("Read %d characters\n", n);

    return n;
}

int test_radio(int argc, char **argv) {
    /* Suppress compiler errors */
    (void)argc;
    (void)argv;

    testRadio();
    
    return 0;
}

