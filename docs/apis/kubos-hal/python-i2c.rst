Python I2C
-------------

Initialization
^^^^^^^^^^^^^

The first step in using I2C is to import the module and choose the i2c bus to be used. The KubOS primary I2C bus is 1. This operation does not open the device file. 

.. code-block:: python

    from i2c import I2C
    
    i2c_device = I2C(bus = 1)
    

Reading
^^^^^^^

Reading from I2C is a pretty simple operation, the slave address, a
buffer and length is passed in. The buffer is filled and the number of
characters read are passed back.

.. code-block:: python
    
    slave_address = 0x51 
    num_read = 20 # number of bytes to read.
    
    data = i2c_device.read(device = slave_address, count = num_read)


Writing
^^^^^^^

Writing to I2C is also a simple operation, the slave address and the command are passed it. The command is written to the slave address and whether or not it was successful and the command written are returned. The commmand must be a string or list.

.. code-block:: python

    slave_address = 0x51
    command = "I2C command"
    
    success,written_command = i2c_device.write(device = slave_address,data = command)