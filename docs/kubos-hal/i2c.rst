Using I2C
-------------

Configuration
^^^^^^^^^^^^^

The first step in using I2C is to configure and bring up the interface.
The I2C HAL provides a configuration structure with the standard I2C
options. This structure should be filled out according to the project's
I2C configuration and then it may be used to initialize the interface in
use.

.. code-block:: c

    KI2CConf conf = { 
        .addressing_mode = K_ADDRESSINGMODE_7BIT,
        .role = K_MASTER,
        .clock_speed = 100000 
    };

    k_i2c_init(K_I2C1, &conf);

Another option for configuration is to use our I2C defaults. The
k_i2c_default_dev_init function will initialize the specified
interface with the default configuration values.

.. code-block:: c

    k_i2c_default_dev_init(K_I2C1);

Reading
^^^^^^^

Reading from I2C is a pretty simple operation, the slave address, a
buffer and length is passed in. The buffer is filled and the number of
characters read are passed back.

.. code-block:: c

    char buffer[100];
    int num_read = 0;
    int slave_addr = 0x80;

    num_read = k_i2c_read(K_I2C1, slave_addr, buffer, 10);

Writing
^^^^^^^

Writing to I2C is also a simple operation, the slave address, a buffer
and length are passed in. The buffer is read from and the number of
characters written are passed back.

.. code-block:: c

    char cmd = 0x40;
    int num_written = 0;
    int slave_addr = 0x80;

    num_written = k_i2c_write(K_I2C1, slave_addr, &cmd, 1);