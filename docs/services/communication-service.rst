Communications Service
======================

The `communications service <https://github.com/kubos/kubos/tree/master/services/communication-service>`__
provides a bridge for communicating with the satellite
in both development and flight environments. The service itself is built to work
across multiple different transports and is easily expandable to work over
additional transports in the future. Currently the communications service is
implemented in :doc:`Lua <../sdk-docs/sdk-lua>`, but in the future it will
likely be implemented in Rust.

Running
-------

This service is currently implemented in Lua. General instructions for running Lua
projects within the Kubos SDK can be found :doc:`here <../sdk-docs/sdk-lua>`.

The communication service has a single required argument - the configuration
file. It is specified like so:

::

    $ cd services/communication-service
    $ luvi-regular. -- config.toml

Configuring
-----------

The communications service is configured using a ``toml`` file which specifies
the transports used on either end of the service.

An example configuration which allows local udp services to communicate over
a dev serial port:

::

    [[communication-service]]
    name = "Local Services"
    transport = "udp"

    [[communication-service]]
    name = "Dev Serial"
    transport = "serial"
    device = "/dev/ttyUSB1"
    baud = 115200

Another example configuration which allows local udp clients to communicate
with remote services over serial:

::

    [[communication-service]]
    name = "Local Clients"
    transport = "udp"
    expose-ports = [ 6000, 7000 ]

    [[communication-service]]
    name = "Dev Serial"
    transport = "serial"
    device = "/dev/ttyUSB4"
    baud = 115200

Transports
----------

UDP
~~~

The majority of the time the communication service will be using a ``udp`` transport on one end.
This transport allows local udp-based services or clients to communicate with the service
and whatever is on the other end of the service.

The ``udp`` transport is selected by specifying ``udp`` as the transport.
There is only one available configuration option for this transport: ``expose-ports``.
The ``expose-ports`` option takes a list of ports to listen on for traffic.
This option is primarily used when clients need to send data over the udp transport,
it is not necessary when services are listening for data.

Example using ``expose-ports``

::

    [[communication-service]]
    name = "Dev Clients"
    transport = "udp"
    expose-ports = [ 8001, 8002 ]

Serial
~~~~~~

The ``serial`` transport allows routing of communication data over a local serial device.
This transport is useful when doing local development on an obc with primarily serial interfaces.

The ``serial`` transport is selected by specifying ``serial`` as the transport.
There are two available configuration options for this transport:

    - ``device`` - The path to the serial port
    - ``baud`` - The speed of serial communications

Example

::

    [[communication-service]]
    name = "Dev Serial"
    transport = "serial"
    path = "/dev/ttyUSB1"
    baud = 9600


Debug Serial
~~~~~~~~~~~~

The ``debug-serial`` transport allows routing of communication data over the debug console.
This transport is meant to be used on an embedded target with debug console and
is primarily meant to be used when no other serial ports are available.

.. note::
   This transport will take full control of the debug console. The only way to terminate it
   is by terminating the service, usually by rebooting the device.

The ``debug-serial`` transport is selected by specifying ``debug-serial`` as the transport.
There are two available configuration options for this transport:

    - ``device`` - The path to the serial port
    - ``baud`` - The speed of serial communications

Example

::

    [[communication-service]]
    name = "Dev Serial"
    transport = "debug-serial"
    path = "/dev/ttyUSB1"
    baud = 9600
