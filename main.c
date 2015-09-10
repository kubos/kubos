
#include <stdio.h>

#ifdef MODULE_LOCATION
    #include "location.h"
#endif

#include "kernel.h"
#include "shell.h"
#ifdef MODULE_NEWLIB
#   include "uart_stdio.h"
#else
#   include "posix_io.h"
#   include "board_uart0.h"
#endif


int hello_world(int argc, char **argv) {
    /* Suppress compiler errors */
    (void)argc;
    (void)argv;
    printf("hello world!\n");
    return 0;
}

const shell_command_t shell_commands[] = {
    {"hello", "prints hello world", hello_world},
#ifdef MODULE_LOCATION
    {"gps_get", "Gets GPS Data", get_gps_data},
#endif
    { NULL, NULL, NULL }
};

int main(void)
{
    shell_t shell;

    puts("Welcome to KubOS! Initializing...");

    /* start shell */
    puts("All up, running the shell now");
#ifndef MODULE_NEWLIB
    (void) posix_open(uart0_handler_pid, 0);
    shell_init(&shell, shell_commands, UART0_BUFSIZE, uart0_readc, uart0_putc);
#else
    shell_init(&shell, shell_commands, UART0_BUFSIZE, getchar, putchar);
#endif
    shell_run(&shell);

    /* should be never reached */
    return 0;
}