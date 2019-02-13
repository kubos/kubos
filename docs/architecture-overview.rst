KubOS Architecture Overview
===========================

The KubOS system is designed to take care of every aspect of a satellite's flight software.


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

Mission applications are anything that governs the behavior of the satellite.
They govern state management, accomplish scripted tasks, monitor onboard behavior, and generally are the brains of the system.
Each application is typically dedicated to a certain mode or isolated task the satellite is supposed to accomplish to keep them lightweight and portable.
They can be simple, such as a telemetry beacon app, or complex, such as a payload operations app.

- :doc:`Mission Applications <app-docs/index>`


Typical Mission Architecture
----------------------------

.. figure:: images/mission_diagram.png
    :align: center

In the above diagram, everything in blue/purple is developed by Kubos.
Everything in red would be mission code, written by the user for their specific mission and payload.
(Hardware services can fall into the user category if they are not integrated into the system already.)

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

Hardware services are GraphQL server endpoints that take in queries and mutations and exercise the hardware API to complete them.
Typically, there is an accompanying hardware API to allow for easy interaction with the hardware.

 - :doc:`Hardware Services <services/hardware-services>`
 - :doc:`GraphQL <services/graphql>`
 - :doc:`Hardware APIs <apis/device-api/index>`

Core Services
~~~~~~~~~~~~~

Core services are all the services that provide critical flight software capability.
Any service that does not interact with hardware and is not specific to a mission falls within this category.
Currently, :doc:`core services <services/core-services>` include:

- Shell service
- Telemetry database service
- File transfer service
- Process monitoring service
- Application service/registry

Payload Integration
~~~~~~~~~~~~~~~~~~~

The payload integration block denotes any software used to integrate payload hardware into KubOS.
Payload hardware can be integrated in any way desired by the user to accomplish mission goals.
If possible, payload services should be modeled after hardware services to simplify the interface with the mission application.
The documentation we have provided shows how to make a payload service mirror a hardware service:

 - :doc:`Payload Services <services/payload-services>`

Mission Applications
~~~~~~~~~~~~~~~~~~~~

Mission applications, as previously discussed, handle all the onboard decision making.
These are, by nature, mission specific, but some of them can be largely reused due to the abstract nature of the hardware integration.
These are typically written or adapted by the user and are the backbone of the operation of the satellite.
It is highly recommended to read more in depth on them to truly understand KubOS.

 - :doc:`Mission Applications <app-docs/index>`

Communication and KubOS
-----------------------

HTTP is used for most internal communication.
UDP is used for the remaining onboard communication as well as over the space/ground link.
The rest of this overview is broken into onboard and space/ground sections to give an example of what to expect from these communication mediums.
These descriptions will not cover every possible use case, but you can also come `talk to us on Slack <https://slack.kubos.co/>`__ if you have additional cases you would like to know about.

Onboard Communication
~~~~~~~~~~~~~~~~~~~~~

Onboard the spacecraft, most communication is centered around mission applications.
Mission applications use :doc:`Graphql <services/graphql>` over HTTP for controlling hardware services and payloads to change the state of the spacecraft or execute operations.
Mission applications get all of their data directly from the hardware services, to ensure they have the most up-to-date information to make decisions.
Typically, a telemetry application will fulfill the role of polling all the services to generate the health and status beacon and log data into the :doc:`telemetry database. <services/telemetry-db>`
There is no other onboard communication that is required by the KubOS system.

Space/Ground Communication
~~~~~~~~~~~~~~~~~~~~~~~~~~

The communication service is responsible for maintaining the space/ground link.
This service provides packet passthrough so the details of the radio link (packetization, the encryption, etc.) are transparent to the onboard services.

The usage of this packet passthrough over the space/ground link is governed by two major use cases: nominal operations and error recovery.

Nominal Operations
^^^^^^^^^^^^^^^^^^

In day-to-day operations, the space/ground link will most commonly be used for a few different purposes:

- Executing :doc:`mission applications <app-docs/index>` on-demand. For instance, triggering a mission application which orients an imaging device to the requested coordinates and takes a picture.
- Automatically sending and receiving health and status information (health and status beacon).
- Querying the :doc:`telemetry database <services/telemetry-db>` for specific hardware status information.
- Downloading payload data files through the :doc:`file transfer service <services/file>`.

These are just examples of nominal communication.
The core function of the communication service is providing a packet passthrough, so a mission operator or flight software developer can really use it in any way they see fit.

Error Recovery
^^^^^^^^^^^^^^

KubOS was designed to make recovery as easy, safe, and powerful as possible.
When the satellite experiences an error or problem that the automatic recovery methods cannot handle, manual diagnosis and recovery might be necessary.
We empower the mission operator to have as many tools as possible:

- The :doc:`shell service <services/shell>` provides complete terminal access to the satellite
- The :doc:`file transfer service <services/file>` allows corrected versions of the software to be uploaded and installed in the satellite
- Each :doc:`hardware service <services/hardware-services>` endpoint can be directly queried/commanded to gather specific debugging data or control hardware, bypassing the core services

Available Languages in KubOS
----------------------------

The primary languages used in KubOS are Rust and Python.

 - :doc:`Rust <sdk-docs/sdk-rust>` is the primary language for the :ref:`services <rust-service-ref>` and mission applications.
 - :doc:`Python <sdk-docs/sdk-python>` is used for easier development of mission applications and :ref:`some services <python-service-ref>`

Other languages (for example, C and C++) are compatible with KubOS, but are not currently directly supported.
C is already used with KubOS for Linux and lower level functionality.
Make sure to `talk to us <https://slack.kubos.co/>`__ if there is another language you'd like to use, as many are easily able to be used within KubOS!
