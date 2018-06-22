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

Kubos Services
~~~~~~~~~~~~~~

A Kubos service is defined as any persistent process that is used to interact with the satellite.
Services rarely make decisions, but will allow the user to accomplish typical flight software tasks such as telemetry storage, file management, shell access, and hardware interaction.

- :doc:`Kubos Services <services/index>`

Mission Applications
~~~~~~~~~~~~~~~~~~~~

Mission applications are anything that governs the behavior of the satellite. They govern state management, accomplish scripted tasks, monitor onboard behavior, and generally are the brains of the system. Each application is typically dedicated to a certain mode or isolated task the satellite is supposed to accomplish to keep them lightweight and portable. They can be simple, such as a telemetry beacon app, or complex, such as a payload operations app.

- :doc:`Mission Applications <what-is-a-mission-application>`


Typical Mission Architecture
----------------------------

.. figure:: images/mission_diagram.png
    :align: center

In the above diagram, everything in blue/purple is developed by Kubos's support system, while everything in red would be mission code, written by the user for their specific mission and payload. (Hardware services can be an exception, if they are not integrated into the system already)

Gateway
~~~~~~~

A gateway is any hardware device or group of devices that provides an external communication mechanism for the satellite.
This ranges from a desktop serial connection to an in-flight radio link through a ground station.

Communication Service
~~~~~~~~~~~~~~~~~~~~~

The communication service processes all communication received from the gateway and prepares it for use by the rest of the satellite system.

Similarly, any messages which need to be sent out of the satellite are processed by the service and then sent through the gateway.

Hardware Services
~~~~~~~~~~~~~~~~~

Hardware services are GraphQL server endpoints that take in queries and mutations and exercise the hardware API to complete them. Typically, there is an accompanying hardware API to allow for easy interaction with the hardware.

 - :doc:`Hardware Services <services/hardware-services>`
 - :doc:`GraphQL <services/graphql>`
 - :doc:`Hardware APIs <apis/device-api/index>`

Core Services
~~~~~~~~~~~~~

Core services are all the services that provide critical flight software capability.
Any service that does not interact with hardware and is not specific to a mission falls within this category.
Currently, core services include: shell service, telemetry database service, file transfer service, process monitoring service, and the application service/registry.

- :doc:`Core Services <services/core-services>`

Payload Integration
~~~~~~~~~~~~~~~~~~~

Payloads can be integrated in any way desired by the user to interact with their payload hardware to accomplish mission goals.
If possible, payload services should be modeled after hardware services to simplify the interface with the mission application.

 - :doc:`Payload Services <services/payload-services>`

Mission Applications
~~~~~~~~~~~~~~~~~~~~

Mission applications, as previously discussed, handle all the onboard decision making.
These are, by nature, mission specific, but some of them can be largely reused due to the abstract nature of the hardware integration.
These are typically written or adapted by the user.
These truly are the backbone of the operation of the satellite, and it is highly recommended to read more in depth on them to truly understand KubOS.

 - :doc:`Mission Applications <what-is-a-mission-application>`


Available Languages in KubOS
----------------------------

The primary languages used in KubOS are Rust and Python. C is also used, but only for the bootloader, kernal, and some APIs.

 - :doc:`Rust <sdk-docs/sdk-rust>` is the primary language for the :ref:`services <rust-service-ref>`
 - :doc:`Python <sdk-docs/sdk-python>` is used for mission applications and :ref:`some services <python-service-ref>`

Rust and Python are used to create services and applications within KubOS. C is only used for linux and lower level functionality.
Other languages can also be easily supported, make sure to `talk to us <https://slack.kubos.co/>`__ if there is another language you'd like to use!

.. Note::
    C is able to be used to create services and applications within the system. We do not list it as an "available language" because we do not include pre-built tools for KubOS specific tasks like we do in Python and Rust. We assume that if you are comfortable enough to build your application or service in C, you can handle the required interactions. We would highly recommend talking to us about the implications before beginning.
