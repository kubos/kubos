Kubos MSP430F5529 HAL Documentation
===================================

The Kubos-HAL-MSP430F5529 module is the Kubos-HAL implementation for the MSP430F5529 MCU.


.. uml::

   @startuml
   interface Kubos_HAL
   interface Kubos_HAL_MSP430F5529
   interface MSP430F5529_HAL
   
   Kubos_HAL <|-- Kubos_HAL_MSP430F5529
   Kubos_HAL_MSP430F5529 o-- MSP430F5529_HAL
   @enduml

.. toctree::
   :caption: APIs
   :name: msp430f5529-hal-apis

   gpio
   i2c
   spi
   uart