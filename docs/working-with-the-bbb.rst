Working with the Beaglebone Black
=================================

Overview
--------

This document is intended as an introductory guide for creating,
loading, and using Kubos projects and other files within the user space
of KubOS Linux on the Beaglebone Black.

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

-  :doc:`Installing the Kubos SDK <sdk-installing>` - Basics of
   setting up the Kubos SDK environment
-  :doc:`Creating your first KubOS Linux project <first-linux-project>` - Steps to
   create and build a Kubos SDK project
-  :doc:`SDK Command Reference <sdk-reference>` - Overview of the
   common Kubos SDK commands
-  :doc:`KubOS Linux Overview <kubos-linux-overview>` - Overview of
   the KubOS Linux components
-  :doc:`KubOS Linux on the Beaglebone Black <kubos-linux-on-bbb>` - Steps to
   build and load KubOS Linux for the Beaglebone Black

Building a Project
------------------

In order to build a project for the Beaglebone, you'll need to create a
Kubos SDK project for KubOS Linux, set the correct target, and then
build it.

::

    $ kubos init -l newproj
    $ kubos target kubos-linux-beaglebone-gcc
    $ kubos build

.. _updating-credentials:

Updating Credentials
--------------------

Ideally, you should not be using the default root user password. If
you've changed it, you'll need to pass the new password to the Kubos
flash utility through the config.json file, which should be located in
the top-level directory of your project. You'll need to create the file
if it doesn't already exist. Update the :json:object:`system.password <system>` parameter with
the password to use.

If you're creating a brand new config.json file, you can just copy and
paste the text below (*newpass* should be your desired password):

::

    {
        "system" : {
            "password" : "newpass"
        }
    }

Setting Initialization Configuration
------------------------------------

There are several :json:object:`config.json <system>` options available which customize how and
when a user application is started:

-  system.initAfterFlash - (Default: false) Tells the Kubos SDK whether
   to start the application as a background daemon after flashing it to
   the board.
-  system.initAtBoot - (Default: false) Tells the Kubos SDK whether to
   generate and install an initialization script.
-  system.runLevel - (Default: 50. Range: 10-99) Sets priority of
   initialization script.

When the :json:object:`system.initAfterFlash <system>` option is turned on, the user
application will be started as a background daemon at the end of the
``kubos flash`` process. This is done using Linux's
``start-stop-daemon`` command. By default this feature is turned off, so
the value of the option will need to be set to "true" by the user in
order to turn it on.

If enabled, an initialization script will be generated and installed
during the flashing process. This script will follow the naming
convention of "S{runLevel}{applicationName}", where "runLevel" is the
initialization priority order of the script. All user application init
scripts will be run after all of the Kubos init scripts, but within the
user scripts, the scripts with the lowest run level will be executed
first. So an application with a run level of 10 will be initialized
before an application with a run level of 50.

The run level of an initialization script can be changed after initially
flashing the script to the board. Simply change the :json:object:`system.runLevel <system>`
value, rebuild the project, and then reflash it to the board. The old
script will be removed as part of the flash process.

USB Connection
--------------

As documented in section 7.5 of the :title:`Beaglebone Black System
Reference Manual`, an FTDI cable can be connected to the serial debug
connector in order to establish a debug console connection.

Flashing the Application
------------------------

Connect an FTDI cable to the debug serial interface and power the board.

Assuming you've successfully built a Kubos SDK project for the board, 
when you issue the ``kubos flash`` the output should look like this:

::

    info: found newproj at source/newproj
    Compatible FTDI device found
    Sending file to board...
    Bytes Sent: 693248/1769379 BPS:8343 ETA 02:08
    Transfer Successful
    Execution time: 21 seconds

The application binary will be loaded into the /home/system/usr/bin
directory on the target board.

If the 'system.initAtBoot' option has been turned on, then a standard
initialization script will be generated and flashed into the
/home/system/etc/init.d directory automatically during the application
flashing process.

If the 'system.initAfterFlash' option has been turned on, then the
application will be started as a background service as the last step in
the application flashing process.

Flashing Non-Application Files
------------------------------

If you would like to flash a file other than the application binary onto
your board, you can add an additional parameter to the usual flash
commad:

::

    $ kubos flash {absolute-path-of-file}

If the name of the file matches the name of the application, as
specified in the module.json file, then the file is assumed to be the
application binary and will be loaded into /home/system/usr/bin on the
target board.

If the name of the file ends in \*.itb, the file is a KubOS Linux
upgrade package and will be loaded into the upgrade partition of the
target board. An internal variable will be set so that the upgrade
package will be installed during the next reboot of the target board.

All other files are assumed to be non-application files (ex. custom
shell scripts) and will be loaded into /home/system/usr/local/bin. Once
they have been flashed, these files can then be manually moved to
another location.

**Note:** The file does not need to reside within a Kubos SDK project,
but the ``kubos flash`` command must still be run from the project,
since that is where the target configuration information is stored.

For example:

::

    $ kubos flash /home/vagrant/not-my-project/test-util.sh

Flash Troubleshooting
---------------------

Flashing a file to the board can fail for various reasons. Sometimes
simply reattempting the command can correct the problem.

If retrying doesn't work, here is a list of some of the error you might
see after running the ``kubos flash`` command and the recovery actions
you can take:

"No compatible FTDI device found"

-  Check that the board is turned on and connected to your computer
-  Check that no other vagrant images are running. Only one VM can have
   control of the USB, so it may be that another instance currently has
   control of the device. You can shutdown a vagrant image with the
   command ``vagrant halt``
-  Verify that the USB is showing up within the vagrant environment with
   the ``lsusb`` command. You should see an FTDI device
-  Verify that the USB has been mapped to a linux device. Issue the
   command ``ls /dev``. You should see a /dev/ttyUSB\* device. If you
   don't, try rebooting your vagrant image (``vagrant halt``,
   ``vagrant up``)

"Transfer Failed: Connection Failed"

-  The SDK was unable to connect to the board
-  Verify that the USB has been mapped to a linux device. Issue the
   command ``ls /dev``. You should see a /dev/ttyUSB\* device. If you
   don't, try rebooting your vagrant image (``vagrant halt``,
   ``vagrant up``)
-  If this error occurs after the transfer process has started, then the
   SDK likely lost connection to the board. Verify that the board is
   still correctly connected and powered and try the flash command
   again.

"Transfer Failed: Invalid Password"

-  The SDK was unable to log into the board. Verify that the password is
   correctly defined in your config.json file by issuing the
   ``kubos config`` command.

System appears to have hung

-  If for some reason file transfer fails, it can take a couple minutes
   for the connection to time out and return control.
-  If you've waited a couple minutes and the system still appears hung,
   please let us know so that we can open a bug report.

Debug Console
-------------

If the Beaglebone is correctly connected to your host computer, you should see
a /dev/ttyUSB\* device in your vagrant VM. The VM is set up to
automatically forward any FTDI cables that connect to a /dev/FTDI device
for ease-of-use.

The vagrant image comes pre-packaged with a minicom serial connection
configuration file for the board debug uart port. You can connect with
this configuration file using the command

::

    $ minicom kubos

Alternatively, you can manually create a serial connection with minicom
(or other terminal application) using the following connection
configuration:

+-------------+----------+
| Setting     | Value    |
+=============+==========+
| Baudrate    | 115200   |
+-------------+----------+
| Bits        | 8        |
+-------------+----------+
| Parity      | N        |
+-------------+----------+
| Stop Bits   | 1        |
+-------------+----------+

Once connected, you can log in using either a user that you've created,
or root, which has a default password of 'Kubos123'.

Fully logged in, the console should look like this:

::

    Welcome to KubOS Linux

    Kubos login: root
    Password: 
    Jan  1 00:00:16 login[212]: root login on 'ttyS0'
    ~ # 

Manual File Transfer
--------------------

If for some reason you want to manually transfer a specific file onto
the board, for example a custom script, you'll need to do the following:

Connect to the board through minicom (the file transfer protocol is not
guaranteed to work with any other terminal program)

::

    $ minicom kubos

Login to the board

::

    Welcome to minicom 2.7

    OPTIONS: I18n
    Compiled on Feb  7 2016, 13:37:27.
    Port /dev/FTDI, 21:26:43

    Press CTRL-A Z for help on special keys


    Welcome to KubOS Linux
    (none) login: root
    Password:
    Jan  1 00:00:11 login[210]: root login on 'ttyS0'
    ~ #

Navigate to the location you'd like the received file to go.

::

    ~ # mkdir righthere
    ~ # cd righthere
    ~/righthere #

Issue the zModem command to prep the board to receive a file

::

    $ rz -bZ

Press **Ctrl+a**, then press **s** to open the minicom file transfer
dialog.

::

    +-[Upload]--+
    | zmodem    |
    | ymodem    |
    | xmodem    |
    | kermit    |
    | ascii     |
    +-----------+

Select zmodem

::

    +-------------------[Select one or more files for upload]-------------------+
    |Directory: /home/vagrant                                                   |
    | [..]                                                                      |
    | [linux]                                                                   |
    | [newprj]                                                                  |
    | minicom.log                                                               |
    |                                                                           |
    |              ( Escape to exit, Space to tag )                             |
    +---------------------------------------------------------------------------+

                   [Goto]  [Prev]  [Show]   [Tag]  [Untag] [Okay]

Select the file to send:

Press ``g`` to open the Goto dialog and navigate to the desired folder
(full pathname required).

Press enter to open the file selector dialog and specify the file you
want within the current folder.

::

    +-------------------[Select one or more files for upload]-------------------+
    |Directory: /home/vagrant/linux/build/kubos-linux-beaglebone-gcc/source     |
    | [..]                                                                      |
    | [CMakeFiles]                                                              |
    | CMakeLists.txt                                                            |
    | CTestTestfile.cmake                                                       |
    | cmake_install.cmake                                                       |
    | linux                                                                     |
    | linux.map                                                                 |
    |                +-----------------------------------------+                |
    |                |No file selected - enter filename:       |                |
    |                |> linux                                  |                |
    |                +-----------------------------------------+                |
    |                                                                           |
    |              ( Escape to exit, Space to tag )                             |
    +---------------------------------------------------------------------------+

                   [Goto]  [Prev]  [Show]   [Tag]  [Untag] [Okay]

You should see a progress dialog as your file is transferred to the
board.

::

    +-----------[zmodem upload - Press CTRL-C to quit]------------+
    |^XB00000000000000rz waiting to receive.Sending: linux        |
    |Bytes Sent:  41984/  99084   BPS:8905     ETA 00:06          |
    |                                                             |
    |                                                             |
    |                                                             |
    |                                                             |
    |                                                             |
    +-------------------------------------------------------------+

Once file transfer is complete, you should be able to press enter and
use your new file

::

    +-----------[zmodem upload - Press CTRL-C to quit]------------+
    |^XB00000000000000rz waiting to receive.Sending: linux        |
    |Bytes Sent:  99084   BPS:7982                                |
    |                                                             |
    |Transfer complete                                            |
    |                                                             |
    | READY: press any key to continue...                         |
    |                                                             |
    +-------------------------------------------------------------+

Press **Ctrl+a**, then **q** to bring up the dialog to exit minicom. Hit
enter to quit without reset.

Example Program
---------------

Let's walk through the steps to create the example KubOS Linux project.

Initialize the project

::

    $ kubos init --linux newproj

Move into the project folder

::

    $ cd newproj

Set the project target

::

    $ kubos target kubos-linux-beaglebone-gcc

Build the project

::

    $ kubos build

Flash the project

::

    $ kubos flash

Log into the board

::

    $ minicom kubos
    Login: root/Kubos123

Run the example application

::

    $ newproj

Output should look like this:

::

    Initializing CSP
    Starting example tasks
    Ping result 80 [ms]
    Packet received on MY_PORT: Hello World
    Ping result 90 [ms]
    Packet received on MY_PORT: Hello World
    Ping result -1 [ms]
    Packet received on MY_PORT: Hello World
    Ping result 60 [ms]
    Packet received on MY_PORT: Hello World
    Ping result 50 [ms]
    Packet received on MY_PORT: Hello World

Press **Ctrl+C** to exit execution.

Press **Ctrl+A**, then **Q** to exit minicom.

Using Peripherals
-----------------

The Beaglebone Black has several different ports available for interacting 
with peripheral devices. Currently, users should interact with these 
devices using the standard Linux functions. A Kubos HAL will be added 
in the future to abstract this process.

UART
~~~~

The Beaglebone Black has 5 UART ports available for use:

+--------------+--------+--------+---------+---------+
| Linux Device | TX Pin | RX Pin | RTS Pin | CTS Pin |
+==============+========+========+=========+=========+
| /dev/ttyS1   | P9.24  | P9.26  | P9.19   | P9.20   |
+--------------+--------+--------+---------+---------+
| /dev/ttyS2   | P9.21  | P9.22  | P9.38   | P9.37   |
+--------------+--------+--------+---------+---------+
| /dev/ttyS3   | P9.42  |        | P8.34   | P8.36   |
+--------------+--------+--------+---------+---------+
| /dev/ttyS4   | P9.13  | P9.11  | P8.33   | P8.35   |
+--------------+--------+--------+---------+---------+
| /dev/ttyS5   | P8.37  | P8.38  | P8.32   | P8.31   |
+--------------+--------+--------+---------+---------+

.. note:: /dev/ttyS3 (UART3) is TX-only

Users can interact with these ports using Linux's `termios <http://man7.org/linux/man-pages/man3/termios.3.html>`__ interface.

`A tutorial on this interface can be found here <http://tldp.org/HOWTO/Serial-Programming-HOWTO/x115.html>`__

I2C
~~~

The Beaglebone Black has two user-accessible I2C buses.

+--------------+---------+---------+
| Linux Device | SCL Pin | SDA Pin |
+==============+=========+=========+
| /dev/i2c-1   | P9.17   | P9.18   |
+--------------+---------+---------+
| /dev/i2c-2   | P9.19   | P9.20   |
+--------------+---------+---------+

`I2C Standards
Doc <http://www.nxp.com/documents/user_manual/UM10204.pdf>`__

KubOS Linux is currently configured to support the I2C standard-mode
speed of 100kHz.

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
    system("echo i2cdevice 0x20 > /sys/bus/i2c/devices/i2c-1/new_device);

    /* Open I2C bus */
    file = open("/dev/i2c-1");

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
    system("echo 0x20 > /sys/bus/i2c/devices/i2c-1/delete_device);

SPI
~~~

The Beaglebone has two SPI buses available with one pre-allocated chipselect pin.

+------+-------+
| Name | Pin   |
+======+=======+
| MOSI | P9.29 |
+------+-------+
| MISO | P9.30 |
+------+-------+
| SCLK | P9.31 |
+------+-------+
| CS   | P9.28 |
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
    write(fd, strValue, 1);
    value = atoi(strValue);
    close(fd);
    
    /* Release the pin */
    fd = open("/sys/class/gpio/unexport", O_WRONLY);
    write(fd, &pin, sizeof(pin)); 
    close(fd);

.. _user-accounts-bbb:

User Accounts
-------------

In general, it is preferred to use a non-root user account to interact
with a Linux system. A default user account 'kubos' is included with
KubOS Linux. Other user accounts can be created using the standard Linux
commands (``adduser``, ``useradd``, etc).

All user accounts should have a home directory in the format
'/home/{username}'.

The ``passwd`` command can be used to change the password of existing user
accounts.

If you change the root user's password, be sure to also update the password in
any :ref:`Kubos SDK project configurations <updating-credentials>`.

KubOS Linux File System
-----------------------

There are a few key directories residing within the KubOS Linux user
space.

/home
~~~~~

All user-created files should reside under the /home directory. This
directory maps to separate partitions from the root file system. As a
result, all files here will remain unchanged if the system goes through
a kernel upgrade or downgrade.

The home directories of all user accounts, except root, should live
under this directory.

--------------

**Any files not residing under the /home directory will be destroyed
during an upgrade/downgrade** 

--------------

The Beaglebone Black has two user data partitions available, one on each storage
device. 

eMMC
^^^^

The user partition on the eMMC device is used as the primary user data storage area.
All system-related `/home/` paths will reside here.

/home/usr/bin
#############

All user-created applications will be loaded into this folder during the
``kubos flash`` process. The directory is included in the system's PATH,
so applications can then be called directly from anywhere, without
needing to know the full file path.

/home/usr/local/bin
###################

All user-created non-application files will be loaded into this folder
during the ``kubos flash`` process. There is currently not a way to set
a destination folder for the ``kubos flash`` command, so if a different
endpoint directory is desired, the files will need to be manually moved.

/home/etc/init.d
################

All user-application initialization scripts live under this directory.
The naming format is 'S{run-level}{application}'.

microSD
^^^^^^^

/home/microsd
#############

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
