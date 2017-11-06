Using SPI
-------------

Configuration
^^^^^^^^^^^^^

The first step in using SPI is to configure and bring up the interface.
The SPI HAL provides a configuration structure with the standard SPI
options. This structure should be filled out according to the project's
SPI configuration and then it may be used to initialize the interface in
use.

.. code-block:: c

    KSPIConf conf = {
        .role = K_SPI_MASTER,
        .direction = K_SPI_DIRECTION_2LINES,
        .data_size = K_SPI_DATASIZE_8BIT,
        .clock_polarity = K_SPI_CPOL_HIGH,
        .clock_phase = K_SPI_CPHA_1EDGE,
        .first_bit = K_SPI_FIRSTBIT_LSB,
        .speed = 100000
    };

    k_spi_init(K_SPI1, &conf);

Reading
^^^^^^^

Reading from SPI is a pretty simple operation, a buffer and length is
passed in. The buffer is filled and the number of characters read are
passed back.

Note - you will need to manually pull down the appropriate chip select
pin before reading.

.. code-block:: c

    uint8_t buffer[100];
    int num_read = 0;
    const int CS = PA4;

    // Pull down chip select k_gpio_write(CS, 0);

    // Perform read operation num_read = k_spi_read(K_SPI1, buffer, 10);

    // Pull up chip select k_gpio_write(CS, 1);

Writing
^^^^^^^

Writing to SPI is also a simple operation, a buffer and length are
passed in. The buffer is read from and the number of characters written
are passed back.

Note - you will need to manually pull down the appropriate chip select
pin before reading.

.. code-block:: c

    uint8_t buffer[100];
    int num_written = 0;
    const int CS = PA4;

    // Pull down chip select k_gpio_write(CS, 0);

    // Perform write operation num_written = k_spi_write(K_SPI1, buffer,
    10);

    // Pull up chip select k_gpio_write(CS, 1);
