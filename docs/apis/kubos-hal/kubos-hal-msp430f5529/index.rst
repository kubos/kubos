Kubos MSP430F5529 HAL Documentation
===================================

The Kubos-HAL-MSP430F5529 module is the Kubos-HAL implementation for the MSP430F5529 MCU.
This module sits as a proxy layer between the Kubos-HAL interface and a more concrete 
implementation module, msp430f5529-hal.


.. uml::

   @startuml
   interface "Kubos HAL" as kubos
   interface "Kubos HAL MSP430F5529" as hal_msp
   interface "MSP430F5529 HAL" as msp

   kubos <|-- hal_msp
   hal_msp <|-- msp
   @enduml

.. toctree::
   :caption: APIs
   :name: msp430f5529-hal-apis

   gpio
   i2c
   spi
   uart