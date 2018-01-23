KubOS Architecture Overview
===========================

The KubOS system is designed to take care of every aspect of the satellite's flight software.  


The KubOS Stack
---------------

.. figure:: images/architecture_stack.png
    :align: center


OBC (On Board Computer)
~~~~~~~~~~~~~~~~~~~~~~~

Starting from the bottom, the OBC is up to you, as long as it is in our supported list. The current list of supported OBCs can be found :doc:`on our main docs page <index>`. We are also continually working to support new platforms, so make sure you `talk to us <https://slack.kubos.com/>`__ if your OBC is not included! 

Kubos Linux
~~~~~~~~~~~

Kubos Linux is Kubos's Linux distro. 

- :doc:`Kubos Linux docs <os-docs/index>`

Kubos APIs
~~~~~~~~~~

Kubos offers a library of APIs to simplify the process of writing mission applications and services. It includes software abstractions for communication protocols, like UART and I2C, as well as hardware abstractions for devices like the `ISIS iMTQ <https://www.isispace.nl/product/isis-magnetorquer-board/>`__

- :doc:`Kubos API Documenation <apis/index>`

Kubos Services
~~~~~~~~~~~~~~

Kubos services are defined as any persistent process that is used to interact with the satellite. Services rarely make decisions, but will allow the user to accomplish typical flight software tasks such as telemetry storage, file management, shell access, hardware interaction, etc. 

- :doc:`Kubos Services <services/index>`

Mission Applications
~~~~~~~~~~~~~~~~~~~~

Mission applications are anything that governs the behavior of the satellite. Some common examples are deployment, housekeeping, and telemetry beaconing. Basically anything that you want the satellite to do autonomously goes into this category.

- :doc:`Mission Applications <what-is-a-mission-application>`


Typical Mission Architecture
----------------------------

.. figure:: images/mission_diagram.png
    :align: center

In the above diagram, everything in blue is typically developed by Kubos, while everything in green would be mission code and written by the user for their specific mission and payload. 

Communication
~~~~~~~~~~~~~

There is internal communication and external communication, all of which is conducted over different types of IP. 

Gateways
^^^^^^^^

Gateways are any and all communication routes to the satellite. This ranges from desktop serial connection to in flight radio link through a ground station. This is mainly just a term to encompass and abstract the notion of the connection to the satellite. 

Communication Service
^^^^^^^^^^^^^^^^^^^^^

The communication service is what integrates the gateway (and the API developed for it) to talk to the mission operation center, Major Tom. 

Hardware Integration
~~~~~~~~~~~~~~~~~~~~

The hardware that is currently supported by KubOS is listed on the :doc:`main docs page <index>`. 

Supported hardware (other than OBCs) is hardware that has both an API and an associated service. 

Hardware Services
^^^^^^^^^^^^^^^^^

Hardware services are GraphQL server endpoints that take in queries and mutations and exercise the hardware API to complete them. 

 - :doc:`Hardware Services <services/hardware-services>`
 - :doc:`GraphQL <services/graphql>`

Hardware APIs
^^^^^^^^^^^^^

Hardware APIs are a two-tier system. The lower tier is specific to the exact piece of hardware and its configuration, and must be written for every piece of hardware integrated. The upper tier accesses the lower tier, and is accessed by the associated Hardware Services. This upper tier is mostly portable between different units of similar purpose (e.g., different brands of battery or ADCS), but sometimes must be augmented. 

 - :doc:`Hardware APIs <apis/index>`

Core Services
~~~~~~~~~~~~~

The Core Services are all the services that provide critical Flight Software capability. Any service that does not interact with hardware or is not specific to a mission falls within this category. Each of these services are discussed in the Services section found :doc:`here <services/core-services>`

Mission Specific Code
~~~~~~~~~~~~~~~~~~~~~

Mission specific code is highlighted in green in the above diagram and refers to anything which is specific to a particular mission. This includes things like the payload service and mission applications. 

Payload Services
^^^^^^^^^^^^^^^^

Payload services should be modeled after hardware services as much as possible, and that is reflected in the given example code. That being said, the payload service is custom for the mission, and can be accomplished any way the payload developer sees fit. 

 - :doc:`Payload Services <services/payload-services>`

Mission Applications
^^^^^^^^^^^^^^^^^^^^

Mission applications, as previously discussed, handle all the onboard decision making. These are, by nature, mission specific, but some of them can be largely reused due to the abstract nature of the hardware integration. These are typically written or adapted by the user. 

 - :doc:`Mission Applications <what-is-a-mission-application>`


Available Languages in KubOS
----------------------------

The primary languages used in KubOS are Rust, Python, and C. 

 - Rust is the primary language for the Services. 
 - Python is used for Mission Applications and some Services. 
 - C is everything else. 

Each language can be used to create projects, services, and applications within KubOS. Other languages can also be easily supported, make sure to `talk to us <https://slack.kubos.com/>`__ if there is another option you'd like to use!