# UART RX

This is a demo program to test receiving UART data in non-blocking mode as an interrupt. It expects to read the incrementing message "Test message nnn" every 5 seconds from `/dev/ttyS1`.

This program should be paired with the UART TX demo program.

To start this program as a background process, use this command:

    $ uartrx &
    
To stop the program nicely, bring it to the foreground with the `fg` command, then stop it with Ctrl+C.