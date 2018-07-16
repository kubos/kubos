Working with the Beaglebone Black
=================================

Overview
--------

This document covers the Kubos Linux features which are specific to the
Beaglebone Black target.

Please refer to :doc:`using-kubos-linux` for a general guide to using Kubos Linux.

Reference Documents
-------------------

Beaglebone Documentation
~~~~~~~~~~~~~~~~~~~~~~~~

- `Beaglebone Black Web Page <https://beagleboard.org/black>`__
- `Beaglebone Black Wiki <http://elinux.org/Beagleboard:BeagleBoneBlack>`__
- `Beaglebone Black Hardware Diagrams <http://beagleboard.org/Support/bone101/#hardware>`__
- `Beaglebone Black System Reference Manual Rev C <http://static6.arrow.com/aropdfconversion/8fff89aa85f5c451318cbdee2facd9c9fac36872/bbb_srm.pdf>`__

Kubos Documentation
~~~~~~~~~~~~~~~~~~~

-  :doc:`first-linux-project` - Basic tutorial for creating your first Kubos SDK project
-  :doc:`../sdk-docs/sdk-cheatsheet` - Overview of the common Kubos SDK commands
-  :doc:`using-kubos-linux` - General guide for interacting with Kubos Linux
-  :doc:`kubos-linux-on-bbb` - Steps to build Kubos Linux for the Beaglebone Black
-  :doc:`../installation-docs/installing-linux-bbb` - Steps to install Kubos Linux

Debug Console Connection
------------------------

As documented in section 7.5 of the :title:`Beaglebone Black System
Reference Manual`, an FTDI cable can be connected to the serial debug
connector in order to establish a debug console connection.

This connection will be passed through to a Kubos Vagrant image as
`/dev/FTDI`.

Status LEDs
-----------

There are four LEDs present on the Beaglebone Black which give some indication of what state
the board is in. When there is only one blinking LED, the board is running Kubos Linux and
the system is currently idle. The LEDs will blink in correspondence with CPU and MMC activity.
If all LEDs are solid, then the system has reached some kind of locked error state.

Peripherals
-----------

The Beaglebone Black has several different ports available for interacting 
with peripheral devices. Currently, users should interact with these 
devices using the standard Linux functions. A Kubos HAL will be added 
in the future to abstract this process.

.. note::

    Kubos Linux for the Pumpkin MBM2 can be used instead of Kubos Linux
    for the Beaglebone Black. In this case, some buses and pins won't be
    available, since they aren't exposed in the MBM2's CSK headers, or are
    dedicated to other uses. See the :ref:`peripherals-mbm2` section for 
    more information.
          
ADC
~~~

The Beaglebone Black has seven analog input pins available:

+------+-------+
| Name | Pin   |
+======+=======+
| AIN0 | P9.39 |
+------+-------+
| AIN1 | P9.40 |
+------+-------+
| AIN2 | P9.37 |
+------+-------+
| AIN3 | P9.38 |
+------+-------+
| AIN4 | P9.33 |
+------+-------+
| AIN5 | P9.36 |
+------+-------+
| AIN6 | P9.35 |
+------+-------+

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
    
The Beaglebone Black uses 12 resolution bits and a reference voltage of 1.8V, so the
resulting equation is

.. math::

    V_{in} = \frac{D * (4095)}{1.8}
    
Ethernet
~~~~~~~~

The Beaglebone Black provides an ethernet port which can be used for things 
like inter-system communication.

The ethernet port is configured to have support for static IPv4 addressing and
can be used with SSH via the included `Dropbear <https://en.wikipedia.org/wiki/Dropbear_(software)>`__ 
package.

Kubos Linux currently guarantees support for TCP, UDP, and SCTP.
Other protocols might be supported by default, but have not been verified.

Resources
^^^^^^^^^

- :ref:`Kubos Ethernet Communication Guide <ethernet>` 
- `TCP tutorial <http://www.linuxhowtos.org/C_C++/socket.htm>`__
- `UDP tutorial <https://www.cs.rutgers.edu/~pxk/417/notes/sockets/udp.html>`__
- `SCTP tutorial <http://petanode.com/blog/posts/introduction-to-the-sctp-socket-api-in-linux.html>`__
- `Packet Sender <https://packetsender.com/>`__ - A tool to send test packets between an OBC and a host computer

.. note:: By default, Windows Firewall will block many incoming packet types. This may impact testing.

Configuration
^^^^^^^^^^^^^

The static IP address can be updated by editing the `/etc/network/interfaces` file.
By default the address is ``10.0.2.20``.

Examples
^^^^^^^^

A couple example programs using the ethernet port can be found in the `examples` folder of the `kubos repo <https://github.com/kubos/kubos/tree/master/examples>`__:

- `kubos-linux-tcprx <https://github.com/kubos/kubos/tree/master/examples/kubos-linux-tcprx>`__ - Receive TCP packets and then reply to the sender
- `kubos-linux-tcptx <https://github.com/kubos/kubos/tree/master/examples/kubos-linux-tcptx>`__ - Send TCP packets to specified IP address and port

GPIO
~~~~

The Beaglebone Black has many GPIO pins available for general use. Pinout diagrams
are available on the `Beaglebone website <http://beagleboard.org/Support/bone101/#hardware>`__.

Any pin that is not dedicated to a previously mentioned peripheral is available for use.

CLI and Script Interface
^^^^^^^^^^^^^^^^^^^^^^^^

To interact with a pin from the command line or from a script, the user will first need to 
generate the pin's device name:

::

    $ echo {pin} > /sys/class/gpio/export

For example, to interact with pin P8.11, which corresponds with GPIO_45, the user will use:

::

    $ echo 45 > /sys/class/gpio/export

Once this command has been issued, the pin will be defined to the system
as '/sys/class/gpio/gpio{pin}'. The user can then set and check the pins
direction and value.

::

    Set pin as output:
    $ echo out > /sys/class/gpio/gpio45/direction

    Set pin's value to 1:
    $ echo 1 > /sys/class/gpio/gpio45/value

    Get pins's value:
    $ cat /sys/class/gpio/gpio45/value

Once finished, the pin can be released:

::

    $ echo {pin} > /sys/class/gpio/unexport

Application Interface
^^^^^^^^^^^^^^^^^^^^^
    
This functionality can also be used from a user's application with Linux's sysfs
interface.

An example program might look like this:

.. code-block:: c
    
    #include <sys/stat.h>
    #include <sys/types.h>
    #include <fcntl.h>
    #include <stdio.h>
    #include <stdlib.h>
    #include <unistd.h>
    
    int fd;
    int pin = 45;
    int value = 1;
    
    /* Define the pin to the system */
    fd = open("/sys/class/gpio/export", O_WRONLY);
    write(fd, &pin, sizeof(pin)); 
    close(fd);
    
    /* Set the pin's direction */
    fd = open("/sys/class/gpio/gpio45/direction", O_WRONLY);
    write(fd, "out", 3);
    close(fd);
    
    /* Set the pin's value */
    fd = open("/sys/class/gpio/gpio45/value", O_WRONLY);
    write(fd, &value, 1);
    close(fd);
    
    /* Read the value back */
    fd = open("/sys/class/gpio/gpio45/value", O_RDONLY);
    char strValue[3];
    read(fd, strValue, 3);
    value = atoi(strValue);
    close(fd);
    
    /* Release the pin */
    fd = open("/sys/class/gpio/unexport", O_WRONLY);
    write(fd, &pin, sizeof(pin)); 
    close(fd);
    
I2C
~~~

The Beaglebone Black has two user-accessible I2C buses.

+--------------+--------------+---------+---------+
| Linux Device | Kubos Device | SCL Pin | SDA Pin |
+==============+==============+=========+=========+
| /dev/i2c-1   | K_I2C1       | P9.17   | P9.18   |
+--------------+--------------+---------+---------+
| /dev/i2c-2   | K_I2C2       | P9.19   | P9.20   |
+--------------+--------------+---------+---------+

`I2C Standards
Doc <http://www.nxp.com/documents/user_manual/UM10204.pdf>`__

Kubos Linux is currently configured to support the I2C standard-mode
speed of 100kHz.

For examples and instructions, see the :doc:`I2C HAL documentation <../apis/kubos-hal/i2c-hal/index>`.

SPI
~~~

The Beaglebone has one SPI bus available with a pre-allocated chip select pin.

**SPI Bus 1**

+------+-------+
| Name | Pin   |
+======+=======+
| MOSI | P9.30 |
+------+-------+
| MISO | P9.29 |
+------+-------+
| SCLK | P9.31 |
+------+-------+
| CS0  | P9.28 |
+------+-------+

Users can interact a device on this bus using Linux's `spidev interface <https://www.kernel.org/doc/Documentation/spi/spidev>`__
The device name will be ``/dev/spidev1.0``.

An example user program to read a value might look like this:

.. code-block:: c

    #include <fcntl.h>
    #include <unistd.h>
    #include <sys/ioctl.h>
    #include <linux/types.h>
    #include <linux/spi/spidev.h>
      
    #define SPI_DEV "/dev/spidev1.0"
    
    int fd;
    uint8_t mode = SPI_MODE_0;
    uint8_t bits = 8;
    uint32_t speed = 100000;
    uint16_t delay = 0;
    
    uint8_t register, shift_reg;
    uint8_t value;
    
    fd = open(SPI_DEV, O_RDWR);
    
    /* Register to read from */
    register = 0xD0;

    struct spi_ioc_transfer tr = {
        .tx_buf = (unsigned long)&register,
        .rx_buf = (unsigned long)&register,
        .len = 1,
        .speed_hz = speed,
        .bits_per_word = bits,
        .cs_change = 0,
        .delay_usecs = delay,
    };

    /* Send request to read */
    ioctl(fd, SPI_IOC_MESSAGE(1), &tr);

    /* Setup buffer to read to */
    tr.tx_buf = &value;
    tr.rx_buf = &value;    
    
    /* Read data */
    ioctl(fd, SPI_IOC_MESSAGE(1), &tr);

    close(fd);

UART
~~~~

The Beaglebone Black has 5 UART ports available for use:

+--------------+--------+--------+---------+---------+
| Linux Device | TX Pin | RX Pin | RTS Pin | CTS Pin |
+==============+========+========+=========+=========+
| /dev/ttyS1   | P9.24  | P9.26  |         |         |
+--------------+--------+--------+---------+---------+
| /dev/ttyS2   | P9.21  | P9.22  |         |         |
+--------------+--------+--------+---------+---------+
| /dev/ttyS3   | P9.42  |        | P8.34   | P8.36   |
+--------------+--------+--------+---------+---------+
| /dev/ttyS4   | P9.13  | P9.11  | P8.33   | P8.35   |
+--------------+--------+--------+---------+---------+
| /dev/ttyS5   | P8.37  | P8.38  | P8.32   | P8.31   |
+--------------+--------+--------+---------+---------+

.. note:: /dev/ttyS3 (UART3) is TX-only. /dev/ttyS1 and /dev/ttyS2 do not 
    have RTS/CTS due to pin conflicts with other buses.

Users can interact with these ports in their applications using Linux's 
`termios <http://man7.org/linux/man-pages/man3/termios.3.html>`__ interface.

`A tutorial on this interface can be found here <http://tldp.org/HOWTO/Serial-Programming-HOWTO/x115.html>`__

Additionally, the ports can be used from the command line:

The ``stty -F {device} [parameters]`` command can be used to 
configure the port. For example, this command will set the
baud rate of `/dev/ttyS1` to 4800::

    $ stty -F /dev/ttyS1 4800
    
The ``echo`` command can be used to transmit basic data out of
the TX pin. For example::

    $ echo "Hello!" > /dev/ttyS1
    
The ``cat`` command can be used to read any data from the RX
pin. For example::

    $ cat < /dev/ttyS1

User Data Partitions
--------------------

The Beaglebone Black has two user data partitions available, one on each storage
device. 

eMMC
~~~~

The user partition on the eMMC device is used as the primary user data storage area.
All system-related `/home/` paths will reside here.

/home/system/usr/bin
^^^^^^^^^^^^^^^^^^^^

All user-created applications will be loaded into this folder during the
``kubos flash`` process. The directory is included in the system's PATH,
so applications can then be called directly from anywhere, without
needing to know the full file path.

/home/system/usr/local/bin
^^^^^^^^^^^^^^^^^^^^^^^^^^

All user-created non-application files will be loaded into this folder
during the ``kubos flash`` process. There is currently not a way to set
a destination folder for the ``kubos flash`` command, so if a different
endpoint directory is desired, the files will need to be manually moved.

/home/system/etc/init.d
^^^^^^^^^^^^^^^^^^^^^^^
All user-application initialization scripts live under this directory.
The naming format is 'S{run-level}{application}'.

microSD
~~~~~~~

/home/microsd
^^^^^^^^^^^^^

This directory points to a partition on the microSD device included with the 
base Beaglebone Black board

.. todo::
    
    EEPROM - /home/eeprom
    (header characters here)
    
    This directory points to the available space of the EEPROM storage included with 
    the Beaglebone Black board. There are 4KB of space available for use.
    
    .. note:: 
    
        While EEPROM storage is more stable and safe than MMC/SD, it also has a much
        more limited number of writes available. This storage should be used carefully.
