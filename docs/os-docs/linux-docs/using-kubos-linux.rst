Using Kubos Linux
=================

TODO: Review/update this doc once all the other new stuff is in place

Overview
--------

This document is intended as a general guide for creating,
loading, and using Kubos projects and other files within the user space
of Kubos Linux.

Reference Documents
-------------------

SDK Documentation
~~~~~~~~~~~~~~~~~

-  :doc:`../installation-docs/sdk-installing`
-  :doc:`../tutorials/first-project`

Board-Specific Documentation
~~~~~~~~~~~~~~~~~~~~~~~~~~~~

-  :doc:`../obc-docs/bbb/index`
-  :doc:`../obc-docs/iobc/index`
-  :doc:`../obc-docs/mbm2/index`

.. _obc-communication:

Communicating with an OBC
-------------------------

There are currently three primary methods for users to communicate directly with their OBCs:

    - Via a debug UART port
    - Via SLIP using a UART port
    - Via an ethernet port (not supported by all boards)

Debug Console
~~~~~~~~~~~~~

Each board will have some debug port available, which will then be connected
to your computer via USB. See the appropriate :ref:`Working with {board} <system-guides>` document
for more information.

If the target board is correctly connected to your host computer, you should
see a `/dev/ttyUSB\*` device in your Vagrant box. The VM is set up to
automatically forward any FTDI cables that connect to a `/dev/FTDI` device
for ease-of-use.

The Vagrant image comes pre-packaged with a minicom serial connection
configuration file. You can connect with this configuration file using the
command

::

    $ minicom kubos

Alternatively, you can manually create a serial connection with minicom
(or other terminal application) using the following connection
configuration:

+-----------+--------+
| Setting   | Value  |
+===========+========+
| Baudrate  | 115200 |
+-----------+--------+
| Bits      | 8      |
+-----------+--------+
| Parity    | N      |
+-----------+--------+
| Stop Bits | 1      |
+-----------+--------+

Once connected, you can log in using either a user that you've created,
or root, which has a default password of 'Kubos123'.

Fully logged in, the console should look like this:

::

    Welcome to Kubos Linux

    Kubos login: root
    Password: 
    Jan  1 00:00:16 login[212]: root login on 'ttyS0'
    ~ # 
    
.. warning::

    Please make sure to either logout of your board, or change it back to the
    root user's home directory before beginning any file transfer

.. _slip:
    
SLIP
~~~~

Using `SLIP <https://en.wikipedia.org/wiki/Serial_Line_Internet_Protocol>`__ over a UART port allows
users to communicate with a target OBC as though it has a normal network connection set up.
This is most useful for communicating with OBCs which do not provide an ethernet port.

All supported boards include SLIP configuration in their ``/etc/network/interfaces`` file.

In order to communicate with an OBC from the :doc:`Kubos SDK <../sdk-docs/index>`, users will need
to do the following:

- Connect an FTDI cable to the pins of the UART port (please refer to the UART section of the
  appropriate :ref:`Working with {board} <system-guides>` document for details about the default
  SLIP UART port for the board)

    - Ground -> Ground
    - TX -> RX
    - RX -> TX
    - RTS -> CTS (Might not be available on all boards/UART ports)
    - CTS -> RTS (Might not be available on all boards/UART ports)
    - Vcc -> ignore

    FTDI cables typically have the following pinout:

    +-----+--------+----------+
    | Pin | Color  | Function |
    +=====+========+==========+
    | 1   | Black  | Ground   |
    +-----+--------+----------+
    | 2   | Brown  | CTS      |
    +-----+--------+----------+
    | 3   | Red    | Vcc      |
    +-----+--------+----------+
    | 4   | Orange | TX       |
    +-----+--------+----------+
    | 5   | Yellow | RX       |
    +-----+--------+----------+
    | 6   | Green  | RTS      |
    +-----+--------+----------+

- Connect the USB portion of the FTDI cable to the host machine
- Issue ``ls /dev`` and identify the ``/dev/ttyUSB*`` device associated with the FTDI cable
- Set up the SLIP device

    - If the UART port has RTS/CTS available, issue the following::
    
        $ sudo slattach -s 115200 -p cslip {USB-device} &
        
    - Otherwise, issue this command instead::
    
        $ sudo slattach -FL -s 115200 -p cslip {USB-device} &

- Define a new network interface for the device::

    $ sudo ifconfig sl0 192.168.0.1 pointopoint 192.168.0.2 up
    
- Finally, ensure that the SLIP traffic will be routed to the SDK's host IP::

    $ sudo route add 192.168.0.1 dev lo
    
Worth noting, the baud rate, protocol, and IP addresses may all be changed.
In this case, the corresponding values in the OBC's ``/etc/network/interfaces`` file should also be
changed to match.

.. _ethernet:

Ethernet
~~~~~~~~

Some OBCs support communication via an ethernet port. Once configured, this port can be used
as an alternate method to access the board's shell interface and to transfer files.

Setup
^^^^^

Connect an ethernet cable from the board to either your computer or an open network port.

Log into the board using the debug console and then edit ``/etc/network/interfaces``.
Update the IP address field to be an address of your choosing.

Once updated, run the following commands in order to make the board use the new address::
    
    $ ifdown eth0; ifup eth0
    
The address can be verified by running the ``ipaddr`` command

Communicating via SSH
~~~~~~~~~~~~~~~~~~~~~

Once a board has been given a valid IP address (via ethernet or SLIP), you can create an SSH connection to it.

This can be done from either the SDK or your host machine.

To connect from the command line, run ``ssh kubos@{ip-address}``.
You will be prompted for the `kubos` account password.

You can also use a tool, like PuTTY, to create an SSH connection.

.. _file-transfer:

File Transfer
-------------

Once your board is connected and running you can begin transferring files
to it. There are two supported methods of file transfer: ``scp`` and ZMODEM over ``minicom``.

SCP
~~~

Transferring files using ``scp`` requires the board to have an IP
connection using either ethernet or SLIP.

Once the IP address has been set, you can transfer files to and from the stack using the ``scp`` command.
This command can be run from either the SDK or your host machine.

For example, if I wanted to send a file on my host machine, `test.txt`, to reside in the `kubos` account's home directory,
given a stack IP of ``10.50.1.10``, I would enter::

    $ scp test.txt kubos@10.50.1.10:/home/kubos
    
.. note::

    While file transfer can be done over a SLIP connection, it is significantly faster and more
    reliable when done over an ethernet connection instead (for boards which have an ethernet port
    available)

Minicom
~~~~~~~

If your board only has serial connections and there is no spare UART for SLIP,
you can still transfer files over the debug serial console using ``minicom``.

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


    Welcome to Kubos Linux
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
    |Directory: /home/vagrant/linux/build/kubos-linux-isis-gcc/source           |
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

Using Peripherals
-----------------

Each board has a variety of different ports available for interacting with
peripheral devices. Currently, users should interact with these devices
using the standard Linux functions. A Kubos HAL will be added in the
future.

Please refer to the appropriate :ref:`Working with {board} <system-guides>` document for more
information about the specific peripheral availability.

.. _user-accounts:

User Accounts
-------------

In general, it is preferred to use a non-root user account to interact
with a Linux system. A default user account 'kubos' is included with
Kubos Linux. Other user accounts can be created using the standard Linux
commands (``adduser``, ``useradd``, etc).

All user accounts should have a home directory in the format
'/home/{username}'.

The ``passwd`` command can be used to change the password of existing user
accounts.

Kubos Linux File System
-----------------------

There are a few key directories residing within the Kubos Linux user
space.

/home
~~~~~

All user-created files should reside under the /home directory. This
directory maps to a separate partition from the root file system. As a
result, all files here will remain unchanged if the system goes through
a kernel upgrade or downgrade.

The home directories of all user accounts, except root, should live
under this directory.

.. warning::

    Any files not residing under the /home directory will be destroyed
    during an upgrade/downgrade
    
/home/system/logs
^^^^^^^^^^^^^^^^^

All log files generated with rsyslog reside in this directory.

/home/system/usr/bin
^^^^^^^^^^^^^^^^^^^^

This directory is included in the system's PATH, so applications placed
here can be called directly from anywhere, without needing to know the
full file path.

/home/system/etc/init.d
^^^^^^^^^^^^^^^^^^^^^^^

All user-application initialization scripts live under this directory.
The naming format is 'S{run-level}{application}'.

Resetting the Boot Environment
------------------------------

.. note::

    This is a case which normal users should never encounter, but becomes more likely when initially testing custom Kubos Linux builds.
    Due to the blocking nature of the behavior, this information has been included in this doc in order to make it more prominent.

If the system goes through the :doc:`full recovery process <kubos-linux-recovery>` and the bootcount is still exceeded,
it will present the U-Boot CLI instead of attempting to boot into Kubos Linux again.

If this occurs, follow the :ref:`instructions for resetting the boot environment <env-reset>`.
