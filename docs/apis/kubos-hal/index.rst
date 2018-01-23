Kubos HAL Documentation
=======================

The Kubos HAL module provides a hardware abstraction layer for the common hardware interfaces
found on cubesats. The interfaces provided span across the different hardware platforms
support by KubOS.

For Linux devices, the `Linux sysfs <https://en.wikipedia.org/wiki/Sysfs>`__ interface 
already provides some abstraction away from the hardware interface. In this instance, the 
Kubos HAL creates further abstraction, removing the need for the user to learn the intricacies 
of Linux system calls.

.. note:: The Kubos Linux HAL is a work in progress. Not all functionality has been implemented yet.

.. uml::

   @startuml
   rectangle "Kubos HAL" as kubos
   rectangle "Kubos Linux HAL" as linux
   rectangle "ISIS-OBC" as iobc
   rectangle "Pumpkin MBM2" as mbm2
   kubos <|-- linux
   linux <|-- iobc
   linux <|-- mbm2
   @enduml

.. toctree::
   :caption: Guides
   :name: hal-guides

   i2c

.. toctree::
   :caption: APIs
   :name: hal-apis

   i2c_api

.. toctree::
   :caption: Platform HALs
   :name:  hal-platforms

   Kubos Linux Devices <kubos-hal-linux/index>

.. toctree::
   :caption: OBC HALs
   :name: hal-obcs

   ISIS iOBC <kubos-hal-iobc/index>