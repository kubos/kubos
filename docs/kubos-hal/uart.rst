Using UART
--------------

Configuration
^^^^^^^^^^^^^

The first step in using UART is to configure and bring up the interface.
The UART HAL provides a configuration structure with the standard UART
options. This structure should be filled out according to the project's
UART configuration and then it may be used to initialize the interface
in use.

.. code-block:: c

    KUARTConf conf = {
        .baud_rate = 115200,
        .word_len = K_WORD_LEN_8BIT,
        .stop_bits = K_STOP_BITS_1,
        .parity = K_PARITY_NONE,
        .rx_queue_len = 32,
        .tx_queue_len = 32,
    };
    k_uart_init(K_UART1, &conf);

Another option for configuration is to use our UART defaults. The
k_uart_conf_defaults function will give you an already filled out
configuration struct with the standard UART parameters (8 bit words, 1
stop bit, no parity). The only thing that needs to be specified is the
interface in use.

.. code-block:: c

    KUARTConf conf = k_uart_conf_defaults();
    k_uart_init(K_UART1, &conf);


Reading
^^^^^^^

Reading from UART is a pretty simple operation, a buffer and length is
passed in. The buffer is filled and the number of characters read are
passed back.

.. code-block:: c

    char buffer[100];
    int num_read = 0;

    num_read = k_uart_read(K_UART1, buffer, 10);

Writing
^^^^^^^

Writing to UART is also a simple operation, a buffer and length are
passed in. The buffer is read from and the number of characters written
are passed back.

.. code-block:: c

    char buffer[20];
    int num_written = 0;

    num_written = k_uart_write(K_UART1, buffer, 10);