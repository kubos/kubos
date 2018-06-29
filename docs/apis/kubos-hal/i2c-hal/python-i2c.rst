Python I2C
-------------

Initialization
^^^^^^^^^^^^^^^

The first step in using I2C is to import the module and choose the I2C bus to be used. This operation does not open the device file. 

.. code-block:: python

    from i2c import I2C
    
    i2c_device = I2C(bus = 1)
    

Reading
^^^^^^^

The read function should be used to read data from an I2C device.
It takes two arguments: the I2C address of the slave device and the number of bytes to read. 

.. code-block:: python
    
    slave_address = 0x51 
    num_read = 20 # Number of bytes to read
    
    data = i2c_device.read(device = slave_address, count = num_read)


Writing
^^^^^^^

The write function should be used to write to an I2C device.
The function takes two arguments:

- The I2C address of the slave device
- The data to be sent
  - The data must be formatted as a string or a list

The function returns two items:

- A boolean indicating success (True), or failure (False)
- An echo of the data that was written

String Command Example:

.. code-block:: python

    slave_address = 0x51
    command = "SUP:RES NOW\x0a" # String command
    
    success,written_command = i2c_device.write(device = slave_address, data = command)
    
List Command Example:

.. code-block:: python

    slave_address = 0x51
    command = [0x53,0x55,0x50,0x3a,0x52,0x45,0x53,0x20,0x4e,0x4f,0x57,0x0a] 
    
    success,written_command = i2c_device.write(device = slave_address, data = command)