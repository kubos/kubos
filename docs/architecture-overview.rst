KubOS Architecture Overview
===========================

The KubOS system is designed to take care of every aspect of the satellite's flight software.  


The KubOS Stack
---------------

.. figure:: images/architecture_stack.png
    :align: center


OBC (On Board Computer)
~~~~~~~~~~~~~~~~~~~~~~~

Starting from the bottom, the OBC is up to you, as long as it is in our :ref:`supported list <supported-boards>`.
We are continually working to support new platforms, so make sure you `talk to us <https://slack.kubos.co/>`__ if your OBC is not included! 

Kubos Linux
~~~~~~~~~~~

Kubos Linux is Kubos's Linux distro. 

- :doc:`Kubos Linux docs <os-docs/index>`

Kubos APIs
~~~~~~~~~~

Kubos offers a library of APIs to simplify the process of writing mission applications and services.
It includes software abstractions for communication protocols, like UART and I2C, as well as hardware abstractions for devices like the `ISIS iMTQ <https://www.isispace.nl/product/isis-magnetorquer-board/>`__

- :doc:`Kubos API Documenation <apis/index>`

Kubos Services
~~~~~~~~~~~~~~

A Kubos service is defined as any persistent process that is used to interact with the satellite.
Services rarely make decisions, but will allow the user to accomplish typical flight software tasks such as telemetry storage, file management, shell access, and hardware interaction. 

- :doc:`Kubos Services <services/index>`

Mission Applications
~~~~~~~~~~~~~~~~~~~~

Mission applications are anything that governs the behavior of the satellite.
Some common examples are deployment, housekeeping, and telemetry beaconing.
Basically anything that you want the satellite to do autonomously goes into this category.

- :doc:`Mission Applications <app-docs/index>`


Typical Mission Architecture
----------------------------

.. figure:: images/mission_diagram.png
    :align: center

In the above diagram, everything in blue is typically developed by Kubos, while everything in green would be mission code, written by the user for their specific mission and payload. 

Gateway
~~~~~~~

A gateway is any hardware device that provides an external communication mechanism for the satellite.
This ranges from a desktop serial connection to an in-flight radio link through a ground station.

Gateway API
~~~~~~~~~~~

The gateway API provides a simplified software interface to be used by the communiction service.

Communication Service
~~~~~~~~~~~~~~~~~~~~~

The communication service processes all communication received from the gateway and prepares it for use by the rest of the satellite system.

Similarly, any messages which need to be sent out of the satellite are processed by the service and then sent through the gateway.

Linux IP Stack
~~~~~~~~~~~~~~

All services use IP in order to communicate with each other. This infrastructure is referred to as the "IP stack".

Hardware Services
~~~~~~~~~~~~~~~~~

Hardware services are GraphQL server endpoints that take in queries and mutations and exercise the hardware API to complete them. 

 - :doc:`Hardware Services <services/hardware-services>`
 - :doc:`GraphQL <services/graphql>`

Hardware APIs
~~~~~~~~~~~~~

Hardware APIs abstract away the particular commands and steps required to operate the device, resulting in smaller, more maintainable code.

 - :doc:`Hardware APIs <apis/device-api/index>`

Core Services
~~~~~~~~~~~~~

Core services are all the services that provide critical flight software capability.
Any service that does not interact with hardware and is not specific to a mission falls within this category.

- :doc:`Core Services <services/core-services>`

Payload Services
~~~~~~~~~~~~~~~~

Payload services are hardware services which implement custom code in order to accomplish mission-specific goals.
Payload services should be modeled after hardware services as much as possible. 
That being said, the payload service is custom for the mission, and can be accomplished any way the payload developer sees fit. 

 - :doc:`Payload Services <services/payload-services>`

Mission Applications
~~~~~~~~~~~~~~~~~~~~

Mission applications, as previously discussed, handle all the onboard decision making.
These are, by nature, mission specific, but some of them can be largely reused due to the abstract nature of the hardware integration.
These are typically written or adapted by the user. 

 - :doc:`Mission Applications <app-docs/index>`


Available Languages in KubOS
----------------------------

The primary languages used in KubOS are Rust, Python, and C. 

 - :doc:`Rust <sdk-docs/sdk-rust>` is the primary language for the :ref:`services <rust-service-ref>`
 - :doc:`Python <sdk-docs/sdk-python>` is used for mission applications and :ref:`some services <python-service-ref>` 
 - C is used for everything else (kernel, bootloader, most APIs, etc)

Each language can be used to create projects, services, and applications within KubOS.
Other languages can also be easily supported, make sure to `talk to us <https://slack.kubos.co/>`__ if there is another option you'd like to use!
