Working with the ISIS-OBC
=========================

Overview
--------

This document covers the Kubos Linux features which are specific to the
ISIS-OBC target, also known as the iOBC.

Please refer to :doc:`../../ecosystem/linux-docs/using-kubos-linux` for a general guide to using Kubos Linux.

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

-  :doc:`../../tutorials/first-obc-project` - Basic tutorial for creating your first KubOS project
-  :doc:`../../ecosystem/linux-docs/using-kubos-linux` - General guide for interacting with Kubos Linux
-  :doc:`../../deep-dive/klb/kubos-linux-on-iobc` - Steps to build Kubos Linux for the iOBC
-  :doc:`installing-linux-iobc` - Steps to install Kubos Linux

Status LEDs
-----------

There are four yellow LEDs present on the iOBC which give some indication of what state
the board is in, along with one red LED which is lit when the system is powered:

-  Three LEDS (solid) - The system is currently running U-Boot
-  One LED (blinking) - The system is currently running Kubos Linux

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

ADC
~~~

The iOBC has four analog input pins available:

+------+-------+
| iOBC | Linux |
+======+=======+
| AIN4 | 0     |
+------+-------+
| AIN5 | 1     |
+------+-------+
| AIN6 | 2     |
+------+-------+
| AIN7 | 3     |
+------+-------+

The pins are available through the Linux device ``/sys/bus/iio/devices/iio\:device0/``.

A single raw output value can be read from each of the pins via
``/sys/bus/iio/devices/iio\:device0/in_voltage{n}_raw``, where `{n}` corresponds to the
Linux AIN number.

This raw value can then be converted to microvolts by multiplying it by the value
found in ``/sys/bus/iio/devices/iio\:device0/in_voltage_scale``.

More information about the capture and use of ADC can be found in
`this guide from Atmel <https://www.at91.com/linux4sam/bin/view/Linux4SAM/IioAdcDriver>`__.

An `ADC example <http://github.com/kubos/kubos/tree/master/examples/adc-thermistor>`__ is
also available for reference in the Kubos repo.

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
    
I2C
~~~

`I2C Standards
Doc <http://www.nxp.com/documents/user_manual/UM10204.pdf>`__

Kubos Linux is currently configured to support the I2C standard-mode
speed of 100kHz.

The I2C bus is available as ``/dev/i2c-0``, or through the Kubos C HAL as ``K_I2C1``.

For examples and instructions, see the :doc:`I2C HAL documentation <../../deep-dive/apis/kubos-hal/i2c-hal/index>`.

PWM
~~~

The iOBC has 6 PWM pins available for use, grouped into three pairs:

    - PWM0 and PWM1
    - PWM2 and PWM3
    - PWM4 and PWM5

Users can interact with these pins through the `PWM sysfs interface <https://www.kernel.org/doc/Documentation/pwm.txt>`__,
and the ``/sys/class/pwm/pwmchip0/`` directory

In order to make a pin available for use, the PWM number should be passed to the ``pwmchip0/export`` file.
For example, the following would be done in order to enable access to PWM0::

    $ echo 0 > /sys/class/pwm/pwmchip0/export
    
After doing so, a new subdirectory will be available, ``pwmchip0/pwm0``.

From here, the pin's properties can be configured and then it can be enabled.

.. note::

    Due to the underlying hardware, each pair of pins must use the same period value.
    They may, however, have differing duty cycles.
    
    The nanosecond value specified for period and duty cycle will be internally converted to the nearest clock divider value.

For example::

    /* Set the period of the generated wave to 1 millisecond */
    $ echo 1000000 > /sys/class/pwm/pwmchip0/pwm0/period
    
    /* Set the duty cycle to 50% (.5 millisecond) */
    $ echo 500000 > /sys/class/pwm/pwmchip0/pwm0/duty_cycle
    
    /* Turn on the signal */
    $ echo 1 > enable
    
Then, to turn the signal off::

    $ echo 0 > enable

SPI
~~~

The iOBC has one SPI bus available for external use with three pre-allocated chip select pins.
All pins are exposed via either an iOBC daughterboard (J5 connection) or optional iOBC header (J3 connection).

**SPI Bus 1**

+------+------------+
| Name | Pin        |
+======+============+
| MOSI | SPI1_MOSI  |
+------+------------+
| MISO | SPI1_MISO  |
+------+------------+
| SCLK | SPI1_SPCK  |
+------+------------+
| CS0  | SPI1_NPCS0 |
+------+------------+
| CS1  | SPI1_NPCS1 |
+------+------------+
| CS2  | SPI1_NPCS2 |
+------+------------+

Users can interact a device on this bus using Linux's `spidev interface <https://www.kernel.org/doc/Documentation/spi/spidev>`__
The device name will be ``/dev/spidev1.n``, where *n* corresponds to the chip select number.

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
    uint32_t speed = 1000000;
    uint16_t delay = 0;
    uint8_t tx[2] = {0};
    uint8_t rx[2] = {0};
    
    uint8_t value;
    
    fd = open(SPI_DEV, O_RDWR);
    
    /* Register to read from */
    tx[0] = 0xD0;

    /* Set up communication configuration */
    struct spi_ioc_transfer tr = {
        .tx_buf = (unsigned long)tx,
        .rx_buf = (unsigned long)rx,
        .len = 1,
        .speed_hz = speed,
        .bits_per_word = bits,
        .cs_change = 0,
        .delay_usecs = delay,
    };

    /* Send request to read */
    ioctl(fd, SPI_IOC_MESSAGE(1), &tr);

    /* Grab result from response buffer */
    value = rx[1];

    close(fd);
    
UART
~~~~

The iOBC has 2 UART ports available for use in varying capacities:

+--------------------+--------+--------+---------+---------+
| Linux Device       | TX Pin | RX Pin | RTS Pin | CTS Pin |
+====================+========+========+=========+=========+
| /dev/ttyS1 (SLIP)  | TX0    | RX0    |         |         |
+--------------------+--------+--------+---------+---------+
| /dev/ttyS3         | TX2    | RX2    | RTS2    | CTS2    |
+--------------------+--------+--------+---------+---------+

Users can interact with these ports using Linux's `termios <http://man7.org/linux/man-pages/man3/termios.3.html>`__ interface.

`A tutorial on this interface can be found here <http://tldp.org/HOWTO/Serial-Programming-HOWTO/x115.html>`__

The ``/dev/ttyS1`` device has been preconfigured to be used for SLIP connections.
Please refer to the :ref:`SLIP instructions <slip>` for more information.

User Data Partition
-------------------

The iOBC has a single user data partition which maps to the `/home`
directory.

The home directories of all user accounts, except root, should live
under this directory.

.. warning::

    Any files not residing under the /home directory will be destroyed
    during an upgrade/downgrade
