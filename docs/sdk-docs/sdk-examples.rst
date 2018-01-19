Example Kubos Projects
======================

We have provided a variety of example applications to help you get started with your Kubos project.
These examples are located in the 'Examples' folder of the `Kubos repo <http://github.com/kubos/kubos/tree/master/examples>`__, 
as well as within the `/home/vagrant/.kubos/kubos/examples` folder of the Kubos SDK box.

Using an Example Application
----------------------------

Each of the example applications contains the files necessary to run as an independent Kubos project. 
This means you can simply copy the desired example to a new folder and immediately build it with the ``kubos build`` command.

"Compatible Targets" indicates which target boards for which the application should execute successfully without modification.

.. note:: 

    The default target for all of these applications is ``stm32f407-disco-gcc``. 
    You will need to manually change the target if this is not your desired endpoint device. 
    
Kubos Linux Examples
--------------------

Default Example
~~~~~~~~~~~~~~~

`Example Code - GitHub <http://github.com/kubos/kubos/tree/master/examples/kubos-linux-example>`__

**Compatible Targets: All Kubos Linux compatible targets**

+----------------------+------------------------+
| High-level Component | Specific Area          |
+======================+========================+
| Kubos CSP [4]_       | sockets, ping, threads |
+----------------------+------------------------+

This is the default application included when the ``kubos init --linux`` command is executed.
It is intended as a basic example of how an application could be written to run on a Kubos Linux system.

The application contains a CSP interaction between client and server tasks.

Every 200ms, the CSP client thread pings the CSP server's address to see if it is available, and then connects and sends a CSP 
packet containing the message "Hello World".

The CSP server thread checks for connections on the CSP port and then prints any received messages to STDOUT.    

TCP Receive
~~~~~~~~~~~

`Example Code - GitHub <http://github.com/kubos/kubos/tree/master/examples/kubos-linux-tcprx>`__

**Compatible Targets: Pumpkin MBM2, Beaglebone Black**

+----------------------+------------------------+
| High-level Component | Specific Area          |
+======================+========================+
| Linux                | sockets, TCP, IPv4     |
+----------------------+------------------------+

This is a demo program to test receiving TCP data over a valid IP connection (the ethernet port for the Pumpkin MBM2 and Beaglebone 
Black targets)

The program will wait for a client to connect over the socket, then read in any messages and send back a reply.

TCP Send
~~~~~~~~

`Example Code - GitHub <http://github.com/kubos/kubos/tree/master/examples/kubos-linux-tcprx>`__

**Compatible Targets: Pumpkin MBM2, Beaglebone Black**

+----------------------+------------------------+
| High-level Component | Specific Area          |
+======================+========================+
| Linux                | sockets, TCP, IPv4     |
+----------------------+------------------------+

This is a demo program to test sending TCP data over a valid IP connection (the ethernet port for the Pumpkin MBM2 and Beaglebone Black 
targets)

The program takes the IP address and port to send to as input parameters, then sends a test message to the requested end point.
It then waits for a reply message to be returned and exits.

    Usage: kubos-linux-tcptx <ip_addr> <port>

UART Receive
~~~~~~~~~~~~

`Example Code - GitHub <http://github.com/kubos/kubos/tree/master/examples/kubos-linux-uartrx>`__

**Compatible Targets: All Kubos Linux compatible targets**

+----------------------+-------------------------------------------------------------------+
| High-level Component | Specific Area                                                     |
+======================+===================================================================+
| Linux                | `termios <http://man7.org/linux/man-pages/man3/termios.3.html>`__ |
+----------------------+-------------------------------------------------------------------+

This is a demo program to test receiving UART data in non-blocking mode as an interrupt. It expects to read the incrementing message 
"Test message nnn" every 5 seconds from `/dev/ttyS1`.

This program should be paired with the UART Send demo program.

UART Send
~~~~~~~~~

`Example Code - GitHub <http://github.com/kubos/kubos/tree/master/examples/kubos-linux-uarttx>`__

**Compatible Targets: All Kubos Linux compatible targets**

+----------------------+-------------------------------------------------------------------+
| High-level Component | Specific Area                                                     |
+======================+===================================================================+
| Linux                | `termios <http://man7.org/linux/man-pages/man3/termios.3.html>`__ |
+----------------------+-------------------------------------------------------------------+

This is a demo program to test UART transmission. It will write an incrementing message "Test message nnn" every 5 seconds out of `/dev/ttyS3`.

This program should be paired with the UART Receive demo program.
    
References
----------

.. [1] :doc:`config.json <sdk-project-config>` - Kubos project file for custom configuration options
.. [3] :doc:`Kubos HAL <../apis/kubos-hal/index>` - Kubos hardware abstraction for interfacing with peripheral devices
.. [4] :doc:`CSP <../apis/libcsp/index>` - (Cubesat Space Protocol) Lightweight communication protocol

.. todo::

    .. [*] `Kubos Slash`
