Kubos APIs
==========

The Kubos SDK provides a variety of APIs to help with the development of mission software:

  - :doc:`Device Interfaces <device-api/index>` - APIs for devices (ex. radio), built on top of the Kubos HAL
  - :doc:`Kubos HAL <kubos-hal/index>` - Hardware interface abstractions (I2C, SPI, etc)
  
For integrating new hardware into the KubOS system and developing new APIs, checkout out our API frameworks:

  - :doc:`API Frameworks <api-frameworks/index>`

Some third-party APIs are also included with the Kubos SDK:

  - :doc:`CSP (Cubesat Space Protocol) <libcsp/index>` - A small protocol stack targeting embedded systems

.. toctree::
    :caption: APIs
    :hidden:
    
    Device Interfaces <device-api/index>
    API Frameworks <api-frameworks/index>
    Kubos HAL <kubos-hal/index>

    CSP <libcsp/index>