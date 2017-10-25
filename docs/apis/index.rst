Kubos APIs
==========

The Kubos SDK provides a variety of APIs to help with the development of mission software:

  - :doc:`Device Interfaces <device-api/index>` - APIs for devices (ex. radio), built on top of the Kubos HAL
  - :doc:`Kubos Core <kubos-core/index>` - Legacy APIs for supported sensors and devices, built on top of the Kubos HAL
  - :doc:`Kubos HAL <kubos-hal/index>` - Hardware interface abstractions (I2C, SPI, etc)
  - :doc:`Kubos IPC <ipc/index>` - Abstracts interprocess communication for KubOS Linux systems
  - :doc:`Kubos Telemetry <telemetry/index>` - Offers a convenient interface for requesting and delivering 
    telemetry data from system components
    
Some third-party APIs are also included with the Kubos SDK:

  - :doc:`CSP (Cubesat Space Protocol) <libcsp/index>` - A small protocol stack targeting embedded systems
  - :doc:`FreeRTOS <freertos/index>` - The real-time operating system on which KubOS RT is built

.. toctree::
    :caption: APIs
    :hidden:
    
    Device Interfaces <device-api/index>
    Kubos Core <kubos-core/index>
    Kubos HAL <kubos-hal/index>
    Kubos IPC <ipc/index>
    Kubos Telemetry <telemetry/index>
   
    CSP <libcsp/index>
    FreeRTOS <freertos/index>