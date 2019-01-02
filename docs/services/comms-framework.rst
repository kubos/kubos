Communications Service
======================

The hardware used to establish communication between a satellite and the ground varies wildly from
mission to mission.
As a result, the communications service provided by KubOS is not a true service.
Instead, it is a framework which can be used to establish communications on a per-mission basis.

Architecture
------------

Data Packets
~~~~~~~~~~~~

All packets sent to/from the communication device will be encapsulated in several layers.

The first layer will be whatever communication protocol the device requires.
For example, AX.25 is frequently used as the header protocol for radio communication.

Inside of this will be a UDP packet, most likely containing a GraphQL request intended for one of
the KubOS or payload services.

TODO: Packet diagram?

Ground Communication
~~~~~~~~~~~~~~~~~~~~

The communications service maintains a constant read thread which listens for messages from the
ground via the primary communications device (ex. radio).

Once a message is received, a message handler thread is spawned and assigned one of the available
handler UDP ports.
This message handler examines the message to determine the internal message destination and then
forwards it on to the appropriate service.
The handler then waits for a reply (within a specified timeout duration), wraps the response in a
UDP packet, and then sends the packet to the primary radio for transmission.

.. figure:: ../images/comms_from_ground.png
    :align: center

Downlink Endpoints
~~~~~~~~~~~~~~~~~~

There are some instances where the satellite might need to send a message to the ground without
first receiving a request for data.
An example would be when sending a periodic health-and-status beacon.

In this case, the communications service may be defined with a downlink endpoint thread (or multiple
threads if more than one radio is available for downlink communication).

Each endpoint is assigned its own UDP port and maintains a constant read thread which listens for
messages from within the satellite which should be transmitted.

When the endpoint's read thread receives a message, it wraps it up in a UDP packet and then sends
it to the endpoint device, via the :doc:`appropriate hardware API <../apis/device-api/index>`.

.. figure:: ../images/comms_to_ground.png
    :align: center

Configuration
-------------

Most missions tend to have a single communications device which communicates with a single end-point
on the ground.
However, some missions have more complex communications systems; for example, having one radio for
transmitting a small health-and-status beacon and then a second radio for transmitting more detailed
telemetry information.

The communications framework provides mechanisms to handle these more complex configurations.

The service's config.toml file should contain the following parameters:

- ``handler_port_min`` - Starting port used to define a range of ports that are used in the message
  handlers that handle messages received from the ground
- ``handler_port_max`` - Ending port used to define a range of ports that are used in the message
  handlers that handle messages received from the ground
- ``downlink_ports`` - (Optional) List of ports used by downlink endpoints that send messages to the
  ground each port in the list will be used by one downlink endpoint
- ``timeout`` - Length of time a message handler should wait for a reply, in milliseconds
- ``ground_ip`` - IP address of the ground gateway
- ``ground_port`` - UDP port of the ground gateway
- ``satellite_ip`` - IP address of the communications service

Implementation
--------------

Because communication methods may vary from mission to mission, it is up to the user to create the
actual communications service which will be used

Please refer to the `communications service's Rust documentation <TODO>` for specific implementation
details.