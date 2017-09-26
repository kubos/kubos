Working with the Pumpkin MBM2
=============================

Overview
--------

This document covers the KubOS Linux features which are specific to the
Pumpkin MBM2 target.

Reference Documents
-------------------

Pumpkin Documentation
~~~~~~~~~~~~~~~~~~~~~

The :title:`CubeSat Kit Motherboard Module (MBM) 2` reference document
is available from Pumpkin and is a useful document for learning what 
each of the hardware components are and how they are connected.

Kubos Documentation
~~~~~~~~~~~~~~~~~~~

-  :doc:`first-linux-project` - Basic tutorial for creating your first KubOS
   Linux SDK project
-  :doc:`using-kubos-linux` - General guide for interacting with KubOS Linux
-  :doc:`KubOS Linux on Pumpkin MBM2 <kubos-linux-on-mbm2>` - Steps to
   build and load KubOS Linux for the Pumpkin MBM2

USB Connection
--------------

The Pumpkin MBM2 should be shipped with a USB Debug Adapter board.

The white connection cable should be plugged into the labeled "UART0"
port on the edge of the board, with the exposed pins facing up.

The USB cable can then be plugged into your computer. Any required
drivers should be automatically installed.

This connection will be passed through to a Kubos Vagrant image as
`/dev/FTDI` and will be used for the serial console.

.. _peripherals-mbm2:

Peripherals
-----------

The Pumpkin MBM2 has several different ports available for interacting 
with peripheral devices. Currently, users should interact with these 
devices using the standard Linux functions. A Kubos HAL will be added 
in the future to abstract this process.

UART
~~~~

The Pumpkin MBM2 has 5 UART ports available for use in varying capacities:

+--------------+--------+--------+---------+---------+
| Linux Device | TX Pin | RX Pin | RTS Pin | CTS Pin |
+==============+========+========+=========+=========+
| /dev/ttyS1   | H1.18  | H1.17  | H1.10   | H1.9    |
+--------------+--------+--------+---------+---------+
| /dev/ttyS2   | H1.8   | H1.7   |         |         |
+--------------+--------+--------+---------+---------+
| /dev/ttyS3   | H1.5   |        |         |         |
+--------------+--------+--------+---------+---------+
| /dev/ttyS4   | H1.16  | H1.15  |         |         |
+--------------+--------+--------+---------+---------+
| /dev/ttyS5   | H1.20  | H1.19  | H1.12   | H1.11   |
+--------------+--------+--------+---------+---------+

Users can interact with these ports using Linux's `termios <http://man7.org/linux/man-pages/man3/termios.3.html>`__ interface.

`A tutorial on this interface can be found here <http://tldp.org/HOWTO/Serial-Programming-HOWTO/x115.html>`__

I2C
~~~

The Pumpkin MBM2 has one user-accessible I2C bus.
Users can connect a new device to it via pins **H1.43** (SCL) and **H1.41** (SDA)
of the CubeSat Kit Bus connectors.

`I2C Standards
Doc <http://www.nxp.com/documents/user_manual/UM10204.pdf>`__

KubOS Linux is currently configured to support the I2C standard-mode
speed of 100kHz.

The I2C bus is available through the Kubos HAL as ``K_I2C1``.

For examples and instructions, see the :doc:`kubos-hal/i2c` and
:doc:`kubos-hal/i2c_api` documents.

ADC
~~~

The Pumpkin MBM2 has seven analog input pins available:

+------+------+
| Name | Pin  |
+======+======+
| AIN0 | H2.8 |
+------+------+
| AIN1 | H2.7 |
+------+------+
| AIN2 | H2.6 |
+------+------+
| AIN3 | H2.5 |
+------+------+
| AIN4 | H2.4 |
+------+------+
| AIN5 | H2.3 |
+------+------+
| AIN6 | H2.2 |
+------+------+

The pins are available through the Linux device ``/sys/bus/iio/devices/iio\:device0/``.

A single raw output value can be read from each of the pins via
``/sys/bus/iio/devices/iio\:device0/in_voltage{n}_raw``, where `{n}` corresponds to the
AIN number of the pin.

Information about setting up continuous data gathering can be found in
`this guide from TI <http://processors.wiki.ti.com/index.php/Linux_Core_ADC_Users_Guide>`__.

To convert the raw ADC value to a voltage, use this equation:

.. math::
    
    V_{in} = \frac{D * (2^n - 1)}{V_{ref}}

Where:

    - :math:`D` = Raw ADC value
    - :math:`n` = Number of ADC resolution bits 
    - :math:`V_{ref}` =  Reference voltage
    
The Pumpkin MBM2 uses 12 resolution bits and a reference voltage of 1.8V, so the
resulting equation is

.. math::

    V_{in} = \frac{D * (4095)}{1.8}


GPIO
~~~~

The CSK headers have 6 GPIO pins available for use.
These pins can be dynamically controlled via the `Linux GPIO Sysfs 
Interface for Userspace <https://www.kernel.org/doc/Documentation/gpio/sysfs.txt>`__
as long as they have not already been assigned to another peripheral.

+---------+------------------+-----------+
| CSK Pin | Linux GPIO Value | Direction |
+=========+==================+===========+
| H1.6    | 65               | Input     |
+---------+------------------+-----------+
| H2.18   | 61               | Output    |
+---------+------------------+-----------+
| H2.21   | 89               | Output    |
+---------+------------------+-----------+
| H2.22   | 87               | Output    |
+---------+------------------+-----------+
| H2.23   | 86               | Output    |
+---------+------------------+-----------+
| H2.24   | 85               | Output    |
+---------+------------------+-----------+

To interact with a pin, the user will first need to generate the pin's
device name:

::

    $ echo {pin} > /sys/class/gpio/export

For example, to interact with pin H2.23 of the CSK header, which corresponds with
GPIO_86, the user will use:

::

    $ echo 86 > /sys/class/gpio/export

Once this command has been issued, the pin will be defined to the system
as '/sys/class/gpio/gpio{pin}'. The user can then set and check the pins
direction and value.

::

    Set H2.23 as output:
    $ echo out > /sys/class/gpio/gpio86/direction

    Set GPIO_86's value to 1:
    $ echo 1 > /sys/class/gpio/gpio86/value

    Get GPIO_86's value:
    $ cat /sys/class/gpio/gpio86/value

.. note:: The GPIO direction should match the value in the above table

User Data Partitions
--------------------

The Pumpkin MBM2 has multiple user data partitions available, one on each storage
device. 

eMMC
~~~~

The user partition on the eMMC device is used as the primary user data storage area.
All system-related `/home/` paths will reside here.

/home/usr/bin
^^^^^^^^^^^^^

All user-created applications will be loaded into this folder during the
``kubos flash`` process. The directory is included in the system's PATH,
so applications can then be called directly from anywhere, without
needing to know the full file path.

/home/usr/local/bin
^^^^^^^^^^^^^^^^^^^

All user-created non-application files will be loaded into this folder
during the ``kubos flash`` process. There is currently not a way to set
a destination folder for the ``kubos flash`` command, so if a different
endpoint directory is desired, the files will need to be manually moved.

/home/etc/init.d
^^^^^^^^^^^^^^^^

All user-application initialization scripts live under this directory.
The naming format is 'S{run-level}{application}'.

microSD
~~~~~~~

/home/microsd
^^^^^^^^^^^^^

This directory points to a partition on the microSD device included with the 
base Beaglebone Black board

.. todo::

    SD over SPI - /home/spisd
    (header characters here)
    
    This directory points to a partition on the SD over SPI device included as a
    peripheral device of the Pumpkin MBM2 board.
    
    EEPROM - /home/eeprom
    (header characters here)
    
    This directory points to the available space of the EEPROM storage included with 
    the base Beaglebone Black board. There are 4KB of space available for use.
    
    .. note:: 
    
        While EEPROM storage is more stable and safe than MMC/SD, it also has a much
        more limited number of writes available. This storage should be used carefully.
