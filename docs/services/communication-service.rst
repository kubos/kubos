Communications Service
======================

The `communications service <https://github.com/kubos/kubos/tree/master/services/communication-service>`__
provides a bridge for communicating with the satellite
in both development and flight environments. The service itself is built to work
across multiple different transports and is easily expandable to work over
additional transports in the future. Currently the communications service is
implemented in :doc:`Lua <../sdk-docs/sdk-lua>`, but in the future it will
likely be implemented in Rust.

The communication service functions essentially as a UDP tunnel between various
physical transports. It is intended to always be used with one UDP transport
and one non-UDP transport. Currently there are two serial-based transports
available, with radio-based transports in the works.

A typical production setup would involve one communication service instance
running on the flight OBC and one instance running on the ground segment.
Each instance would use a UDP transport in order to communicate with
local services or clients. The OBC's instance would use a transport
custom to the radio gateway. The ground's instance would also use
a custom transport designed for the ground side of the gateway.

.. uml::

   @startuml

   title "Comments - Sequence Diagram"

   participant "UDP Transport" as h_udp
   participant "Ground Communication Service" as host
   participant "External Gateway" as ext
   participant "OBC Communication Service" as obc
   participant "UDP Transport" as o_udp

   h_udp -> host : Client Request (UDP Packet)
   host -> ext : Encoded Message
   ext -> obc : Encoded Message
   obc -> o_udp : Decoded Client Request (UDP Packet)

   o_udp -> obc : Service Response (UDP Packet)
   obc -> ext : Encoded Message
   ext -> host : Encoded Message
   host -> h_udp : Decoded Service Response (UDP Packet)

   @enduml

Running
-------

This service is currently implemented in Lua. General instructions for running Lua
projects within the Kubos SDK can be found :doc:`here <../sdk-docs/sdk-lua>`.

The communication service has a single required argument - the configuration
file. It is specified like so:

::

    $ cd services/communication-service
    $ luvi-regular . -- config.toml

Configuring
-----------

The communications service is configured using a ``toml`` file which specifies
the transports used on either end of the service.

An example configuration which allows local UDP services to communicate over
a dev serial port:

::

    [[communication-service]]
    name = "Local Services"
    transport = "udp"

    [[communication-service]]
    name = "Dev Serial"
    transport = "serial"
    device = "/dev/ttyS3"
    baud = 115200

Another example configuration which allows local UDP clients to communicate
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

TODO write blurb

UDP
~~~

The majority of the time the communication service will be using a ``UDP`` transport on one end.
This transport allows the communication service to send or receive raw UDP packets
on the local network.

Configure the service to use this transport by specifying ``udp``.
There is a single optional configuration option for this transport: ``expose-ports``.
The ``expose-ports`` option takes a list of ports to listen on for traffic.
This option is used when clients need to send data over the UDP transport;
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
This transport is useful when doing local development on an OBC with primarily serial interfaces.

Configure the service to use this transport by specifying ``serial``.
There are two required configuration options for this transport:

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
This transport is meant to be used on an embedded target with a debug console and
is primarily meant to be used when no other serial ports are available.

.. note::
   When using this transport the service *must* be run while on the debug console.
   The service will take full control of the debug console once started.
   The only way to step it is by terminating the service, usually by rebooting the device.

Configure the service to use this transport by specifying ``debug-serial``.
There is a single required configuration option for this transport:

    - ``baud`` - The speed of serial communications

Example

::

    [[communication-service]]
    name = "Dev Serial"
    transport = "debug-serial"
    baud = 9600
