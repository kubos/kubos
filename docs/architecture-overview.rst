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

Communication and KubOS
-----------------------

Universally, the method of communication is UDP. This includes onboard and the space/ground link. The rest of this overview is broken into onboard and space/ground sections to give an example of what to expect from these communication mediums. These descriptions will not cover every possible use case, but you can also come `talk to us on Slack <https://slack.kubos.co/>`__ if you have addition cases you would like to know about.

Onboard Communication
~~~~~~~~~~~~~~~~~~~~~

Onboard the spacecraft, most communication is centered around mission applications. Mission applications use Graphql over UDP for controlling hardware services and payloads to change the state of the spacecraft or execute operations. Mission applications get all of their data directly from the hardware services, to ensure they have the most up-to-date information to make decisions. Typically, a telemetry application will fulfill the role of polling all the services to generate the health and status beacon and log data into the telemetry database. There is no other onboard communication that is required by the KubOS system.

Space/Ground Communication
~~~~~~~~~~~~~~~~~~~~~~~~~~

As stated before, all communication is UDP. The way this is acheived for the space/ground link is through the communication service. This service provides UDP passthrough so the details of the radio link packetization, the encryption, etc. are transparent to the onboard services. See the :doc:`service documentation <services/communication-service>` for more details of how this is accomplished. The usage of this UDP passthrough over the space/ground link is governed by two major use cases: nominal operations and error recovery. Since these are so drastically different they have been broken out individually.

Nominal Operations
^^^^^^^^^^^^^^^^^^

For nominal operations, the user will issue GraphQL commands to the mission application service with arguments that will cause the mission application to execute a desired operation or state change. The application service will run the application with the arguments and return the state information of the application (similar to an ACK, but with more information). For an example of this commanding process we can look at the case of an imaging satellite. You could have a mission application entitled "image target" that you pass the lat/long to through a GraphQL mutation to the mission application service. This "image target" application will then read its position data from the GPS service, command the ADCS to track the target, and command the imager to take the picture when it reaches the appropriate time.

So far, this only covers commanding the satellite. For downlinking data, there are 3 major methods that are employed: health and status beacon, telemetry database queries, and file transfer. The health and status beacon is constantly being output by the telemetry application, and covers the high level state information of the spacecraft. Telemetry database queries are issued to collect data logged between passes, more detailed data on certain subsystems, or general data for storage on the ground (if the link budget allows). Lastly, file transfers are the primary method for payload data to be transferred to ground. For example, in the example above for the imaging spacecraft, the user would receive the status of the operation from the health and status beacon, issue a command to the file transfer service to downlink the resulting image, and issue a telemetry query to get the temperature/power measurements collected during the operation time window.

These are just examples of nominal communication. The core function of the communication service is providing a UDP passthrough, so a mission operator or flight software developer can really use it in any way they see fit.

In the case provided, this mode's link is generally used for:

- Mission application service commands
- Health and status beacon
- Telemetry database queries
- File transfers of payload data

Error Recovery
^^^^^^^^^^^^^^

The other major use case is recovery. KubOS was designed to make recovery as easy, safe, and powerful as possible. When the satellite experiences an error or problem that the automatic recovery methods cannot handle, manual diagnosis and recovery might be necessary. We empower the mission operator to have as many tools as possible. In an error recovery situation, the primary link usage would most likely come from the shell and file services. The shell service provides complete terminal access to the satellite and the file transfer service allows the transfer of new applications/services/binaries/images to the satellite.

In addition to these tools, each of the hardware service endpoints are also available to be queried/commanded directly from ground. Since they are GraphQL UDP endpoints, the mission control software can directly access them to take immediate action if necessary, so only a single command needs to get through to make drastic changes to recover the satellite. In fact, the mission control software would be accessing the hardware in the same way as the mission application does locally, so it can exercise the same level of control.

Overall this mode's link is generally used for:

- Shell service terminal command/terminal output
- File service uploads to update onboard software
- Telemetry database queries to diagnose what happened
- Direct hardware queries and mutations and their responses
- Health and status beacon

Available Languages in KubOS
----------------------------

The primary languages used in KubOS are Rust and Python. C is also used, but only for the bootloader, kernal, and some APIs.

 - :doc:`Rust <sdk-docs/sdk-rust>` is the primary language for the :ref:`services <rust-service-ref>`
 - :doc:`Python <sdk-docs/sdk-python>` is used for mission applications and :ref:`some services <python-service-ref>`

Rust and Python are used to create services and applications within KubOS. C is only used for linux and lower level functionality.
Other languages can also be easily supported, make sure to `talk to us <https://slack.kubos.co/>`__ if there is another language you'd like to use!

.. Note::
    C is able to be used to create services and applications within the system. We do not list it as an "available language" because we do not include pre-built tools for KubOS specific tasks like we do in Python and Rust. We assume that if you are comfortable enough to build your application or service in C, you can handle the required interactions. We would highly recommend talking to us about the implications before beginning.
