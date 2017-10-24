Working with the ISIS-OBC
=========================

Overview
--------

This document covers the KubOS Linux features which are specific to the
ISIS-OBC target, also known as the iOBC.

Please refer to :doc:`using-kubos-linux` for a general guide to using KubOS Linux.

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
   
Status LEDs
-----------

There are four yellow LEDs present on the iOBC which give some indication of what state
the board is in, along with one red LED which is lit when the system is powered:

-  Three LEDS (solid) - The system is currently running U-Boot
-  One LED (blinking) - The system is currently running KubOS Linux

Debug Console Connection
------------------------

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

UART
~~~~

The iOBC has 2 UART ports available for use in varying capacities:

+--------------+--------+--------+---------+---------+
| Linux Device | TX Pin | RX Pin | RTS Pin | CTS Pin |
+==============+========+========+=========+=========+
| /dev/ttyS1   | TX0    | RX0    |         |         |
+--------------+--------+--------+---------+---------+
| /dev/ttyS3   | TX2    | RX2    | RTS2    | CTS2    |
+--------------+--------+--------+---------+---------+

Users can interact with these ports using Linux's `termios <http://man7.org/linux/man-pages/man3/termios.3.html>`__ interface.

`A tutorial on this interface can be found here <http://tldp.org/HOWTO/Serial-Programming-HOWTO/x115.html>`__

I2C
~~~

`I2C Standards
Doc <http://www.nxp.com/documents/user_manual/UM10204.pdf>`__

KubOS Linux is currently configured to support the I2C standard-mode
speed of 100kHz.

The I2C bus is available through the Kubos HAL as ``K_I2C1``.

For examples and instructions, see the :doc:`kubos-hal/i2c` and
:doc:`kubos-hal/i2c_api` documents.

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

The external SPI bus is not currently available to the user space. It
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
