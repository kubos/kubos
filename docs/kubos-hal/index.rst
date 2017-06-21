Kubos HAL Documentation
=======================

.. warning:: The Kubos-HAL functionality is currently only available on Kubos-RT.

The Kubos-HAL module provides a hardware abstraction layer for the common hardware interfaces
found on cubesats. The interfaces provided span across the different hardware platforms
support by KubOS. Currently there is a HAL implemented for Kubos-RT on the STM32F4 and MSP430F5529 platforms.
Each platform-specific HAL implements the same set of functions provided by the top-level Kubos-HAL.

.. uml::

   @startuml
   Kubos_HAL <|-- STM32F4_HAL
   Kubos_HAL <|-- MSP430F5529_HAL
   @enduml

.. note:: The Kubos-Linux platform currently exposes HAL-like functionality through the Linux device files

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

   STM32F4 <kubos-hal-stm32f4/index>
   MSP430F5529 <kubos-hal-msp430f5529/index>

.. toctree::
   :caption: OBC HALs
   :name: hal-obcs

   ISIS iOBC <kubos-hal-iobc/index>