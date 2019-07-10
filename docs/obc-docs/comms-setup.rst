Communicating with an OBC
=========================

There are currently three primary methods for users to communicate directly with their OBCs:

    - Via a debug UART port
    - Via SLIP using a UART port
    - Via an ethernet port (not supported by all boards)

Debug Console
-------------

Each board will have some debug port available, which will then be connected to your computer via
USB.
See the appropriate `KubOS for the {OBC} > Working with the {OBC}` document for more information.

Running Locally
~~~~~~~~~~~~~~~

If you are not using the Kubos SDK image and are instead running everything locally, you will need
to manually set up a serial connection.

Connect your OBC to your host computer and then determine the device name which it was assigned.
It will likely be a ``/dev/ttyUSB*`` device for Linux users, a ``dev/tty.usb*`` device for OSX
users, and a ``COM*`` device for Windows users.

Open your terminal application of choice (PuTTY, minicom, etc) and set up a connection with the
following configuration values:

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

Minicom
^^^^^^^

In case you have not ever set up a serial connection to a device before, here are the instructions
to set up a connection using `minicom <https://en.wikipedia.org/wiki/Minicom>`__:

- Execute this command: `minicom -s`
- The "configuration" menu will be displayed. Use the arrow keys to navigate to "Serial port setup"
  and press Enter
- Press 'A' to navigate to the "Serial Device" field. Update the field with the serial debug device
  name. Press Enter to apply the change
- Press 'F' to toggle "Hardware Flow Control" to "No"
- Press Enter to exit this submenu
- Navigate down to "Exit" (not "Exit from Minicom"!) and press Enter to exit the configuration menu
  and start the serial connection

Using the SDK
~~~~~~~~~~~~~

If the target board is correctly connected to your host computer, you should see a `/dev/ttyUSB\*`
device in your SDK instance.
The VM is set up to automatically forward any FTDI cables that connect to a `/dev/FTDI` device for
ease-of-use.

The Vagrant image comes pre-packaged with a minicom serial connection configuration file.
You can connect with this configuration file using this command::

    $ minicom kubos

Logging In
~~~~~~~~~~

Once connected, you can log in using either a user that you've created, the default `kubos` user,
or `root`. The `kubos` and `root` users both have a default password of 'Kubos123'.

Fully logged in, the console should look like this:

::

    Welcome to Kubos Linux

    Kubos login: kubos
    Password: 
    /home/kubos # 

.. _ethernet:

Ethernet
--------

Some OBCs support communication via an ethernet port. Once configured, this port can be used
as an alternate method to access the board's shell interface and to transfer files.

Determining a Suitable IP Address
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

KubOS does not use DHCP, so all IP addresses are statically defined.

.. warning::

    All OBCs which support ethernet connections come with a predefined IP address.
    It is very likely that this default address will be incompatible with whatever network the OBC
    is connected to.
    As a result, you will need to do some research in order to determine a suitable IP address for
    your OBC.

If you are connecting your OBC to a network switch (which gives it the ability to access
the internet and potentially be accessed remotely), find the IP address your host machine is using
for the same network, and then pick a different host number (the last number in the IP address).

.. note::

    If you are connecting your OBC to a large corporate network, you might need to contact your IT
    department in order to be assigned an IP address.
    
If you are connecting your OBC directly to your host computer's ethernet port, you'll need to first
determine and/or set the IP address of the host computer for that connection.

Commands like ``ifconfig`` (or ``ipconfig`` for Windows users) should display the current address of
the ethernet connection. Worth noting, it may be necessary to first connect the OBC to the host
computer in order for the connection to appear in the command output.

Please consult your relevant OS' documentation for instructions on setting a static IP address.

Configuring the OBC
~~~~~~~~~~~~~~~~~~~

Connect an ethernet cable from the OBC to either your computer or an open network port.

Log into the OBC using the debug console and then edit ``/etc/network/interfaces``.
Update the IP address field to be an address of your choosing.

Once updated, run the following commands in order to make the OBC use the new address::
    
    $ ifdown eth0; ifup eth0
    
The address can be verified by running the ``ipaddr`` command

.. _slip:
    
SLIP
----

Using `SLIP <https://en.wikipedia.org/wiki/Serial_Line_Internet_Protocol>`__ over a UART port allows
users to communicate with a target OBC as though it has a normal network connection set up.
This is most useful for communicating with OBCs which do not provide an ethernet port.

All supported boards include a SLIP configuration in their ``/etc/network/interfaces`` file.

In order to communicate with an OBC from the :doc:`Kubos SDK <../sdk-docs/index>`, or from a host
machine running Linux, users will need to do the following:

- Connect an FTDI cable to the pins of the UART port (please refer to the UART section of the
  appropriate `KubOS for the {OBC} > Working with the {OBC}` document for details about the default
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


Communicating via SSH
---------------------

Once a board has been given a valid IP address (via ethernet or SLIP), you can create an SSH connection to it.

This can be done from either the SDK or your host machine.

To connect from the command line, run ``ssh kubos@{ip-address}``.
You will be prompted for the `kubos` account password.

You can also use a tool, like PuTTY, to create an SSH connection.

.. _file-transfer:

File Transfer
~~~~~~~~~~~~~

Once your board is connected and running you can begin transferring files
to it. There are two supported methods of file transfer: ``scp`` and ZMODEM over ``minicom``.

SCP
---

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
-------

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