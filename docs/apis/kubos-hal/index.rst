Kubos HAL Documentation
=======================

The Kubos HAL module provides a hardware abstraction layer for the common hardware interfaces
found on cubesats. The interfaces provided span across the different hardware platforms
support by KubOS.

Currently there is a HAL implemented for KubOS RT on the STM32F4 and MSP430F5529 platforms.
Each platform-specific HAL implements the same set of functions provided by the top-level Kubos-HAL.

For KubOS Linux devices, the `Linux sysfs <https://en.wikipedia.org/wiki/Sysfs>`__ interface 
already provides some abstraction away from the hardware interface. In this instance, the 
Kubos HAL creates further abstraction, removing the need for the user to learn the intricacies 
of Linux system calls.

.. note:: The KubOS Linux HAL is a work in progress. Not all functionality has been implemented yet.

.. uml::

   @startuml
   rectangle "Kubos HAL" as kubos
   rectangle "KubOS Linux HAL" as linux
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
   spi
   uart

.. toctree::
   :caption: APIs
   :name: hal-apis

   gpio
   i2c_api
   sdio
   spi_api
   uart_api

.. toctree::
   :caption: Platform HALs
   :name:  hal-platforms

   KubOS Linux Devices <kubos-hal-linux/index>

.. toctree::
   :caption: OBC HALs
   :name: hal-obcs

   ISIS iOBC <kubos-hal-iobc/index>