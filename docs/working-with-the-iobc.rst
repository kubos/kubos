Working with the ISIS-OBC
=========================

Overview
--------

This document covers the KubOS Linux features which are specific to the
ISIS-OBC target, also known as the iOBC.

Reference Documents
-------------------

iOBC Documentation
~~~~~~~~~~~~~~~~~~

The :title:`ISIS-OBC Quickstart Guide` should have been packaged with the iOBC
and is a useful document for learning what each of the hardware
components are, how to connect them, and what drivers need to be
installed to support them.

Kubos Documentation
~~~~~~~~~~~~~~~~~~~

-  :doc:`first-linux-project` - Basic tutorial for creating your first KubOS
   Linux SDK project
-  :doc:`using-kubos-linux` - General guide for interacting with KubOS Linux
-  :doc:`KubOS Linux on iOBC <kubos-linux-on-iobc>` - Steps to
   build and load KubOS Linux for the iOBC

USB Connection
--------------

The iOBC should be shipped with an FTDI cable. This cable should be
connected to the programming adapter, which should then be connected to
the iOBC, to create the debug UART connection. User file transfer will
take place using this connection.

Additionally, a SAM-ICE JTAG must also be connected in order to successfully
connect with an iOBC.

Peripherals
-----------

The iOBC has several different ports available for interacting with
peripheral devices. Currently, users should interact with these devices
using the standard Linux functions. A Kubos HAL will be added in the
future for the iOBC.

I2C
~~~

`I2C Standards
Doc <http://www.nxp.com/documents/user_manual/UM10204.pdf>`__

KubOS Linux is currently configured to support the I2C standard-mode
speed of 100kHz.

The I2C bus is available to the userspace as the '/dev/i2c-0' device.
Users will need to add their peripheral device to the system and then
open the bus in order to communicate. Once communication is complete,
the bus should be closed and the device definition should be removed.

Since the peripheral devices will be different for each client, they
will need to be `dynamically added in the userspace (method
4) <https://www.kernel.org/doc/Documentation/i2c/instantiating-devices>`__.

The bus is then opened using the standard Linux ``open`` function and
used for communication with the standard ``write`` and ``read``
functions. These functions are described in the `Linux I2C dev-interface
doc <https://www.kernel.org/doc/Documentation/i2c/dev-interface>`__. The
buffer used in the ``write`` and ``read`` functions will most likely
follow the common I2C structure of "{register, value}"

The user program should look something like this:

.. code-block:: c

    /* Add device to system */
    system("echo i2cdevice 0x20 > /sys/bus/i2c/devices/i2c-0/new_device);

    /* Open I2C bus */
    file = open("/dev/i2c-0");

    /* Configure I2C bus to point to desired slave */
    ioctl(file, I2C_SLAVE, 0x20);

    /* Start of communication logic */
    buffer = {0x10, 0x34};
    write(file, buffer, sizeof(buffer));

    read(file, buffer, lengthToRead); 
    /* End of communication logic */

    /* Close I2C bus */
    close(file);

    /* Remove device */
    system("echo 0x20 > /sys/bus/i2c/devices/i2c-0/delete_device);

GPIO
~~~~

The iOBC has 27 GPIO pins available. These pins can be dynamically
controlled via the `Linux GPIO Sysfs Interface for
Userspace <https://www.kernel.org/doc/Documentation/gpio/sysfs.txt>`__
as long as they have not already been assigned to another peripheral.

To interact with a pin, the user will first need to generate the pin's
device name:

::

    $ echo {pin} > /sys/class/gpio/export

The {pin} value can be found in the below chart:

+---------------+--------------------+
| iOBC GPIO #   | Linux GPIO Value   |
+===============+====================+
| 0             | 42                 |
+---------------+--------------------+
| 1             | 43                 |
+---------------+--------------------+
| 2             | 44                 |
+---------------+--------------------+
| 3             | 45                 |
+---------------+--------------------+
| 4             | 52                 |
+---------------+--------------------+
| 5             | 53                 |
+---------------+--------------------+
| 6             | 54                 |
+---------------+--------------------+
| 7             | 55                 |
+---------------+--------------------+
| 8             | 56                 |
+---------------+--------------------+
| 9             | 57                 |
+---------------+--------------------+
| 10            | 58                 |
+---------------+--------------------+
| 11            | 59                 |
+---------------+--------------------+
| 12            | 60                 |
+---------------+--------------------+
| 13            | 61                 |
+---------------+--------------------+
| 14            | 62                 |
+---------------+--------------------+
| 15            | 63                 |
+---------------+--------------------+
| 16            | 12                 |
+---------------+--------------------+
| 17            | 13                 |
+---------------+--------------------+
| 18            | 14                 |
+---------------+--------------------+
| 19            | 15                 |
+---------------+--------------------+
| 20            | 16                 |
+---------------+--------------------+
| 21            | 17                 |
+---------------+--------------------+
| 22            | 18                 |
+---------------+--------------------+
| 23            | 19                 |
+---------------+--------------------+
| 24            | 20                 |
+---------------+--------------------+
| 25            | 21                 |
+---------------+--------------------+
| 26            | 22                 |
+---------------+--------------------+

For example, to interact with the iOBC's GPIO5 pin, which has a Linux
GPIO value of 53, the user will use:

::

    $ echo 53 > /sys/class/gpio/export

Once this command has been issued, the pin will be defined to the system
as '/sys/class/gpio/gpio{pin}'. The user can then set and check the pins
direction and value.

::

    Set GPIO5 as output:
    $ echo out > /sys/class/gpio/gpio53/direction

    Set GPIO23's value to 1:
    $ echo 1 > /sys/class/gpio/gpio19/value

    Get GPIO10's value:
    $ cat /sys/class/gpio/gpio58/value

SPI
~~~

The external SPI bus is not currently available to the userspace. It
will be added in a future release.

User Data Partition
-------------------

The iOBC has a single user data partition which maps to the `/home` 
directory.

The home directories of all user accounts, except root, should live
under this directory.

.. warning::

    Any files not residing under the /home directory will be destroyed
    during an upgrade/downgrade
