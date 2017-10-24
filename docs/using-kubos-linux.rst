Using KubOS Linux
=================

Overview
--------

This document is intended as an general guide for creating,
loading, and using Kubos projects and other files within the user space
of KubOS Linux.

Reference Documents
-------------------

SDK Documentation
~~~~~~~~~~~~~~~~~

-  :doc:`sdk-installing`
-  :doc:`first-linux-project`
-  :doc:`sdk-reference`

Board-Specific Documentation
~~~~~~~~~~~~~~~~~~~~~~~~~~~~

-  :doc:`working-with-the-bbb`
-  :doc:`working-with-the-iobc`
-  :doc:`working-with-the-mbm2`

Debug Console
-------------

Each board will have some debug port available, which will then be connected
to your computer via USB. See the appropriate `Working with {board}` document
for more information.

If the target board is correctly connected to your host computer, you should 
see a `/dev/ttyUSB\*` device in your vagrant VM. The VM is set up to
automatically forward any FTDI cables that connect to a `/dev/FTDI` device
for ease-of-use.

The vagrant image comes pre-packaged with a minicom serial connection
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

    Welcome to KubOS Linux

    Kubos login: root
    Password: 
    Jan  1 00:00:16 login[212]: root login on 'ttyS0'
    ~ # 
    
.. warning::

    Please make sure to either logout of your board, or change it back to the
    root user's home directory before beginning any file transfer

User Applications
-----------------

User applications are created, built, and flashed onto a target board
as Kubos SDK projects.

Building a Project
~~~~~~~~~~~~~~~~~~

Projects are created and built with the standard Kubos SDK commands.

For example:

::

    $ kubos init -l newproj
    $ kubos target kubos-linux-isis-gcc
    $ kubos build
    
.. note:: 

    You must use the ``-l|--linux`` option with ``kubos init`` in order
    to properly create a project for KubOS Linux.
    
Configuring a Project
~~~~~~~~~~~~~~~~~~~~~

Once a project has been created, it can be configured with the project's
`config.json` file. This file is not created by default, so will need
to be manually created. It will reside in the top-level directory of
the Kubos SDK project.

More information about project configuration can be found in the
:doc:`sdk-project-config` doc.

There are a few relevant configuration options for KubOS Linux:

.. _updating-credentials:

Updating Credentials
^^^^^^^^^^^^^^^^^^^^

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
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

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

Flashing an Application
~~~~~~~~~~~~~~~~~~~~~~~

The board should be connected to your computer and it should be fully powered.

.. warning::

    Please make sure to either logout of your board, or change it back to the
    root user's home directory before beginning any file transfer

Assuming you've successfully built a Kubos SDK project for the desired target, 
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

Example Program
~~~~~~~~~~~~~~~

Let's walk through the steps to create the example KubOS Linux project.

Initialize the project

::

    $ kubos init --linux newproj

Move into the project folder

::

    $ cd newproj

Set the project target

::

    $ kubos target kubos-linux-isis-gcc

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

Flashing Non-Application Files
------------------------------

.. warning::

    Please make sure to either logout of your board, or change it back to the
    root user's home directory before beginning any file transfer

If you would like to flash a file other than the application binary onto
your board, you can add an additional parameter to the usual flash
command:

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

See the :ref:`flash-troubleshooting` of our FAQ guide for troubleshooting
tips.

Manual File Transfer
--------------------

If for some reason you want to manually transfer a specific file onto
the target board, for example a custom script, you'll need to do the following:

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

Please refer to the appropriate `Working with {board}` document for more
information about the specific peripheral availability.

.. _user-accounts:

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
directory maps to a separate partition from the root file system. As a
result, all files here will remain unchanged if the system goes through
a kernel upgrade or downgrade.

The home directories of all user accounts, except root, should live
under this directory.

--------------

**Any files not residing under the /home directory will be destroyed
during an upgrade/downgrade** 

--------------

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
