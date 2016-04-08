/**
 * Build this example on linux with:
 * ./waf configure --enable-examples --enable-if-kiss --with-driver-uart=linux --enable-crc32 clean build
 */

#include <stdio.h>
#include "kubos-core/arch/k_uart.h"
#include "kubos-core/arch/k_timer.h"
#include <time.h>


/* Setup callback from USART RX to KISS RS */
void my_uart_rx(void * arg, uint8_t * buf, int len, void * pxTaskWoken) {
	printf("rx'd %s\n", buf);
}

int main(int argc, char **argv) {
    struct uart_conf conf;

    conf.device = argc != 2 ? "/dev/pts/5" : argv[1];
    conf.baudrate = 500000;


	/* Run UART init */
	uart_init(&conf);
	uart_set_callback(NULL, my_uart_rx);

    uart_putstr("ryan", 5);

    while(1) {
        sleep(1);
        uart_putstr("ryan", 5);
    }

    return 0;

}
