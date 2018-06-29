Using I2C in C
==============

.. toctree::
    :maxdepth: 1
    
    I2C API <c-i2c-api>
    Underlying Linux HAL <linux-i2c>

Initialization
--------------

The first step in using I2C is to configure and bring up the interface.
The :cpp:func:`k_i2c_default_dev_init` function will initialize a connection
with the specified I2C bus using the default configuration values.

.. code-block:: c

    KI2CStatus status;
    
    status = k_i2c_default_dev_init(K_I2C1);
    if (status != I2C_OK)
    {
        fprintf(stderr, "Failed to initialize I2C bus connection: %d\n", status);
        return -1;
    }

Writing
-------

The :cpp:func:`k_i2c_write` function should be used to write to an I2C device.
The function takes four arguments:

- The I2C bus to use for communication
- The I2C address of the slave device
- A pointer to the data to be written
- The number of bytes to be written

The function returns a :cpp:type:`KI2CStatus` value.
``I2C_OK`` indicates that the function completed successfully.

.. code-block:: c

    KI2CStatus status;
    char cmd = 0x40;
    int slave_addr = 0x80;

    status = k_i2c_write(K_I2C1, slave_addr, &cmd, 1);
    if (status != I2C_OK)
    {
        fprintf(stderr, "Failed to write to I2C device: %d\n", status);
        return -1;
    }
    
Reading
-------


The :cpp:func:`k_i2c_read` function should be used to read data from an I2C device.
The function takes four arguments:

- The I2C bus to use for communication
- The I2C address of the slave device
- A pointer to the read buffer
- The number of bytes to be read

The function returns a :cpp:type:`KI2CStatus` value.
``I2C_OK`` indicates that the function completed successfully.

.. code-block:: c

    KI2CStatus status;
    char buffer[100];
    int slave_addr = 0x80;

    status = k_i2c_read(K_I2C1, slave_addr, buffer, 10);
    if (status != I2C_OK)
    {
        fprintf(stderr, "Failed to read from I2C device: %d\n", status);
        return -1;
    }
    
Termination
-----------

Once all I2C work has been completed, the :cpp:func:`k_i2c_terminate` function
should be used to close the connection with the I2C bus and perform system cleanup.

.. code-block:: c

    k_i2c_terminate(K_I2C1);